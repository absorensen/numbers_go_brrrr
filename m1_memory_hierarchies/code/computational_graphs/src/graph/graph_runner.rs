use std::collections::HashMap;

use crate::shared::tensor2d::Tensor2D;

use super::graph_validation::validate_graph_operators;
use super::nodes::{self, Node, NodeOperator};

use crate::shared::graph_operators::GraphOperator;
use crate::shared::graph_operators::GraphOperator::*;

pub struct GraphRunner {
    graph_operators_are_valid: bool,
    nodes: Vec<Node>,
    nodes_are_valid: bool,
    data_buffers: Vec<Tensor2D>,
    data_buffers_are_valid: bool,
    fuse_operators: bool,
}

impl GraphRunner {
    pub fn new(graph_operators: &Vec<GraphOperator>, fuse_operators: bool) -> Self {
        let mut runner: GraphRunner = GraphRunner {
            graph_operators_are_valid: false,
            nodes: Vec::<Node>::new(),
            nodes_are_valid: false,
            data_buffers: Vec::<Tensor2D>::new(),
            data_buffers_are_valid: false,
            fuse_operators,
        };
        runner.graph_operators_are_valid = validate_graph_operators(graph_operators);

        runner.compute_nodes(graph_operators, runner.fuse_operators);
        runner.data_buffers_are_valid = true;

        runner
    }

    fn get_new_key(operator_counts: &mut HashMap<NodeOperator, u32>, key: &NodeOperator) -> String {
        let error_message: &str = "Failed to get value from hash map in graph_runner::get_new_key";
        let value: &mut u32 = operator_counts
            .get_mut(key)
            .unwrap_or_else(|| panic!("{}", error_message));
        let index: u32 = *value;
        *value = index + 1;
        format!("{:?}_{}", key, index)
    }

    fn verify_previous_node_and_get_index(nodes: &Vec<Node>, key: &NodeOperator) -> usize {
        let previous_node: &Node = &nodes[nodes.len() - 1];
        match previous_node.operator {
            NodeOperator::Transfer => {}
            _ => {
                panic!("Invalid node operator! Was expecting DeviceToDevice before {:?}, but found {:?}", *key, previous_node.operator);
            }
        }

        if previous_node.buffer_indices.len() == 1 {
            previous_node.buffer_indices[0]
        } else {
            panic!("There was more than one buffer index described in a DeviceToDevice operator. This is currently invalid behavior!");
        }
    }

    // This is made a lot more complicated by reusing buffers
    // If each node owned its own buffers with no reusage
    // We would need to keep less track of buffers
    fn compute_nodes(&mut self, graph_operators: &Vec<GraphOperator>, fuse_operators: bool) {
        if !self.graph_operators_are_valid {
            panic!("Invalid graph being sent to compute_nodes!");
        }

        let mut operator_counts: HashMap<NodeOperator, u32> = HashMap::<NodeOperator, u32>::new();
        operator_counts.insert(NodeOperator::Input, 0);
        operator_counts.insert(NodeOperator::Output, 0);
        operator_counts.insert(NodeOperator::Transfer, 0);
        operator_counts.insert(NodeOperator::Linear, 0);
        operator_counts.insert(NodeOperator::ReLU, 0);
        operator_counts.insert(NodeOperator::Softmax, 0);
        operator_counts.insert(NodeOperator::LinearReLU, 0);
        operator_counts.insert(NodeOperator::LinearReLUSoftmax, 0);

        let mut operator_index: usize = 0;
        while operator_index < graph_operators.len() {
            let operator: &GraphOperator = &graph_operators[operator_index];

            match operator {
                Empty => {}
                // Maybe put a device to device split in here for simpler code in the other operators
                HostToDevice { input } => {
                    let key: NodeOperator = NodeOperator::Input;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(input.clone());
                    let buffer_indices: Vec<usize> = vec![self.data_buffers.len() - 1];

                    let node: Node = Node::new(new_key, key, buffer_indices.clone());
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let node: Node = Node::new(new_key, NodeOperator::Transfer, buffer_indices);
                    self.nodes.push(node);
                }
                DeviceToHost => {
                    let key: NodeOperator = NodeOperator::Output;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let buffer_indices: Vec<usize> = vec![input_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                Linear { weights, bias } => {
                    let mut key: NodeOperator = NodeOperator::Linear;

                    if fuse_operators {
                        if let ReLU = graph_operators[operator_index + 1] {
                            match graph_operators[operator_index + 2] {
                                Softmax => {
                                    key = NodeOperator::LinearReLUSoftmax;
                                    operator_index += 2;
                                }
                                _ => {
                                    key = NodeOperator::LinearReLU;
                                    operator_index += 1;
                                }
                            }
                        }
                    }

                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(weights.clone());
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(bias.clone());
                    let bias_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers
                        .push(Tensor2D::new(0.0, bias.row_count, bias.column_count));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: Node = Node::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                // Note this is not inplace
                ReLU => {
                    let key: NodeOperator = NodeOperator::ReLU;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let input_buffer: &Tensor2D = &self.data_buffers[input_index];
                    self.data_buffers.push(Tensor2D::new(
                        0.0,
                        input_buffer.row_count,
                        input_buffer.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> = vec![input_index, output_index];
                    let node: Node = Node::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                Softmax => {
                    let key: NodeOperator = NodeOperator::Softmax;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let input_buffer: &Tensor2D = &self.data_buffers[input_index];
                    // This should be more flexible, but Softmax always outputs a flattened vector
                    self.data_buffers.push(Tensor2D::new(
                        0.0,
                        input_buffer.row_count,
                        input_buffer.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> = vec![input_index, output_index];
                    let node: Node = Node::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                LinearReLUFused { weights, bias } => {
                    let key: NodeOperator = NodeOperator::LinearReLU;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(weights.clone());
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(bias.clone());
                    let bias_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers
                        .push(Tensor2D::new(0.0, bias.row_count, bias.column_count));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: Node = Node::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                LinearReLUSoftmaxFused { weights, bias } => {
                    let key: NodeOperator = NodeOperator::LinearReLUSoftmax;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(weights.clone());
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(bias.clone());
                    let bias_index: usize = self.data_buffers.len() - 1;

                    // This should be more flexible, but Softmax always outputs a flattened vector
                    self.data_buffers
                        .push(Tensor2D::new(0.0, bias.row_count, bias.column_count));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: Node = Node::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperator = NodeOperator::Transfer;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: Node = Node::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
            }

            operator_index += 1;
        }
        self.graph_operators_are_valid = true;
    }

    // In a more correct system, not meant for teaching/learning
    // we might find the correct data buffers here and pass the correct
    // buffers explicitly to the functions. Or at the very least
    // enforce more correctness in the data passed along to
    // the CPUNodeOperator functions.
    fn submit_operator_commands(node_vector: &Vec<Node>, data_buffers: &mut [Tensor2D]) {
        for node in node_vector {
            match node.operator {
                NodeOperator::Input => {}
                NodeOperator::Output => {}
                NodeOperator::Transfer => {}
                NodeOperator::Linear => {
                    nodes::linear(node, data_buffers);
                }
                NodeOperator::ReLU => {
                    nodes::relu(node, data_buffers);
                }
                NodeOperator::Softmax => {
                    nodes::softmax(node, data_buffers);
                }
                NodeOperator::LinearReLU => {
                    nodes::linear_relu(node, data_buffers);
                }
                NodeOperator::LinearReLUSoftmax => {
                    nodes::linear_relu_softmax(node, data_buffers);
                }
            }
        }
    }

    pub fn run(&mut self) -> Tensor2D {
        if !self.graph_operators_are_valid {
            panic!(
                "Tried to run the a CPU computational graph with an unvalidated graph_operators!"
            );
        }

        if !self.data_buffers_are_valid {
            panic!("Tried to run the a CPU computational graph with an unvalidated data_buffers!");
        }

        Self::submit_operator_commands(&self.nodes, &mut self.data_buffers);

        // Based on the restrictions we have put on our graph, the last node has to be
        // the output.
        self.data_buffers[self.nodes[self.nodes.len() - 2].buffer_indices[0]].clone()
    }
}
