mod command;
mod control_panel;
mod render_engine;

use std::{thread, time::Instant};

use command::Command;
use control_panel::ControlPanel;
use crossbeam_channel::{unbounded, Receiver, Sender};
use render_engine::RenderEngine;

use wgpu::{Adapter, AdapterInfo, Device, Instance, Queue};
use winit::{
    event::{
        ElementState,
        Event::{self, *},
        KeyboardInput, VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

// Simple wrapper to handle different window ids.
struct WindowSelector {
    control_panel_id: u64,
    render_panel_id: u64,
}

impl WindowSelector {
    pub fn new(control_panel_id: WindowId, render_panel_id: WindowId) -> Self {
        let control_panel_id: u64 = control_panel_id.into();
        let render_panel_id: u64 = render_panel_id.into();
        WindowSelector {
            control_panel_id,
            render_panel_id,
        }
    }

    #[inline(always)]
    pub fn select_window(&self, id: &WindowId) -> usize {
        let id: u64 = (*id).into();
        if id == self.control_panel_id {
            0
        } else if id == self.render_panel_id {
            1
        } else {
            1000
        }
    }
}

async fn run_loop(
    gpu_handles: GPUHandles,
    event_loop: EventLoop<()>,
    window_selector: WindowSelector,
    mut control_panel: ControlPanel,
    transmitter: Sender<Command>,
) {
    let start_time: Instant = Instant::now();

    // For handling constant redrawing of specific control panel widgets.
    let mut redraw_gui: bool = false;
    let mut gui_has_focus: bool = false;

    control_panel.initialize(&transmitter);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&gpu_handles.instance, &gpu_handles.adapter);
        let transmitter: &Sender<Command> = &transmitter;

        control_panel.platform.handle_event(&event); // In doubt about this one
                                                     // *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::MouseInput { state, .. } => match state {
                    // Always redraw the control panel when a button has been pressed
                    // or released.
                    ElementState::Pressed => {
                        if window_selector.select_window(&window_id) == 0 {
                            control_panel.redraw(
                                transmitter,
                                &mut gui_has_focus,
                                &mut redraw_gui,
                                &start_time,
                                &gpu_handles.device,
                                &gpu_handles.queue,
                            );
                        }
                    }
                    ElementState::Released => {
                        if window_selector.select_window(&window_id) == 0 {
                            control_panel.redraw(
                                transmitter,
                                &mut gui_has_focus,
                                &mut redraw_gui,
                                &start_time,
                                &gpu_handles.device,
                                &gpu_handles.queue,
                            );
                        }
                    }
                },

                // Redraw the control panel when the cursor moves on it.
                // The render engine will always redraw anyway.
                WindowEvent::CursorMoved { .. } => {
                    if window_selector.select_window(&window_id) == 0 {
                        control_panel.redraw(
                            transmitter,
                            &mut gui_has_focus,
                            &mut redraw_gui,
                            &start_time,
                            &gpu_handles.device,
                            &gpu_handles.queue,
                        );
                    }
                }

                // Handle resizing of the specific window.
                WindowEvent::Resized(size) => match window_selector.select_window(&window_id) {
                    0 => control_panel.resize(&gpu_handles.device, size),
                    1 => transmitter
                        .send(Command::Resize { new_size: size })
                        .unwrap(),
                    _ => (),
                },

                // Handle shutdown by clicking the close button in the upper right.
                WindowEvent::CloseRequested => {
                    transmitter.send(Command::Shutdown { value: true }).unwrap();
                    *control_flow = ControlFlow::Exit;
                }

                // Most of the keyboard events are sent directly to the render engine
                // through the transmitter channel. In general, it is assumed that
                // you don't control the GUI through the keyboard.
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    if state == ElementState::Pressed {
                        use VirtualKeyCode::*;
                        match key {
                            Escape => {
                                transmitter.send(Command::Shutdown { value: true }).unwrap();
                                *control_flow = ControlFlow::Exit;
                            }
                            W => transmitter.send(Command::KeyEventW).unwrap(),
                            A => transmitter.send(Command::KeyEventA).unwrap(),
                            S => transmitter.send(Command::KeyEventS).unwrap(),
                            D => transmitter.send(Command::KeyEventD).unwrap(),
                            Q => transmitter.send(Command::KeyEventQ).unwrap(),
                            E => transmitter.send(Command::KeyEventE).unwrap(),
                            Comma => transmitter.send(Command::KeyEventComma).unwrap(),
                            Period => transmitter.send(Command::KeyEventPeriod).unwrap(),
                            _ => (),
                        }
                    }
                }
                _ => (),
            },

            // Only redraw the control panel for specific redraw requests.
            // This is to keep the control panel light on processing.
            // The render engine is running on its own thread and redraws
            // every single frame, so no redraw request needed.
            RedrawRequested(window_id) => {
                if window_selector.select_window(&window_id) == 0 {
                    control_panel.redraw(
                        transmitter,
                        &mut gui_has_focus,
                        &mut redraw_gui,
                        &start_time,
                        &gpu_handles.device,
                        &gpu_handles.queue,
                    )
                }
            }

            // This event happens once all the other events have been cleared.
            // The additional redraws are for when a GUI element has focus and
            // needs to be constantly redrawn. It could for example be the
            // text entry widget.
            MainEventsCleared => {
                if redraw_gui || gui_has_focus {
                    redraw_gui = false;
                    control_panel.redraw(
                        transmitter,
                        &mut gui_has_focus,
                        &mut redraw_gui,
                        &start_time,
                        &gpu_handles.device,
                        &gpu_handles.queue,
                    );
                }
            }

            _ => {}
        }
    });
}

// A convenience wrapper for interfacing with the GPU.
pub struct GPUHandles {
    pub queue: Queue,
    pub adapter: Adapter,
    pub instance: Instance,
    pub device: Device,
}

impl GPUHandles {
    pub fn new() -> Self {
        let instance: Instance = wgpu::Instance::default();

        // You might want to change this to prefer a certain backend or a high power GPU.
        let adapter: Adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                ..Default::default()
            }))
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        // If you want to run with a webgl backend, you 
        // can set the limits to one of the downlevels
        let (device, queue): (Device, Queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .expect("Failed to create device");

        GPUHandles {
            queue,
            adapter,
            instance,
            device,
        }
    }
}

impl Default for GPUHandles {
    fn default() -> Self {
        Self::new()
    }
}

// Checks whether the system has a findable adapter (GPU).
// Returns false if no adapter is found.
pub fn self_test() -> bool {
    println!("Performing self test to check system for compatibility.");
    let instance: Instance = wgpu::Instance::default();

    // `request_adapter` instantiates the general connection to the GPU
    let adapter_option: Option<Adapter> =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()));

    // Handle whether we find a GPU or not.
    match adapter_option {
        Some(adapter) => {
            let info: AdapterInfo = adapter.get_info();
            println!("Found GPU: {:?}", info);
            true
        }
        None => {
            println!("Failed to find a usable GPU. This framework will only run CPU code.");
            false
        }
    }
}

pub fn run() {
    {
        env_logger::init();
        if !self_test() {
            panic!("Failed to find a suitable wgpu adapter!");
        }

        // Actually instantiate the GPU handles we need, like queue, device and adapter.
        // The render engine won't use these, but make its own.
        let gpu_handles: GPUHandles = GPUHandles::new();

        // Our render and control window sizes, and space between them.
        const RENDER_WINDOW_SIZE: winit::dpi::PhysicalSize<u32> =
            winit::dpi::PhysicalSize::new(1420, 1080);
        const CONTROL_WINDOW_SIZE: winit::dpi::PhysicalSize<u32> =
            winit::dpi::PhysicalSize::new(400, 1080);
        const WINDOW_PADDING: u32 = 16;

        // This is winit's event loop which will handle our input events.
        let event_loop: EventLoop<()> = EventLoop::new();

        // Create control panel
        let control_panel: ControlPanel = ControlPanel::build(
            &gpu_handles,
            &event_loop,
            CONTROL_WINDOW_SIZE,
            WINDOW_PADDING,
        );

        // Create render engine
        let mut render_engine: RenderEngine = RenderEngine::build(
            &gpu_handles,
            &event_loop,
            RENDER_WINDOW_SIZE,
            WINDOW_PADDING,
            control_panel.window.outer_size().width,
        );

        // Create the window selector which will be used for
        // matching events to the relevant window.
        let window_selector: WindowSelector =
            WindowSelector::new(control_panel.window_id, render_engine.window_id);

        // Create the transmit/receive pair. This runs on a crossbeam_channel 
        // and only ever has a single transmitter (app/gui thread)
        // which sends commands to be received by the render engine.
        let (transmitter, receiver): (Sender<Command>, Receiver<Command>) = unbounded::<Command>();

        // Initialize the render engine and sent it on its merry way doing its
        // own render/event loop. 
        let _render_handle: thread::JoinHandle<()> = thread::spawn(move || {
            render_engine.initialize();
            render_engine.render_loop(receiver);
        });

        // Let's start it up!
        pollster::block_on(run_loop(
            gpu_handles,
            event_loop,
            window_selector,
            control_panel,
            transmitter,
        ));
    }
}
