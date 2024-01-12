use minifb::{Window, WindowOptions, ScaleMode};
use ultraviolet::Vec2;

// Julia code adapted from Taichi's Hello World
// https://docs.taichi-lang.org/docs/hello_world
fn generate_julia_pixel(
    row_index: usize, 
    column_index: usize, 
    height: usize,
    t: f32
) -> f64 {
    let c = Vec2::new(-0.8, t.cos() * 0.2);
    let mut z = Vec2::new(column_index as f32 / height as f32 - 1.0, row_index as f32 / height as f32 - 0.5) * 2.0;
    let mut iterations = 0;
    while z.mag() < 20.0 && iterations < 50 {
        z = Vec2::new(z[0] * z[0] - z[1] * z[1], 2.0 * z[0] * z[1]) + c;
        iterations += 1;
    }

    (1.0 - iterations as f32 * 0.02) as f64 * 255.0
}

pub struct PrettyScreensaver {
    width: usize,
    height: usize,
    colors: usize,
    image_buffer: Vec<f64>,
    window_buffer: Vec<u32>,
    window: Window,
    time_step: u32,
}

impl PrettyScreensaver {
    pub fn new() -> Self {
        let height = 320;
        let width = height * 2;
        let colors = 3;
        let time_step = 0;
    
        let mut image_buffer: Vec<f64> = vec![0.0; (width * height * colors) as usize];

        let t = time_step as f32 * 0.03;
        for row_index in 0..height {
            for column_index in 0..width {
                let pixel_value = generate_julia_pixel(row_index, column_index, height, t);
                let output_index: usize = (height - row_index - 1) * colors * width + column_index * colors;
                image_buffer[output_index + 0] = pixel_value;
                image_buffer[output_index + 1] = pixel_value;
                image_buffer[output_index + 2] = pixel_value;
            }
        }

        let window_buffer: Vec<u32> = image_buffer
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

        let mut window = Window::new(
            "Professor Bærentzen's Screensaver",
            width as usize,
            height as usize,
            WindowOptions {
                resize: true,
                scale_mode: ScaleMode::Center,
                ..WindowOptions::default()
            },
        )
        .expect("Unable to open Window");

        window.update_with_buffer(
                &window_buffer,
                width as usize,
                height as usize,
            ).unwrap();

        Self {width, height, colors, image_buffer, window_buffer, window, time_step}
    }

    pub fn update(&mut self) {
        self.time_step += 1;

        let t = self.time_step as f32 * 0.03;
        for row_index in 0..self.height {
            for column_index in 0..self.width {
                let pixel_value = generate_julia_pixel(row_index, column_index, self.height,  t);
                let output_index: usize = (self.height - row_index - 1) * self.colors * self.width + column_index * self.colors;
                self.image_buffer[output_index + 0] = pixel_value;
                self.image_buffer[output_index + 1] = pixel_value;
                self.image_buffer[output_index + 2] = pixel_value;
            }
        }
    
        self.window_buffer = self.image_buffer
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();
    
        self.window.update_with_buffer(
                &self.window_buffer,
                self.width as usize,
                self.height as usize,
            ).unwrap();
    }
}