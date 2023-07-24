use std::io::Read;
use std::mem;
use std::time::{Instant, Duration};

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

fn print_example(function: &str, iteration_count: usize, data_count: usize) {
    println!(
        "Now running {} example with {} elements for {} iterations",
        function,
        data_count,
        iteration_count
    );
}

fn test_permuted(iteration_count: usize, data_count: usize) -> f32{
    let mut rng: ThreadRng = rand::thread_rng();

    let data: Vec<f32> = (0..data_count).into_iter().map(|_| rng.gen::<f32>()).collect();
    let mut indices: Vec<usize> = (0..data_count).collect();
    indices.shuffle(&mut rng);

    let mut total_sum: f32 = 0.0;
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let mut sum: f32 = 0.0;
        for index in &indices {
            let index: usize = *index;
            sum += data[index];
        }
        total_sum += sum;
    }
    let elapsed_time: Duration = now.elapsed();

    let bytes_used: usize = data.len() * mem::size_of::<f32>() + indices.len() * mem::size_of::<usize>();
    println!("{} ms for permuted test taking {} bytes of memory", elapsed_time.as_millis() as f64, bytes_used);

    total_sum
} 

fn test_executed_permuted(iteration_count: usize, data_count: usize) -> f32 {
    let mut rng: ThreadRng = rand::thread_rng();

    let data: Vec<f32> = (0..data_count).into_iter().map(|_| rng.gen::<f32>()).collect();
    let mut indices: Vec<usize> = (0..data_count).collect();
    indices.shuffle(&mut rng);

    let data: Vec<f32> = indices.iter().map(|x| data[*x]).collect();

    let mut total_sum: f32 = 0.0;
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let mut sum: f32 = 0.0;
        for value in &data {
            sum += *value;
        }
        total_sum += sum;
    }
    let elapsed_time: Duration = now.elapsed();

    let bytes_used: usize = data.len() * mem::size_of::<f32>();
    println!("{} ms for executed permuted test taking {} bytes of memory", elapsed_time.as_millis() as f64, bytes_used);

    total_sum
} 

fn test_permuted_rows(iteration_count: usize, data_count: usize, row_length: usize) -> f32{
    let mut rng: ThreadRng = rand::thread_rng();

    let data: Vec<f32> = (0..data_count).into_iter().map(|_| rng.gen::<f32>()).collect();
    let mut indices: Vec<usize> = (0..(data_count/row_length)).collect();
    indices.shuffle(&mut rng);

    let mut total_sum: f32 = 0.0;
    let now: Instant = Instant::now();
    for _ in 0..iteration_count {
        let mut sum: f32 = 0.0;
        for index in &indices {
            let index: usize = *index;
            for column_index in 0..row_length {
                sum += data[index * row_length + column_index];
            }
        }
        total_sum += sum;
    }
    let elapsed_time: Duration = now.elapsed();
    let bytes_used: usize = data.len() * mem::size_of::<f32>() + indices.len() * mem::size_of::<usize>();
    println!("{} ms for permuted rows test taking {} bytes of memory", elapsed_time.as_millis() as f64, bytes_used);

    total_sum
} 

fn test(iteration_count: usize, data_count: usize, row_length: usize) {
    println!("Running tests for {} elements for {} iterations", data_count, iteration_count);
    let mut sums: f32 = 0.0;
    sums += test_permuted(iteration_count, data_count);
    sums += test_executed_permuted(iteration_count, data_count);
    sums += test_permuted_rows(iteration_count, data_count, row_length);
    println!("Sums were: {}", sums);
    println!("");
}

// Add different size tests and random access testing in addition to the sum test
fn main() {
    let iteration_count: usize = 100_000;
    let data_count: usize = 100000;
    let row_length: usize = 100;
    test(iteration_count, data_count, row_length);
}
