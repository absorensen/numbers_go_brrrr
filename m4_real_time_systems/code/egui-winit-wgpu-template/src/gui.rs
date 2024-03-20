use crossbeam_channel::Sender;
use egui::epaint::Shadow;
use egui::{Align2, Context, Response, Ui, ViewportId, Visuals};
use egui_wgpu::renderer::ScreenDescriptor;
use egui_wgpu::Renderer;

use egui_winit::State;
use wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use winit::event::WindowEvent;
use winit::window::Window;

use crate::config::Config;
use crate::command::Command;

pub struct GuiRenderer {
    pub context: Context,
    state: State,
    renderer: Renderer,
}

impl GuiRenderer {
    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: &Window,
    ) -> GuiRenderer {
        let egui_context: Context = Context::default();
        let id: ViewportId = egui_context.viewport_id();

        const BORDER_RADIUS: f32 = 2.0;

        let visuals: Visuals = Visuals {
            window_rounding: egui::Rounding::same(BORDER_RADIUS),
            window_shadow: Shadow::NONE,
            ..Default::default()
        };

        egui_context.set_visuals(visuals);

        let egui_state: State = egui_winit::State::new(egui_context.clone(), id, &window, None, None);

        let egui_renderer: Renderer = egui_wgpu::renderer::Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
        );

        Self {
            context: egui_context,
            state: egui_state,
            renderer: egui_renderer,
        }
    }

    pub fn initialize(&self, app_config: &mut Config, commands: &Sender<Command>) {
        commands
            .send(Command::RotateTriangle {
                value: app_config.rotate_triangle,
            })
            .unwrap();

        commands
            .send(Command::SetTriangleSpeed {
                speed: app_config.triangle_speed,
            })
            .unwrap();
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
        commands: &Sender<Command>,
        app_config: &mut Config,
    ) {
        // self.state.set_pixels_per_point(window.scale_factor() as f32);
        let raw_input: egui::RawInput = self.state.take_egui_input(&window);
        let full_output: egui::FullOutput = self.context.run(raw_input, |_ui| {
            Self::ui(&self.context, app_config, commands);
        });

        self.state
            .handle_platform_output(&window, full_output.platform_output);

        let tris: Vec<egui::ClippedPrimitive> = 
            self
                .context
                .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(&device, &queue, *id, &image_delta);
        }

        self.renderer
            .update_buffers(&device, &queue, encoder, &tris, &screen_descriptor);
        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("egui main render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        self.renderer.render(&mut render_pass, &tris, &screen_descriptor);

        // Clean up render pass and textures
        drop(render_pass);
        for texture_id in &full_output.textures_delta.free {
            self.renderer.free_texture(texture_id)
        }
    }

    // Instead of communicating interactions through the sender
    // we could have returned a Vec<Command> instead, this requires
    // an allocation, though. We could find a data structure to reuse though.
    // But sending the commands through the sender allows these commands to
    // be handled in the render_engines loop just like the commands
    // sent by the event loop in the main thread.
    fn ui(
        ui: &Context,
        app_config: &mut Config,
        commands: &Sender<Command>,
    ) {

        egui::Window::new("egui-winit-wgpu-template")
        // .vscroll(true)
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .show(&ui, |ui| {
            // Rotate triangle section
            // Toggle, speed value and event handling
            ui.horizontal(|ui: &mut Ui| {
                if ui
                    .checkbox(&mut app_config.rotate_triangle, "Rotate Triangle")
                    .changed()
                {
                    commands
                        .send(Command::RotateTriangle {
                            value: app_config.rotate_triangle,
                        })
                        .unwrap()
                };
                ui.label("Triangle Speed");
                let triangle_speed_response: Response = ui.add(
                    egui::widgets::DragValue::new(&mut app_config.triangle_speed)
                        .clamp_range(0.0..=std::f32::consts::TAU*2.0)
                        .fixed_decimals(1)
                        .speed(0.1),
                );

                if triangle_speed_response.changed() {
                    commands
                        .send(Command::SetTriangleSpeed {
                            speed: app_config.triangle_speed,
                        })
                        .unwrap()
                };
            });

            // This button opens a file dialog and 
            // sets the scene_path to that path.
            ui.horizontal(|ui: &mut Ui| {
                if ui.button("Open file..").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        app_config.config_save_path = path.display().to_string();
                    }
                }
                ui.label("Path");

                ui.text_edit_singleline(&mut app_config.config_save_path);
            });
        });
    }
}
