use wgpu::{Adapter, AdapterInfo, Instance, RequestAdapterOptions};

fn main() {
    env_logger::init();
    
    println!("Hello there!");

    println!("Performing self test to check system for compatibility...");
    // Instantiates instance of wgpu
    let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let adapter_request: RequestAdapterOptions = RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };

    // `request_adapter` instantiates the general connection to the GPU
    let adapter_option: Option<Adapter> = pollster::block_on(instance.request_adapter(&adapter_request));

    match adapter_option {
        Some(adapter) => {
            let info: AdapterInfo = adapter.get_info();
            println!("Found GPU: {:?}", info);
        }
        None => {
            println!("Failed to find a usable GPU. This framework will only run CPU code.");
        }
    }
}
