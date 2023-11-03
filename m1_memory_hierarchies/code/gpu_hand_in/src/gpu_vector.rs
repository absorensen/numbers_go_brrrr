use std::mem;

use wgpu::{Buffer, util::DeviceExt, CommandEncoder};

use crate::utility::GPUHandles;

pub struct GPUVector {
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
                &self.storage_buffer,
                0,
                self.staging_buffer.as_ref().unwrap(),
                0,
                (self.cpu_data.len() * mem::size_of::<f32>()) as u64,
            );
        }
    }
}
