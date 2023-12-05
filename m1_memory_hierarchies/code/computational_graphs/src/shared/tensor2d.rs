// We won't enforce it in this tutorial
// But it is assumed that all the active
// data in the tensor is located in
// indices 0 to row_count*column_count
#[derive(Clone, Debug, Default)]
pub struct Tensor2D {
    pub data: Vec<f32>,
    pub row_count: usize,
    pub column_count: usize,
}

impl Tensor2D {
    pub fn new(scale: f32, row_count: usize, column_count: usize) -> Self {
        let mut data: Vec<f32> = Vec::<f32>::new();
        // This might not be the best thing performance-wise,
        // but the efficiency of this function does not matter for
        // this tutorial.
        for index in 0..row_count * column_count {
            data.push(index as f32 * scale);
        }

        Tensor2D {
            data,
            row_count,
            column_count,
        }
    }

    pub fn linear(input: &Tensor2D, weights: &Tensor2D, bias: &Tensor2D) -> Tensor2D {
        // Create a matrix and set all initial values to 0.0
        let mut output: Tensor2D = Tensor2D::new(0.0, input.row_count, weights.column_count);

        Tensor2D::linear_preallocated(input, weights, bias, &mut output);
        output
    }

    pub fn len(&self) -> usize {
        self.row_count * self.column_count
    }

    #[inline(always)]
    fn linear_assert(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        debug_assert!(
            0 < input.row_count,
            "\ninput.row_count must be larger than 0. Current value: {}.",
            input.row_count
        );
        debug_assert!(
            0 < input.column_count,
            "\ninput.column_count must be larger than 0. Current value: {}.",
            input.column_count
        );

        debug_assert!(
            0 < weights.row_count,
            "\nweights.row_count must be larger than 0. Current value: {}.",
            weights.row_count
        );
        debug_assert!(
            0 < weights.column_count,
            "\nweights.column_count must be larger than 0. Current value: {}.",
            weights.column_count
        );

        debug_assert!(
            0 < bias.row_count,
            "\nbias.row_count must be larger than 0. Current value: {}.",
            bias.row_count
        );
        debug_assert!(
            0 < bias.column_count,
            "\nbias.column_count must be larger than 0. Current value: {}.",
            bias.column_count
        );

        debug_assert!(
            0 < output.row_count,
            "\noutput.row_count must be larger than 0. Current value: {}.",
            output.row_count
        );
        debug_assert!(
            0 < output.column_count,
            "\noutput.column_count must be larger than 0. Current value: {}.",
            output.column_count
        );

        debug_assert_eq!(bias.row_count, output.row_count, "\nMismatch - bias.row_count & output.row_count\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", bias.row_count, bias.column_count, output.row_count, output.column_count);
        debug_assert_eq!(bias.column_count, output.column_count, "\nMismatch - bias.column_count & output.column_count\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", bias.row_count, bias.column_count, output.row_count, output.column_count);

        debug_assert_eq!(input.row_count, output.row_count, "\nMismatch - input.row_count & output.row_count\ninput - rows: {} columns: {}.\n out - rows: {} columns: {}.", input.row_count, input.column_count, output.row_count, output.column_count);
        debug_assert_eq!(weights.column_count, output.column_count, "\nMismatch - weights.column_count & output.column_count\nweights - rows: {} columns: {}.\n out - rows: {} columns: {}.", weights.row_count, weights.column_count, output.row_count, output.column_count);
        debug_assert_eq!(input.column_count, weights.row_count, "\nMismatch - input.column_count & weights.row_count\ninput - rows: {} columns: {}.\n weights - rows: {} columns: {}.", input.row_count, input.column_count, weights.row_count, weights.column_count);
    }

    #[inline(always)]
    fn linear_relu_softmax_assert(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        debug_assert!(
            0 < input.row_count,
            "\ninput.row_count must be larger than 0. Current value: {}.",
            input.row_count
        );
        debug_assert!(
            0 < input.column_count,
            "\ninput.column_count must be larger than 0. Current value: {}.",
            input.column_count
        );

        debug_assert!(
            0 < weights.row_count,
            "\nweights.row_count must be larger than 0. Current value: {}.",
            weights.row_count
        );
        debug_assert!(
            0 < weights.column_count,
            "\nweights.column_count must be larger than 0. Current value: {}.",
            weights.column_count
        );

        debug_assert!(
            0 < bias.row_count,
            "\nbias.row_count must be larger than 0. Current value: {}.",
            bias.row_count
        );
        debug_assert!(
            0 < bias.column_count,
            "\nbias.column_count must be larger than 0. Current value: {}.",
            bias.column_count
        );

        debug_assert!(
            0 < output.row_count,
            "\noutput.row_count must be larger than 0. Current value: {}.",
            output.row_count
        );
        debug_assert!(
            0 < output.column_count,
            "\noutput.column_count must be larger than 0. Current value: {}.",
            output.column_count
        );

        debug_assert_eq!(bias.len(), output.len(), "\nMismatch - bias.len() & output.len()\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", bias.row_count, bias.column_count, output.row_count, output.column_count);
    }

    pub fn linear_preallocated(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                for inner_dimension in 0..input.column_count {
                    output.data[row_output * output.column_count + column_output] += input.data
                        [row_output * input.column_count + inner_dimension]
                        * weights.data[inner_dimension * weights.column_count + column_output];
                }
            }
        }

        // When we know the underlying data structure is a 1D array
        // we could merely increment the index and have a for-loop from
        // 0..(bias.row_count*bias.column_count)
        for row in 0..bias.row_count {
            for column in 0..bias.column_count {
                let index: usize = row * output.column_count + column;
                output.data[index] += bias.data[index];
            }
        }
    }

    #[inline(always)]
    pub fn linear_preallocated_inline(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                for inner_dimension in 0..input.column_count {
                    output.data[row_output * output.column_count + column_output] += input.data
                        [row_output * input.column_count + inner_dimension]
                        * weights.data[inner_dimension * weights.column_count + column_output];
                }
            }
        }

        // When we know the underlying data structure is a 1D array
        // we could merely increment the index and have a for-loop from
        // 0..(bias.row_count*bias.column_count)
        for row in 0..bias.row_count {
            for column in 0..bias.column_count {
                let index: usize = row * output.column_count + column;
                output.data[index] += bias.data[index];
            }
        }
    }

    pub fn linear_local_accumulation(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                for inner_dimension in 0..input.column_count {
                    result += input.data[row_output * input.column_count + inner_dimension]
                        * weights.data[inner_dimension * weights.column_count + column_output];
                }
                output.data[row_output * output.column_count + column_output] = result;
            }
        }

        // When we know the underlying data structure is a 1D array
        // we could merely increment the index and have a for-loop from
        // 0..(bias.row_count*bias.column_count)
        for index in 0..(bias.row_count * bias.column_count) {
            output.data[index] += bias.data[index];
        }
    }

    pub fn linear_optimized(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                let mut index_input: usize = row_output * input.column_count;
                let mut index_weights: usize = column_output;
                for _ in 0..input.column_count {
                    result += input.data[index_input] * weights.data[index_weights];
                    index_input += 1;
                    index_weights += weights.column_count;
                }

                // Try this with bias read
                let index: usize = row_output * output.column_count + column_output;
                output.data[index] = result + bias.data[index];
            }
        }
    }

    pub fn relu(x: &Tensor2D) -> Tensor2D {
        // Create a matrix and set all initial values to 0.0
        let mut out: Tensor2D = Tensor2D::new(0.0, x.row_count, x.column_count);

        Self::relu_preallocated(x, &mut out);

        out
    }

    pub fn relu_preallocated(input: &Tensor2D, output: &mut Tensor2D) {
        for index in 0..(output.column_count * output.row_count) {
            output.data[index] = input.data[index].max(0.0);
        }
    }

    pub fn relu_inplace(data: &mut Tensor2D) {
        for index in 0..(data.column_count * data.row_count) {
            data.data[index] = data.data[index].max(0.0);
        }
    }

    #[inline(always)]
    pub fn relu_inplace_inline(data: &mut Tensor2D) {
        for index in 0..(data.column_count * data.row_count) {
            data.data[index] = data.data[index].max(0.0);
        }
    }

    pub fn softmax(input: &Tensor2D) -> Tensor2D {
        let mut output: Tensor2D = Tensor2D::new(0.0, input.row_count, input.column_count);

        Self::softmax_preallocated(input, &mut output);

        output
    }

    pub fn softmax_preallocated(input: &Tensor2D, output: &mut Tensor2D) {
        let mut max: f32 = f32::NEG_INFINITY;
        for index in 0..(output.column_count * output.row_count) {
            if max < input.data[index] {
                max = input.data[index];
            }
        }

        let mut sum: f32 = 0.0;
        for index in 0..(output.column_count * output.row_count) {
            sum += (input.data[index] - max).exp();
        }

        let offset: f32 = max + sum.ln();

        for index in 0..(output.column_count * output.row_count) {
            output.data[index] = (input.data[index] - offset).exp();
        }
    }

    pub fn softmax_inplace(out: &mut Tensor2D) {
        let mut max: f32 = f32::NEG_INFINITY;
        for index in 0..(out.column_count * out.row_count) {
            if max < out.data[index] {
                max = out.data[index];
            }
        }

        let mut sum: f32 = 0.0;
        for index in 0..(out.column_count * out.row_count) {
            sum += (out.data[index] - max).exp();
        }

        let offset: f32 = max + sum.ln();

        for index in 0..(out.column_count * out.row_count) {
            out.data[index] = (out.data[index] - offset).exp();
        }
    }

    #[inline(always)]
    pub fn softmax_inplace_inline(out: &mut Tensor2D) {
        let mut max: f32 = f32::NEG_INFINITY;
        for index in 0..(out.column_count * out.row_count) {
            if max < out.data[index] {
                max = out.data[index];
            }
        }

        let mut sum: f32 = 0.0;
        for index in 0..(out.column_count * out.row_count) {
            sum += (out.data[index] - max).exp();
        }

        let offset: f32 = max + sum.ln();

        for index in 0..(out.column_count * out.row_count) {
            out.data[index] = (out.data[index] - offset).exp();
        }
    }

    #[inline]
    pub fn linear_local_accumulation_relu(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                let mut index_input: usize = row_output * input.column_count;
                let mut index_weights: usize = column_output;
                for _ in 0..input.column_count {
                    result += input.data[index_input] * weights.data[index_weights];
                    index_input += 1;
                    index_weights += weights.column_count;
                }
                output.data[row_output * output.column_count + column_output] = result;
            }
        }

        for index in 0..(bias.row_count * bias.column_count) {
            output.data[index] = (output.data[index] + bias.data[index]).max(0.0);
        }
    }

    #[inline]
    pub fn linear_optimized_relu(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                let mut index_input: usize = row_output * input.column_count;
                let mut index_weights: usize = column_output;
                for _ in 0..input.column_count {
                    result += input.data[index_input] * weights.data[index_weights];
                    index_input += 1;
                    index_weights += weights.column_count;
                }

                let index: usize = row_output * output.column_count + column_output;
                output.data[index] = (result + bias.data[index]).max(0.0);
            }
        }
    }

    // Maybe just inline
    #[inline]
    pub fn linear_relu_softmax_fused_fission(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_relu_softmax_assert(input, weights, bias, output);

        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                let mut index_input: usize = row_output * input.column_count;
                let mut index_weights: usize = column_output;
                for _ in 0..input.column_count {
                    result += input.data[index_input] * weights.data[index_weights];
                    index_input += 1;
                    index_weights += weights.column_count;
                }

                let index: usize = row_output * output.column_count + column_output;
                output.data[index] = result;
            }
        }

        for index in 0..(bias.row_count * bias.column_count) {
            output.data[index] += bias.data[index];
        }

        let mut max: f32 = f32::NEG_INFINITY;
        for index in 0..(bias.row_count * bias.column_count) {
            let result: f32 = (output.data[index] + bias.data[index]).max(0.0);
            max = max.max(result);
            output.data[index] = result;
        }

        let mut sum: f32 = 0.0;
        for index in 0..(output.column_count * output.row_count) {
            sum += (output.data[index] - max).exp();
        }

        let offset: f32 = max + sum.ln();

        for index in 0..(output.column_count * output.row_count) {
            output.data[index] = (output.data[index] - offset).exp();
        }
    }

    // Maybe just inline
    #[inline]
    pub fn linear_relu_softmax_fused(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        Self::linear_relu_softmax_assert(input, weights, bias, output);

        let mut max: f32 = f32::NEG_INFINITY;
        for row_output in 0..output.row_count {
            for column_output in 0..output.column_count {
                let mut result: f32 = 0.0;
                let mut index_input: usize = row_output * input.column_count;
                let mut index_weights: usize = column_output;
                for _ in 0..input.column_count {
                    result += input.data[index_input] * weights.data[index_weights];
                    index_input += 1;
                    index_weights += weights.column_count;
                }

                let index: usize = row_output * output.column_count + column_output;
                result = (result + bias.data[index]).max(0.0);
                max = max.max(result);

                output.data[index] = result;
            }
        }

        let mut sum: f32 = 0.0;
        for index in 0..(output.column_count * output.row_count) {
            sum += (output.data[index] - max).exp();
        }

        let offset: f32 = max + sum.ln();

        for index in 0..(output.column_count * output.row_count) {
            output.data[index] = (output.data[index] - offset).exp();
        }
    }

    // Just for testing
    #[inline(always)]
    pub fn subtraction(left: &Tensor2D, right: &Tensor2D) -> Tensor2D {
        debug_assert_eq!(left.len(), right.len());

        let mut output: Tensor2D = Tensor2D::new(0.0, left.row_count, left.column_count);
        for index in 0..(left.column_count * left.row_count) {
            output.data[index] = left.data[index] - right.data[index];
        }

        output
    }

    // Just for testing.
    // Get the sum of all active elements
    // Mostly for verifying correctness
    pub fn sum(&self) -> f32 {
        let mut sum: f32 = 0.0;
        for index in 0..self.row_count * self.column_count {
            sum += self.data[index];
        }

        sum
    }
}
