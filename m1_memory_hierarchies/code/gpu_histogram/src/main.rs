mod utility;
mod gpu_vector;
mod histogram;
use std::time::Instant;

use crate::histogram::histogram;

use utility::{self_test, GPUHandles, initialize_gpu};

use rand::{thread_rng, Rng};

fn benchmark_function(
    name: &str,
    shader: &str, 
    time_limit_seconds: f32, 
    debug: bool, 
    shuffle_data: bool,
    handles: &GPUHandles, 
    data_count: usize, 
    bin_count: usize, 
    elements_per_thread: usize
) {
    // Setup our CPU-side data
    let mut rng = thread_rng();
    let input: Vec<f32> =
    if shuffle_data {
        // We need to create several versions of these to cycle between
        // to get better measurements.
        (0..data_count).into_iter().map(
            |_| 
            rng.gen_range(0.0..((bin_count-1) as f32))
            ).collect()
        
    } else {
        (0..data_count).into_iter().map(
            |element| 
            1.0 / data_count as f32 * 
            element as f32 * bin_count as f32 * 0.9999).collect()
    };


    let start: Instant = Instant::now();
    let mut stop: Instant = Instant::now();
    let mut iterations: usize = 0;
    while (stop-start).as_secs_f32() < time_limit_seconds {
        assert!(histogram(debug, &input, &handles, shader, data_count, bin_count, elements_per_thread));
        stop = Instant::now();
        iterations += 1;
    }
    println!("{} ran {} iterations for {} ms\n", name, iterations, (stop-start).as_millis());
}

fn main() {
    // Initialize the env_logger to get usueful messages from wgpu.
    env_logger::init();

    // Is there a compatible GPU on the system?
    // Use pollster::block_on to block on async functions.
    // Think of it like this - this is a function which
    // uses the GPU. With block_on() we are insisting
    // on waiting until all the interaction with the GPU
    // and the tasks set in motion on the GPU are finished.
    if !pollster::block_on(self_test()) {
        panic!("Was unable to confirm that your system is compatible with this sample!");
    }

    // Keep track of the handles to central stuff like device and queue.
    let handles: GPUHandles = pollster::block_on(initialize_gpu()).expect("Was unsuccesful in creating GPU Handles");

    let data_count: usize = 2000000;
    let bin_count: usize = 1024;
    let elements_per_thread: usize = 8;
    let debug: bool = false;
    let shuffle_data: bool = false;
    let time_limit_seconds: f32 = 2.0;

    println!("RUNNING HISTOGRAM BENCHMARK:");
    println!("shuffle_data: {}", shuffle_data);
    println!("data_count: {}", data_count);
    println!("bin_count: {}", bin_count);
    println!("elements_per_thread: {}", elements_per_thread);
    println!("============================");
    println!("");

    // Incorrect result
    // assert!(histogram(&handles, include_str!("histogram.wgsl"), data_count, bin_count, 1));

    let histogram_atomic_name: &str = "histogram_atomic.wgsl";
    let histogram_atomic_shader: &str = include_str!("histogram_atomic.wgsl");
    benchmark_function(
        histogram_atomic_name,
        histogram_atomic_shader, 
        time_limit_seconds, 
        debug,
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        1
    );

    let histogram_shared_name: &str = "histogram_shared.wgsl";
    let histogram_shared_shader: &str = include_str!("histogram_shared.wgsl");
    benchmark_function(
        histogram_shared_name,
        histogram_shared_shader, 
        time_limit_seconds, 
        debug, 
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        1
    );

    let histogram_non_coalesced_name: &str = "histogram_non_coalesced.wgsl";
    let histogram_non_coalesced_shader: &str = include_str!("histogram_non_coalesced.wgsl");
    benchmark_function(
        histogram_non_coalesced_name,
        histogram_non_coalesced_shader, 
        time_limit_seconds, 
        debug, 
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        elements_per_thread
    );

    let histogram_local_name: &str = "histogram_local.wgsl";
    let histogram_local_shader: &str = include_str!("histogram_local.wgsl");
    benchmark_function(
        histogram_local_name,
        histogram_local_shader, 
        time_limit_seconds, 
        debug, 
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        elements_per_thread
    );

    let histogram_sparse_unoptimized_name: &str = "histogram_sparse_unoptimized.wgsl";
    let histogram_sparse_unoptimized_shader: &str = include_str!("histogram_sparse_unoptimized.wgsl");
    benchmark_function(
        histogram_sparse_unoptimized_name,
        histogram_sparse_unoptimized_shader, 
        time_limit_seconds, 
        debug, 
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        elements_per_thread
    );

    let histogram_sparse_name: &str = "histogram_sparse.wgsl";
    let histogram_sparse_shader: &str = include_str!("histogram_sparse.wgsl");
    benchmark_function(
        histogram_sparse_name,
        histogram_sparse_shader, 
        time_limit_seconds, 
        debug, 
        shuffle_data,
        &handles, 
        data_count, 
        bin_count, 
        elements_per_thread
    );

}
