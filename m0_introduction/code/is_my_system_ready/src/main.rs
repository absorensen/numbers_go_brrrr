use wgpu::{Adapter, AdapterInfo, Instance, RequestAdapterOptions};

fn main() {
    env_logger::init();
    
    println!("Hello there!");

    println!("Performing self test to check system for compatibility...");
    // Instantiates instance of wgpu
    // This is just the most basic thing we need.
    // It allows us to check whether there is even a GPU backend to run something on.
    // We don't need the GPU in this module, but we need it in the next one.
    let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    // We request a high performance adapter. If you are on a laptop, you most likely don't
    // have one. If you have a gamer laptop or a desktop system you might have an integrated
    // GPU as well as a more powerful dedicated GPU, or just a powerful dedicated GPU.
    // The difference between the two is that the integrated GPU shares RAM with the CPU,
    // while the dediacated GPU has its own RAM and the CPU has to transfer data across 
    // something called a PCIe bus, which makes data transfer costlier.
    let adapter_request: RequestAdapterOptions = RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    };

    // `request_adapter` instantiates the general connection to the GPU
    let adapter_option: Option<Adapter> = pollster::block_on(instance.request_adapter(&adapter_request));

    // Check whether we actually got a functional GPU.
    match adapter_option {
        Some(adapter) => {
            // If we found a GPU, let's print some info so the user can check that there is a GPU, what type it is
            // and what the backend being used in the end is.
            let info: AdapterInfo = adapter.get_info();
            println!("Found GPU: {:?}", info);
        }
        None => {
            println!("Failed to find a usable GPU. This framework will only run CPU code.");
        }
    }
}
