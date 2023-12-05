use std::vec::Drain;

use crate::shared::tensor2d::Tensor2D;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum NodeOperator {
    Input,
    Output,
    Transfer,
    Linear,
    ReLU,
    Softmax,
    LinearReLU,
    LinearReLUSoftmax,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub operator: NodeOperator,
    pub buffer_indices: Vec<usize>,
}

impl Node {
    pub fn new(name: String, operator: NodeOperator, buffer_indices: Vec<usize>) -> Self {
        Node {
            name,
            operator,
            buffer_indices,
        }
    }
}

// Due to Rust's borrowing rules, this is slightly complicated as we need
// multiple shared references and mutable references. If we guaranteed
// that buffer_indices[0..N] were sequential we could do with just one split
// but we can't, so we have to guarantee the borrow checker that we don't have overlaps.
// The drain is not the result from this function as it needs to have the references vector
// survive.
fn sorted_mutable_references<'a>(
    node: &'a Node,
    data_buffers: &'a mut [Tensor2D],
) -> Vec<(usize, &'a mut Tensor2D)> {
    let indices: Vec<(usize, usize)> = node
        .buffer_indices
        .iter()
        .enumerate()
        .map(|data| (data.0, *data.1))
        .collect();

    // Only engage with the relevant buffers
    let filtered_buffers: Vec<(usize, &mut Tensor2D)> = data_buffers
        .iter_mut()
        .enumerate()
        .filter(|pair| node.buffer_indices.contains(&pair.0))
        .collect();
    let mut references: Vec<(usize, &mut Tensor2D)> =
        Vec::<(usize, &mut Tensor2D)>::with_capacity(indices.len());

    // We know filtered_buffers only contain the buffers that
    // are relevant to us. Now we just have to pair the buffers with the
    // correct ordering and then sort, for the output to be drained correctly
    // afterwards.
    for (data_index, buffer) in filtered_buffers {
        for (buffer_enumeration, buffer_index) in &indices {
            if data_index == *buffer_index {
                references.push((*buffer_enumeration, buffer));
                break;
            }
        }
    }

    // Resort the references by their original ordering to match it to the correct buffer name
    references.sort_by(|a, b| a.0.cmp(&b.0));
    references
}

pub fn linear(node: &Node, data_buffers: &mut [Tensor2D]) {
    if node.buffer_indices.len() != 4 {
        panic!(
            "cpu_nodes::linear function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let mut references: Vec<(usize, &mut Tensor2D)> = sorted_mutable_references(node, data_buffers);
    let mut drain: Drain<(usize, &mut Tensor2D)> = references.drain(0..references.len());

    let input: &Tensor2D = drain.next().unwrap().1;
    let weights: &Tensor2D = drain.next().unwrap().1;
    let bias: &Tensor2D = drain.next().unwrap().1;
    let output: &mut Tensor2D = drain.next().unwrap().1;

    Tensor2D::linear_optimized(input, weights, bias, output)
}

pub fn relu(node: &Node, data_buffers: &mut [Tensor2D]) {
    if node.buffer_indices.len() != 2 {
        panic!(
            "nodes::relu function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let mut references: Vec<(usize, &mut Tensor2D)> = sorted_mutable_references(node, data_buffers);
    let mut drain: Drain<(usize, &mut Tensor2D)> = references.drain(0..references.len());

    let input: &Tensor2D = drain.next().unwrap().1;
    let output: &mut Tensor2D = drain.next().unwrap().1;

    // Note that in the immediate CPU version we are using relu_inplace_inline
    // which has better performance and works on directly on the given input, which has to be mutable.
    // Due to the way the graph is currently setup, this isn't implemented for the CPU graph,
    // but it could be a possible optimization. Wink. Wink.
    Tensor2D::relu_preallocated(input, output);
}

pub fn softmax(node: &Node, data_buffers: &mut [Tensor2D]) {
    if node.buffer_indices.len() != 2 {
        panic!(
            "nodes::softmax function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let mut references: Vec<(usize, &mut Tensor2D)> = sorted_mutable_references(node, data_buffers);
    let mut drain: Drain<(usize, &mut Tensor2D)> = references.drain(0..references.len());

    let input: &Tensor2D = drain.next().unwrap().1;
    let output: &mut Tensor2D = drain.next().unwrap().1;

    // Note that in the immediate CPU version we are using softmax_optimized
    // which has better performance and works on directly on the given input, which has to be mutable.
    // Due to the way the graph is currently setup, this isn't implemented for the CPU graph,
    // but it could be a possible optimization. Wink. Wink.
    Tensor2D::softmax_preallocated(input, output);
}

pub fn linear_relu(node: &Node, data_buffers: &mut [Tensor2D]) {
    if node.buffer_indices.len() != 4 {
        panic!(
            "cpu_nodes::linear_relu function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let mut references: Vec<(usize, &mut Tensor2D)> = sorted_mutable_references(node, data_buffers);
    let mut drain: Drain<(usize, &mut Tensor2D)> = references.drain(0..references.len());

    let input: &Tensor2D = drain.next().unwrap().1;
    let weights: &Tensor2D = drain.next().unwrap().1;
    let bias: &Tensor2D = drain.next().unwrap().1;
    let output: &mut Tensor2D = drain.next().unwrap().1;

    Tensor2D::linear_local_accumulation_relu(input, weights, bias, output);
}

pub fn linear_relu_softmax(node: &Node, data_buffers: &mut [Tensor2D]) {
    if node.buffer_indices.len() != 4 {
        panic!(
            "cpu_nodes::linear_relu_softmax function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let mut references: Vec<(usize, &mut Tensor2D)> = sorted_mutable_references(node, data_buffers);
    let mut drain: Drain<(usize, &mut Tensor2D)> = references.drain(0..references.len());

    let input: &Tensor2D = drain.next().unwrap().1;
    let weights: &Tensor2D = drain.next().unwrap().1;
    let bias: &Tensor2D = drain.next().unwrap().1;
    let output: &mut Tensor2D = drain.next().unwrap().1;

    Tensor2D::linear_relu_softmax_fused_fission(input, weights, bias, output);
}
