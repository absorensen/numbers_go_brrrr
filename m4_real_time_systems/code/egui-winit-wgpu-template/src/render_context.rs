use std::num::NonZeroU64;

use log::info;
use wgpu::{util::DeviceExt, Adapter, AdapterInfo, BindGroup, BindGroupLayout, Buffer, Device, Instance, PipelineLayout, Queue, RenderPipeline, ShaderModule, Surface, SurfaceCapabilities, TextureFormat};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{uniforms::Uniforms, vertex::Vertex};

// Vertices for the simple triangle
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

// Vertex indices for the simple triangle
const INDICES: &[u16] = &[0, 1, 2, /* padding */ 0];

pub struct RenderContext {
    pub window: Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl RenderContext {
    // Unless you're into graphics programming, this
    // function probably isn't for you. Don't worry,
    // focus on structure, architecture and components.
    pub fn build(
        window: Window,
    ) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // The surface needs to live as long as the window that created it.
        let surface: Surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter: Adapter = pollster::block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))          
            .unwrap();

        let info: AdapterInfo = adapter.get_info();
        info!("Standalone Renderer Found GPU: {:?}", info);

        // Create the logical device and command queue
        // If using the same device and queue as the control panel the rendering fails.
        // Create the logical device and command queue
        let (device, queue): (Device, Queue) =
            pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            ))
            .expect("Failed to create device when building RenderEngine.");

        let size: PhysicalSize<u32> = window.inner_size();

        let surface_caps: SurfaceCapabilities = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format: TextureFormat = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // This template just uses wgsl, but you could swap this out for compiling hlsl, glsl or rust-gpu to spir-v.
        let shader: ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(
                std::borrow::Cow::Borrowed(
                    include_str!("shader.wgsl")
                )
            ),
        });

        let uniform: Uniforms = Uniforms {
            angle: 0.0,
        };

        let uniform_buffer: Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&[uniform.angle]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout: BindGroupLayout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: Some(NonZeroU64::new(4).unwrap()),
                },
                count: None,
                }],
            label: Some("uniform_bind_group_layout"),
            }
        );

        let uniform_bind_group: BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                // resource: uniform_buffer.as_entire_binding(),
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: Some(NonZeroU64::new(4).unwrap()),
                })
            }],
            layout: &uniform_bind_group_layout,
            label: Some("uniform_bind_group"),
        });

        let pipeline_layout: PipelineLayout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline: RenderPipeline =
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    //cull_mode: Some(wgpu::Face::Back), ATTENTION - If you are not doing a dumb triangle example, this should probably be enabled
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    // This render pipeline is also shared with the GUI rendering
                    // Due to it using sRGB colors we aren't allowed to do MSAA.
                    // We could get around this by using one render pipeline for
                    // GUI rendering and one render pipeline for the rest.
                    // Or we could switch to different colors than sRGB and use
                    // 1 < count when running with the --nogui flag.
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        // For the triangle
        let vertex_buffer: Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer: Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices: u32 = INDICES.len() as u32;

        Self { 
            window, 
            surface, 
            device, 
            queue,
            config,
            render_pipeline, 
            uniform, 
            uniform_buffer,
            uniform_bind_group, 
            vertex_buffer,
            index_buffer,
            num_indices
        }
    }
}
