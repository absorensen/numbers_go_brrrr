// std lib
use std::{path::PathBuf, thread};

// crates
use crossbeam_channel::{unbounded, Receiver, Sender};
use winit::{
    event::*,
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

// crates from this project
mod config;
mod application_arguments;
mod render_context;
mod gui;
mod render_engine;
mod command;
mod engine_status;
mod uniforms;
mod vertex;

use command::Command;
use render_engine::RenderEngine;
use crate::{application_arguments::ApplicationArguments, config::Config, engine_status::EngineStatus};

fn wait_for_engine_shutdown(
    engine_status_receiver: &Receiver<EngineStatus>,
    ewlt: &EventLoopWindowTarget<()>
) {
    while let Ok(status) = engine_status_receiver.recv() {
        match status {
            EngineStatus::Shutdown { value } => if value { break; },
        }
    };
    ewlt.exit();
}

fn input_events_loop(
    engine_command_sender: Sender<Command>, 
    engine_status_receiver: Receiver<EngineStatus>, 
    render_window_id: WindowId,
    event_loop: EventLoop<()>
) {
    let _ = event_loop.run(move |event, ewlt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == render_window_id {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    } => {
                        engine_command_sender.send(Command::Shutdown { value: true }).expect("Failed to transmit shutdown command to render engine");
                        wait_for_engine_shutdown(&engine_status_receiver, ewlt);
                        return;
                    },
                    WindowEvent::Resized(physical_size) => {
                        engine_command_sender.send(Command::Resize { new_size: *physical_size}).expect("Failed to transmit resize command to render engine");
                    },
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        match key.as_ref() {
                            Key::Named(NamedKey::Escape) => {
                                engine_command_sender.send(Command::Shutdown { value: true }).expect("Failed to transmit shutdown command to render engine");
                                wait_for_engine_shutdown(&engine_status_receiver, ewlt);
                                return;                 
                            }
                            Key::Character("W") => engine_command_sender.send(Command::KeyEventW).expect("Failed to transmit 'W' key press command to render engine"),
                            Key::Character("A") => engine_command_sender.send(Command::KeyEventA).expect("Failed to transmit 'A' key press command to render engine"),
                            Key::Character("S") => engine_command_sender.send(Command::KeyEventS).expect("Failed to transmit 'S' key press command to render engine"),
                            Key::Character("D") => engine_command_sender.send(Command::KeyEventD).expect("Failed to transmit 'D' key press command to render engine"),
                            Key::Character("Q") => engine_command_sender.send(Command::KeyEventQ).expect("Failed to transmit 'Q' key press command to render engine"),
                            Key::Character("E") => engine_command_sender.send(Command::KeyEventE).expect("Failed to transmit 'E' key press command to render engine"),
                            Key::Character("Comma")=> engine_command_sender.send(Command::KeyEventComma).expect("Failed to transmit ',' key press command to render engine"),
                            Key::Character("Period") => engine_command_sender.send(Command::KeyEventPeriod).expect("Failed to transmit '.' key press command to render engine"),
                            _ => (),
                        }
                    },
                    _ => {},
                }
                engine_command_sender.send(Command::HandleInputGui { event: event.clone() }).expect("Failed to transmit GUI input event");
            }
            },
            _ => {}
        }
    }
);
}

pub fn run() {
    env_logger::init();
    let arguments: ApplicationArguments = ApplicationArguments::get_from_arguments();

    let config_path: Option<PathBuf> = 
        if arguments.has_valid_config_path() {
            Some(*arguments.config_path)
        } else if let Some(path) = rfd::FileDialog::new().pick_file() {
            Some(path)
        } else {
            None
        };

    let app_config: Config = if config_path.is_some() {
        let result: Result<Config, std::io::Error> = Config::deserialize_from_path(&config_path.expect("Failed to get path from argument to GuiRenderer::new()"));
        match result {
            Ok(config) => config,
            Err(_) => Config::default(),
        }
    } else {
        Config::default()
    };


    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    const RENDER_WINDOW_SIZE: winit::dpi::PhysicalSize<u32> =
        winit::dpi::PhysicalSize::new(800, 600);
    const WINDOW_PADDING: u32 = 16;
    let window: Window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title("engine panel")
            .with_inner_size(RENDER_WINDOW_SIZE)
            .build(&event_loop)
            .unwrap();
    window.set_outer_position(winit::dpi::PhysicalPosition::new(
        WINDOW_PADDING,
        WINDOW_PADDING,
    ));
    let render_window_id: WindowId = window.id();
    
    let mut render_engine: RenderEngine = 
        RenderEngine::build(
            window, 
            app_config, 
            arguments.no_gui
        );

    let (engine_command_sender, engine_command_receiver): (Sender<command::Command>, Receiver<command::Command>) = unbounded::<command::Command>();
    let (engine_status_sender, engine_status_receiver): (Sender<EngineStatus>, Receiver<EngineStatus>) = unbounded::<EngineStatus>();

    // Doesn't actually do anything right now, but you could find something
    // for it to do later (not a hint).
    render_engine.initialize();

    let command_sender_clone: Sender<Command> = engine_command_sender.clone();

    // Test on Apple. If the rendering is very slow, perhaps the event loop
    // should be on the auxiliary thread and rendering on the main thread.
    // This might pose some issues with the event loop which does not implement Send
    let render_handle: thread::JoinHandle<()> = thread::spawn(move || {
        render_engine.command_loop(
            command_sender_clone,
            engine_command_receiver,
            engine_status_sender,
            arguments.no_gui
        );
    });

    input_events_loop(
        engine_command_sender, 
        engine_status_receiver, 
        render_window_id, 
        event_loop
    );

    // Do something for shutting down?
    // Not right now. The event loop handles the shutdown
    // as it owns the necessary components (ewlt).
    render_handle.join().expect("Failed to join render thread");
}
