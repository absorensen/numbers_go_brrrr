
use crate::{utility::{GPUHandles, mean_square_error, are_vectors_equivalent, Uniform, run_compute_shader}, gpu_vector::GPUVector};

fn vector_add_cpu(input_a: &Vec<f32>, input_b: &Vec<f32>) -> Vec<f32> {
    assert!(input_a.len() == input_b.len());
    
    let mut output: Vec<f32> = vec![0.0; input_a.len()];
    
    for index in 0..input_a.len() {
        output[index] = input_a[index] + input_b[index];
    }

    output
}

pub fn vector_add(handles: &GPUHandles) -> bool {
    // Setup our CPU-side data
    let element_count: usize = 100;
    let input_a: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32).collect();
    let input_b: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32 * 0.1).collect();
    let output: Vec<f32> = vec![0.0; element_count];

    let ground_truth: Vec<f32> = vector_add_cpu(&input_a, &input_b);


    // Create our uniform for telling the shader how big the vectors are.
    let uniform: Uniform = Uniform::new(handles, element_count, 0, 0, 0);

    // Create the GPU vectors.
    // Note the true at the end of the output vector creation.
    // This will result in a staging_buffer being created, which we
    // can read from on the CPU.
    let input_a: GPUVector = GPUVector::new(&handles, input_a, "input_a", false);
    let input_b: GPUVector = GPUVector::new(&handles, input_b, "input_b", false);
    let mut output: GPUVector = GPUVector::new(&handles, output, "output", true);

    // We will use 32 threads in a work group/warp
    // We are doing this in 1 dimension, but could do it in
    // up to 3 dimensions.
    let block_size_x: usize = 32;
    let launch_blocks_x: u32 = ((element_count + block_size_x - 1) / block_size_x) as u32;
    let block_size_y: usize = 1;
    let launch_blocks_y: u32 = 1;
    let shader_file: &'static str = include_str!("vector_add.wgsl");
    let shader_function: &str = "vector_add";

    // Reuse this function for the convolution and matrix multiplication tasks
    run_compute_shader(
        handles,
        block_size_x, 
        launch_blocks_x,
        block_size_y,
        launch_blocks_y,
        shader_file,
        shader_function,
        &uniform,
        &input_a,
        &input_b,
        &mut output,
    );

    println!("vector_add MSE: {}", mean_square_error(&ground_truth, &output.cpu_data));
    let success: bool = are_vectors_equivalent(&ground_truth, &output.cpu_data);
    println!("vector_add success: {}!", success);

    success
}