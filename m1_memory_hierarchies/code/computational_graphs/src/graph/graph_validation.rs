use crate::shared::graph_operators::GraphOperator;
use crate::shared::graph_operators::GraphOperator::*;
use crate::shared::tensor2d::Tensor2D;

pub fn linear_dimension_check(input: &Tensor2D, weights: &Tensor2D, bias: &Tensor2D) {
    assert!(
        0 < input.row_count,
        "\ninput.row_count must be larger than 0. Current value: {}.",
        input.row_count
    );

    assert!(
        0 < input.column_count,
        "\ninput.column_count must be larger than 0. Current value: {}.",
        input.column_count
    );

    assert!(
        0 < weights.row_count,
        "\nweights.row_count must be larger than 0. Current value: {}.",
        weights.row_count
    );

    assert!(
        0 < weights.column_count,
        "\nweights.column_count must be larger than 0. Current value: {}.",
        weights.column_count
    );

    assert!(
        0 < bias.row_count,
        "\nbias.row_count must be larger than 0. Current value: {}.",
        bias.row_count
    );

    assert!(
        0 < bias.column_count,
        "\nbias.column_count must be larger than 0. Current value: {}.",
        bias.column_count
    );

    assert_eq!(
        input.column_count,
        weights.row_count,
        "\nMismatch - input.column_count & weights.row_count\ninput - rows: {} columns: {}.\n weights - rows: {} columns: {}.", 
        input.row_count,
        input.column_count,
        weights.row_count,
        weights.column_count
    );
}

// Every dimension is legal in this operator, it is up to the other operators to reject
fn validate_host_to_device(current_index: usize, graph: &[GraphOperator]) -> bool {
    if let HostToDevice { input: _ } = &graph[current_index] {
        if current_index != 0 {
            println!("Something went wrong in validate_host_to_device. The HostToDevice operator was not at the beginning of the graph.");
            return false;
        }
    } else {
        println!("Something went wrong in validate_host_to_device. Current operator was not HostToDevice");
        return false;
    }

    true
}

// Every dimension is legal in this operator, it is up to the other operators to reject.
// Normally this wouldn't be, but we have elected to overwrite the existing data whenever
// an output is transferred back to the host.
fn validate_device_to_host(current_index: usize, graph: &Vec<GraphOperator>) -> bool {
    if let DeviceToHost = &graph[current_index] {
        if current_index != (graph.len() - 1) {
            println!("Something went wrong in validate_device_to_host. Current operator was not DeviceToHost");
            return false;
        }
    } else {
        println!("Something went wrong in validate_device_to_host. Current operator was not DeviceToHost");
        return false;
    }

    true
}

fn validate_linear_dimensions(
    current_index: usize,
    graph: &[GraphOperator],
    current_weights: &Tensor2D,
    current_bias: &Tensor2D,
) -> bool {
    // Search for nearest dimension dictating operation
    for predecessor_index in (0..current_index).rev() {
        match &graph[predecessor_index] {
            HostToDevice { input } => {
                linear_dimension_check(input, current_weights, current_bias);
                return true;
            }
            Linear { weights: _, bias } => {
                linear_dimension_check(bias, current_weights, current_bias);
                return true;
            }
            LinearReLUFused { weights: _, bias } => {
                linear_dimension_check(bias, current_weights, current_bias);
                return true;
            }
            LinearReLUSoftmaxFused { weights: _, bias } => {
                linear_dimension_check(bias, current_weights, current_bias);
                return true;
            }
            DeviceToHost => {
                panic!("Found a DeviceToHost node before a linear layer node. This wasn't part of the contrived example!");
            }
            Empty => {
                panic!("Found an Empty node before a linear layer node. This wasn't part of the contrived example!");
            }
            _ => {
                //Predecessor operator was probably ReLU or Softmax
            }
        }
    }

    true
}

fn validate_relu(current_index: usize, graph: &[GraphOperator]) -> bool {
    if let ReLU {} = &graph[current_index] {
    } else {
        println!("Something went wrong in validate_relu. Current operator was not HostToDevice");
        return false;
    }

    true
}

fn validate_softmax(current_index: usize, graph: &[GraphOperator]) -> bool {
    if let Softmax {} = &graph[current_index] {
    } else {
        println!("Something went wrong in validate_softmax. Current operator was not HostToDevice");
        return false;
    }

    true
}

// For our contrived example, for a graph to be valid it has to begin
// with HostToDevice and end with DeviceToHost, perhaps later
// we will support running the same input in a loop, or
// running a graph with new input every time.
fn validate_transfers(graph: &Vec<GraphOperator>) -> bool {
    let mut found_valid_host_to_device: bool = false;
    let mut found_valid_device_to_host: bool = false;

    for index in 0..graph.len() {
        let operator: &GraphOperator = &graph[index];
        match operator {
            HostToDevice { input } => {
                if index == 0 {
                    if 0 < input.row_count && 0 < input.column_count {
                        found_valid_host_to_device = true;
                    }
                } else {
                    return false;
                }
            }
            DeviceToHost => {
                if index == graph.len() - 1 {
                    found_valid_device_to_host = true;
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    found_valid_device_to_host && found_valid_host_to_device
}

// Just for learning purposes the only real requirements we will have will be
// matching dimensions and each graph beginning with a transfer to device
// and ending with a transfer from device
// All validation is retrospective, each operator will look for valid predecessors.
pub fn validate_graph_operators(graph: &Vec<GraphOperator>) -> bool {
    let graph_length: usize = graph.len();
    let mut graph_is_validated: bool = true;

    graph_is_validated = graph_is_validated && validate_transfers(graph);

    // Scanning graph for valid sizes
    for current_index in 0..graph_length {
        let current: &GraphOperator = &graph[current_index];

        let valid_operator: bool = match current {
            GraphOperator::Empty => {
                continue;
            }
            GraphOperator::HostToDevice { input: _ } => {
                validate_host_to_device(current_index, graph)
            }
            GraphOperator::DeviceToHost => validate_device_to_host(current_index, graph),
            GraphOperator::Linear { weights, bias } => {
                validate_linear_dimensions(current_index, graph, weights, bias)
            }
            GraphOperator::ReLU => validate_relu(current_index, graph),
            GraphOperator::Softmax => validate_softmax(current_index, graph),
            GraphOperator::LinearReLUFused { weights, bias } => {
                validate_linear_dimensions(current_index, graph, weights, bias)
            }
            GraphOperator::LinearReLUSoftmaxFused { weights, bias } => {
                validate_linear_dimensions(current_index, graph, weights, bias)
            }
        };
        graph_is_validated = graph_is_validated && valid_operator;
    }

    graph_is_validated
}
