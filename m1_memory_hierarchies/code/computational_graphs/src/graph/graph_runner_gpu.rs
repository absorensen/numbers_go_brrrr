use std::collections::HashMap;

use wgpu::{BufferSlice, CommandEncoder, ComputePipeline, ShaderModule};

use crate::shared::graph_operators::GraphOperator::*;
use crate::shared::tensor2d::Tensor2D;
use crate::shared::tensor2d_gpu::Tensor2DGPU;
use crate::shared::{gpu_utilities::GPUHandles, graph_operators::GraphOperator};

use super::graph_validation::validate_graph_operators;
use super::nodes_gpu::{self, NodeGPU, NodeOperatorGPU};

pub struct GraphRunnerGPU {
    graph_operators_are_valid: bool,
    nodes: Vec<NodeGPU>,
    nodes_are_valid: bool,
    data_buffers: Vec<Tensor2DGPU>,
    data_buffers_are_valid: bool,
    fuse_operators: bool,
    use_cache: bool,
    shader_cache: HashMap<String, ShaderModule>,
    pipeline_cache: HashMap<String, ComputePipeline>,
}

impl GraphRunnerGPU {
    pub fn new(
        gpu_handles: &GPUHandles,
        graph_operators: &Vec<GraphOperator>,
        fuse_operators: bool,
        use_cache: bool,
    ) -> Self {
        let mut shader_cache: HashMap<String, ShaderModule> =
            HashMap::<String, ShaderModule>::new();
        let mut pipeline_cache: HashMap<String, ComputePipeline> =
            HashMap::<String, ComputePipeline>::new();

        if use_cache {
            Self::populate_caches(
                gpu_handles,
                fuse_operators,
                &mut shader_cache,
                &mut pipeline_cache,
            );
        }

        let mut runner: GraphRunnerGPU = GraphRunnerGPU {
            graph_operators_are_valid: false,
            nodes: Vec::<NodeGPU>::new(),
            nodes_are_valid: false,
            data_buffers: Vec::<Tensor2DGPU>::new(),
            data_buffers_are_valid: false,
            fuse_operators,
            use_cache,
            shader_cache,
            pipeline_cache,
        };
        runner.graph_operators_are_valid = validate_graph_operators(graph_operators);

        runner.compute_nodes(gpu_handles, graph_operators, fuse_operators);
        runner
    }

    fn populate_caches(
        gpu_handles: &GPUHandles,
        fuse_operators: bool,
        shader_cache: &mut HashMap<String, ShaderModule>,
        pipeline_cache: &mut HashMap<String, ComputePipeline>,
    ) {
        //Linear,
        nodes_gpu::build_linear_elements(gpu_handles, shader_cache, pipeline_cache, false);

        //ReLU,
        nodes_gpu::build_relu_elements(gpu_handles, shader_cache, pipeline_cache);

        //Softmax,
        nodes_gpu::build_softmax_elements(gpu_handles, shader_cache, pipeline_cache);

        if fuse_operators {
            //LinearReLU,
            nodes_gpu::build_linear_elements(gpu_handles, shader_cache, pipeline_cache, true);
        }
    }

    fn get_new_key(
        operator_counts: &mut HashMap<NodeOperatorGPU, u32>,
        key: &NodeOperatorGPU,
    ) -> String {
        let error_message: &str = "Failed to get value from hash map in graph_runner::get_new_key";
        let value: &mut u32 = operator_counts
            .get_mut(key)
            .unwrap_or_else(|| panic!("{}", error_message));
        let index: u32 = *value;
        *value = index + 1;
        format!("{:?}_{}", key, index)
    }

    fn verify_previous_node_and_get_index(nodes: &Vec<NodeGPU>, key: &NodeOperatorGPU) -> usize {
        let previous_node: &NodeGPU = &nodes[nodes.len() - 1];
        match previous_node.operator {
            NodeOperatorGPU::DeviceToDevice => {}
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
    fn compute_nodes(
        &mut self,
        gpu_handles: &GPUHandles,
        graph_operators: &Vec<GraphOperator>,
        fuse_operators: bool,
    ) {
        if !self.graph_operators_are_valid {
            panic!("Invalid graph being sent to compute_nodes!");
        }

        let mut operator_counts: HashMap<NodeOperatorGPU, u32> =
            HashMap::<NodeOperatorGPU, u32>::new();
        operator_counts.insert(NodeOperatorGPU::HostToDevice, 0);
        operator_counts.insert(NodeOperatorGPU::DeviceToHost, 0);
        operator_counts.insert(NodeOperatorGPU::DeviceToDevice, 0);
        operator_counts.insert(NodeOperatorGPU::Linear, 0);
        operator_counts.insert(NodeOperatorGPU::ReLU, 0);
        operator_counts.insert(NodeOperatorGPU::Softmax, 0);
        operator_counts.insert(NodeOperatorGPU::LinearReLU, 0);
        operator_counts.insert(NodeOperatorGPU::LinearReLUSoftmax, 0);

        let mut operator_index: usize = 0;
        while operator_index < graph_operators.len() {
            let operator: &GraphOperator = &graph_operators[operator_index];

            match operator {
                Empty => {}
                // Maybe put a device to device split in here for simpler code in the other operators
                HostToDevice { input } => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::HostToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "input"),
                        input,
                    ));
                    let buffer_indices: Vec<usize> = vec![self.data_buffers.len() - 1];

                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices.clone());
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                DeviceToHost => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToHost;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let buffer_indices: Vec<usize> = vec![input_index];
                    let node: NodeGPU =
                        NodeGPU::new(new_key, NodeOperatorGPU::DeviceToHost, buffer_indices);
                    self.nodes.push(node);
                }
                Linear { weights, bias } => {
                    let mut key: NodeOperatorGPU = NodeOperatorGPU::Linear;

                    if fuse_operators {
                        if let ReLU = graph_operators[operator_index + 1] {
                            match graph_operators[operator_index + 2] {
                                Softmax => {
                                    key = NodeOperatorGPU::LinearReLUSoftmax;
                                    operator_index += 2;
                                }
                                _ => {
                                    key = NodeOperatorGPU::LinearReLU;
                                    operator_index += 1;
                                }
                            }
                        }
                    }

                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "weights"),
                        weights,
                    ));
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "bias"),
                        bias,
                    ));
                    let bias_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(Tensor2DGPU::new(
                        gpu_handles,
                        &format!("{}_{}", new_key, "output"),
                        0.0,
                        bias.row_count,
                        bias.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: NodeGPU = NodeGPU::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                // Note this is not inplace
                ReLU => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::ReLU;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let input_buffer: &Tensor2DGPU = &self.data_buffers[input_index];
                    self.data_buffers.push(Tensor2DGPU::new(
                        gpu_handles,
                        &format!("{}_{}", new_key, "output"),
                        0.0,
                        input_buffer.row_count,
                        input_buffer.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> = vec![input_index, output_index];
                    let node: NodeGPU = NodeGPU::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                Softmax => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::Softmax;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);

                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    let input_buffer: &Tensor2DGPU = &self.data_buffers[input_index];
                    // This should be more flexible, but Softmax always outputs a flattened vector
                    self.data_buffers.push(Tensor2DGPU::new(
                        gpu_handles,
                        &format!("{}_{}", new_key, "output"),
                        0.0,
                        input_buffer.row_count * input_buffer.column_count,
                        1,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> = vec![input_index, output_index];
                    let node: NodeGPU = NodeGPU::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                LinearReLUFused { weights, bias } => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::LinearReLU;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "weights"),
                        weights,
                    ));
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "bias"),
                        bias,
                    ));
                    let bias_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(Tensor2DGPU::new(
                        gpu_handles,
                        &format!("{}_{}", new_key, "output"),
                        0.0,
                        bias.row_count,
                        bias.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: NodeGPU = NodeGPU::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
                LinearReLUSoftmaxFused { weights, bias } => {
                    let key: NodeOperatorGPU = NodeOperatorGPU::LinearReLUSoftmax;
                    let input_index: usize =
                        Self::verify_previous_node_and_get_index(&self.nodes, &key);
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "weights"),
                        weights,
                    ));
                    let weights_index: usize = self.data_buffers.len() - 1;

                    self.data_buffers.push(Tensor2DGPU::from_tensor2d(
                        gpu_handles,
                        &format!("{}_{}", new_key, "bias"),
                        bias,
                    ));
                    let bias_index: usize = self.data_buffers.len() - 1;

                    // This should be more flexible, but Softmax always outputs a flattened vector
                    self.data_buffers.push(Tensor2DGPU::new(
                        gpu_handles,
                        &format!("{}_{}", new_key, "output"),
                        0.0,
                        bias.row_count,
                        bias.column_count,
                    ));
                    let output_index: usize = self.data_buffers.len() - 1;

                    let buffer_indices: Vec<usize> =
                        vec![input_index, weights_index, bias_index, output_index];
                    let node: NodeGPU = NodeGPU::new(new_key.clone(), key, buffer_indices);
                    self.nodes.push(node);

                    let key: NodeOperatorGPU = NodeOperatorGPU::DeviceToDevice;
                    let new_key: String = Self::get_new_key(&mut operator_counts, &key);
                    let buffer_indices: Vec<usize> = vec![output_index];
                    let node: NodeGPU = NodeGPU::new(new_key, key, buffer_indices);
                    self.nodes.push(node);
                }
            }

            operator_index += 1;
        }
        self.graph_operators_are_valid = true;
    }

    fn submit_operator_commands(
        gpu_handles: &GPUHandles,
        use_cache: bool,
        shader_cache: &HashMap<String, ShaderModule>,
        pipeline_cache: &HashMap<String, ComputePipeline>,
        node_vector: &[NodeGPU],
        data_buffers: &[Tensor2DGPU],
        encoder: &mut CommandEncoder,
    ) {
        for node in node_vector {
            match node.operator {
                NodeOperatorGPU::HostToDevice => {
                    // The graph runner handles transfers itself
                }
                NodeOperatorGPU::DeviceToHost => {
                    // The graph runner handles transfers itself
                }
                NodeOperatorGPU::DeviceToDevice => {
                    // The graph runner handles transfers itself
                }
                NodeOperatorGPU::Linear => {
                    nodes_gpu::linear(
                        gpu_handles,
                        use_cache,
                        shader_cache,
                        pipeline_cache,
                        node,
                        data_buffers,
                        encoder,
                        false,
                    );
                }
                NodeOperatorGPU::ReLU => {
                    nodes_gpu::relu(
                        gpu_handles,
                        use_cache,
                        shader_cache,
                        pipeline_cache,
                        node,
                        data_buffers,
                        encoder,
                    );
                }
                NodeOperatorGPU::Softmax => {
                    nodes_gpu::softmax(
                        gpu_handles,
                        use_cache,
                        shader_cache,
                        pipeline_cache,
                        node,
                        data_buffers,
                        encoder,
                    );
                }
                NodeOperatorGPU::LinearReLU => {
                    nodes_gpu::linear(
                        gpu_handles,
                        use_cache,
                        shader_cache,
                        pipeline_cache,
                        node,
                        data_buffers,
                        encoder,
                        true,
                    );
                }
                NodeOperatorGPU::LinearReLUSoftmax => {
                    nodes_gpu::linear_relu_softmax(
                        gpu_handles,
                        use_cache,
                        shader_cache,
                        pipeline_cache,
                        node,
                        data_buffers,
                        encoder,
                    );
                }
            }
        }
    }

    fn submit_operations(&mut self, gpu_handles: &GPUHandles) {
        let mut encoder: CommandEncoder = gpu_handles
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            // Submit operator commands to run with the buffers
            Self::submit_operator_commands(
                gpu_handles,
                self.use_cache,
                &self.shader_cache,
                &self.pipeline_cache,
                &self.nodes,
                &self.data_buffers,
                &mut encoder,
            );

            // Submit commands
            gpu_handles.queue.submit(Some(encoder.finish()));
        }
    }

    async fn retrieve_output(&mut self, gpu_handles: &GPUHandles) -> Tensor2D {
        // Transfer result back
        let last_index: usize = self.data_buffers.len() - 1;
        let output: &mut Tensor2DGPU = &mut self.data_buffers[last_index];
        let buffer_slice: BufferSlice = output.staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        output.receiver = Some(receiver);

        gpu_handles.device.poll(wgpu::Maintain::Wait);

        if output.live_data_on_device {
            output.retrieve_results().await;
        }

        output.data.clone()
    }

    pub async fn run(&mut self, gpu_handles: &GPUHandles, iteration_count: usize) -> Tensor2D {
        if !self.graph_operators_are_valid {
            panic!("Failed to validate the computational graph!");
        }

        for _ in 0..iteration_count {
            self.submit_operations(gpu_handles);
        }
        self.retrieve_output(gpu_handles).await
    }
}
