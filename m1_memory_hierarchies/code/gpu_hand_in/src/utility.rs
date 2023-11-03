use std::borrow::Cow;

use wgpu::{Queue, Device, Adapter, AdapterInfo, Instance, RequestAdapterOptions, ShaderModule, ComputePipeline, BindGroupLayout, BindingResource, BindGroup, BindGroupEntry};

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
    let epsilon: f32 = 0.00001;

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