// In this example/template, this is a render engine, but it could be anything
// you'd like to put in there. It could be a program that takes in input
// from the webcam, does some computer visiony deep learning on the image
// and then presents a modified image to the screen or anything else you could come up with.
// If the engine was bigger, it might be a good idea to seperate this component into two.
// One being the engine, another being the renderer.

// std lib
use std::{iter, time::{Duration, Instant}};

// crates
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use egui_wgpu::renderer::ScreenDescriptor;
use log::error;
use wgpu::{
    CommandEncoder,
    RenderPass,
    SurfaceError,
    SurfaceTexture,
    TextureView
};
use winit::window::Window;

// own crates
use crate::{
    command::Command,
    config::Config,
    engine_status::EngineStatus,
    gui::GuiRenderer, render_context::RenderContext
};

pub enum RenderEngineError {
    OutdatedSurfaceTexture,
    OutOfMemory,
    Described{message: String},
}

impl RenderEngineError {
    pub fn report_surface_error(error: SurfaceError, errors: &mut Vec<RenderEngineError>) {
        match error {
            SurfaceError::Lost => { errors.push(RenderEngineError::Described { message: "SurfaceError::Lost".to_string() }) },
            SurfaceError::OutOfMemory => { errors.push(RenderEngineError::OutOfMemory) },
            SurfaceError::Outdated => { errors.push(RenderEngineError::OutdatedSurfaceTexture) },
            SurfaceError::Timeout => { errors.push(RenderEngineError::Described { message: "SurfaceError::Timeout".to_string() }) },
        }
    }

}

struct AppContext {
    should_shutdown: bool,
    is_shutdown: bool,
    error_states: Vec<RenderEngineError>,
    rotate_triangle: bool,
    rotate_triangle_angle: f32,
    rotate_triangle_speed: f32,
    should_render: bool,
    time_of_last_render: Instant,
}

pub struct RenderEngine {
    render_context: RenderContext,
    app_context: AppContext,
    gui_renderer: Option<GuiRenderer>,
    app_config: Config,
}

impl RenderEngine {
    pub fn build(
        window: Window,
        app_config: Config,
        no_gui: bool,
    ) -> Self {

        // Initial values, which should be overwritten by the control panel.
        let rotate_triangle_angle: f32 = 0.0;
        let rotate_triangle_speed: f32 = app_config.triangle_speed;

        let render_context: RenderContext = RenderContext::build(window);

        let gui_renderer: Option<GuiRenderer> =
        if no_gui {
            None
        } else {
            Some(GuiRenderer::new(
                &render_context.device,                             // wgpu Device
                render_context.config.format,   // TextureFormat
                None,                           // this can be None
                1,                                     // samples
                &render_context.window,                             // winit Window
            ))
        };

        // This could be done in a lower impact way with
        // bits of a u64 used as error state flags.
        let error_states: Vec<RenderEngineError> = Vec::<RenderEngineError>::new();
        RenderEngine {
            render_context,
            gui_renderer,
            app_context: 
                AppContext {
                    should_shutdown: false,
                    is_shutdown: false,
                    error_states,
                    rotate_triangle: true,
                    rotate_triangle_angle,
                    rotate_triangle_speed,
                    should_render: true,
                    time_of_last_render: Instant::now(),
                },
            app_config,
        }
    }

    pub fn initialize(&mut self) {}

    pub fn resize(
        &mut self,
        command_sender: &Sender<Command>,
        new_size: winit::dpi::PhysicalSize<u32>,
        no_gui: bool
    ) {
        if 0 < new_size.width && 0 < new_size.height {
            self.render_context.config.width = new_size.width;
            self.render_context.config.height = new_size.height;
            self.render_context.surface.configure(&self.render_context.device, &self.render_context.config);
            self.render(command_sender, no_gui);
        }
    }

    pub fn shutdown(&mut self, status_sender: &Sender<EngineStatus>) {
        let result: Result<(), std::io::Error> = self.app_config.serialize();
        match result {
            Ok(_) => {},
            _ => error!("Failed to serialize app_config to path: {}", self.app_config.config_save_path),
        };
        status_sender.send(EngineStatus::Shutdown { value: true }).expect("Failed to send shutdown successful message.");
        self.app_context.is_shutdown = true;
    }

    fn evaluate_rotating_triangle(&mut self, time_delta: &Duration) {
        // Calculate new rotation value and send it to the uniform buffer.
        // Replacing this with push constants might be a good idea.
        if self.app_context.rotate_triangle && self.app_context.rotate_triangle_speed != 0.0 {
            self.app_context.rotate_triangle_angle += self.app_context.rotate_triangle_speed * time_delta.as_secs_f32();
            if 360.0 < self.app_context.rotate_triangle_angle {
                self.app_context.rotate_triangle_angle -= 360.0;
            }
            if self.app_context.rotate_triangle_angle < 0.0 {
                self.app_context.rotate_triangle_angle += 360.0;
            }

            self.render_context.uniform.angle = self.app_context.rotate_triangle_angle;

            if self.app_context.should_render {
                self.render_context.queue.write_buffer(
                    &self.render_context.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[self.render_context.uniform]),
                );
            }
        }
    }

    fn get_current_texture(&mut self) -> Result<SurfaceTexture, SurfaceError> {
        // Get the texture we want to write to and present to the screen.
        let result: Result<SurfaceTexture, SurfaceError> = self
            .render_context.surface
            .get_current_texture();

        // First pass try to resolve any errors
        match result {
            Ok(surface_texture) => return Ok(surface_texture),
            Err(error) => {
                match error {
                    SurfaceError::Outdated => {
                        self.render_context.surface.configure(&self.render_context.device, &self.render_context.config);
                        self.render_context.window.request_redraw();
                    },
                    SurfaceError::Lost => {
                        todo!("Recreate pipeline/window");
                    }
                    SurfaceError::OutOfMemory => {
                        todo!("Either recreate or go into error state");
                    }
                    SurfaceError::Timeout => {
                        todo!("Handle a timeout? Maybe just try again?");
                    }
                }
            }
        }

        let result: Result<SurfaceTexture, SurfaceError> = self
            .render_context.surface
            .get_current_texture();

        match result {
            Ok(surface_texture) => return Ok(surface_texture),
            Err(error) => {
                error!("Render engine caught error: {:?}", error);
                return Err(error);
            }
        }
    } 

    fn handle_errors(&mut self) {
        loop {
            let error: Option<RenderEngineError> = self.app_context.error_states.pop();
            if error.is_none() { break; }
            let error: RenderEngineError = error.expect("Failed to get error after .is_none() check");
            match error {
                RenderEngineError::Described { message: _ } => {
                    todo!("Send message back to main thread using channel.");
                },
                RenderEngineError::OutOfMemory => {
                    todo!("Recreate pipeline or crash, engine. Tell user to use less memory.");
                },
                RenderEngineError::OutdatedSurfaceTexture => {
                    self.render_context.surface.configure(&self.render_context.device, &self.render_context.config);
                    self.render_context.window.request_redraw();
                },
            }
        }
    }

    // This function is called every time we want to render a new frame.
    // It will get a bit graphics programming technical. Just try to get a hold
    // of the big picture, don't worry too much about the details.
    pub fn render(
        &mut self,
        command_sender: &Sender<Command>,
        no_gui: bool
    ) {
        self.handle_errors();
        let current_time: Instant = Instant::now();
        let time_delta: Duration = current_time - self.app_context.time_of_last_render;
        self.app_context.time_of_last_render = current_time;
        
        self.evaluate_rotating_triangle(&time_delta);
        
        if !self.app_context.should_render {
            return;
        }

        // Get the texture we want to write to and present to the screen.
        let output_texture_result: Result<SurfaceTexture, SurfaceError> = self.get_current_texture();
        if output_texture_result.is_err() {
            RenderEngineError::report_surface_error(output_texture_result.err().expect("Failed to get underlying error after is_err() check"), &mut self.app_context.error_states);
            return;
        }
        let output: SurfaceTexture = 
            output_texture_result
                .ok()
                .expect("Checked result turned out not be ok.");

        let view: TextureView = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder: CommandEncoder =
            self.render_context.device
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
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            render_pass.set_pipeline(&self.render_context.render_pipeline);
            render_pass.set_bind_group(0, &self.render_context.uniform_bind_group, &[]);

            // Draw the triangle. For this you might eventually replace it with something more complicated.
            render_pass.set_vertex_buffer(0, self.render_context.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.render_context.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.render_context.num_indices, 0, 0..1);
        }

        if !no_gui {
            match &mut self.gui_renderer {
                Some(gui) => {
                    let screen_descriptor: ScreenDescriptor = ScreenDescriptor {
                        size_in_pixels: [self.render_context.config.width, self.render_context.config.height],
                        pixels_per_point: self.render_context.window.scale_factor() as f32,
                    };

                    gui.draw(
                        &self.render_context.device,
                        &self.render_context.queue,
                        &mut encoder,
                        &self.render_context.window,
                        &view,
                        screen_descriptor,
                        command_sender,
                        &mut self.app_config,
                    );
                },
                None => {},
            }
        }


        self.render_context.queue.submit(iter::once(encoder.finish()));

        output.present();
    }

    // Keeps the renderer alive, handles errors and incoming events from the GUI and window event loop.
    pub fn command_loop(
        &mut self,
        command_sender: Sender<Command>,
        command_receiver: Receiver<Command>,
        status_sender: Sender<EngineStatus>,
        no_gui: bool,
    ) {
        if !no_gui {
            match &self.gui_renderer {
                Some(gui) => {
                    gui.initialize(
                        &mut self.app_config,
                        &command_sender                    );
                },
                None => {},
            }
        }

        while !self.app_context.should_shutdown {
            loop {
                let result: Result<Command, TryRecvError> = command_receiver.try_recv();
                if result.is_err() {
                    break;
                }
                let command: Command = result.unwrap();
                match command {
                    Command::Resize { new_size } => {
                        self.resize(&command_sender, new_size, no_gui);
                    }
                    Command::RotateTriangle { value } => {
                        self.app_context.rotate_triangle = value;
                    }
                    Command::SetTriangleSpeed { speed } => {
                        self.app_context.rotate_triangle_speed = speed;
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
                            self.app_context.should_shutdown = true;
                        }
                    }
                    Command::HandleInputGui { event } => {
                        if !no_gui{
                            match &mut self.gui_renderer {
                                Some(gui) => {
                                    gui.handle_input(&self.render_context.window, &event);
                                },
                                None => {},
                            }
                        }
                    }
                }
            }

            self.render(&command_sender, no_gui);
        }
        self.shutdown(&status_sender);
        return;
    }
}
