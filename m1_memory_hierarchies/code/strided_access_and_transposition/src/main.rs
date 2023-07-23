use std::{time::{Instant, Duration}, mem};

struct Matrix2D {
    is_transposed: bool,
    row_count: usize,
    column_count: usize,
    data: Vec<f32>,
}

impl Matrix2D {
    fn new(row_count: usize, column_count: usize, scale: f32) -> Self {
        let data: Vec<f32> = (0..row_count*column_count).into_iter().map(|x| x as f32 * scale).collect();

        Matrix2D { is_transposed: false, row_count, column_count, data }
    }

    fn transpose (&mut self) {
        if !self.is_transposed {
            self.is_transposed = true;
            for row_index in 0..self.row_count {
                for column_index in 0..self.column_count {
                    if row_index == column_index { break };
                    let index_a: usize = row_index * self.column_count + column_index;
                    let element_a: f32 = self.data[index_a];

                    let index_b: usize = column_index * self.row_count + row_index;
                    let element_b: f32 = self.data[index_b];

                    self.data[index_b] = element_a;
                    self.data[index_a] = element_b;
                }
            }
            mem::swap(&mut self.row_count, &mut self.column_count);
        }
    }

    fn multiply(input_a: &Matrix2D, input_b: &Matrix2D, output: &mut Matrix2D) {
        if input_b.is_transposed {
            for row_output in 0..output.row_count {
                for column_output in 0..output.column_count {
                    let mut sum: f32 = 0.0;
                    for inner_dimension in 0..input_a.column_count {
                        sum += 
                            input_a.data[row_output * input_a.column_count + inner_dimension] * 
                            input_b.data[row_output * input_b.column_count + inner_dimension];
                    }
                    output.data[row_output * output.column_count + column_output] = sum;
                }
            }
        } else {
            for row_output in 0..output.row_count {
                for column_output in 0..output.column_count {
                    let mut sum: f32 = 0.0;
                    for inner_dimension in 0..input_a.column_count {
                        sum += 
                            input_a.data[row_output * input_a.column_count + inner_dimension] * 
                            input_b.data[inner_dimension * input_b.column_count + column_output];
                    }
                    output.data[row_output * output.column_count + column_output] = sum;
                }
            }
        }
    }
}

fn print_example(outer_dimension: usize, inner_dimension: usize) {
    println!(
        "Now running {}x{} = {}x{} x {}x{} example",
        inner_dimension,
        inner_dimension,
        outer_dimension,
        inner_dimension,
        inner_dimension,
        outer_dimension
    );
}

fn test(outer_dimension: usize, inner_dimension: usize, iteration_count: usize) {
    print_example(outer_dimension, inner_dimension);
    let input_a: Matrix2D = Matrix2D::new(outer_dimension, inner_dimension, 0.1);
    let mut input_b: Matrix2D = Matrix2D::new(inner_dimension, outer_dimension, 0.2);
    let mut output: Matrix2D = Matrix2D::new(outer_dimension, outer_dimension, 0.0);

    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        Matrix2D::multiply(&input_a, &input_b, &mut output)
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for non_transposed", elapsed_time.as_millis() as f64);


    input_b.transpose();
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        Matrix2D::multiply(&input_a, &input_b, &mut output)
    }
    let elapsed_time: Duration = now.elapsed();
    println!("{} ms for transposed", elapsed_time.as_millis() as f64);

    println!("");
}

fn main() {
    let iteration_count: usize = 1_000;

    let outer_dimension: usize = 10;
    let inner_dimension: usize = 100;
    test(outer_dimension, inner_dimension, iteration_count);
}
