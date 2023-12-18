use winit::dpi::PhysicalSize;

pub enum Command {
    Resize { new_size: PhysicalSize<u32> },
    RotateTriangle { value: bool },
    SetTriangleSpeed { speed: f32 },
    LoadScene { path: String },
    Render { value: bool },
    KeyEventW,
    KeyEventA,
    KeyEventS,
    KeyEventD,
    KeyEventQ,
    KeyEventE,
    KeyEventComma,
    KeyEventPeriod,
    Shutdown { value: bool },
}
