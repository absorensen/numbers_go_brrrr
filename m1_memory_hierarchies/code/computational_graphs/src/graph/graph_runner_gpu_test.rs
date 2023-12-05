#[cfg(test)]
mod tests {
    use crate::{
        graph::graph_runner_gpu::GraphRunnerGPU,
        shared::{
            gpu_utilities::{initialize_gpu, GPUHandles},
            graph_operators::GraphOperator,
            tensor2d::Tensor2D,
        },
    };

    const ERROR_TOLERANCE: f32 = 0.00001;

    // This is for verification purposes only
    // we don't care about making this fast
    fn subtract_tensors(left: &Tensor2D, right: &Tensor2D) -> Tensor2D {
        debug_assert_eq!(
            left.row_count, right.row_count,
            "\ninput - rows: {} columns: {}.\n out - rows: {} columns: {}.",
            left.row_count, left.column_count, right.row_count, right.column_count
        );
        debug_assert_eq!(
            left.column_count, right.column_count,
            "\nweights - rows: {} columns: {}.\n out - rows: {} columns: {}.",
            left.row_count, left.column_count, right.row_count, right.column_count
        );

        let mut out: Tensor2D = Tensor2D::new(0.0, left.row_count, left.column_count);

        for index in 0..(left.row_count * left.column_count) {
            out.data[index] = left.data[index] - right.data[index];
        }

        out
    }

    #[test]
    fn linear() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::linear() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension_input in 1..outer_dimension_range {
            for outer_dimension_weights in 1..outer_dimension_range {
                for inner_dimension in 1..inner_dimension_range {
                    let input: Tensor2D =
                        Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
                    let weights: Tensor2D =
                        Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
                    let bias: Tensor2D =
                        Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

                    let output_cpu: Tensor2D = Tensor2D::linear(&input, &weights, &bias);

                    let graph_operators: Vec<GraphOperator> = vec![
                        GraphOperator::HostToDevice { input },
                        GraphOperator::Linear { weights, bias },
                        GraphOperator::DeviceToHost,
                    ];

                    let fuse_operators: bool = false;
                    let cache_elements: bool = false;
                    let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                        &gpu_handles,
                        &graph_operators,
                        fuse_operators,
                        cache_elements,
                    );
                    let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                    let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
                    println!("{:?}", difference);
                    println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
                }
            }
        }
    }

    #[test]
    fn relu() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::relu() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let input: Tensor2D = Tensor2D::new(0.5, outer_dimension, inner_dimension);

                let output_cpu: Tensor2D = Tensor2D::relu(&input);

                let graph_operators: Vec<GraphOperator> = vec![
                    GraphOperator::HostToDevice { input },
                    GraphOperator::ReLU,
                    GraphOperator::DeviceToHost,
                ];

                let fuse_operators: bool = false;
                let cache_elements: bool = false;
                let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                    &gpu_handles,
                    &graph_operators,
                    fuse_operators,
                    cache_elements,
                );
                let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
                println!("{:?}", difference);
                println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
            }
        }
    }

    #[test]
    fn softmax() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::softmax() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let input: Tensor2D = Tensor2D::new(0.5, outer_dimension, inner_dimension);

                let output_cpu: Tensor2D = Tensor2D::softmax(&input);

                let graph_operators: Vec<GraphOperator> = vec![
                    GraphOperator::HostToDevice { input },
                    GraphOperator::Softmax,
                    GraphOperator::DeviceToHost,
                ];

                let fuse_operators: bool = false;
                let cache_elements: bool = false;
                let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                    &gpu_handles,
                    &graph_operators,
                    fuse_operators,
                    cache_elements,
                );
                let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
                println!("{:?}", difference);
                println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
            }
        }
    }

    #[test]
    fn linear_relu() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::linear_relu() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension_input in 1..outer_dimension_range {
            for outer_dimension_weights in 1..outer_dimension_range {
                for inner_dimension in 1..inner_dimension_range {
                    let input: Tensor2D =
                        Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
                    let weights: Tensor2D =
                        Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
                    let bias: Tensor2D =
                        Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

                    let output_cpu: Tensor2D = Tensor2D::linear(&input, &weights, &bias);
                    let output_cpu: Tensor2D = Tensor2D::relu(&output_cpu);

                    let graph_operators: Vec<GraphOperator> = vec![
                        GraphOperator::HostToDevice { input },
                        GraphOperator::LinearReLUFused { weights, bias },
                        GraphOperator::DeviceToHost,
                    ];

                    let fuse_operators: bool = false;
                    let cache_elements: bool = false;
                    let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                        &gpu_handles,
                        &graph_operators,
                        fuse_operators,
                        cache_elements,
                    );
                    let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                    let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
                    println!("{:?}", difference);
                    println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
                }
            }
        }
    }

    #[test]
    fn linear_relu_softmax() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::linear_relu_softmax() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension_input in 1..outer_dimension_range {
            for outer_dimension_weights in 1..outer_dimension_range {
                for inner_dimension in 1..inner_dimension_range {
                    let input: Tensor2D =
                        Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
                    let weights: Tensor2D =
                        Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
                    let bias: Tensor2D =
                        Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

                    let output_cpu: Tensor2D = Tensor2D::linear(&input, &weights, &bias);
                    let output_cpu: Tensor2D = Tensor2D::relu(&output_cpu);
                    let output_cpu: Tensor2D = Tensor2D::softmax(&output_cpu);

                    let graph_operators: Vec<GraphOperator> = vec![
                        GraphOperator::HostToDevice { input },
                        GraphOperator::LinearReLUSoftmaxFused { weights, bias },
                        GraphOperator::DeviceToHost,
                    ];

                    let fuse_operators: bool = false;
                    let cache_elements: bool = false;
                    let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                        &gpu_handles,
                        &graph_operators,
                        fuse_operators,
                        cache_elements,
                    );
                    let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                    let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
                    println!("{:?}", difference);
                    println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
                }
            }
        }
    }

    #[test]
    fn transfers() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in graph_runner_test::linear_relu_softmax() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        for outer_dimension_input in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let input: Tensor2D = Tensor2D::new(0.5, outer_dimension_input, inner_dimension);

                let expected_output: Tensor2D = input.clone();

                let graph_operators: Vec<GraphOperator> = vec![
                    GraphOperator::HostToDevice { input },
                    GraphOperator::DeviceToHost,
                ];

                let fuse_operators: bool = false;
                let cache_elements: bool = false;
                let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
                    &gpu_handles,
                    &graph_operators,
                    fuse_operators,
                    cache_elements,
                );
                let output: Tensor2D = pollster::block_on(graph_runner.run(&gpu_handles, 1));

                let difference: Tensor2D = Tensor2D::subtraction(&expected_output, &output);
                println!("{:?}", difference);
                println!("{:?}", difference.data.iter().map(|x| x.abs()).sum::<f32>());
            }
        }
    }
}
