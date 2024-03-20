use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

pub enum Command {
    Resize { new_size: PhysicalSize<u32> },
    RotateTriangle { value: bool },
    SetTriangleSpeed { speed: f32 },
    KeyEventW,
    KeyEventA,
    KeyEventS,
    KeyEventD,
    KeyEventQ,
    KeyEventE,
    KeyEventComma,
    KeyEventPeriod,
    Shutdown { value: bool },
    HandleInputGui{event: WindowEvent},
}
