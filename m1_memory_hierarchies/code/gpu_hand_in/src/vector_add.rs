use wgpu::{Buffer, util::DeviceExt, ShaderModule, ComputePipeline, BindGroupLayout, BindingResource, BindGroup, CommandEncoder, ComputePass, BufferSlice, BufferView};

use crate::{utility::{GPUHandles, create_shader_module, create_compute_pipeline, create_bind_group, mean_square_error, are_vectors_equivalent}, gpu_vector::GPUVector};

fn vector_add_cpu(input_a: &Vec<f32>, input_b: &Vec<f32>) -> Vec<f32> {
    assert!(input_a.len() == input_b.len());
    
    let mut output: Vec<f32> = vec![0.0; input_a.len()];
    
    for index in 0..input_a.len() {
        output[index] = input_a[index] + input_b[index];
    }

    output
}

// We create this struct to send global information (a uniform in graphics API parlance)
// to all threads. If this were a 2 dimensional example we could also send
// more dimensional information or whatever else we could think of.
// In general, we will need to reduce things to be closer to raw memory
// when we transfer data to be outside of Rust, which anything on the GPU is.
// It has no notion of the memory layout of Rust.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VectorAddElements {
    pub data: [u32; 1],
}

struct VectorAddUniform {
    pub storage_buffer: Buffer,
}

impl VectorAddUniform {
    pub fn new(handles: &GPUHandles, element_count: usize) -> Self {
        let elements: VectorAddElements = VectorAddElements {
            data: [element_count as u32],
        };

        // The storage buffer to actually run our shader on.
        // The data transfer is handled by create_buffer_init.
        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("VectorAddUniform"),
                    contents: bytemuck::cast_slice(&elements.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST
                        | wgpu::BufferUsages::COPY_SRC,
                });

        Self {
            storage_buffer,
        }
    }
}


pub fn vector_add(handles: &GPUHandles) -> bool {
    // Setup our CPU-side data
    let element_count: usize = 100000;
    let input_a: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32).collect();
    let input_b: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32).collect();
    let output: Vec<f32> = vec![0.0; element_count];

    let ground_truth: Vec<f32> = vector_add_cpu(&input_a, &input_b);


    // Create our uniform for telling the shader how big the vectors are.
    let uniform: VectorAddUniform = VectorAddUniform::new(handles, element_count);

    // Create the GPU vectors.
    // Note the true at the end of the output vector creation.
    // This will result in a staging_buffer being created, which we
    // can read from on the CPU.
    let input_a: GPUVector = GPUVector::new(&handles, input_a, "input_a", false);
    let input_b: GPUVector = GPUVector::new(&handles, input_b, "input_b", false);
    let mut output: GPUVector = GPUVector::new(&handles, output, "output", true);

    // We will use 32 threads in a work group/warp
    // We are doing this in 1 dimension, but could do it in
    // up to 3 dimensions.
    let block_size: usize = 32;
    let launch_blocks: u32 = ((element_count + block_size - 1) / block_size) as u32;

    // Compile the shader allowing us to call specific
    // functions when dispatching our compute shader.
    let cs_module: ShaderModule = create_shader_module(
        &handles,
        include_str!("vector_add.wgsl"),
    );

    // "main" is our entry point, as in the function that is
    // actually dispatched. That function can of course call
    // other functions.
    // In normal graphics a pipeline would have more than 1
    // shader, which gives the name more purpose, but
    // when just using a single shader, you can think of it
    // as that shader, with the entry point defined and
    // some accompanying state like bindings
    // which the pipeline keeps track of.
    let compute_pipeline: ComputePipeline =
    create_compute_pipeline(&handles, &cs_module, "main");

    // Instantiates the bind group, specifying the binding of buffers.
    // In this setup we can't just supply arbitrary buffers, they have to be bound
    // to specific slots before running it.
    let bind_group_layout: BindGroupLayout = compute_pipeline.get_bind_group_layout(0);
    let to_be_bound: Vec<(u32, BindingResource)> = vec![
        (0, uniform.storage_buffer.as_entire_binding()),
        (1, input_a.storage_buffer.as_entire_binding()),
        (2, input_b.storage_buffer.as_entire_binding()),
        (3, output.storage_buffer.as_entire_binding()),
    ];
    // We have defined our bindings, now create the bind group
    let bind_group: BindGroup = create_bind_group(&handles, &bind_group_layout, to_be_bound);

    // The command encode is essentially just a list of commands
    // we can accumulate and then send together to the GPU.
    // The command list emitted by the command encoder
    // will be added to the queue, once it has been finished.
    let mut encoder: CommandEncoder = handles
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    // This enclosing scope makes sure the ComputePass is dropped.
    {
        let mut cpass: ComputePass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("vector_add"),
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("add_vectors");
        cpass.dispatch_workgroups(launch_blocks, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        println!("Dispatching {} blocks of {} threads each for a total of {} threads processing {} elements of data!", launch_blocks, block_size, launch_blocks as usize * block_size, element_count);
    }

    // Add the command to the encoder copying output back to CPU
    output.transfer_from_gpu_to_cpu_mut(&mut encoder);

    // Finish our encoder and submit it to the queue.
    handles.queue.submit(Some(encoder.finish()));

    // Get a receiver channel that we can use for getting our data back to the CPU.
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();

    // Get ready to receive the data from the GPU.
    let staging_buffer: &Buffer = output.staging_buffer.as_ref().unwrap();
    let buffer_slice: BufferSlice = staging_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Synchronize with GPU - wait until it is done executing all commands.
    handles.device.poll(wgpu::Maintain::Wait);

    output.cpu_data =
        // Block on the receiver until it is ready to emit the data
        // from the GPU.
        if let Some(Ok(())) = pollster::block_on(receiver.receive()) {
            let data: BufferView = buffer_slice.get_mapped_range();
            // We actually receive this data as raw bytes &[u8] so we
            // recast it to f32.
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

            // Clean up and return the data.
            drop(data);
            staging_buffer.unmap();
            result
        } else {
            panic!("Failed to retrieve results from the gpu!")
        };

    println!("vector_add MSE: {}", mean_square_error(&ground_truth, &output.cpu_data));
    let success: bool = are_vectors_equivalent(&ground_truth, &output.cpu_data);
    println!("vector_add success: {}!", success);

    success
}