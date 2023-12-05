use std::borrow::Cow;

use wgpu::{Queue, Device, Adapter, AdapterInfo, Instance, RequestAdapterOptions, ShaderModule, ComputePipeline, BindGroupLayout, BindingResource, BindGroup, BindGroupEntry, CommandEncoder, ComputePass, Buffer, BufferSlice, BufferView, util::DeviceExt};

use crate::gpu_vector::GPUVector;

// Try hovering your mouse over these types and see
// what the messages are!
pub struct GPUHandles {
    pub queue: Queue,
    pub device: Device,
    pub adapter: Adapter,
    pub adapter_info: AdapterInfo,
}

pub async fn self_test() -> bool {
    println!("Performing self test to check system for compatibility.");
    // Instantiates instance of wgpu
    let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    // We request an adapter with high performace. In the case of both
    // an integrated and a dedicated GPU, it should prefer the dedicated
    // GPU. We don't require a compatible surface, which is what would
    // allows us to present to screen. We are not doing graphics
    // so we don't need it.
    let adapter_request: RequestAdapterOptions = RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };

    // `request_adapter` instantiates the general connection to the GPU
    let adapter_option: Option<Adapter> = instance.request_adapter(&adapter_request).await;

    match adapter_option {
        Some(adapter) => {
            let info: AdapterInfo = adapter.get_info();
            println!("Found GPU: {:?}", info);
            true
        }
        None => {
            println!("Failed to find a usable GPU. This framework will only run CPU code.");
            false
        }
    }
}

pub async fn initialize_gpu() -> Option<GPUHandles> {
    // Instantiates instance of wgpu
    let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    // `request_adapter` instantiates the general connection to the GPU
    let adapter: Adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None, // We aren't doing any graphics
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find a usable GPU!");

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue): (Device, Queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let adapter_info: AdapterInfo = adapter.get_info();

    let gpu_handles: GPUHandles = GPUHandles {
        queue,
        device,
        adapter,
        adapter_info,
    };

    Some(gpu_handles)
}

// Compile our shader code.
pub fn create_shader_module(gpu_handles: &GPUHandles, shader: &str) -> ShaderModule {
    gpu_handles
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader)),
        })
}

// Create a compute pipeline.
pub fn create_compute_pipeline(
    gpu_handles: &GPUHandles,
    module: &ShaderModule,
    entry_point: &str,
) -> ComputePipeline {
    gpu_handles
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module,
            entry_point,
        })
}

// Create a bind group from a vector
// of bindings.
pub fn create_bind_group(
    gpu_handles: &GPUHandles,
    bind_group_layout: &BindGroupLayout,
    to_be_bound: Vec<(u32, BindingResource)>,
) -> BindGroup {
    let mut entries: Vec<BindGroupEntry> = vec![];

    for (binding, resource) in to_be_bound {
        let entry: BindGroupEntry = BindGroupEntry { binding, resource };
        entries.push(entry);
    }

    gpu_handles
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: entries.as_slice(),
        })
}

pub fn are_vectors_equivalent(a: &Vec<f32>, b: &Vec<f32>) -> bool {
    let epsilon: f32 = 0.001;

    for index in 0..a.len() {
        if epsilon < (a[index] - b[index]).abs() {
            return false;
        } 
    }

    true
}

pub fn mean_square_error(a: &Vec<f32>, b: &Vec<f32>) -> f64 {
    let mut result: f64 = 0.0;

    for index in 0..a.len() {
        let difference: f64 = a[index] as f64 - b[index] as f64;
        result += (difference * difference) / a.len() as f64;
    }

    result
}

// We create this struct to send global information (a uniform in graphics API parlance)
// to all threads. If this were a 2 dimensional example we could also send
// more dimensional information or whatever else we could think of.
// In general, we will need to reduce things to be closer to raw memory
// when we transfer data to be outside of Rust, which anything on the GPU is.
// It has no notion of the memory layout of Rust.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformElements {
    pub data: [u32; 4],
}

pub struct Uniform {
    pub storage_buffer: Buffer,
}

impl Uniform {
    pub fn new(
        handles: &GPUHandles,
        argument_0: usize,
        argument_1: usize,
        argument_2: usize,
        argument_3: usize
    ) -> Self {
        let elements: UniformElements = UniformElements {
            data: [
                argument_0 as u32, 
                argument_1 as u32, 
                argument_2 as u32, 
                argument_3 as u32
                ],
        };

        // The storage buffer to actually run our shader on.
        // The data transfer is handled by create_buffer_init.
        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform"),
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

pub fn run_compute_shader(
    handles: &GPUHandles,
    block_size_x: usize, 
    launch_blocks_x: u32,
    block_size_y: usize,
    launch_blocks_y: u32, 
    shader_file: &'static str, 
    shader_function: &str,
    uniform: &Uniform,
    input_a: &GPUVector,
    input_b: &GPUVector,
    output: &mut GPUVector,
) {
    
    // Compile the shader allowing us to call specific
    // functions when dispatching our compute shader.
    let cs_module: ShaderModule = create_shader_module(
        &handles,
        shader_file,
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
    create_compute_pipeline(&handles, &cs_module, shader_function);

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
        cpass.dispatch_workgroups(launch_blocks_x, launch_blocks_y, 1); // Number of cells to run, the (x,y,z) size of item being processed
        println!("Dispatching {} x blocks of {} threads and {} y blocks of {} threads each for a total of {} threads!", launch_blocks_x, block_size_x, launch_blocks_y, block_size_y, launch_blocks_x as usize * launch_blocks_y as usize * block_size_x * block_size_y);
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
}