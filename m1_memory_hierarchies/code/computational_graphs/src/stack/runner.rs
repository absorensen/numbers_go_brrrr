use crate::shared::{
    benchmark_plot::draw_benchmark_plot,
    configuration::Configuration,
    performance_measurement::{benchmark_function_vector, PerformanceMeasurements},
    tensor2d::Tensor2D,
};

fn naive_linear_layer_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _output: Tensor2D = Tensor2D::linear_layer(input, weights, bias);
}

fn preallocated_linear_layer_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_preallocated(input, weights, bias, output);
}

fn inline_linear_layer_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_preallocated_inline(input, weights, bias, output);
}

fn local_accumulation_linear_layer_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_local_accumulation(input, weights, bias, output);
}

fn optimized_linear_layer_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_optimized(input, weights, bias, output);
}

fn linear_layer_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "naive".to_string(),
        "preallocated".to_string(),
        "inline".to_string(),
        "local_accumulation".to_string(),
        "optimized".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        naive_linear_layer_benchmark,
        preallocated_linear_layer_benchmark,
        inline_linear_layer_benchmark,
        local_accumulation_linear_layer_benchmark,
        optimized_linear_layer_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Linear Layer",
        "benchmarks/stack/",
        "linear_layer_cpu_benchmark_stack.png",
        all_measurements,
        config.log_scale,
    );
}

fn linear_layer(config: &Configuration) {
    if config.run_performance_benchmark {
        linear_layer_benchmark(config);
        return;
    }

    let input: Tensor2D = Tensor2D::new(0.5, 4, 3);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let weights: Tensor2D = Tensor2D::new(1.0, 3, 4);
    if 3 < config.debug_level {
        println!("Tensor2D weights");
        println!("{:?}", weights);
    }

    let bias: Tensor2D = Tensor2D::new(0.1, 4, 4);
    if 3 < config.debug_level {
        println!("Tensor2D bias");
        println!("{:?}", bias);
    }

    let output: Tensor2D = Tensor2D::linear_layer(&input, &weights, &bias);

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

fn naive_relu_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _: Tensor2D = Tensor2D::relu(input);
}

fn preallocated_relu_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::relu_preallocated(input, output);
}

fn inplace_relu_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::relu_inplace(input);
}

fn inline_relu_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::relu_inplace_inline(input);
}

fn relu_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "naive".to_string(),
        "preallocated".to_string(),
        "inplace".to_string(),
        "inline".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        naive_relu_benchmark,
        preallocated_relu_benchmark,
        inplace_relu_benchmark,
        inline_relu_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - ReLu",
        "benchmarks/stack/",
        "relu_cpu_stack.png",
        all_measurements,
        config.log_scale,
    );
}

fn relu(config: &Configuration) {
    if config.run_performance_benchmark {
        relu_benchmark(config);
        return;
    }

    let input: Tensor2D = Tensor2D::new(-0.5, 4, 3);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let output: Tensor2D = Tensor2D::relu(&input);

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

fn naive_softmax_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _: Tensor2D = Tensor2D::softmax(input);
}

fn preallocated_softmax_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::softmax_preallocated(input, output);
}

fn inplace_softmax_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::softmax_inplace(input);
}

fn inline_softmax_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::softmax_inplace_inline(input);
}

fn softmax_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "naive".to_string(),
        "preallocated".to_string(),
        "inplace".to_string(),
        "inline".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        naive_softmax_benchmark,
        preallocated_softmax_benchmark,
        inplace_softmax_benchmark,
        inline_softmax_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Softmax",
        "benchmarks/stack/",
        "softmax_cpu_stack.png",
        all_measurements,
        config.log_scale,
    );
}

fn softmax(config: &Configuration) {
    if config.run_performance_benchmark {
        softmax_benchmark(config);
        return;
    }

    let input: Tensor2D = Tensor2D::new(-0.5, 4, 3);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let output: Tensor2D = Tensor2D::softmax(&input);

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

// Maybe show the plot with all of these three set to inline always, just to show that it is not always beneficial to demand inlining
fn naive_linear_relu_softmax_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let output_linear: Tensor2D = Tensor2D::linear_layer(input, weights, bias);
    let output_relu: Tensor2D = Tensor2D::relu(&output_linear);
    let _output_softmax: Tensor2D = Tensor2D::softmax(&output_relu);
}

fn local_accumulation_linear_relu_softmax_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_local_accumulation(input, weights, bias, output);
    Tensor2D::relu_inplace_inline(output);
    Tensor2D::softmax_inplace_inline(output);
}

fn fused_fission_linear_relu_softmax_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_relu_softmax_fused_fission(input, weights, bias, output);
}

fn fused_linear_relu_softmax_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_relu_softmax_fused(input, weights, bias, output);
}

fn fused_fission_linear_relu_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_local_accumulation_relu(input, weights, bias, output);
}

fn fused_linear_relu_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_layer_optimized_relu(input, weights, bias, output);
}

fn linear_relu_softmax_fused_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "naive".to_string(),
        "inline-optimized".to_string(),
        "fused-fission".to_string(),
        "fused".to_string(),
        "linear-relu-fission".to_string(),
        "linear-relu-optimized".to_string(),
    ];
    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        naive_linear_relu_softmax_benchmark,
        local_accumulation_linear_relu_softmax_benchmark,
        fused_fission_linear_relu_softmax_benchmark,
        fused_linear_relu_softmax_benchmark,
        fused_fission_linear_relu_benchmark,
        fused_linear_relu_benchmark,
    ];
    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Fused Linear/ReLu/Softmax",
        "benchmarks/stack/",
        "linear_relu_softmax_fused_cpu_stack.png",
        all_measurements,
        config.log_scale,
    );
}

fn linear_relu_softmax_fused(config: &Configuration) {
    if config.run_performance_benchmark {
        linear_relu_softmax_fused_benchmark(config);
        return;
    }

    let input: Tensor2D = Tensor2D::new(0.5, 4, 3);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let weights: Tensor2D = Tensor2D::new(1.0, 3, 4);
    if 3 < config.debug_level {
        println!("Tensor2D weights");
        println!("{:?}", weights);
    }

    let bias: Tensor2D = Tensor2D::new(0.1, 4, 4);
    if 3 < config.debug_level {
        println!("Tensor2D bias");
        println!("{:?}", bias);
    }

    let mut output: Tensor2D = Tensor2D::new(0.1, 4, 4);
    if 3 < config.debug_level {
        println!("Tensor2D output");
        println!("{:?}", output);
    }

    Tensor2D::linear_relu_softmax_fused(&input, &weights, &bias, &mut output);

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

pub fn execute(config: &Configuration) {
    linear_layer(config);
    relu(config);
    softmax(config);
    linear_relu_softmax_fused(config);
}
