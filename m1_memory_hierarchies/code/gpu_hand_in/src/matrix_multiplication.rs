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
    let left_matrix: Vec<f32> = (0..outer_dimension_left*inner_dimension).map(|x| x as f32 * 1.0).collect();
    let right_matrix: Vec<f32> = (0..inner_dimension*outer_dimension_right).map(|x| x as f32 * -0.1).collect();
    let ground_truth: Vec<f32> = matrix_multiplication_cpu(&left_matrix, &right_matrix, outer_dimension_left, inner_dimension, outer_dimension_right);
    
    //
    // 1) Make one version of matrix multiplication using the GPU. Ensure that it is correct.
    //
    // 2) Make another version which uses tiling through shared memory and local accumulation in a register.
    // A tiling reference: http://www.csce.uark.edu/~mqhuang/courses/4643/s2016/lecture/GPU_Lecture_3.pdf
    // Try to ensure that you use coalesced accessing as much as possible when loading data into shared
    // memory.
    //
    // 3) After ensuring correctness - time the two functions.
    // 
    // 4) How big do the matrices have to be before you see a big performance difference between the
    // naive and shared memory versions?
    //
    // 5) What happens when you set the block size to different multiples of 32? Why do you think that is?
    //
    // 6) What is the optimal tile size?
    //
    // 7) Make a third version starting from the tiled version, but pads the matrices with 0's to the nearest
    // multiple of the block size? So, if you had a block size of 32 and a 30x30 * 28x29
    // multiplication you padded both with 0's to get 32x32 * 32x32.
    // HINT - You can now remove some if-guards.
    //
    // 8) How big are the performance differences if you make new versions of all the shaders with the
    // restriction that the second matrix is transposed? Is the different between the transposed and
    // non-transposed different for the naive and tiled versions different? Why do you think that is?
    //
    // HINT - You need a run_compute_shader() call per type of compute shader.
    // Figure out what the arguments are supposed to be (see vector_add.rs) and
    // call the correct shader function in the correct shader file.
    //

    //
    // YOUR CODE HERE
    let data_naive: Vec<f32> = ground_truth.clone(); // Remove this and replace with your own data
    let data_tiled: Vec<f32> = ground_truth.clone(); // Remove this and replace with your own data
    let data_padded: Vec<f32> = ground_truth.clone(); // Remove this and replace with your own data
    //

    // Naive
    println!("matrix multiplication naive MSE: {}", mean_square_error(&ground_truth, &data_naive));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_naive);
    println!("matrix multiplication naive success: {}!", success);

    // Tiled
    println!("matrix multiplication tiled MSE: {}", mean_square_error(&ground_truth, &data_tiled));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_tiled);
    println!("matrix multiplication tiled success: {}!", success);

    // Padded
    println!("matrix multiplication padded MSE: {}", mean_square_error(&ground_truth, &data_padded));
    let success: bool = are_vectors_equivalent(&ground_truth, &data_padded);
    println!("matrix multiplication padded success: {}!", success);

    success
}