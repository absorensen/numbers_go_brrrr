mod utility;
mod gpu_vector;
mod histogram;
use std::time::Instant;

use crate::histogram::histogram;

use utility::{self_test, GPUHandles, initialize_gpu};

fn benchmark_function(
    name: &str,
    shader: &str, 
    time_limit_seconds: f32, 
    debug:bool, 
    handles: &GPUHandles, 
    data_count: usize, 
    bin_count: usize, 
    elements_per_thread: usize
) {
    let start: Instant = Instant::now();
    let mut stop: Instant = Instant::now();
    let mut iterations: usize = 0;
    while (stop-start).as_secs_f32() < time_limit_seconds {
        assert!(histogram(debug, &handles, shader, data_count, bin_count, elements_per_thread));
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
    let bin_count: usize = 10;
    let elements_per_thread: usize = 16;
    let debug: bool = false;
    let time_limit_seconds: f32 = 2.0;

    println!("RUNNING HISTOGRAM BENCHMARK:");
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
        &handles, 
        data_count, 
        bin_count, 
        1
    );

    let histogram_local_name: &str = "histogram_local.wgsl";
    let histogram_local_shader: &str = include_str!("histogram_local.wgsl");
    benchmark_function(
        histogram_local_name,
        histogram_local_shader, 
        time_limit_seconds, 
        debug, 
        &handles, 
        data_count, 
        bin_count, 
        1
    );

}
