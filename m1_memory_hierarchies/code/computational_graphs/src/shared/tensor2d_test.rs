#[cfg(test)]
mod tests {
    use crate::shared::tensor2d::Tensor2D;

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

    fn test_function_preallocated(
        outer_dimension_input: usize,
        outer_dimension_weights: usize,
        inner_dimension: usize,
        expected: fn(&Tensor2D, &Tensor2D, &Tensor2D) -> Tensor2D,
        test: fn(&Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D),
    ) -> f32 {
        let mut input: Tensor2D = Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
        let weights: Tensor2D = Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
        let bias: Tensor2D = Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

        let expected_output: Tensor2D = expected(&input, &weights, &bias);

        let mut output: Tensor2D =
            Tensor2D::new(0.0, outer_dimension_input, outer_dimension_weights);
        test(&mut input, &weights, &bias, &mut output);

        subtract_tensors(&expected_output, &output).sum().abs()
    }

    fn single_test_function_preallocated(
        dimension_row: usize,
        dimension_column: usize,
        scale: f32,
        expected: fn(&Tensor2D) -> Tensor2D,
        test: fn(&Tensor2D, &mut Tensor2D),
    ) -> f32 {
        let input: Tensor2D = Tensor2D::new(scale, dimension_row, dimension_column);
        let expected_output: Tensor2D = expected(&input);

        let mut output: Tensor2D = Tensor2D::new(scale, dimension_row, dimension_column);
        test(&input, &mut output);

        subtract_tensors(&expected_output, &output).sum().abs()
    }

    fn single_test_function_inplace(
        dimension_row: usize,
        dimension_column: usize,
        scale: f32,
        expected: fn(&Tensor2D) -> Tensor2D,
        test: fn(&mut Tensor2D),
    ) -> f32 {
        let mut input: Tensor2D = Tensor2D::new(scale, dimension_row, dimension_column);
        let expected_output: Tensor2D = expected(&input);

        test(&mut input);

        subtract_tensors(&expected_output, &input).sum().abs()
    }

    #[test]
    fn subtraction() {
        let outer_dimension_range: usize = 15;
        let inner_dimension_range: usize = 12;

        let expected_result: f32 = 0.0;

        for outer_dimension in 1..outer_dimension_range {
            for inner_dimension in 1..inner_dimension_range {
                let left: Tensor2D = Tensor2D::new(0.5, outer_dimension, inner_dimension);
                let right: Tensor2D = Tensor2D::new(0.5, outer_dimension, inner_dimension);
                let output: Tensor2D = subtract_tensors(&left, &right);
                let result: f32 = output.sum();

                let abs_result_difference: f32 = (expected_result - result).abs();
                assert!(abs_result_difference < ERROR_TOLERANCE);
            }
        }
    }

    #[test]
    fn linear() {
        let outer_dimension_input: usize = 3;
        let outer_dimension_weights: usize = 4;
        let inner_dimension: usize = 4;

        let expected_result: f32 = 1116.6;

        let input: Tensor2D = Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
        let weights: Tensor2D = Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
        let bias: Tensor2D = Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

        let output: Tensor2D = Tensor2D::linear(&input, &weights, &bias);

        let abs_result_difference: f32 = (expected_result - output.sum()).abs();

        assert!(abs_result_difference < ERROR_TOLERANCE);
    }

    #[test]
    fn linear_preallocated() {
        let outer_dimension_input_max: usize = 10;
        let outer_dimension_weights_max: usize = 10;
        let inner_dimension_max: usize = 10;

        for outer_dimension_input in 1..outer_dimension_input_max {
            for outer_dimension_weights in 1..outer_dimension_weights_max {
                for inner_dimension in 1..inner_dimension_max {
                    let abs_result_difference: f32 = test_function_preallocated(
                        outer_dimension_input,
                        outer_dimension_weights,
                        inner_dimension,
                        Tensor2D::linear,
                        Tensor2D::linear_preallocated,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn linear_preallocated_inline() {
        let outer_dimension_input_max: usize = 10;
        let outer_dimension_weights_max: usize = 10;
        let inner_dimension_max: usize = 10;

        for outer_dimension_input in 1..outer_dimension_input_max {
            for outer_dimension_weights in 1..outer_dimension_weights_max {
                for inner_dimension in 1..inner_dimension_max {
                    let abs_result_difference: f32 = test_function_preallocated(
                        outer_dimension_input,
                        outer_dimension_weights,
                        inner_dimension,
                        Tensor2D::linear,
                        Tensor2D::linear_preallocated_inline,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn linear_local_accumulation() {
        let outer_dimension_input_max: usize = 10;
        let outer_dimension_weights_max: usize = 10;
        let inner_dimension_max: usize = 10;

        for outer_dimension_input in 1..outer_dimension_input_max {
            for outer_dimension_weights in 1..outer_dimension_weights_max {
                for inner_dimension in 1..inner_dimension_max {
                    let abs_result_difference: f32 = test_function_preallocated(
                        outer_dimension_input,
                        outer_dimension_weights,
                        inner_dimension,
                        Tensor2D::linear,
                        Tensor2D::linear_local_accumulation,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn linear_optimized() {
        let outer_dimension_input_max: usize = 10;
        let outer_dimension_weights_max: usize = 10;
        let inner_dimension_max: usize = 10;

        for outer_dimension_input in 1..outer_dimension_input_max {
            for outer_dimension_weights in 1..outer_dimension_weights_max {
                for inner_dimension in 1..inner_dimension_max {
                    let abs_result_difference: f32 = test_function_preallocated(
                        outer_dimension_input,
                        outer_dimension_weights,
                        inner_dimension,
                        Tensor2D::linear,
                        Tensor2D::linear_optimized,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn relu() {
        let row_count: usize = 4;
        let column_count: usize = 3;
        let scale: f32 = 0.5;
        let expected_result: f32 = 33.0;

        let input: Tensor2D = Tensor2D::new(scale, row_count, column_count);
        let output: Tensor2D = Tensor2D::relu(&input);
        let abs_result_difference: f32 = (expected_result - output.sum()).abs();
        assert!(abs_result_difference < ERROR_TOLERANCE);

        let row_count: usize = 4;
        let column_count: usize = 3;
        let scale: f32 = -0.5;
        let expected_result: f32 = 0.0;

        let input: Tensor2D = Tensor2D::new(scale, row_count, column_count);
        let output: Tensor2D = Tensor2D::relu(&input);
        let abs_result_difference: f32 = (expected_result - output.sum()).abs();
        assert!(abs_result_difference < ERROR_TOLERANCE);
    }

    #[test]
    fn relu_preallocated() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_preallocated(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::relu,
                        Tensor2D::relu_preallocated,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn relu_inplace() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_inplace(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::relu,
                        Tensor2D::relu_inplace,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn relu_inplace_inline() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_inplace(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::relu,
                        Tensor2D::relu_inplace_inline,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn softmax() {
        let row_count: usize = 4;
        let column_count: usize = 3;
        let scale: f32 = 0.5;
        let expected_result: f32 = 1.0;

        let input: Tensor2D = Tensor2D::new(scale, row_count, column_count);
        let output: Tensor2D = Tensor2D::softmax(&input);
        let abs_result_difference: f32 = (expected_result - output.sum()).abs();
        assert!(abs_result_difference < ERROR_TOLERANCE);

        let row_count: usize = 4;
        let column_count: usize = 3;
        let scale: f32 = -0.5;
        let expected_result: f32 = 1.0;

        let input: Tensor2D = Tensor2D::new(scale, row_count, column_count);
        let output: Tensor2D = Tensor2D::softmax(&input);
        let abs_result_difference: f32 = (expected_result - output.sum()).abs();
        assert!(abs_result_difference < ERROR_TOLERANCE);
    }

    #[test]
    fn softmax_preallocated() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_preallocated(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::softmax,
                        Tensor2D::softmax_preallocated,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn softmax_inplace() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_inplace(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::softmax,
                        Tensor2D::softmax_inplace,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn softmax_inplace_inline() {
        let row_count_max: usize = 10;
        let column_count_max: usize = 10;
        let step: f32 = 0.2;
        let start: f32 = -3.14;
        let stop: f32 = 2.1;

        for row_count in 1..row_count_max {
            for column_count in 1..column_count_max {
                let mut scale: f32 = start;
                while scale < stop {
                    let abs_result_difference: f32 = single_test_function_inplace(
                        row_count,
                        column_count,
                        scale,
                        Tensor2D::softmax,
                        Tensor2D::softmax_inplace_inline,
                    );
                    assert!(abs_result_difference < ERROR_TOLERANCE);
                    scale += step;
                }
            }
        }
    }

    #[test]
    fn linear_relu_softmax_fused() {
        let outer_dimension_input_max: usize = 10;
        let outer_dimension_weights_max: usize = 10;
        let inner_dimension_max: usize = 10;

        for outer_dimension_input in 1..outer_dimension_input_max {
            for outer_dimension_weights in 1..outer_dimension_weights_max {
                for inner_dimension in 1..inner_dimension_max {
                    let input: Tensor2D =
                        Tensor2D::new(0.5, outer_dimension_input, inner_dimension);
                    let weights: Tensor2D =
                        Tensor2D::new(1.0, inner_dimension, outer_dimension_weights);
                    let bias: Tensor2D =
                        Tensor2D::new(0.1, outer_dimension_input, outer_dimension_weights);

                    let mut output_not_fused: Tensor2D =
                        Tensor2D::new(0.0, outer_dimension_input, outer_dimension_weights);
                    Tensor2D::linear_optimized(
                        &input,
                        &weights,
                        &bias,
                        &mut output_not_fused,
                    );
                    Tensor2D::relu_inplace(&mut output_not_fused);
                    Tensor2D::softmax_inplace_inline(&mut output_not_fused);

                    let mut output_fused: Tensor2D =
                        Tensor2D::new(0.0, outer_dimension_input, outer_dimension_weights);
                    Tensor2D::linear_relu_softmax_fused(&input, &weights, &bias, &mut output_fused);

                    let abs_result_difference: f32 =
                        subtract_tensors(&output_fused, &output_not_fused)
                            .sum()
                            .abs();

                    // We modify the error tolerance here as there is a
                    // significant difference in the numerical computations
                    // due to the fusion
                    assert!(abs_result_difference < 10.0 * ERROR_TOLERANCE);
                }
            }
        }
    }
}
