use std::time::{Duration, Instant};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::{
    configuration::Configuration, gpu_utilities::GPUHandles, graph_operators::GraphOperator,
    tensor2d::Tensor2D,
};

#[derive(Debug, Default, Clone)]
pub struct PerformanceMeasurements {
    pub name: String,
    pub sizes: Vec<usize>,
    pub normalized_times: Vec<f32>,
}

impl PerformanceMeasurements {
    // It is assumed that the times are microseconds
    pub fn build_from_measurements(
        name: String,
        sizes: Vec<usize>,
        times_microseconds: Vec<(u128, usize)>,
    ) -> Self {
        debug_assert_eq!(sizes.len(), times_microseconds.len());

        let mut normalized_times: Vec<f32> = vec![0.0; sizes.len()];

        for size_index in 0..times_microseconds.len() {
            let (timing, iterations): (u128, usize) = times_microseconds[size_index];
            normalized_times[size_index] = (timing as f64 / iterations as f64) as f32;
        }

        Self {
            name,
            sizes,
            normalized_times,
        }
    }

    pub fn zipped(&self) -> Vec<(usize, f32)> {
        let output: Vec<(usize, f32)> = self
            .sizes
            .iter()
            .copied()
            .zip(self.normalized_times.clone())
            .collect();

        output
    }
}

//
// Utility
//
pub fn benchmark_function_vector(
    config: &Configuration,
    names: Vec<String>,
    functions: Vec<fn(&mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)>,
    all_measurements: &mut Vec<PerformanceMeasurements>,
) {
    assert!(functions.len() == all_measurements.len());
    assert!(names.len() == all_measurements.len());

    let range_count: usize = config.loop_range.len();

    let test_count: usize = all_measurements.len();
    for test_index in 0..test_count {
        let mut performance_measurements: Vec<(u128, usize)> = vec![(0, 0); range_count];
        let mut total_elements_per_measurement: Vec<usize> = vec![0; range_count];
        let function = functions[test_index];
        for (size_index, size) in config.loop_range.iter().enumerate() {
            let size: usize = *size;
            let mut input: Tensor2D = Tensor2D::new(0.5, size, size);
            let weights: Tensor2D = Tensor2D::new(1.0, size, size);
            let bias: Tensor2D = Tensor2D::new(0.1, size, size);
            let mut out: Tensor2D = Tensor2D::new(0.0, size, size);

            let now: Instant = Instant::now();
            for _ in 0..config.loop_count {
                function(&mut input, &weights, &bias, &mut out);
            }
            let elapsed_time: Duration = now.elapsed();
            performance_measurements[size_index] = (elapsed_time.as_nanos(), config.loop_count);
            total_elements_per_measurement[size_index] = size * size;
        }
        let normalized_measurements: PerformanceMeasurements =
            PerformanceMeasurements::build_from_measurements(
                names[test_index].clone(),
                total_elements_per_measurement,
                performance_measurements,
            );
        all_measurements[test_index] = normalized_measurements;
    }
}

pub fn benchmark_function_vector_gpu(
    config: &Configuration,
    names: Vec<String>,
    gpu_handles: &GPUHandles,
    functions: Vec<fn(&GPUHandles, &mut Tensor2D, &Tensor2D, &Tensor2D, &mut Tensor2D)>,
    all_measurements: &mut Vec<PerformanceMeasurements>,
) {
    assert!(functions.len() == all_measurements.len());
    assert!(names.len() == all_measurements.len());

    let range_count: usize = config.loop_range.len();

    let test_count: usize = all_measurements.len();
    for test_index in 0..test_count {
        let mut performance_measurements: Vec<(u128, usize)> = vec![(0, 0); range_count];
        let mut total_elements_per_measurement: Vec<usize> = vec![0; range_count];
        let function = functions[test_index];
        for (size_index, size) in config.loop_range.iter().enumerate() {
            let size: usize = *size;
            let mut input: Tensor2D = Tensor2D::new(0.5, size, size);
            let weights: Tensor2D = Tensor2D::new(1.0, size, size);
            let bias: Tensor2D = Tensor2D::new(0.1, size, size);
            let mut out: Tensor2D = Tensor2D::new(0.0, size, size);

            let now: Instant = Instant::now();
            for _ in 0..config.loop_count {
                function(gpu_handles, &mut input, &weights, &bias, &mut out);
            }
            let elapsed_time: Duration = now.elapsed();
            performance_measurements[size_index] = (elapsed_time.as_nanos(), config.loop_count);
            total_elements_per_measurement[size_index] = size * size;
        }
        let normalized_measurements: PerformanceMeasurements =
            PerformanceMeasurements::build_from_measurements(
                names[test_index].clone(),
                total_elements_per_measurement,
                performance_measurements,
            );
        all_measurements[test_index] = normalized_measurements;
    }
}

#[derive(Clone)]
pub enum GraphFunction {
    Cpu,
    Immediate,
    Graph,
    GraphLoop,
}

fn benchmark_function_vector_gpu_graph_inner_loop(
    gpu_handles: &GPUHandles,
    config: &Configuration,
    measurement_index: usize,
    size: usize,
    depth: usize,
    function_type: &GraphFunction,
    function: fn(&GPUHandles, &Vec<GraphOperator>, usize, &mut Tensor2D),
    performance_measurements: &mut [(u128, usize)],
    total_elements_per_measurement: &mut [usize],
    measure_depth: bool,
) {
    let input: Tensor2D = Tensor2D::new(0.5, size, size);
    let mut graph: Vec<GraphOperator> = vec![GraphOperator::HostToDevice { input }];

    let mut rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64((depth * size) as u64);
    for _ in 0..depth {
        let weights: Tensor2D = Tensor2D::new(0.5, size, size);
        let bias: Tensor2D = Tensor2D::new(0.1, size, size);
        graph.push(GraphOperator::Linear { weights, bias });

        let layer_type: usize = rng.gen_range(0..2);
        if layer_type == 1 {
            graph.push(GraphOperator::ReLU);
        }
    }
    match graph[graph.len() - 1] {
        GraphOperator::ReLU => {}
        _ => graph.push(GraphOperator::ReLU),
    };

    graph.push(GraphOperator::Softmax);
    graph.push(GraphOperator::DeviceToHost);

    let mut out: Tensor2D = Tensor2D::new(0.0, size, size);
    match function_type {
        GraphFunction::Cpu | GraphFunction::Immediate | GraphFunction::Graph => {
            let now: Instant = Instant::now();
            for _ in 0..config.loop_count {
                function(gpu_handles, &graph, config.loop_count, &mut out);
            }
            let elapsed_time: Duration = now.elapsed();
            performance_measurements[measurement_index] =
                (elapsed_time.as_nanos(), config.loop_count);
        }
        GraphFunction::GraphLoop => {
            let now: Instant = Instant::now();
            function(gpu_handles, &graph, config.loop_count, &mut out);
            let elapsed_time: Duration = now.elapsed();
            performance_measurements[measurement_index] =
                (elapsed_time.as_nanos(), config.loop_count);
        }
    }
    if measure_depth {
        total_elements_per_measurement[measurement_index] = depth;
    } else {
        total_elements_per_measurement[measurement_index] = size * size;
    }
}

pub fn benchmark_function_vector_gpu_graph(
    config: &Configuration,
    names: Vec<String>,
    gpu_handles: &GPUHandles,
    functions: &Vec<(
        GraphFunction,
        fn(&GPUHandles, &Vec<GraphOperator>, usize, &mut Tensor2D),
    )>,
    all_measurements: &mut Vec<PerformanceMeasurements>,
    measure_depth: bool,
) {
    assert!(functions.len() == all_measurements.len());
    assert!(names.len() == all_measurements.len());
    assert!(4 < config.default_graph_layer_count);
    assert!(4 < config.default_graph_operator_size);
    assert!(4 < config.graph_depth_range.len());

    let range_count: usize = config.loop_range.len();

    for test_index in 0..functions.len() {
        let mut performance_measurements: Vec<(u128, usize)> = vec![(0, 0); range_count];
        let mut total_elements_per_measurement: Vec<usize> = vec![0; range_count];
        let (function_type, function): (
            &GraphFunction,
            fn(&GPUHandles, &Vec<GraphOperator>, usize, &mut Tensor2D),
        ) = (&functions[test_index].0, functions[test_index].1);

        if measure_depth {
            for (depth_index, depth) in config.graph_depth_range.iter().enumerate() {
                let size: usize = config.default_graph_operator_size;
                benchmark_function_vector_gpu_graph_inner_loop(
                    gpu_handles,
                    config,
                    depth_index,
                    size,
                    *depth,
                    function_type,
                    function,
                    &mut performance_measurements,
                    &mut total_elements_per_measurement,
                    measure_depth,
                );
            }
        } else {
            for (size_index, size) in config.loop_range.iter().enumerate() {
                let size: usize = *size;
                let depth: usize = config.default_graph_layer_count;
                benchmark_function_vector_gpu_graph_inner_loop(
                    gpu_handles,
                    config,
                    size_index,
                    size,
                    depth,
                    function_type,
                    function,
                    &mut performance_measurements,
                    &mut total_elements_per_measurement,
                    measure_depth,
                );
            }
        }
        let normalized_measurements: PerformanceMeasurements =
            PerformanceMeasurements::build_from_measurements(
                names[test_index].clone(),
                total_elements_per_measurement,
                performance_measurements,
            );
        all_measurements[test_index] = normalized_measurements;
    }
}
