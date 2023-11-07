use crate::utility::{GPUHandles, mean_square_error, are_vectors_equivalent};

fn matrix_multiplication_cpu(
    left_matrix: &Vec<f32>,
    right_matrix: &Vec<f32>,
    outer_dimension_left_length: usize,
    inner_dimension_length: usize,
    outer_dimension_right_length: usize,
) -> Vec<f32> {
    let mut output: Vec<f32> = vec![0.0; outer_dimension_left_length * outer_dimension_right_length];
    for row_output in 0..outer_dimension_left_length {
        for column_output in 0..outer_dimension_right_length {
            for inner_dimension in 0..inner_dimension_length {
                output[row_output * outer_dimension_right_length + column_output] += left_matrix
                    [row_output * inner_dimension_length + inner_dimension]
                    * right_matrix[inner_dimension * outer_dimension_right_length + column_output];
            }
        }
    }
    output
}

fn test_ground_truth() -> bool {
    let outer_dimension_left: usize = 4;
    let inner_dimension: usize = 3;
    let outer_dimension_right: usize = 3;
    let left: Vec<f32> = vec![ 1.0, 0.0, 1.0, 2.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 2.0];
    let right: Vec<f32> = vec![ 1.0, 2.0, 1.0, 2.0, 3.0, 1.0, 4.0, 2.0, 2.0];
    let ground_truth_output: Vec<f32> = vec![ 5.0, 4.0, 3.0, 8.0, 9.0, 5.0, 6.0, 5.0, 3.0, 11.0, 9.0, 6.0];

    let output: Vec<f32> = matrix_multiplication_cpu(&left, &right, outer_dimension_left, inner_dimension, outer_dimension_right);

    assert!(output.len() == ground_truth_output.len());
    for index in 0..ground_truth_output.len() {
        if 0.00001 < (output[index] - ground_truth_output[index]).abs() {
            return false;
        }
    }

    true
}

pub fn matrix_multiplication(handles: &GPUHandles) -> bool {
    // A small test to ensure that the matrix_multiplication_cpu function is actually correct.
    let ground_truth_is_correct: bool = test_ground_truth();
    println!("Matrix multiplication ground truth function is correct: {}", ground_truth_is_correct);
    assert!(ground_truth_is_correct);


    // Use big data dimensions to make sure the cost of transferring
    // doesn't dominate the time spent in the function.
    let outer_dimension_left: usize = 400; // M
    let inner_dimension: usize = 320; // N
    let outer_dimension_right: usize = 540;// K
    let left_matrix: Vec<f32> = (0..outer_dimension_left*inner_dimension).map(|x| x as f32 * 0.1).collect();
    let right_matrix: Vec<f32> = (0..inner_dimension*outer_dimension_right).map(|x| x as f32 * 0.1).collect();
    let ground_truth: Vec<f32> = matrix_multiplication_cpu(&left_matrix, &right_matrix, outer_dimension_left, inner_dimension, outer_dimension_right);
    let dummy_data: Vec<f32> = ground_truth.clone();
    
    //
    // YOUR CODE HERE
    // Make one version of matrix multiplication using the GPU. Ensure that it is correct.
    // Make another version which uses tiling through shared memory and local accumulation in
    // a register. A tiling reference: http://www.csce.uark.edu/~mqhuang/courses/4643/s2016/lecture/GPU_Lecture_3.pdf
    // After ensuring correctness - time the two functions.
    // How big do the matrices have to be before you see a big performance
    // difference?

    println!("matrix_multiplication MSE: {}", mean_square_error(&ground_truth, &dummy_data));
    let success: bool = are_vectors_equivalent(&ground_truth, &dummy_data);
    println!("matrix_multiplication success: {}!", success);

    success
}