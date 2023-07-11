pub fn gentle_lifetimes() {
    // Lifetimes are omnipresent in Rust, it's just not
    // THAT often you are asked to write them yourself.
    // Lifetimes can be a bit of a head-mangler, but 
    // it's basically this - the compiler needs to know
    // how long variables are supposed to be valid.
    // Not just for knowing when to drop variables
    // but also for guaranteeing that references are 
    // valid. Rc and Arc guarantee that the data stays
    // alive as long as it is needed. Box guarantees that
    // the data it holds survives as long as the box. 
    // But what about a reference?
    
    // If you do the following:
    let variable: f32 = 3.50;
    let variable_ref: &f32 = &variable;
    // it is easy for the compiler to see for
    // how long variable_ref should be valid.

    // But if the scope is different, it can be hard
    // to reason, and not just guess, about how long
    // a reference should survive. In that case it 
    // is necessary to annotate a lifetime. 
    
    fn get_reference<'a>(data: &'a u32) -> &'a u32 {
        &data
    }
    // A variable X may have a lifetime of 'a,
    // but the reference to X, Y, we could annotate
    // with the lifetime 'b, as well as the view of
    // X we have at the time. 'a and 'b are completely
    // symbolic. In this example, we just guarantee that
    // X will live AT LEAST as long as Y, but 
    // it is completely ok for X to live even long.
    // We are just making minimal assurances.

    // One more lifetime detail - 'static.
    // 'static means that the lifetime is the
    // entirety of the program. This is especially
    // seen for hardcoded &str types.
    // For example when you hardcode an error message
    // for a .expect("My error message"). That message 
    // is hardcoded into the binary for the program,
    // and it is then legal to make any reference to
    // that string as we have a minimal guarantee
    // of eternity (the programs lifetime).

    // Read along further for a more complex example.
    // You don't really need to as the guide tries to
    // minimize explicit lifetimes
    references_example();

    // Read more about lifetimes here: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
}

// The two vectors don't really matter much.
// For a fully functional example see 
// m1_memory_hierarchies/code/computational_graphs/
// src/graph/nodes.rs::sorted_mutable_references
fn references_example() {
    let size: usize = 10;
    let tensors_count: usize = 10;
    let nodes_count: usize = tensors_count / 2;

    let mut tensors: Vec<Tensor2D> = Vec::<Tensor2D>::new();
    for _ in 0..tensors_count {
        let data: Vec<f32> = (0..(size * size)).into_iter().map(|x| (x as f32) * 0.1).collect();
        tensors.push(
            Tensor2D { 
                data, 
                row_count: size, 
                column_count: size 
            }
        );
    }

    let mut nodes: Vec<Node> = Vec::<Node>::new();
    for index in 0..nodes_count {
        nodes.push(
            Node{
                name: "input".to_string(), 
                operator: NodeOperator::Transfer, 
                buffer_indices: vec![nodes_count * 2]
            }
        );
    }

    // This is difficult and necessary to ensure that we don't have
    // multiple mutable references to the same data.
    let references: Vec<(usize, &mut Tensor2D)> = 
        sorted_mutable_references(&nodes[2], &mut tensors);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum NodeOperator {
    Input,
    Output,
    Transfer,
    LinearLayer,
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

#[derive(Clone, Debug, Default)]
pub struct Tensor2D {
    pub data: Vec<f32>,
    pub row_count: usize,
    pub column_count: usize,
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

    // Make sure the new vector is preallocated to be big enough.
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
                // Break 
                break;
            }
        }
    }

    // Resort the references by their original ordering to match it to the correct buffer name
    references.sort_by(|a, b| a.0.cmp(&b.0));
    references
}