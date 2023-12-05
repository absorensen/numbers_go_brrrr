#![allow(dead_code)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::identity_op
)]

mod graph;
mod immediate;
mod op_code_compiler;
mod shared;
mod cpu;

use shared::{
    configuration::Configuration,
    gpu_utilities::{self, initialize_gpu, GPUHandles},
};

pub async fn run() {
    env_logger::init();

    // If not wgpu compatible, then alert the user
    let debug_level: u32 = 4;
    let run_performance_benchmark: bool = true;
    let loop_count: usize = 25;
    let loop_range: Vec<usize> = (2u32..8u32).map(|x| 2usize.pow(x)).collect();
    let log_scale: bool = false;
    let compatible_gpu_found: bool = pollster::block_on(gpu_utilities::self_test());
    let warmup_gpu: bool = true;
    let default_graph_layer_count: usize = 64; // Only used for benchmarking graph functions
    let default_graph_operator_size: usize = 256; // Only used for benchmarking graph functions
    let graph_depth_range: Vec<usize> = (2u32..8u32).map(|x| 2usize.pow(x)).collect();

    let configuration: Configuration = Configuration::build_gpu(
        debug_level,
        run_performance_benchmark,
        loop_count,
        loop_range,
        log_scale,
        compatible_gpu_found,
        warmup_gpu,
        default_graph_layer_count,
        default_graph_operator_size,
        graph_depth_range,
    );
    cpu::runner::execute(&configuration);

    if configuration.compatible_gpu_found {
        let gpu_handles: GPUHandles = initialize_gpu(configuration.warmup_gpu)
            .await
            .expect("Failed to acquire GPU Handles");

        pollster::block_on(immediate::runner::execute(&gpu_handles, &configuration));
        pollster::block_on(graph::runner::execute(&gpu_handles, &configuration));
        op_code_compiler::runner::compile_linear_shader(&gpu_handles, true);
    }
}
