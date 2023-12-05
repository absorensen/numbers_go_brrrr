use crate::shared::{
    benchmark_plot::draw_benchmark_plot,
    configuration::Configuration,
    performance_measurement::{benchmark_function_vector, PerformanceMeasurements},
    tensor2d::Tensor2D,
};

fn naive_linear_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _output: Tensor2D = Tensor2D::linear(input, weights, bias);
}

fn preallocated_linear_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_preallocated(input, weights, bias, output);
}

fn inline_linear_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_preallocated_inline(input, weights, bias, output);
}

fn local_accumulation_linear_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_local_accumulation(input, weights, bias, output);
}

fn optimized_linear_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_optimized(input, weights, bias, output);
}

fn linear_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "shared::tensor2d::linear".to_string(),
        "shared::tensor2d::linear_preallocated".to_string(),
        "shared::tensor2d::linear_preallocated_inline".to_string(),
        "shared::tensor2d::linear_local_accumulation".to_string(),
        "shared::tensor2d::linear_optimized".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        naive_linear_benchmark,
        preallocated_linear_benchmark,
        inline_linear_benchmark,
        local_accumulation_linear_benchmark,
        optimized_linear_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "CPU Benchmark - Linear",
        "benchmarks/cpu/",
        "cpu_linear_benchmark.png",
        all_measurements,
        config.log_scale,
    );
}

fn linear(config: &Configuration) {
    if config.run_performance_benchmark {
        linear_benchmark(config);
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

    let output: Tensor2D = Tensor2D::linear(&input, &weights, &bias);

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

fn relu_naive_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _: Tensor2D = Tensor2D::relu(input);
}

fn relu_preallocated_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::relu_preallocated(input, output);
}

fn relu_inplace_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::relu_inplace(input);
}

fn relu_inplace_inline_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::relu_inplace_inline(input);
}

fn relu_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "shared::tensor2d::relu".to_string(),
        "shared::tensor2d::relu_preallocated".to_string(),
        "shared::tensor2d::relu_inplace".to_string(),
        "shared::tensor2d::relu_inplace_inline".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        relu_naive_benchmark,
        relu_preallocated_benchmark,
        relu_inplace_benchmark,
        relu_inplace_inline_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "CPU Benchmark - ReLu",
        "benchmarks/cpu/",
        "cpu_relu_benchmark.png",
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

fn softmax_naive_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let _: Tensor2D = Tensor2D::softmax(input);
}

fn softmax_preallocated_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::softmax_preallocated(input, output);
}

fn softmax_inplace_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::softmax_inplace(input);
}

fn softmax_inplace_inline_benchmark(
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    Tensor2D::softmax_inplace_inline(input);
}

fn softmax_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "shared::tensor2d::softmax".to_string(),
        "shared::tensor2d::softmax_preallocated".to_string(),
        "shared::tensor2d::softmax_inplace".to_string(),
        "shared::tensor2d::softmax_inplace_inline".to_string(),
    ];

    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        softmax_naive_benchmark,
        softmax_preallocated_benchmark,
        softmax_inplace_benchmark,
        softmax_inplace_inline_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "CPU Benchmark - Softmax",
        "benchmarks/cpu/",
        "cpu_softmax_benchmark.png",
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
fn linear_relu_softmax_naive_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let output_linear: Tensor2D = Tensor2D::linear(input, weights, bias);
    let output_relu: Tensor2D = Tensor2D::relu(&output_linear);
    let _output_softmax: Tensor2D = Tensor2D::softmax(&output_relu);
}

fn linear_relu_softmax_local_accumulation_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_local_accumulation(input, weights, bias, output);
    Tensor2D::relu_inplace_inline(output);
    Tensor2D::softmax_inplace_inline(output);
}

fn linear_relu_softmax_fused_fission_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_relu_softmax_fused_fission(input, weights, bias, output);
}

fn linear_relu_softmax_fused_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_relu_softmax_fused(input, weights, bias, output);
}

fn linear_local_accumulation_relu_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_local_accumulation_relu(input, weights, bias, output);
}

fn linear_optimized_relu_benchmark(
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2D::linear_optimized_relu(input, weights, bias, output);
}

fn linear_relu_softmax_benchmark(config: &Configuration) {
    let names: Vec<String> = vec![
        "shared::tensor2d::linear_relu_softmax".to_string(),
        "shared::tensor2d::linear_relu_softmax_local_accumulation".to_string(),
        "shared::tensor2d::linear_relu_softmax_fused_fission".to_string(),
        "shared::tensor2d::linear_relu_softmax_fused".to_string(),
        "shared::tensor2d::linear_local_accumulation_relu".to_string(),
        "shared::tensor2d::linear_optimized_relu".to_string(),
    ];
    let functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        linear_relu_softmax_naive_benchmark,
        linear_relu_softmax_local_accumulation_benchmark,
        linear_relu_softmax_fused_fission_benchmark,
        linear_relu_softmax_fused_benchmark,
        linear_local_accumulation_relu_benchmark,
        linear_optimized_relu_benchmark,
    ];
    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector(config, names, functions, &mut all_measurements);

    draw_benchmark_plot(
        "CPU Benchmark - Fused Linear/ReLu/Softmax",
        "benchmarks/cpu/",
        "cpu_linear_relu_softmax_fused_benchmark.png",
        all_measurements,
        config.log_scale,
    );
}

fn linear_relu_softmax_fused(config: &Configuration) {
    if config.run_performance_benchmark {
        linear_relu_softmax_benchmark(config);
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
    linear(config);
    relu(config);
    softmax(config);
    linear_relu_softmax_fused(config);
}
