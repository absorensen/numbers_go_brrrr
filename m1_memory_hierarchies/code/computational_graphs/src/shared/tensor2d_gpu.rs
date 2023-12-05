use futures_intrusive::channel::shared::{OneshotReceiver, OneshotSender};
use wgpu::{util::DeviceExt, Buffer, BufferAsyncError, BufferSlice, BufferView, CommandEncoder};

use super::{gpu_utilities::GPUHandles, tensor2d::Tensor2D};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LinearDimensions {
    pub data: [u32; 8],
}

#[derive(Debug)]
pub struct LinearUniform {
    pub dimensions: LinearDimensions,
    pub storage_buffer: Buffer,
}

impl LinearUniform {
    pub fn from_tensor_2d(
        handles: &GPUHandles,
        label: &str,
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &Tensor2D,
    ) -> Self {
        let dimensions: LinearDimensions = LinearDimensions {
            data: [
                input.row_count as u32,
                input.column_count as u32,
                weights.row_count as u32,
                weights.column_count as u32,
                bias.row_count as u32,
                bias.column_count as u32,
                output.row_count as u32,
                output.column_count as u32,
            ],
        };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&dimensions.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            dimensions,
            storage_buffer,
        }
    }

    pub fn from_tensor_2d_gpu(
        handles: &GPUHandles,
        label: &str,
        input: &Tensor2DGPU,
        weights: &Tensor2DGPU,
        bias: &Tensor2DGPU,
        output: &Tensor2DGPU,
    ) -> Self {
        let dimensions: LinearDimensions = LinearDimensions {
            data: [
                input.row_count as u32,
                input.column_count as u32,
                weights.row_count as u32,
                weights.column_count as u32,
                bias.row_count as u32,
                bias.column_count as u32,
                output.row_count as u32,
                output.column_count as u32,
            ],
        };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&dimensions.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            dimensions,
            storage_buffer,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        std::mem::size_of::<LinearDimensions>() as u64
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ReluDimensions {
    pub data: [u32; 2],
}

pub struct ReluUniform {
    pub dimensions: ReluDimensions,
    pub storage_buffer: Buffer,
}

impl ReluUniform {
    pub fn new(handles: &GPUHandles, label: &str, input: &Tensor2D) -> Self {
        let dimensions: ReluDimensions = ReluDimensions {
            data: [input.row_count as u32, input.column_count as u32],
        };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&dimensions.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            dimensions,
            storage_buffer,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        std::mem::size_of::<ReluDimensions>() as u64
    }

}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SumElements {
    pub data: [u32; 2],
}

pub struct SumUniform {
    pub elements: SumElements,
    pub storage_buffer: Buffer,
}

impl SumUniform {
    pub fn new(
        handles: &GPUHandles,
        label: &str,
        element_count: usize,
        workgroup_count: usize,
    ) -> Self {
        let elements: SumElements = SumElements {
            data: [element_count as u32, workgroup_count as u32],
        };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&elements.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            elements,
            storage_buffer,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        std::mem::size_of::<SumElements>() as u64
    }

}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SoftmaxDimensions {
    pub data: [u32; 1],
}

pub struct SoftmaxUniform {
    pub dimensions: SoftmaxDimensions,
    pub storage_buffer: Buffer,
}

impl SoftmaxUniform {
    pub fn new(handles: &GPUHandles, label: &str, element_count: usize) -> Self {
        let dimensions: SoftmaxDimensions = SoftmaxDimensions {
            data: [element_count as u32],
        };

        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&dimensions.data),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                });

        Self {
            dimensions,
            storage_buffer,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        std::mem::size_of::<SoftmaxDimensions>() as u64
    }

}

#[derive(Debug)]
pub struct Tensor2DGPU {
    pub staging_buffer: Buffer,
    pub storage_buffer: Buffer,
    pub row_count: usize,
    pub column_count: usize,
    pub element_size: usize,
    pub data: Tensor2D,
    pub live_data_on_device: bool,
    pub sender: Option<OneshotSender<Result<(), BufferAsyncError>>>,
    pub receiver: Option<OneshotReceiver<Result<(), BufferAsyncError>>>,
}

impl Tensor2DGPU {
    pub fn from_tensor2d(handles: &GPUHandles, label: &str, tensor: &Tensor2D) -> Self {
        let element_size: usize = std::mem::size_of::<f32>();
        let slice_size: usize = tensor.row_count * tensor.column_count * element_size;
        let size: u64 = slice_size as wgpu::BufferAddress;

        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer: Buffer = handles.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&tensor.data),
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST
                        | wgpu::BufferUsages::COPY_SRC,
                });

        // For tutorial brevity and ease of understanding we clone the tensor instead of holding a reference.
        // One option could be to hold a shared pointer such as RC<Tensor2D> or ARC<Tensor2D>
        Self {
            staging_buffer,
            storage_buffer,
            row_count: tensor.row_count,
            column_count: tensor.column_count,
            element_size,
            data: tensor.clone(),
            live_data_on_device: false,
            sender: None,
            receiver: None,
        }
    }

    pub fn new(
        handles: &GPUHandles,
        label: &str,
        scale: f32,
        row_count: usize,
        column_count: usize,
    ) -> Self {
        let element_size: usize = std::mem::size_of::<f32>();
        let slice_size: usize = row_count * column_count * element_size;
        let size: u64 = slice_size as wgpu::BufferAddress;

        let tensor: Tensor2D = Tensor2D::new(scale, row_count, column_count);

        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer: Buffer = handles.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer: Buffer =
            handles
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(&tensor.data),
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST
                        | wgpu::BufferUsages::COPY_SRC,
                });

        // For tutorial brevity and ease of understanding we clone the tensor instead of holding a reference.
        // One option could be to hold a shared pointer such as RC<Tensor2D> or ARC<Tensor2D>
        Self {
            staging_buffer,
            storage_buffer,
            row_count: tensor.row_count,
            column_count: tensor.column_count,
            element_size,
            data: tensor,
            live_data_on_device: false,
            sender: None,
            receiver: None,
        }
    }

    pub fn copy_from_gpu_mut(&mut self, encoder: &mut CommandEncoder) {
        if self.live_data_on_device {
            return;
        }
        encoder.copy_buffer_to_buffer(
            &self.storage_buffer,
            0,
            &self.staging_buffer,
            0,
            self.size(),
        );
        self.live_data_on_device = true;
    }

    pub async fn retrieve_results(&mut self) {
        if !self.live_data_on_device {
            println!("Already retrieved results from GPU, no reason to do it again.");
            return;
        }
        if self.receiver.is_none() {
            println!("Tried to get_results for a Tensor2DGPU, without having a receiver in place. You are probably calling the functions in the wrong order.");
            return;
        }

        let buffer_slice: BufferSlice = self.staging_buffer.slice(..);

        let result: Vec<f32> =
            if let Some(Ok(())) =
                self.receiver
                    .take() // Take ownership of the option and leave None in it's place. This is to enforce the fact that this is a oneshot receiver.
                    .expect("Took the receiver from the tensor in Tensor2DGPU::get_results, but the option did not contain a receiver.")
                    .receive().await {
            let data: BufferView = buffer_slice.get_mapped_range();
            let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

            drop(data);
            self.staging_buffer.unmap();
            self.live_data_on_device = false;
            result
        } else {
            panic!("Failed to retrieve results from the gpu!")
        };

        self.data.data = result;
    }

    #[inline(always)]
    pub fn linear_assert(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        debug_assert!(
            0 < input.row_count,
            "\ninput.row_count must be larger than 0. Current value: {}.",
            input.row_count
        );

        debug_assert!(
            0 < input.column_count,
            "\ninput.column_count must be larger than 0. Current value: {}.",
            input.column_count
        );

        debug_assert!(
            0 < weights.row_count,
            "\nweights.row_count must be larger than 0. Current value: {}.",
            weights.row_count
        );

        debug_assert!(
            0 < weights.column_count,
            "\nweights.column_count must be larger than 0. Current value: {}.",
            weights.column_count
        );

        debug_assert!(
            0 < bias.row_count,
            "\nbias.row_count must be larger than 0. Current value: {}.",
            bias.row_count
        );

        debug_assert!(
            0 < bias.column_count,
            "\nbias.column_count must be larger than 0. Current value: {}.",
            bias.column_count
        );

        debug_assert!(
            0 < output.row_count,
            "\noutput.row_count must be larger than 0. Current value: {}.",
            output.row_count
        );

        debug_assert!(
            0 < output.column_count,
            "\noutput.column_count must be larger than 0. Current value: {}.",
            output.column_count
        );

        debug_assert_eq!(
            bias.row_count,
            output.row_count,
            "\nMismatch - bias.row_count & output.row_count\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", 
            bias.row_count,
            bias.column_count,
            output.row_count,
            output.column_count);

        debug_assert_eq!(
            bias.column_count,
            output.column_count,
            "\nMismatch - bias.column_count & output.column_count\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", 
            bias.row_count,
            bias.column_count,
            output.row_count,
            output.column_count
        );

        debug_assert_eq!(
            input.row_count,
            output.row_count,
            "\nMismatch - input.row_count & output.row_count\ninput - rows: {} columns: {}.\n out - rows: {} columns: {}.", 
            input.row_count,
            input.column_count,
            output.row_count,
            output.column_count
        );

        debug_assert_eq!(
            weights.column_count,
            output.column_count,
            "\nMismatch - weights.column_count & output.column_count\nweights - rows: {} columns: {}.\n out - rows: {} columns: {}.", 
            weights.row_count,
            weights.column_count,
            output.row_count,
            output.column_count
        );

        debug_assert_eq!(
            input.column_count,
            weights.row_count,
            "\nMismatch - input.column_count & weights.row_count\ninput - rows: {} columns: {}.\n weights - rows: {} columns: {}.", 
            input.row_count,
            input.column_count,
            weights.row_count,
            weights.column_count
        );
    }

    #[inline(always)]
    pub fn linear_relu_softmax_assert(
        input: &Tensor2D,
        weights: &Tensor2D,
        bias: &Tensor2D,
        output: &mut Tensor2D,
    ) {
        debug_assert!(
            0 < input.row_count,
            "\ninput.row_count must be larger than 0. Current value: {}.",
            input.row_count
        );

        debug_assert!(
            0 < input.column_count,
            "\ninput.column_count must be larger than 0. Current value: {}.",
            input.column_count
        );

        debug_assert!(
            0 < weights.row_count,
            "\nweights.row_count must be larger than 0. Current value: {}.",
            weights.row_count
        );

        debug_assert!(
            0 < weights.column_count,
            "\nweights.column_count must be larger than 0. Current value: {}.",
            weights.column_count
        );

        debug_assert!(
            0 < bias.row_count,
            "\nbias.row_count must be larger than 0. Current value: {}.",
            bias.row_count
        );

        debug_assert!(
            0 < bias.column_count,
            "\nbias.column_count must be larger than 0. Current value: {}.",
            bias.column_count
        );

        debug_assert!(
            0 < output.row_count,
            "\noutput.row_count must be larger than 0. Current value: {}.",
            output.row_count
        );

        debug_assert!(
            0 < output.column_count,
            "\noutput.column_count must be larger than 0. Current value: {}.",
            output.column_count
        );

        debug_assert_eq!(
            bias.len(),
            output.len(),
            "\nMismatch - bias.len() & output.len()\nbias - rows: {} columns: {}.\n out - rows: {} columns: {}.", 
            bias.row_count,
            bias.column_count,
            output.row_count,
            output.column_count
        );
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.row_count * self.column_count
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        (self.row_count * self.column_count * self.element_size) as u64
    }
}
