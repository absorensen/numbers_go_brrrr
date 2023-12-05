#[cfg(test)]
mod tests {
    use crate::immediate::nodes::{
        linear_from_tensor_2d_blocking, linear_relu_softmax_from_tensor_2d_blocking,
        linear_relu_softmax_fused_from_tensor_2d_blocking,
        linearrelu_softmax_from_tensor_2d_blocking, relu_from_tensor_2d, softmax_from_tensor_2d,
        sum_from_tensor_2d,
    };
    use crate::shared::gpu_utilities::{initialize_gpu, GPUHandles};
    use crate::shared::tensor2d::Tensor2D;
    use crate::shared::tensor2d_gpu::Tensor2DGPU;

    const ERROR_TOLERANCE: f32 = 0.00001;

    // This is for verification purposes only
    // we don't care about making this fast
    fn subtract_tensors_gpu(left: &Tensor2D, right: &mut Tensor2DGPU) -> Tensor2D {
        debug_assert_eq!(
            left.row_count * left.column_count,
            right.row_count * right.column_count,
            "\ninput - rows: {} columns: {}.\n out - rows: {} columns: {}.",
            left.row_count,
            left.column_count,
            right.row_count,
            right.column_count
        );

        if right.live_data_on_device {
            pollster::block_on(right.retrieve_results());
        }

        let mut out: Tensor2D = Tensor2D::new(0.0, left.row_count, left.column_count);

        for index in 0..(left.row_count * left.column_count) {
            out.data[index] = left.data[index] - right.data.data[index];
        }

        out
    }

    fn subtract_tensors(left: &Tensor2D, right: &Tensor2D) -> Tensor2D {
        debug_assert_eq!(
            left.row_count * left.column_count,
            right.row_count * right.column_count,
            "\ninput - rows: {} columns: {}.\n out - rows: {} columns: {}.",
            left.row_count,
            left.column_count,
            right.row_count,
            right.column_count
        );

        let mut out: Tensor2D = Tensor2D::new(0.0, left.row_count, left.column_count);

        for index in 0..(left.row_count * left.column_count) {
            out.data[index] = left.data[index] - right.data[index];
        }

        out
    }

    fn linear_and_fused_test_function(
        gpu_handles: &GPUHandles,
        outer_dimension_range: usize,
        inner_dimension_range: usize,
        expected: fn(&Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D),
        test: fn(&GPUHandles, &Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D),
        is_fused: bool,
    ) {
        for outer_dimension_input in 1..outer_dimension_range {
            for outer_dimension_weights in 1..outer_dimension_range {
                for inner_dimension in 1..inner_dimension_range {
                    let input: Tensor2D =
                        Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
                    let weights: Tensor2D =
                        Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
                    let bias: Tensor2D =
                        Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);
                    let mut expected_output: Tensor2D =
                        Tensor2D::new(0.0, outer_dimension_input, outer_dimension_weights);
                    let mut output: Tensor2D = Tensor2D::new(
                        0.0,
                        if is_fused {
                            outer_dimension_input * outer_dimension_weights
                        } else {
                            outer_dimension_input
                        },
                        if is_fused { 1 } else { outer_dimension_weights },
                    );

                    expected(&input, &weights, &bias, &mut expected_output);
                    let expected_result: f32 = output.sum();
                    println!("expected result: {:?}", expected_result);

                    test(&gpu_handles, &input, &weights, &bias, &mut output);

                    let result: f32 = output.sum();
                    println!("result: {:?}", result);

                    let difference_output: Tensor2D = subtract_tensors(&expected_output, &output);

                    let abs_result_difference: f32 = difference_output.sum().abs();

                    assert!(abs_result_difference < ERROR_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn initialize_gpu_testing() {
        env_logger::init();
    }

    #[test]
    fn subtract() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::subtract() test");

        let outer_dimension_range: usize = 32;
        let inner_dimension_range: usize = 32;
        let scale: f32 = 0.5;

        for outer_dimension in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let input: Tensor2D = Tensor2D::new(scale, outer_dimension, inner_dimension);
                let mut input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(
                    &gpu_handles,
                    "test::subtraction::input_device",
                    &input,
                );
                let result_tensor: Tensor2D = subtract_tensors_gpu(&input, &mut input_device);

                let result: f32 = result_tensor.sum().abs();

                assert!(result < ERROR_TOLERANCE);
            }
        }
    }

    #[test]
    fn sum() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::sum() test");

        let outer_dimension_range: usize = 33;
        let inner_dimension_range: usize = 33;

        for outer_dimension in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let input: Tensor2D = Tensor2D::new(0.5, outer_dimension, inner_dimension);
                let expected_result: f32 = input.sum();

                let result: f32 = pollster::block_on(sum_from_tensor_2d(&gpu_handles, &input));
                let abs_result_difference: f32 = (expected_result - result).abs();

                assert!(abs_result_difference < ERROR_TOLERANCE);
            }
        }
    }

    #[test]
    fn softmax() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::softmax() test");

        let dimension_range: usize = 128;

        for dimension in 1..dimension_range {
            let input: Tensor2D = Tensor2D::new(0.5, dimension, 1);
            let expected_result: f32 = Tensor2D::softmax(&input).sum();

            let mut output: Tensor2D = Tensor2D::new(0.5, dimension, 1);
            pollster::block_on(softmax_from_tensor_2d(&gpu_handles, &input, &mut output));
            let result: f32 = output.sum();
            let abs_result_difference: f32 = (expected_result - result).abs();

            assert!(abs_result_difference < ERROR_TOLERANCE);
        }
    }

    #[test]
    fn linear() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::linear() test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        linear_and_fused_test_function(
            &gpu_handles,
            outer_dimension_range,
            inner_dimension_range,
            Tensor2D::linear_preallocated,
            linear_from_tensor_2d_blocking,
            false,
        );
    }

    #[test]
    fn relu() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::softmax() test");

        let dimension_range: usize = 128;

        for dimension in 1..dimension_range {
            let input: Tensor2D = Tensor2D::new(0.5, dimension, 1);
            let expected_result: f32 = Tensor2D::relu(&input).sum();

            let mut output: Tensor2D = Tensor2D::new(0.5, dimension, 1);
            pollster::block_on(relu_from_tensor_2d(&gpu_handles, &input, &mut output));
            let result: f32 = output.sum();
            let abs_result_difference: f32 = (expected_result - result).abs();

            assert!(abs_result_difference < ERROR_TOLERANCE);
        }
    }

    #[test]
    fn linear_relu_softmax() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::linear_relu_softmax test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        linear_and_fused_test_function(
            &gpu_handles,
            outer_dimension_range,
            inner_dimension_range,
            Tensor2D::linear_relu_softmax_fused,
            linear_relu_softmax_from_tensor_2d_blocking,
            true,
        );
    }

    #[test]
    fn linearrelu_softmax() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::linearrelu_softmax test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        linear_and_fused_test_function(
            &gpu_handles,
            outer_dimension_range,
            inner_dimension_range,
            Tensor2D::linear_relu_softmax_fused,
            linearrelu_softmax_from_tensor_2d_blocking,
            true,
        );
    }

    #[test]
    fn linear_relu_softmax_fused() {
        let gpu_handles: GPUHandles = pollster::block_on(initialize_gpu(true))
            .expect("Failed to get GPU handles in immediate::linear_relu_softmax_fused test");

        let outer_dimension_range: usize = 8;
        let inner_dimension_range: usize = 8;

        linear_and_fused_test_function(
            &gpu_handles,
            outer_dimension_range,
            inner_dimension_range,
            Tensor2D::linear_relu_softmax_fused,
            linear_relu_softmax_fused_from_tensor_2d_blocking,
            true,
        );
    }
}
