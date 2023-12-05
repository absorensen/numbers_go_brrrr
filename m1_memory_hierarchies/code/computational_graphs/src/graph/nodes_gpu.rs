use core::panic;
use std::collections::HashMap;

use wgpu::{
    BindGroup, BindGroupLayout, BindingResource, CommandEncoder, ComputePass, ComputePipeline,
    ShaderModule,
};

use crate::shared::{
    gpu_utilities::{create_bind_group, create_compute_pipeline, create_shader_module, GPUHandles},
    tensor2d_gpu::{LinearUniform, ReluUniform, SoftmaxUniform, Tensor2DGPU},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum NodeOperatorGPU {
    HostToDevice,
    DeviceToHost,
    DeviceToDevice,
    Linear,
    ReLU,
    Softmax,
    LinearReLU,
    LinearReLUSoftmax,
}

#[derive(Debug)]
pub struct NodeGPU {
    pub name: String,
    pub operator: NodeOperatorGPU,
    pub buffer_indices: Vec<usize>,
}

impl NodeGPU {
    pub fn new(name: String, operator: NodeOperatorGPU, buffer_indices: Vec<usize>) -> Self {
        NodeGPU {
            name,
            operator,
            buffer_indices,
        }
    }
}

// Linear Layer
pub fn build_linear_elements(
    gpu_handles: &GPUHandles,
    shader_cache: &mut HashMap<String, ShaderModule>,
    pipeline_cache: &mut HashMap<String, ComputePipeline>,
    use_fused_with_relu: bool,
) {
    let key: String = "Linear".to_string();

    let cs_module: ShaderModule = create_shader_module(
        gpu_handles,
        include_str!("../shared/shaders/linear.wgsl"),
    );

    let entry_point: &str = "main";
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

    shader_cache.insert(key.clone(), cs_module);
    pipeline_cache.insert(key, compute_pipeline);

    if use_fused_with_relu {
        let cs_module: ShaderModule = create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/linear.wgsl"),
        );
        let key: String = "LinearReLU".to_string();
        let entry_point: &str = "main_with_relu";
        let compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &cs_module, entry_point);

        shader_cache.insert(key.clone(), cs_module);
        pipeline_cache.insert(key, compute_pipeline);
    }
}

pub fn linear(
    gpu_handles: &GPUHandles,
    use_cache: bool,
    shader_cache: &HashMap<String, ShaderModule>,
    pipeline_cache: &HashMap<String, ComputePipeline>,
    node: &NodeGPU,
    data_buffers: &[Tensor2DGPU],
    encoder: &mut CommandEncoder,
    use_fused_with_relu: bool,
) {
    if node.buffer_indices.len() != 4 {
        panic!(
            "nodes::linear function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let input: &Tensor2DGPU = &data_buffers[node.buffer_indices[0]];
    let weights: &Tensor2DGPU = &data_buffers[node.buffer_indices[1]];
    let bias: &Tensor2DGPU = &data_buffers[node.buffer_indices[2]];
    let output: &Tensor2DGPU = &data_buffers[node.buffer_indices[3]];

    // Normally these would be right next to the lines where they are used
    // but this section is based on user input and can cause errors.
    // It is placed here for visibility.
    let block_size: usize = 8;
    let launch_blocks_x: u32 = ((output.row_count + block_size - 1) / block_size) as u32;
    let launch_blocks_y: u32 = ((output.column_count + block_size - 1) / block_size) as u32;

    let uniform_device: LinearUniform = LinearUniform::from_tensor_2d_gpu(
        gpu_handles,
        "Linear Layer Uniform",
        input,
        weights,
        bias,
        output,
    );

    let shader_module: Option<ShaderModule> = if use_cache {
        None
    } else {
        Some(create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/linear.wgsl"),
        ))
    };

    let cs_module: &ShaderModule = if use_cache {
        let key: &str = if use_fused_with_relu {
            "LinearReLU"
        } else {
            "Linear"
        };
        if shader_cache.contains_key(key) {
            &shader_cache[key]
        } else {
            panic!("Tried to get a cached {} shader in graph::nodes::linear(), but failed to find it in the shader cache!", key);
        }
    } else {
        shader_module
            .as_ref()
            .expect("Failed to get a reference to compute shader module")
    };

    let pipeline: Option<ComputePipeline> = if !use_cache {
        let entry_point: &str = if use_fused_with_relu {
            "main_with_relu"
        } else {
            "main"
        };
        Some(create_compute_pipeline(gpu_handles, cs_module, entry_point))
    } else {
        None
    };

    let compute_pipeline: &ComputePipeline = if use_cache {
        let key: &str = if use_fused_with_relu {
            "LinearReLU"
        } else {
            "Linear"
        };
        if pipeline_cache.contains_key(key) {
            &pipeline_cache[key]
        } else {
            panic!("Tried to get a cached {} pipeline in graph::nodes::linear(), but failed to find it in the pipeline cache!", key);
        }
    } else {
        pipeline
            .as_ref()
            .expect("Failed to get a reference to compute pipeline in graph::nodes::linear")
    };

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input.storage_buffer.as_entire_binding()),
        (2, weights.storage_buffer.as_entire_binding()),
        (3, bias.storage_buffer.as_entire_binding()),
        (4, output.storage_buffer.as_entire_binding()),
    ];
    let bind_group: BindGroup = create_bind_group(gpu_handles, &bind_group_layout, to_be_bound);

    {
        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some(if use_fused_with_relu {
                "linear_relu_graph"
            } else {
                "linear_graph"
            }),
        });
        cpass.set_pipeline(compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("linear_graph");
        cpass.dispatch_workgroups(launch_blocks_x, launch_blocks_y, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
}

// ReLU
pub fn build_relu_elements(
    gpu_handles: &GPUHandles,
    shader_cache: &mut HashMap<String, ShaderModule>,
    pipeline_cache: &mut HashMap<String, ComputePipeline>,
) {
    let key: String = "ReLU".to_string();

    let cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/relu.wgsl"));

    let entry_point: &str = "main";
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

    shader_cache.insert(key.clone(), cs_module);
    pipeline_cache.insert(key, compute_pipeline);
}

pub fn relu(
    gpu_handles: &GPUHandles,
    use_cache: bool,
    shader_cache: &HashMap<String, ShaderModule>,
    pipeline_cache: &HashMap<String, ComputePipeline>,
    node: &NodeGPU,
    data_buffers: &[Tensor2DGPU],
    encoder: &mut CommandEncoder,
) {
    if node.buffer_indices.len() != 2 {
        panic!(
            "nodes::relu function expected 1 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let input: &Tensor2DGPU = &data_buffers[node.buffer_indices[0]];
    let output: &Tensor2DGPU = &data_buffers[node.buffer_indices[1]];

    let uniform: ReluUniform = ReluUniform::new(gpu_handles, "Relu Uniform", &input.data);

    // let cs_module: ShaderModule =
    //     create_shader_module(gpu_handles, include_str!("../shared/shaders/relu.wgsl"));
    // let compute_pipeline: ComputePipeline =
    //     create_compute_pipeline(gpu_handles, &cs_module, "main");

    let shader_module: Option<ShaderModule> = if use_cache {
        None
    } else {
        Some(create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/relu.wgsl"),
        ))
    };

    let cs_module: &ShaderModule = if use_cache {
        let key: &str = "ReLU";
        if shader_cache.contains_key(key) {
            &shader_cache[key]
        } else {
            panic!("Tried to get a cached {} shader in graph::nodes::relu(), but failed to find it in the shader cache!", key);
        }
    } else {
        shader_module
            .as_ref()
            .expect("Failed to get a reference to compute shader module in graph::nodes::relu")
    };

    let pipeline: Option<ComputePipeline> = if use_cache {
        None
    } else {
        Some(create_compute_pipeline(gpu_handles, cs_module, "main"))
    };
    let compute_pipeline: &ComputePipeline = if use_cache {
        let key: &str = "ReLU";
        if pipeline_cache.contains_key(key) {
            &pipeline_cache[key]
        } else {
            panic!("Tried to get a cached {} pipeline in graph::nodes::relu(), but failed to find it in the pipeline cache!", key);
        }
    } else {
        pipeline
            .as_ref()
            .expect("Failed to get a reference to compute pipeline in graph::nodes::relu")
    };

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform.storage_buffer.as_entire_binding()),
        (1, input.storage_buffer.as_entire_binding()),
        (2, output.storage_buffer.as_entire_binding()),
    ];
    let bind_group: BindGroup = create_bind_group(gpu_handles, &bind_group_layout, to_be_bound);

    {
        let mut cpass: ComputePass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Relu Graph");
        cpass.dispatch_workgroups(
            (input.row_count + 31 / 32) as u32,
            (input.column_count + 31 / 32) as u32,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }
}

// Softmax
pub fn build_softmax_elements(
    gpu_handles: &GPUHandles,
    shader_cache: &mut HashMap<String, ShaderModule>,
    pipeline_cache: &mut HashMap<String, ComputePipeline>,
) {
    let cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/softmax.wgsl"));
    let entry_point: &str = "single_pass_max";
    let key: String = format!("Softmax_{}", entry_point);
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

    pipeline_cache.insert(key, compute_pipeline);

    let entry_point: &str = "single_pass_sum";
    let key: String = format!("Softmax_{}", entry_point);
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

    pipeline_cache.insert(key, compute_pipeline);

    let entry_point: &str = "map";
    let key: String = format!("Softmax_{}", entry_point);
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

    pipeline_cache.insert(key, compute_pipeline);

    shader_cache.insert("Softmax".to_string(), cs_module);
}

pub fn softmax(
    gpu_handles: &GPUHandles,
    use_cache: bool,
    shader_cache: &HashMap<String, ShaderModule>,
    pipeline_cache: &HashMap<String, ComputePipeline>,
    node: &NodeGPU,
    data_buffers: &[Tensor2DGPU],
    encoder: &mut CommandEncoder,
) {
    if node.buffer_indices.len() != 2 {
        panic!(
            "nodes::softmax function expected 2 input buffer, received {}",
            node.buffer_indices.len()
        );
    }

    let input: &Tensor2DGPU = &data_buffers[node.buffer_indices[0]];
    let output: &Tensor2DGPU = &data_buffers[node.buffer_indices[1]];

    let uniform: SoftmaxUniform = SoftmaxUniform::new(gpu_handles, "Softmax Uniform", input.len());
    let global_max: Tensor2DGPU = Tensor2DGPU::new(gpu_handles, "Softmax Global Max", 0.0, 1, 1);
    let global_offset: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "Softmax Global Offset", 0.0, 1, 1);

    let shader_module: Option<ShaderModule> = if use_cache {
        None
    } else {
        Some(create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/softmax.wgsl"),
        ))
    };
    let cs_module: &ShaderModule = if use_cache {
        let key: &str = "Softmax";
        if shader_cache.contains_key(key) {
            &shader_cache[key]
        } else {
            panic!("Tried to get a cached {} shader in graph::nodes::softmax(), but failed to find it in the shader cache!", key);
        }
    } else {
        shader_module
            .as_ref()
            .expect("Failed to get a reference to compute shader module")
    };

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform.storage_buffer.as_entire_binding()),
        (1, input.storage_buffer.as_entire_binding()),
        (2, global_max.storage_buffer.as_entire_binding()),
    ];

    // Instantiates the bind group, once again specifying the binding of buffers.
    {
        let max_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(
                gpu_handles,
                cs_module,
                "single_pass_max",
            ))
        };
        let max_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "Softmax_single_pass_max";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::softmax(), but failed to find it in the pipeline cache!", key);
            }
        } else {
            max_pipeline
                .as_ref()
                .expect("Failed to get a reference to compute pipeline in graph::nodes::softmax")
        };

        let max_bind_group_layout: BindGroupLayout = max_compute_pipeline.get_bind_group_layout(0);
        let max_bind_group: BindGroup =
            create_bind_group(gpu_handles, &max_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Max"),
        });
        cpass.set_pipeline(max_compute_pipeline);
        cpass.set_bind_group(0, &max_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Max");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform.storage_buffer.as_entire_binding()),
        (1, input.storage_buffer.as_entire_binding()),
        (2, global_max.storage_buffer.as_entire_binding()),
        (3, global_offset.storage_buffer.as_entire_binding()),
    ];
    {
        let sum_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(
                gpu_handles,
                cs_module,
                "single_pass_sum",
            ))
        };
        let sum_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "Softmax_single_pass_sum";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::softmax(), but failed to find it in the pipeline cache!", key);
            }
        } else {
            sum_pipeline
                .as_ref()
                .expect("Failed to get a reference to compute pipeline in graph::nodes::softmax")
        };

        let sum_bind_group_layout: BindGroupLayout = sum_compute_pipeline.get_bind_group_layout(0);
        let sum_bind_group: BindGroup =
            create_bind_group(gpu_handles, &sum_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Sum"),
        });
        cpass.set_pipeline(sum_compute_pipeline);
        cpass.set_bind_group(0, &sum_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Sum");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform.storage_buffer.as_entire_binding()),
        (1, input.storage_buffer.as_entire_binding()),
        (3, global_offset.storage_buffer.as_entire_binding()),
        (4, output.storage_buffer.as_entire_binding()),
    ];
    let block_size: usize = 32;
    {
        // let map_compute_pipeline: ComputePipeline =
        //     create_compute_pipeline(gpu_handles, &cs_module, "map");

        let map_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(gpu_handles, cs_module, "map"))
        };
        let map_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "Softmax_map";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::softmax(), but failed to find it in the pipeline cache!", key);
            }
        } else {
            map_pipeline
                .as_ref()
                .expect("Failed to get a reference to compute pipeline in graph::nodes::softmax")
        };

        let map_bind_group_layout: BindGroupLayout = map_compute_pipeline.get_bind_group_layout(0);
        let map_bind_group: BindGroup =
            create_bind_group(gpu_handles, &map_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Map"),
        });
        cpass.set_pipeline(map_compute_pipeline);
        cpass.set_bind_group(0, &map_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Map");
        cpass.dispatch_workgroups(((input.len() + block_size - 1) / block_size) as u32, 1, 1);
        // Number of cells to run, the (x,y,z) size of item being processed
    }
}

// LinearReLUSoftmax
pub fn linear_relu_softmax(
    gpu_handles: &GPUHandles,
    use_cache: bool,
    shader_cache: &HashMap<String, ShaderModule>,
    pipeline_cache: &HashMap<String, ComputePipeline>,
    node: &NodeGPU,
    data_buffers: &[Tensor2DGPU],
    encoder: &mut CommandEncoder,
) {
    if node.buffer_indices.len() != 4 {
        panic!(
            "nodes::linear_relu_softmax function expected 4 buffers, received {}",
            node.buffer_indices.len()
        );
    }

    let input: &Tensor2DGPU = &data_buffers[node.buffer_indices[0]];
    let weights: &Tensor2DGPU = &data_buffers[node.buffer_indices[1]];
    let bias: &Tensor2DGPU = &data_buffers[node.buffer_indices[2]];
    let output: &Tensor2DGPU = &data_buffers[node.buffer_indices[3]];

    let intermediate: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "intermediate",
        0.0,
        bias.row_count,
        bias.column_count,
    );

    let linear_block_size: usize = 8;
    let linear_launch_blocks_x: u32 =
        ((intermediate.row_count + linear_block_size - 1) / linear_block_size) as u32;
    let linear_launch_blocks_y: u32 =
        ((intermediate.column_count + linear_block_size - 1) / linear_block_size) as u32;

    let linear_uniform: LinearUniform = LinearUniform::from_tensor_2d_gpu(
        gpu_handles,
        "Linear Layer Uniform",
        input,
        weights,
        bias,
        &intermediate,
    );
    let softmax_uniform: SoftmaxUniform =
        SoftmaxUniform::new(gpu_handles, "Softmax Uniform", intermediate.len());
    let softmax_global_max: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "Softmax Global Max", 0.0, 1, 1);
    let softmax_global_offset: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "Softmax Global Offset", 0.0, 1, 1);

    let linear_shader_module: Option<ShaderModule> = if use_cache {
        None
    } else {
        Some(create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/linear.wgsl"),
        ))
    };

    let linear_cs_module: &ShaderModule = if use_cache {
        let key: &str = "LinearReLU";
        if shader_cache.contains_key(key) {
            &shader_cache[key]
        } else {
            panic!("Tried to get a cached {} shader in graph::nodes::linear_relu_softmax(), but failed to find it!", key);
        }
    } else {
        linear_shader_module.as_ref().expect(
            "Failed to get a reference to compute pipeline in graph::nodes::linear_relu_softmax",
        )
    };

    let softmax_shader_module: Option<ShaderModule> = if use_cache {
        None
    } else {
        Some(create_shader_module(
            gpu_handles,
            include_str!("../shared/shaders/softmax.wgsl"),
        ))
    };
    let softmax_cs_module: &ShaderModule = if use_cache {
        let key: &str = "Softmax";
        if shader_cache.contains_key(key) {
            &shader_cache[key]
        } else {
            panic!("Tried to get a cached {} shader in graph::nodes::linear_relu_softmax(), but failed to find it in the shader cache!", key);
        }
    } else {
        softmax_shader_module.as_ref().expect(
            "Failed to get a reference to compute pipeline in graph::nodes::linear_relu_softmax",
        )
    };

    {
        let linear_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(
                gpu_handles,
                linear_cs_module,
                "main_with_relu",
            ))
        };
        let linear_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "LinearReLU";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::linear_relu_softmax(), but failed to find it!", key);
            }
        } else {
            linear_pipeline.as_ref().expect("Failed to get a reference to compute pipeline in graph::nodes::linear_relu_softmax")
        };

        let to_be_bound: Vec<(u32, BindingResource)> = vec![
            (0, linear_uniform.storage_buffer.as_entire_binding()),
            (1, input.storage_buffer.as_entire_binding()),
            (2, weights.storage_buffer.as_entire_binding()),
            (3, bias.storage_buffer.as_entire_binding()),
            (4, intermediate.storage_buffer.as_entire_binding()),
        ];

        let linear_bind_group_layout: BindGroupLayout =
            linear_compute_pipeline.get_bind_group_layout(0);
        let linear_bind_group: BindGroup =
            create_bind_group(gpu_handles, &linear_bind_group_layout, to_be_bound);
        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("linear_immediate"),
        });
        cpass.set_pipeline(linear_compute_pipeline);
        cpass.set_bind_group(0, &linear_bind_group, &[]);
        cpass.insert_debug_marker("linear_immediate");
        cpass.dispatch_workgroups(linear_launch_blocks_x, linear_launch_blocks_y, 1);
        // Number of cells to run, the (x,y,z) size of item being processed
    }

    {
        let to_be_bound: Vec<(u32, BindingResource)> = vec![
            (0, softmax_uniform.storage_buffer.as_entire_binding()),
            (1, intermediate.storage_buffer.as_entire_binding()),
            (2, softmax_global_max.storage_buffer.as_entire_binding()),
        ];

        let max_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(
                gpu_handles,
                softmax_cs_module,
                "single_pass_max",
            ))
        };
        let max_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "Softmax_single_pass_max";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::softmax(), but failed to find it in the pipeline cache!", key);
            }
        } else {
            max_pipeline
                .as_ref()
                .expect("Failed to get a reference to compute pipeline in graph::nodes::softmax")
        };

        let max_bind_group_layout: BindGroupLayout = max_compute_pipeline.get_bind_group_layout(0);
        let max_bind_group: BindGroup =
            create_bind_group(gpu_handles, &max_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Max"),
        });
        cpass.set_pipeline(max_compute_pipeline);
        cpass.set_bind_group(0, &max_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Max");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    {
        let to_be_bound: Vec<(u32, BindingResource)> = vec![
            (0, softmax_uniform.storage_buffer.as_entire_binding()),
            (1, intermediate.storage_buffer.as_entire_binding()),
            (2, softmax_global_max.storage_buffer.as_entire_binding()),
            (3, softmax_global_offset.storage_buffer.as_entire_binding()),
        ];

        let sum_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, softmax_cs_module, "single_pass_sum");
        let sum_bind_group_layout: BindGroupLayout = sum_compute_pipeline.get_bind_group_layout(0);
        let sum_bind_group: BindGroup =
            create_bind_group(gpu_handles, &sum_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Sum"),
        });
        cpass.set_pipeline(&sum_compute_pipeline);
        cpass.set_bind_group(0, &sum_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Sum");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    let block_size: usize = 32;
    {
        let to_be_bound: Vec<(u32, BindingResource)> = vec![
            (0, softmax_uniform.storage_buffer.as_entire_binding()),
            (1, intermediate.storage_buffer.as_entire_binding()),
            (3, softmax_global_offset.storage_buffer.as_entire_binding()),
            (4, output.storage_buffer.as_entire_binding()),
        ];

        let map_pipeline: Option<ComputePipeline> = if use_cache {
            None
        } else {
            Some(create_compute_pipeline(
                gpu_handles,
                softmax_cs_module,
                "map",
            ))
        };
        let map_compute_pipeline: &ComputePipeline = if use_cache {
            let key: &str = "Softmax_map";
            if pipeline_cache.contains_key(key) {
                &pipeline_cache[key]
            } else {
                panic!("Tried to get a cached {} pipeline in graph::nodes::softmax(), but failed to find it in the pipeline cache!", key);
            }
        } else {
            map_pipeline
                .as_ref()
                .expect("Failed to get a reference to compute pipeline in graph::nodes::softmax")
        };

        let map_bind_group_layout: BindGroupLayout = map_compute_pipeline.get_bind_group_layout(0);
        let map_bind_group: BindGroup =
            create_bind_group(gpu_handles, &map_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Map"),
        });
        cpass.set_pipeline(map_compute_pipeline);
        cpass.set_bind_group(0, &map_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Map");
        cpass.dispatch_workgroups(
            ((intermediate.len() + block_size - 1) / block_size) as u32,
            1,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }
}
