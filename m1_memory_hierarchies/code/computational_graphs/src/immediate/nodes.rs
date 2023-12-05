use wgpu::{
    BindGroup, BindGroupLayout, BindingResource, BufferSlice, CommandEncoder, ComputePass,
    ComputePipeline, ShaderModule,
};

use crate::shared::{
    gpu_utilities::{create_bind_group, create_compute_pipeline, create_shader_module, GPUHandles},
    tensor2d::Tensor2D,
    tensor2d_gpu::{LinearUniform, ReluUniform, SoftmaxUniform, SumUniform, Tensor2DGPU},
};

pub async fn linear(
    gpu_handles: &GPUHandles,
    entry_point: &str,
    input: &Tensor2DGPU,
    weights: &Tensor2DGPU,
    bias: &Tensor2DGPU,
    output: &mut Tensor2DGPU,
) {
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

    let cs_module: ShaderModule = create_shader_module(
        gpu_handles,
        include_str!("../shared/shaders/linear.wgsl"),
    );
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, entry_point);

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

    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("linear_immediate"),
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("linear_immediate");
        cpass.dispatch_workgroups(launch_blocks_x, launch_blocks_y, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    output.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = output.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    output.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    output.retrieve_results().await;
}

pub async fn linear_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2DGPU::linear_assert(input, weights, bias, output);

    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let weights_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "weights", weights);
    let bias_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "bias", bias);
    let mut output_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "output", output);

    linear(
        gpu_handles,
        "main",
        &input_device,
        &weights_device,
        &bias_device,
        &mut output_device,
    )
    .await;
    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub fn linear_from_tensor_2d_blocking(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    pollster::block_on(linear_from_tensor_2d(
        gpu_handles,
        input,
        weights,
        bias,
        output,
    ));
}

pub async fn linear_with_relu_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2DGPU::linear_assert(input, weights, bias, output);

    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let weights_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "weights", weights);
    let bias_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "bias", bias);
    let mut output_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "output", output);

    linear(
        gpu_handles,
        "main_with_relu",
        &input_device,
        &weights_device,
        &bias_device,
        &mut output_device,
    )
    .await;
    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub async fn relu_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    output: &mut Tensor2D,
) {
    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let mut output_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "output", output);
    relu(gpu_handles, &input_device, &mut output_device).await;
    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub async fn relu(
    gpu_handles: &GPUHandles,
    input_device: &Tensor2DGPU,
    output_device: &mut Tensor2DGPU,
) {
    let uniform_device: ReluUniform =
        ReluUniform::new(gpu_handles, "Relu Uniform", &input_device.data);

    let cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/relu.wgsl"));
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, "main");

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input_device.storage_buffer.as_entire_binding()),
        (2, output_device.storage_buffer.as_entire_binding()),
    ];
    let bind_group: BindGroup = create_bind_group(gpu_handles, &bind_group_layout, to_be_bound);

    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass: ComputePass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Relu Immediate");
        cpass.dispatch_workgroups(
            (input_device.row_count + 31 / 32) as u32,
            (input_device.column_count + 31 / 32) as u32,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }
    output_device.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = output_device.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    output_device.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    output_device.retrieve_results().await;
}

pub async fn relu_inplace_from_tensor_2d(gpu_handles: &GPUHandles, data: &mut Tensor2D) {
    let mut data_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", data);
    relu_inplace(gpu_handles, &mut data_device).await;
    if data_device.live_data_on_device {
        data_device.retrieve_results().await;
    }
    *data = data_device.data.clone();
}

pub async fn relu_inplace(gpu_handles: &GPUHandles, data_device: &mut Tensor2DGPU) {
    let uniform_device: ReluUniform =
        ReluUniform::new(gpu_handles, "Relu Uniform", &data_device.data);

    let cs_module: ShaderModule = create_shader_module(
        gpu_handles,
        include_str!("../shared/shaders/relu_inline.wgsl"),
    );
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, "main");

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, data_device.storage_buffer.as_entire_binding()),
    ];
    let bind_group: BindGroup = create_bind_group(gpu_handles, &bind_group_layout, to_be_bound);

    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass: ComputePass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Relu Inplace Immediate");
        cpass.dispatch_workgroups(
            (data_device.row_count + 31 / 32) as u32,
            (data_device.column_count + 31 / 32) as u32,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }
    data_device.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = data_device.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    data_device.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    data_device.retrieve_results().await;
}

// Do an alternative version that does tree reduction with a loop, but with a single shader.
pub async fn sum(
    gpu_handles: &GPUHandles,
    input_device: &Tensor2DGPU,
    output_device: &mut Tensor2DGPU,
) -> f32 {
    let uniform_device: SumUniform =
        SumUniform::new(gpu_handles, "Sum Uniform", input_device.len(), 1);

    let cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/sum.wgsl"));
    let compute_pipeline: ComputePipeline =
        create_compute_pipeline(gpu_handles, &cs_module, "single_pass_sum");

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input_device.storage_buffer.as_entire_binding()),
        (2, output_device.storage_buffer.as_entire_binding()),
    ];
    let bind_group: BindGroup = create_bind_group(gpu_handles, &bind_group_layout, to_be_bound);

    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass: ComputePass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Sum Immediate");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    output_device.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = output_device.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    output_device.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    output_device.retrieve_results().await;

    output_device.data.data[0]
}

pub async fn sum_from_tensor_2d(gpu_handles: &GPUHandles, input: &Tensor2D) -> f32 {
    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let output_element_count: usize = 1;
    let mut output_device: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "output", 0.0, output_element_count, 1);
    sum(gpu_handles, &input_device, &mut output_device).await
}

pub async fn softmax_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    output: &mut Tensor2D,
) {
    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let mut output_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "output", output);
    softmax(gpu_handles, &input_device, &mut output_device).await;
    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub async fn softmax(
    gpu_handles: &GPUHandles,
    input_device: &Tensor2DGPU,
    output_device: &mut Tensor2DGPU,
) {
    let uniform_device: SoftmaxUniform =
        SoftmaxUniform::new(gpu_handles, "Softmax Uniform", input_device.len());
    let global_max_device: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "Softmax Global Max", 0.0, 1, 1);
    let global_offset_device: Tensor2DGPU =
        Tensor2DGPU::new(gpu_handles, "Softmax Global Offset", 0.0, 1, 1);

    let cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/softmax.wgsl"));

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input_device.storage_buffer.as_entire_binding()),
        (2, global_max_device.storage_buffer.as_entire_binding()),
    ];

    // Instantiates the bind group, once again specifying the binding of buffers.
    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let max_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &cs_module, "single_pass_max");
        let max_bind_group_layout: BindGroupLayout = max_compute_pipeline.get_bind_group_layout(0);
        let max_bind_group: BindGroup =
            create_bind_group(gpu_handles, &max_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Max"),
        });
        cpass.set_pipeline(&max_compute_pipeline);
        cpass.set_bind_group(0, &max_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Max");
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input_device.storage_buffer.as_entire_binding()),
        (2, global_max_device.storage_buffer.as_entire_binding()),
        (3, global_offset_device.storage_buffer.as_entire_binding()),
    ];
    {
        let sum_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &cs_module, "single_pass_sum");
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

    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform_device.storage_buffer.as_entire_binding()),
        (1, input_device.storage_buffer.as_entire_binding()),
        (3, global_offset_device.storage_buffer.as_entire_binding()),
        (4, output_device.storage_buffer.as_entire_binding()),
    ];
    let block_size: usize = 32;
    {
        let map_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &cs_module, "map");
        let map_bind_group_layout: BindGroupLayout = map_compute_pipeline.get_bind_group_layout(0);
        let map_bind_group: BindGroup =
            create_bind_group(gpu_handles, &map_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Map"),
        });
        cpass.set_pipeline(&map_compute_pipeline);
        cpass.set_bind_group(0, &map_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Map");
        cpass.dispatch_workgroups(
            ((input_device.len() + block_size - 1) / block_size) as u32,
            1,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }

    output_device.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = output_device.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    output_device.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    output_device.retrieve_results().await;
}

pub async fn linear_relu_softmax_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2DGPU::linear_relu_softmax_assert(input, weights, bias, output);

    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let weights_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "weights", weights);
    let bias_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "bias", bias);
    let mut intermediate_device: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "intermediate",
        0.0,
        bias.row_count,
        bias.column_count,
    );
    let mut output_device: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "output",
        0.0,
        bias.row_count * bias.column_count,
        1,
    );

    linear(
        gpu_handles,
        "main",
        &input_device,
        &weights_device,
        &bias_device,
        &mut intermediate_device,
    )
    .await;
    relu_inplace(gpu_handles, &mut intermediate_device).await;
    softmax(gpu_handles, &intermediate_device, &mut output_device).await;

    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub fn linear_relu_softmax_from_tensor_2d_blocking(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
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

pub async fn linearrelu_softmax_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2DGPU::linear_relu_softmax_assert(input, weights, bias, output);

    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let weights_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "weights", weights);
    let bias_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "bias", bias);
    let mut intermediate_device: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "intermediate",
        0.0,
        bias.row_count,
        bias.column_count,
    );
    let mut output_device: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "output",
        0.0,
        bias.row_count * bias.column_count,
        1,
    );

    linear(
        gpu_handles,
        "main_with_relu",
        &input_device,
        &weights_device,
        &bias_device,
        &mut intermediate_device,
    )
    .await;
    softmax(gpu_handles, &intermediate_device, &mut output_device).await;

    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub fn linearrelu_softmax_from_tensor_2d_blocking(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
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

pub async fn linear_relu_softmax_fused(
    gpu_handles: &GPUHandles,
    input: &Tensor2DGPU,
    weights: &Tensor2DGPU,
    bias: &Tensor2DGPU,
    output: &mut Tensor2DGPU,
) {
    let intermediate: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "intermediate",
        0.0,
        bias.row_count,
        bias.column_count,
    );
    let linear_entry_point: &str = "main_with_relu";

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

    let linear_cs_module: ShaderModule = create_shader_module(
        gpu_handles,
        include_str!("../shared/shaders/linear.wgsl"),
    );
    let softmax_cs_module: ShaderModule =
        create_shader_module(gpu_handles, include_str!("../shared/shaders/softmax.wgsl"));

    let mut encoder: CommandEncoder = gpu_handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let to_be_bound: Vec<(u32, BindingResource)> = vec![
            (0, linear_uniform.storage_buffer.as_entire_binding()),
            (1, input.storage_buffer.as_entire_binding()),
            (2, weights.storage_buffer.as_entire_binding()),
            (3, bias.storage_buffer.as_entire_binding()),
            (4, intermediate.storage_buffer.as_entire_binding()),
        ];

        let linear_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &linear_cs_module, linear_entry_point);
        let linear_bind_group_layout: BindGroupLayout =
            linear_compute_pipeline.get_bind_group_layout(0);
        let linear_bind_group: BindGroup =
            create_bind_group(gpu_handles, &linear_bind_group_layout, to_be_bound);
        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("linear_immediate"),
        });
        cpass.set_pipeline(&linear_compute_pipeline);
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

        let max_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &softmax_cs_module, "single_pass_max");
        let max_bind_group_layout: BindGroupLayout = max_compute_pipeline.get_bind_group_layout(0);
        let max_bind_group: BindGroup =
            create_bind_group(gpu_handles, &max_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Max"),
        });
        cpass.set_pipeline(&max_compute_pipeline);
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
            create_compute_pipeline(gpu_handles, &softmax_cs_module, "single_pass_sum");
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

        let map_compute_pipeline: ComputePipeline =
            create_compute_pipeline(gpu_handles, &softmax_cs_module, "map");
        let map_bind_group_layout: BindGroupLayout = map_compute_pipeline.get_bind_group_layout(0);
        let map_bind_group: BindGroup =
            create_bind_group(gpu_handles, &map_bind_group_layout, to_be_bound);

        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Softmax - Map"),
        });
        cpass.set_pipeline(&map_compute_pipeline);
        cpass.set_bind_group(0, &map_bind_group, &[]);
        cpass.insert_debug_marker("Softmax Immediate - Map");
        cpass.dispatch_workgroups(
            ((intermediate.len() + block_size - 1) / block_size) as u32,
            1,
            1,
        ); // Number of cells to run, the (x,y,z) size of item being processed
    }

    output.copy_from_gpu_mut(&mut encoder);

    gpu_handles.queue.submit(Some(encoder.finish()));

    let buffer_slice: BufferSlice = output.staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    output.receiver = Some(receiver);

    gpu_handles.device.poll(wgpu::Maintain::Wait);

    output.retrieve_results().await;
}

pub async fn linear_relu_softmax_fused_from_tensor_2d(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
    weights: &Tensor2D,
    bias: &Tensor2D,
    output: &mut Tensor2D,
) {
    Tensor2DGPU::linear_relu_softmax_assert(input, weights, bias, output);

    let input_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "input", input);
    let weights_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "weights", weights);
    let bias_device: Tensor2DGPU = Tensor2DGPU::from_tensor2d(gpu_handles, "bias", bias);
    let mut output_device: Tensor2DGPU = Tensor2DGPU::new(
        gpu_handles,
        "output",
        0.0,
        bias.row_count * bias.column_count,
        1,
    );

    linear_relu_softmax_fused(
        gpu_handles,
        &input_device,
        &weights_device,
        &bias_device,
        &mut output_device,
    )
    .await;

    if output_device.live_data_on_device {
        output_device.retrieve_results().await;
    }
    *output = output_device.data.clone();
}

pub fn linear_relu_softmax_fused_from_tensor_2d_blocking(
    gpu_handles: &GPUHandles,
    input: &Tensor2D,
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
