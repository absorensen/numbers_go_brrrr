use crate::shared::graph_operators::GraphOperator::*;
use crate::{
    graph::graph_runner::GraphRunner,
    immediate,
    shared::{
        benchmark_plot::draw_benchmark_plot,
        configuration::Configuration,
        gpu_utilities::GPUHandles,
        graph_operators::GraphOperator,
        performance_measurement::{
            benchmark_function_vector_gpu_graph, GraphFunction, PerformanceMeasurements,
        },
        tensor2d::Tensor2D,
    },
};

use super::{graph_runner_gpu::GraphRunnerGPU, graph_validation};

fn cpu_benchmark(
    _gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let mut intermediate_output: Tensor2D = Tensor2D::default();
    if !graph_validation::validate_graph_operators(graph) {
        panic!("graph::runner::cpu_benchmark() was given an invalid graph!");
    }

    for operator in graph {
        match operator {
            Empty => {}
            HostToDevice { input } => {
                intermediate_output = input.clone();
            }
            DeviceToHost => {}
            Linear { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                Tensor2D::linear_optimized(
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                );
                intermediate_output = temp_output;
            }
            ReLU => {
                Tensor2D::relu_inplace_inline(&mut intermediate_output);
            }
            Softmax => {
                Tensor2D::softmax_inplace_inline(&mut intermediate_output);
            }
            LinearReLUFused { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                Tensor2D::linear_optimized_relu(
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                );
                intermediate_output = temp_output;
            }
            LinearReLUSoftmaxFused { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                Tensor2D::linear_relu_softmax_fused_fission(
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                );
                intermediate_output = temp_output;
            }
        }
    }

    *output = intermediate_output;
}

fn cpu_graph_benchmark(
    _gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = true;
    let mut graph_runner: GraphRunner = GraphRunner::new(graph, fuse_operators);
    *output = graph_runner.run();
}

fn immediate_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let mut intermediate_output: Tensor2D = Tensor2D::default();
    if !graph_validation::validate_graph_operators(graph) {
        panic!("graph::runner::immediate_benchmark() was given an invalid graph!");
    }

    for operator in graph {
        match operator {
            Empty => {}
            HostToDevice { input } => {
                intermediate_output = input.clone();
            }
            DeviceToHost => {}
            Linear { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                pollster::block_on(immediate::nodes::linear_from_tensor_2d(
                    gpu_handles,
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                ));
                intermediate_output = temp_output;
            }
            ReLU => {
                pollster::block_on(immediate::nodes::relu_inplace_from_tensor_2d(
                    gpu_handles,
                    &mut intermediate_output,
                ));
            }
            Softmax => {
                let mut temp_output: Tensor2D = Tensor2D::new(
                    0.0,
                    intermediate_output.row_count,
                    intermediate_output.column_count,
                );
                pollster::block_on(immediate::nodes::softmax_from_tensor_2d(
                    gpu_handles,
                    &intermediate_output,
                    &mut temp_output,
                ));
                intermediate_output = temp_output;
            }
            LinearReLUFused { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                pollster::block_on(immediate::nodes::linear_with_relu_from_tensor_2d(
                    gpu_handles,
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                ));
                intermediate_output = temp_output;
            }
            LinearReLUSoftmaxFused { weights, bias } => {
                let mut temp_output: Tensor2D =
                    Tensor2D::new(0.0, bias.row_count, bias.column_count);
                pollster::block_on(immediate::nodes::linear_relu_softmax_fused_from_tensor_2d(
                    gpu_handles,
                    &intermediate_output,
                    weights,
                    bias,
                    &mut temp_output,
                ));
                intermediate_output = temp_output;
            }
        }
    }

    *output = intermediate_output;
}

fn graph_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = false;
    let cache_elements: bool = false;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, 1));
}

fn graph_fused_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = true;
    let cache_elements: bool = false;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, 1));
}

fn graph_cached_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = false;
    let cache_elements: bool = true;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, 1));
}

fn graph_cached_fused_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    _iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = true;
    let cache_elements: bool = true;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, 1));
}

fn graph_loop_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = false;
    let cache_elements: bool = false;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, iteration_count));
}

fn graph_loop_fused_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = true;
    let cache_elements: bool = false;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, iteration_count));
}

fn graph_loop_cached_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = false;
    let cache_elements: bool = true;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, iteration_count));
}

fn graph_loop_cached_fused_benchmark(
    gpu_handles: &GPUHandles,
    graph: &Vec<GraphOperator>,
    iteration_count: usize,
    output: &mut Tensor2D,
) {
    let fuse_operators: bool = true;
    let cache_elements: bool = true;
    let mut graph_runner: GraphRunnerGPU =
        GraphRunnerGPU::new(gpu_handles, graph, fuse_operators, cache_elements);
    *output = pollster::block_on(graph_runner.run(gpu_handles, iteration_count));
}

fn graph_benchmarks(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec![
        "graph::runner::cpu".to_string(),
        "graph::runner::cpu_graph".to_string(),
        "graph::runner::immediate".to_string(),
        "graph::runner::graph".to_string(),
        "graph::runner::graph_fused".to_string(),
        "graph::runner::graph_cached".to_string(),
        "graph::runner::graph_cached_fused".to_string(),
        "graph::runner::graph_loop".to_string(),
        "graph::runner::graph_loop_fused".to_string(),
        "graph::runner::graph_loop_cached".to_string(),
        "graph::runner::graph_loop_cached_fused".to_string(),
    ];

    let functions: Vec<(
        GraphFunction,
        fn(&GPUHandles, &Vec<GraphOperator>, usize, &mut Tensor2D),
    )> = vec![
        (GraphFunction::Cpu, cpu_benchmark),
        (GraphFunction::Cpu, cpu_graph_benchmark),
        (GraphFunction::Immediate, immediate_benchmark),
        (GraphFunction::Graph, graph_benchmark),
        (GraphFunction::Graph, graph_fused_benchmark),
        (GraphFunction::Graph, graph_cached_benchmark),
        (GraphFunction::Graph, graph_cached_fused_benchmark),
        (GraphFunction::GraphLoop, graph_loop_benchmark),
        (GraphFunction::GraphLoop, graph_loop_fused_benchmark),
        (GraphFunction::GraphLoop, graph_loop_cached_benchmark),
        (GraphFunction::GraphLoop, graph_loop_cached_fused_benchmark),
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    let measure_depth: bool = false;
    benchmark_function_vector_gpu_graph(
        config,
        names.clone(),
        gpu_handles,
        &functions,
        &mut all_measurements,
        measure_depth,
    );

    draw_benchmark_plot(
        format!(
            "Graphs Benchmark - Size(x) - Depth {}",
            config.default_graph_layer_count
        )
        .as_str(),
        "benchmarks/graphs/",
        "graphs_size_benchmark.png",
        all_measurements,
        config.log_scale,
    );

    let mut all_measurements: Vec<PerformanceMeasurements> =
    vec![PerformanceMeasurements::default(); functions.len()];

    let measure_depth: bool = true;
    benchmark_function_vector_gpu_graph(
        config,
        names,
        gpu_handles,
        &functions,
        &mut all_measurements,
        measure_depth,
    );

    draw_benchmark_plot(
        format!(
            "Graphs Benchmark - Depth(x) - Size {}",
            config.default_graph_operator_size
        )
        .as_str(),
        "benchmarks/graphs/",
        "graphs_depth_benchmark.png",
        all_measurements,
        config.log_scale,
    );


    // Two more plots which remove the CPU and Immediate mode
    let names: Vec<String> = vec![
        "graph::runner::graph".to_string(),
        "graph::runner::graph_fused".to_string(),
        "graph::runner::graph_cached".to_string(),
        "graph::runner::graph_cached_fused".to_string(),
        "graph::runner::graph_loop".to_string(),
        "graph::runner::graph_loop_fused".to_string(),
        "graph::runner::graph_loop_cached".to_string(),
        "graph::runner::graph_loop_cached_fused".to_string(),
    ];

    let functions: Vec<(
        GraphFunction,
        fn(&GPUHandles, &Vec<GraphOperator>, usize, &mut Tensor2D),
    )> = vec![
        (GraphFunction::Graph, graph_benchmark),
        (GraphFunction::Graph, graph_fused_benchmark),
        (GraphFunction::Graph, graph_cached_benchmark),
        (GraphFunction::Graph, graph_cached_fused_benchmark),
        (GraphFunction::GraphLoop, graph_loop_benchmark),
        (GraphFunction::GraphLoop, graph_loop_fused_benchmark),
        (GraphFunction::GraphLoop, graph_loop_cached_benchmark),
        (GraphFunction::GraphLoop, graph_loop_cached_fused_benchmark),
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    let measure_depth: bool = false;
    benchmark_function_vector_gpu_graph(
        config,
        names.clone(),
        gpu_handles,
        &functions,
        &mut all_measurements,
        measure_depth,
    );

    draw_benchmark_plot(
        format!(
            "Graphs Only Benchmark - Size(x) - Depth {}",
            config.default_graph_layer_count
        )
        .as_str(),
        "benchmarks/graphs/",
        "graphs_only_size_benchmark.png",
        all_measurements,
        config.log_scale,
    );

    let mut all_measurements: Vec<PerformanceMeasurements> =
    vec![PerformanceMeasurements::default(); functions.len()];

    let measure_depth: bool = true;
    benchmark_function_vector_gpu_graph(
        config,
        names,
        gpu_handles,
        &functions,
        &mut all_measurements,
        measure_depth,
    );

    draw_benchmark_plot(
        format!(
            "Graphs Only Benchmark - Depth(x) - Size {}",
            config.default_graph_operator_size
        )
        .as_str(),
        "benchmarks/graphs/",
        "graphs_only_depth_benchmark.png",
        all_measurements,
        config.log_scale,
    );

}

pub async fn execute(gpu_handles: &GPUHandles, config: &Configuration) {
    if config.run_performance_benchmark {
        graph_benchmarks(config, gpu_handles);
        return;
    }

    let input: Tensor2D = Tensor2D::new(0.1, 10, 10);

    let weights_a: Tensor2D = Tensor2D::new(0.02, 10, 10);
    let bias_a: Tensor2D = Tensor2D::new(0.01, 10, 10);

    let weights_b: Tensor2D = Tensor2D::new(0.04, 10, 10);
    let bias_b: Tensor2D = Tensor2D::new(0.02, 10, 10);

    let weights_c: Tensor2D = Tensor2D::new(0.06, 10, 10);
    let bias_c: Tensor2D = Tensor2D::new(0.03, 10, 10);

    let output_cpu: Tensor2D = Tensor2D::linear(&input, &weights_a, &bias_a);
    let output_cpu: Tensor2D = Tensor2D::relu(&output_cpu);
    let output_cpu: Tensor2D = Tensor2D::linear(&output_cpu, &weights_b, &bias_b);
    let output_cpu: Tensor2D = Tensor2D::relu(&output_cpu);
    let output_cpu: Tensor2D = Tensor2D::linear(&output_cpu, &weights_c, &bias_c);
    let output_cpu: Tensor2D = Tensor2D::relu(&output_cpu);
    let output_cpu: Tensor2D = Tensor2D::softmax(&output_cpu);

    let graph_operators: Vec<GraphOperator> = vec![
        HostToDevice { input },
        Linear {
            weights: weights_a,
            bias: bias_a,
        },
        ReLU,
        Linear {
            weights: weights_b,
            bias: bias_b,
        },
        ReLU,
        Linear {
            weights: weights_c,
            bias: bias_c,
        },
        ReLU,
        Softmax,
        DeviceToHost,
    ];

    let fuse_operators: bool = true;
    let cache_elements: bool = true;
    // let mut graph_runner: GraphRunner = GraphRunner::new(&gpu_handles, graph_operators, fuse_operators, cache_elements);
    // let output: Tensor2D = graph_runner.run(&gpu_handles).await;
    let mut graph_runner: GraphRunner = GraphRunner::new(&graph_operators, fuse_operators);
    let output: Tensor2D = graph_runner.run();
    println!("cpu output: {:?}", output);

    let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
    println!("cpu difference: {:?}", difference);
    println!(
        "cpu difference abs sum: {:?}",
        difference.data.iter().map(|x| x.abs()).sum::<f32>()
    );

    let mut graph_runner: GraphRunnerGPU = GraphRunnerGPU::new(
        gpu_handles,
        &graph_operators,
        fuse_operators,
        cache_elements,
    );
    let output: Tensor2D = graph_runner.run(gpu_handles, 1).await;
    println!("gpu output: {:?}", output);

    let difference: Tensor2D = Tensor2D::subtraction(&output_cpu, &output);
    println!("gpu difference: {:?}", difference);
    println!(
        "gpu difference abs sum: {:?}",
        difference.data.iter().map(|x| x.abs()).sum::<f32>()
    );
}
