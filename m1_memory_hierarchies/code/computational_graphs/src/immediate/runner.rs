// https://blog.redwarp.app/image-filters/

use crate::shared::{
    benchmark_plot::draw_benchmark_plot,
    configuration::Configuration,
    gpu_utilities::{initialize_gpu, GPUHandles},
    performance_measurement::{benchmark_function_vector_gpu, PerformanceMeasurements},
    tensor2d::Tensor2D,
};

use super::nodes::{
    linear_layer_from_tensor_2d, linear_layer_with_relu_from_tensor_2d,
    linear_relu_softmax_from_tensor_2d, linear_relu_softmax_fused_from_tensor_2d,
    linearrelu_softmax_from_tensor_2d, relu_from_tensor_2d, relu_inplace_from_tensor_2d,
    softmax_from_tensor_2d, sum_from_tensor_2d,
};

fn immediate_linear_layer_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linear_layer_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

fn immediate_linear_layer_with_relu_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linear_layer_with_relu_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

fn linear_layer_benchmark(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec!["immediate".to_string(), "with_relu".to_string()];

    let functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        immediate_linear_layer_benchmark,
        immediate_linear_layer_with_relu_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector_gpu(config, names, gpu_handles, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Linear Layer - Immediate - GPU",
        "benchmarks/immediate/",
        "linear_layer_gpu_immediate.png",
        all_measurements,
        config.log_scale,
    );
}

async fn linear_layer(config: &Configuration, gpu_handles: &GPUHandles) {
    if config.run_performance_benchmark {
        linear_layer_benchmark(config, gpu_handles);
        return;
    }

    let input: Tensor2D = Tensor2D::new(0.5, 4, 4);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let weights: Tensor2D = Tensor2D::new(1.0, 4, 4);
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

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }

    linear_layer_from_tensor_2d(gpu_handles, &input, &weights, &bias, &mut output).await;

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

fn immediate_relu_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(relu_from_tensor_2d(gpu_handles, input, output));
}

fn immediate_relu_inplace_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    pollster::block_on(relu_inplace_from_tensor_2d(gpu_handles, input));
}

fn relu_benchmark(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec!["naive".to_string(), "inplace".to_string()];

    let functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> =
        vec![immediate_relu_benchmark, immediate_relu_inplace_benchmark];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector_gpu(config, names, gpu_handles, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - ReLu - Immediate - GPU",
        "benchmarks/immediate/",
        "relu_gpu_immediate.png",
        all_measurements,
        config.log_scale,
    );
}

async fn relu(config: &Configuration, gpu_handles: &GPUHandles) {
    if config.run_performance_benchmark {
        relu_benchmark(config, gpu_handles);
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

fn immediate_sum_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    _output: &mut Tensor2D,
) {
    let result: f32 = pollster::block_on(sum_from_tensor_2d(gpu_handles, input));
    let _x: f32 = 2.0 * result + 5.0;
}

fn sum_benchmark(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec!["naive".to_string()];

    let functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> =
        vec![immediate_sum_benchmark];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector_gpu(config, names, gpu_handles, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Sum - Immediate - GPU",
        "benchmarks/immediate/",
        "sum_gpu_immediate.png",
        all_measurements,
        config.log_scale,
    );
}

async fn sum(config: &Configuration, gpu_handles: &GPUHandles) {
    if config.run_performance_benchmark {
        sum_benchmark(config, gpu_handles);
        return;
    }

    let input: Tensor2D = Tensor2D::new(-0.5, 4, 3);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let output: f32 = sum_from_tensor_2d(gpu_handles, &input).await;

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }
}

fn immediate_softmax_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    _weights: &Tensor2D,
    _bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(softmax_from_tensor_2d(gpu_handles, input, output));
}

fn softmax_benchmark(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec!["naive".to_string()];

    let functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> =
        vec![immediate_softmax_benchmark];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector_gpu(config, names, gpu_handles, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Softmax - Immediate - GPU",
        "benchmarks/immediate/",
        "softmax_gpu_immediate.png",
        all_measurements,
        config.log_scale,
    );
}

async fn softmax(config: &Configuration, gpu_handles: &GPUHandles) {
    if config.run_performance_benchmark {
        softmax_benchmark(config, gpu_handles);
        return;
    }

    println!("Softmax function");
    let input: Tensor2D = Tensor2D::new(-0.5, 10, 1);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let mut output: Tensor2D = Tensor2D::new(0.0, 10, 1);
    if 3 < config.debug_level {
        println!("Tensor2D output");
        println!("{:?}", output);
    }

    pollster::block_on(softmax_from_tensor_2d(gpu_handles, &input, &mut output));

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

fn immediate_linear_relu_softmax_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linear_relu_softmax_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

fn immediate_linearrelu_softmax_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linearrelu_softmax_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

fn immediate_linear_relu_softmax_fused_benchmark(
    gpu_handles: &GPUHandles,
    input: &mut Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linear_relu_softmax_fused_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

fn linear_relu_softmax_fused_benchmark(config: &Configuration, gpu_handles: &GPUHandles) {
    let names: Vec<String> = vec![
        "linear_relu_softmax".to_string(),
        "linearrelu_softmax".to_string(),
        "linear_relu_softmax_fused".to_string(),
    ];

    let functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)> = vec![
        immediate_linear_relu_softmax_benchmark,
        immediate_linearrelu_softmax_benchmark,
        immediate_linear_relu_softmax_fused_benchmark,
    ];

    let mut all_measurements: Vec<PerformanceMeasurements> =
        vec![PerformanceMeasurements::default(); functions.len()];

    benchmark_function_vector_gpu(config, names, gpu_handles, functions, &mut all_measurements);

    draw_benchmark_plot(
        "Benchmark - Linear/ReLU/Softmax - Fused - Immediate - GPU",
        "benchmarks/immediate/",
        "linear_relu_softmax_fused_gpu_immediate.png",
        all_measurements,
        config.log_scale,
    );
}

async fn linear_relu_softmax_fused(config: &Configuration, gpu_handles: &GPUHandles) {
    if config.run_performance_benchmark {
        linear_relu_softmax_fused_benchmark(config, gpu_handles);
        return;
    }

    let input: Tensor2D = Tensor2D::new(0.5, 4, 4);
    if 3 < config.debug_level {
        println!("Tensor2D input");
        println!("{:?}", input);
    }

    let weights: Tensor2D = Tensor2D::new(1.0, 4, 4);
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

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }

    linear_relu_softmax_fused_from_tensor_2d(gpu_handles, &input, &weights, &bias, &mut output)
        .await;

    if 2 < config.debug_level {
        println!("Output");
        println!("{:?}", output);
    }

    let evaluation_sum: f32 = output.sum();
    if 1 < config.debug_level {
        println!("Evaluation sum: {:?}", evaluation_sum);
    }
}

pub async fn execute(gpu_handles: &GPUHandles, config: &Configuration) {
    linear_layer(config, gpu_handles).await;
    relu(config, gpu_handles).await;
    sum(config, gpu_handles).await;
    softmax(config, gpu_handles).await;
    linear_relu_softmax_fused(config, gpu_handles).await;
}
