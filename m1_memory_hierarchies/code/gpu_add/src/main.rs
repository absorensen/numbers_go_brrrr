use std::{borrow::Cow, mem};

use wgpu::{
    Queue, 
    Device, 
    Adapter, 
    AdapterInfo, 
    Instance, 
    RequestAdapterOptions, 
    ShaderModule, 
    ComputePipeline, 
    BindGroupLayout, 
    BindingResource, 
    BindGroup, 
    BindGroupEntry, 
    Buffer, util::DeviceExt, CommandEncoder, ComputePass, BufferSlice, BufferView,
};

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

struct GPUVector {
    // Our initial, cpu-side data
    pub cpu_data: Vec<f32>,

    // The staging buffer which we back to the CPU with. It represents
    // memory CPU side. In a more complex setup we might have a staging
    // buffer GPU side, before transferring from the staging buffer
    // to the storage buffer which is accesible.
    // We only need the staging buffer if we transfer the data back to the CPU
    pub staging_buffer: Option<Buffer>,

    // The buffer that will be used for our vector addition compute shader
    // The transfer from our data vector is hidden by
    // create_buffer_init(). If we wanted more control and better performance
    // we would do this ourselves by using staging buffers and perhaps
    // asynchronous transfers.
    pub storage_buffer: Buffer,
}

impl GPUVector {
    pub fn new(handles: &GPUHandles, cpu_data: Vec<f32>, label: &str, ouput_buffer: bool) -> Self {
        let element_size: usize = std::mem::size_of::<f32>();
        let slice_size: usize = cpu_data.len() * element_size;
        let size: u64 = slice_size as wgpu::BufferAddress;


        // If we want to retrieve the GPU results to the CPU we 
        // create the staging buffer, but don't actually copy anything in there yet.
        // Note that we give the storage buffer hints to how this buffer will be used.
        let staging_buffer: Option<Buffer> = 
            if !ouput_buffer {
                None
            } else {
                Some(handles.device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }))
            };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&cpu_data),
                    usage: 
                        wgpu::BufferUsages::STORAGE |
                        wgpu::BufferUsages::COPY_DST | 
                        wgpu::BufferUsages::COPY_SRC,
                });

        GPUVector { cpu_data, staging_buffer, storage_buffer }
    }

    // For a bit more nuance to staging buffers and copy to copy 
    // https://www.reddit.com/r/wgpu/comments/13zqe1u/can_someone_please_explain_to_me_the_whole_buffer/
    pub fn transfer_from_gpu_to_cpu_mut(&mut self, encoder: &mut CommandEncoder) {
        // We copy from the shader-visible GPU storage buffer
        // to the CPU-visible staging buffer.
        if self.staging_buffer.is_some() {
            encoder.copy_buffer_to_buffer(
                self.storage_buffer.as_ref(),
                0,
                self.staging_buffer.as_ref().unwrap(),
                0,
                (self.cpu_data.len() * mem::size_of::<f32>()) as u64,
            );
        }
    }
}

fn main() {
    // Initialize the env_logger to get usueful messages from wgpu.
    env_logger::init();

    // Is there a compatible GPU on the system?
    // Use pollster::block_on to block on async functions.
    // Think of it like this - this is a function which
    // uses the GPU. With block_on() we are insisting
    // on waiting until all the interaction with the GPU
    // and the tasks set in motion on the GPU are finished.
    if !pollster::block_on(self_test()) {
        panic!("Was unable to confirm that your system is incompatible with this sample!");
    }

    // Setup our CPU-side data
    let element_count: usize = 100;
    let input_a: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32).collect();
    let input_b: Vec<f32> = (0..element_count).into_iter().map(|element| element as f32).collect();
    let output: Vec<f32> = vec![0.0; element_count];

    // Keep track of the handles to central stuff like device and queue.
    let handles: GPUHandles = pollster::block_on(initialize_gpu()).expect("Was unsuccesful in creating GPU Handles");

    // Create our uniform for telling the shader how big the vectors are.
    let uniform: VectorAddUniform = VectorAddUniform::new(&handles, element_count);

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
        include_str!("add_vectors.wgsl"),
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

    println!("Results were: {:?}", output.cpu_data);

}

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
// f bindings.
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
