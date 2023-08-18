use std::{
    iter,
    time::{Duration, Instant},
};

use crossbeam_channel::{Receiver, TryRecvError};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, CommandEncoder, Device, PipelineLayout,
    Queue, RenderPass, RenderPipeline, ShaderModule, Surface, SurfaceCapabilities,
    SurfaceConfiguration, SurfaceTexture, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowId},
};

use crate::{Command, GPUHandles};

// Hello Triangle based on https://github.com/sotrh/learn-wgpu/blob/master/code/beginner/tutorial4-buffer/src/lib.rs

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

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

const INDICES: &[u16] = &[0, 1, 2, /* padding */ 0];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    angle: f32,
}

pub struct RenderEngine {
    window: Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    uniform: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    shutdown: bool,
    rotate_triangle: bool,
    rotate_triangle_angle: f32,
    rotate_triangle_speed: f32,
    should_render: bool,
    time_of_last_render: Instant,
    pub window_id: WindowId,
}

impl RenderEngine {
    pub fn build(
        gpu_handles: &GPUHandles,
        event_loop: &EventLoop<()>,
        window_size: winit::dpi::PhysicalSize<u32>,
        window_padding: u32,
        window_offset: u32,
    ) -> Self {
        let window: Window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title("engine panel")
            .with_inner_size(window_size)
            .build(event_loop)
            .unwrap();
        window.set_outer_position(winit::dpi::PhysicalPosition::new(
            window_padding + window_offset,
            window_padding,
        ));

        let surface: Surface = unsafe { gpu_handles.instance.create_surface(&window) }.unwrap();

        // Create the logical device and command queue
        // If using the same device and queue as the control panel the rendering fails.
        // Create the logical device and command queue
        let (device, queue): (Device, Queue) =
            pollster::block_on(gpu_handles.adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            ))
            .expect("Failed to create device when building RenderEngine.");

        let size: PhysicalSize<u32> = window.inner_size();

        let caps: SurfaceCapabilities = surface.get_capabilities(&gpu_handles.adapter);
        let config: SurfaceConfiguration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // This template just uses wgsl, but you could swap this out for compiling hlsl, glsl or rust-gpu to spir-v.
        let shader: ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Initial values, which should be overwritten by the control panel.
        let rotate_triangle_angle: f32 = 0.0;
        let rotate_triangle_speed: f32 = 1.0;
        let uniform: Uniforms = Uniforms {
            angle: rotate_triangle_angle,
        };

        // Uniform buffer for the triangle rotation speed.
        let uniform_buffer: Buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform"),
            contents: bytemuck::cast_slice(&[uniform; 4]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });

        let uniform_buffer_bind_group_layout: BindGroupLayout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_buffer_layout"),
            });

        let uniform_bind_group: BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline_layout: PipelineLayout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_buffer_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline: RenderPipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
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

        let shutdown: bool = false;

        let window_id: WindowId = window.id();

        RenderEngine {
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
            num_indices,
            shutdown,
            rotate_triangle: false,
            rotate_triangle_angle,
            rotate_triangle_speed,
            should_render: true,
            time_of_last_render: Instant::now(),
            window_id,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.window.request_redraw();
        }
    }

    pub fn initialize(&mut self) {}

    pub fn shutdown(&mut self) {}

    // Fill this out yourself.
    fn load_scene(&mut self, _path: String) {}

    fn evaluate_rotating_triangle(&mut self, time_delta: &Duration) {
        // Calculate new rotation value and send it to the uniform buffer.
        // Replacing this with push constants might be a good idea.
        if self.rotate_triangle && self.rotate_triangle_speed != 0.0 {
            self.rotate_triangle_angle += self.rotate_triangle_speed * time_delta.as_secs_f32();
            if 360.0 < self.rotate_triangle_angle {
                self.rotate_triangle_angle -= 360.0;
            }
            if self.rotate_triangle_angle < 0.0 {
                self.rotate_triangle_angle += 360.0;
            }

            self.uniform.angle = self.rotate_triangle_angle;

            if self.should_render {
                self.queue.write_buffer(
                    &self.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[self.uniform]),
                );
            }
        }
    }

    pub fn render(&mut self) {
        let current_time: Instant = Instant::now();
        let time_delta: Duration = current_time - self.time_of_last_render;
        self.time_of_last_render = current_time;
        
        self.evaluate_rotating_triangle(&time_delta);
        
        if !self.should_render {
            return;
        }

        // Get the texture we want to write to and present to the screen.
        let output: SurfaceTexture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view: TextureView = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: CommandEncoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass: RenderPass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_pipeline(&self.render_pipeline);

            // Draw the triangle. For this you might eventually replace it with something more complicated.
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    pub fn render_loop(&mut self, command_receiver: Receiver<Command>) {
        while !self.shutdown {
            loop {
                let result: Result<Command, TryRecvError> = command_receiver.try_recv();
                if result.is_err() {
                    break;
                }
                let command: Command = result.unwrap();
                match command {
                    Command::Resize { new_size } => {
                        self.resize(new_size);
                    }
                    Command::RotateTriangle { value } => {
                        self.rotate_triangle = value;
                    }
                    Command::SetTriangleSpeed { speed } => {
                        self.rotate_triangle_speed = speed;
                    }
                    Command::LoadScene { path } => {
                        self.load_scene(path);
                    }
                    Command::Render { value } => {
                        self.should_render = value;
                    }
                    Command::KeyEventW => {}
                    Command::KeyEventA => {}
                    Command::KeyEventS => {}
                    Command::KeyEventD => {}
                    Command::KeyEventQ => {}
                    Command::KeyEventE => {}
                    Command::KeyEventComma => {}
                    Command::KeyEventPeriod => {}
                    Command::Shutdown { value } => {
                        if value {
                            self.shutdown = true;
                        }
                    }
                }
            }

            self.render();
        }
        self.shutdown();
    }
}
