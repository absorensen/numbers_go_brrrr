// Heavily based on: https://github.com/hasenbanck/egui_example/blob/master/src/main.rs

use std::{iter, time::Instant};

use crossbeam_channel::Sender;
use egui::{ClippedPrimitive, Context, FontDefinitions, FullOutput, Response, ScrollArea, Ui};
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use wgpu::{
    CommandEncoder, Surface, SurfaceCapabilities, SurfaceConfiguration, SurfaceTexture,
    TextureFormat, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowId},
};

use crate::{command::Command, GPUHandles};

pub struct ControlPanel {
    pub window_id: WindowId,
    // Rendering state
    pub window: Window,
    pub surface: wgpu::Surface,
    pub platform: Platform,
    config: wgpu::SurfaceConfiguration,
    render_pass: RenderPass,
    // All of our buttons' state
    should_render: bool,
    rotate_triangle: bool,
    triangle_speed: f32,
    scene_path: String,
}

impl ControlPanel {
    pub fn build(
        gpu_handles: &GPUHandles,
        event_loop: &EventLoop<()>,
        window_size: winit::dpi::PhysicalSize<u32>,
        window_padding: u32,
    ) -> Self {
        let window: Window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title("control panel")
            .with_inner_size(window_size)
            .build(event_loop)
            .unwrap();

        window.set_outer_position(winit::dpi::PhysicalPosition::new(
            window_padding,
            window_padding,
        ));

        let surface: Surface = unsafe { gpu_handles.instance.create_surface(&window) }.unwrap();

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

        surface.configure(&gpu_handles.device, &config);

        let platform: Platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let surface_format: TextureFormat =
            surface.get_capabilities(&gpu_handles.adapter).formats[0];
        let render_pass: RenderPass = RenderPass::new(&gpu_handles.device, surface_format, 1);

        let path: String = "".to_string();
        let window_id: WindowId = window.id();

        ControlPanel {
            window,
            surface,
            config,
            render_pass,
            platform,
            should_render: true,
            rotate_triangle: true,
            triangle_speed: 0.5,
            scene_path: path,
            window_id,
        }
    }

    // The control panel needs to send all of the relevant initial
    // values to the render engine, otherwise the values won't
    // be used until the buttons are used.
    pub fn initialize(&self, commands: &Sender<Command>) {
        commands
            .send(Command::RotateTriangle {
                value: self.rotate_triangle,
            })
            .unwrap();

        commands
            .send(Command::SetTriangleSpeed {
                speed: self.triangle_speed,
            })
            .unwrap();

        commands
            .send(Command::Render {
                value: self.should_render,
            })
            .unwrap();
    }

    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(device, &self.config);
        self.window.request_redraw();
    }

    pub fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }

    pub fn redraw(
        &mut self,
        commands: &Sender<Command>,
        has_focus: &mut bool,
        redraw_gui: &mut bool,
        start_time: &Instant,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.platform
            .update_time(start_time.elapsed().as_secs_f64());

        let output_frame: SurfaceTexture = self.get_current_texture();
        let output_view: TextureView = output_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Begin to draw the UI frame.
        self.platform.begin_frame();

        // Draw the control panel.
        self.ui(&self.platform.context(), commands, has_focus, redraw_gui);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output: FullOutput = self.platform.end_frame(Some(&self.window));
        let paint_jobs: Vec<ClippedPrimitive> =
            self.platform.context().tessellate(full_output.shapes);

        let mut encoder: CommandEncoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        // Upload all resources for the GPU.
        let screen_descriptor: ScreenDescriptor = ScreenDescriptor {
            physical_width: self.config.width,
            physical_height: self.config.height,
            scale_factor: self.window.scale_factor() as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta;
        self.render_pass
            .add_textures(device, queue, &tdelta)
            .expect("add texture ok");
        self.render_pass
            .update_buffers(device, queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        self.render_pass
            .execute(
                &mut encoder,
                &output_view,
                &paint_jobs,
                &screen_descriptor,
                Some(wgpu::Color::BLACK),
            )
            .unwrap();

        // Submit the commands.
        queue.submit(iter::once(encoder.finish()));

        // Redraw egui
        output_frame.present();

        // Cleanup
        self.render_pass
            .remove_textures(tdelta)
            .expect("remove texture ok");
    }

    fn ui(
        &mut self,
        context: &Context,
        commands: &Sender<Command>,
        has_focus: &mut bool,
        redraw_gui: &mut bool,
    ) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("control panel");

            // Basically all of our buttons
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    // Organize window button
                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }

                    // Dark/Light mode button
                    ui.horizontal(|ui: &mut Ui| {
                        ui.label("egui theme:");
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });

                    // Render button
                    // If not checked, the renderer won't redraw
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.checkbox(&mut self.should_render, "Render").changed() {
                            commands
                                .send(Command::Render {
                                    value: self.should_render,
                                })
                                .unwrap()
                        };
                    });

                    // Rotate triangle section
                    // Toggle, speed value and event handling
                    ui.horizontal(|ui: &mut Ui| {
                        if ui
                            .checkbox(&mut self.rotate_triangle, "Rotate Triangle")
                            .changed()
                        {
                            commands
                                .send(Command::RotateTriangle {
                                    value: self.rotate_triangle,
                                })
                                .unwrap()
                        };
                        ui.label("Triangle Speed");
                        let triangle_speed_response: Response = ui.add(
                            egui::widgets::DragValue::new(&mut self.triangle_speed)
                                .clamp_range(-std::f32::consts::TAU..=std::f32::consts::TAU)
                                .fixed_decimals(1)
                                .speed(0.1),
                        );

                        if triangle_speed_response.gained_focus() {
                            *has_focus = true;
                            *redraw_gui = true;
                        }
                        if triangle_speed_response.lost_focus() {
                            *has_focus = false;
                        }
                        if triangle_speed_response.changed() {
                            commands
                                .send(Command::SetTriangleSpeed {
                                    speed: self.triangle_speed,
                                })
                                .unwrap()
                        };
                    });

                    // Load scene section. You'll have to fill out this
                    // functionality yourself.
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Load Scene").changed() {
                            commands
                                .send(Command::LoadScene {
                                    path: self.scene_path.clone(),
                                })
                                .unwrap();
                            *redraw_gui = true;
                        };
                        ui.label("Path");

                        let text_edit_singleline_response: Response =
                            ui.text_edit_singleline(&mut self.scene_path);
                        if text_edit_singleline_response.gained_focus() {
                            *has_focus = true;
                            *redraw_gui = true;
                        }
                        if text_edit_singleline_response.lost_focus() {
                            *has_focus = false;
                        }
                    });

                    // This button opens a file dialog and 
                    // sets the scene_path to that path.
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Open file..").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.scene_path = path.display().to_string();
                            }
                        }
                    });
                });
            });
        });
    }
}
