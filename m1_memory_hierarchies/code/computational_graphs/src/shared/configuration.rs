#[derive(Clone, Debug, Default)]
pub struct Configuration {
    pub debug_level: u32,
    pub run_performance_benchmark: bool,
    pub loop_count: usize,
    pub loop_range: Vec<usize>,
    pub log_scale: bool,
    pub compatible_gpu_found: bool,
    pub warmup_gpu: bool,
    pub default_graph_layer_count: usize,
    pub default_graph_operator_size: usize,
    pub graph_depth_range: Vec<usize>,
}

impl Configuration {
    pub fn build(
        debug_level: u32,
        run_performance_benchmark: bool,
        loop_count: usize,
        loop_range: Vec<usize>,
        log_scale: bool,
    ) -> Self {
        Self {
            debug_level,
            run_performance_benchmark,
            loop_count,
            loop_range,
            log_scale,
            compatible_gpu_found: false,
            warmup_gpu: false,
            default_graph_layer_count: 0,
            default_graph_operator_size: 0,
            graph_depth_range: Vec::<usize>::new(),
        }
    }

    pub fn build_gpu(
        debug_level: u32,
        run_performance_benchmark: bool,
        loop_count: usize,
        loop_range: Vec<usize>,
        log_scale: bool,
        compatible_gpu_found: bool,
        warmup_gpu: bool,
        default_graph_layer_count: usize,
        default_graph_operator_size: usize,
        graph_depth_range: Vec<usize>,
    ) -> Self {
        assert_eq!(loop_range.len(), graph_depth_range.len());

        Self {
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
        }
    }
}
