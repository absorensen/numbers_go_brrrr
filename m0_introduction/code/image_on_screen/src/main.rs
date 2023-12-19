use minifb::{ScaleMode, Window, WindowOptions, Key};
use rand::Rng;

fn main() {

    let width = 512;
    let height = 384;
    let colors = 3;

    let mut image_buffer: Vec<f64> = vec![0.0; (width * height * colors) as usize];
    let mut rng = rand::thread_rng();

    for row_index in 0..height {
        for column_index in 0..width {
            let output_index: usize = (height - row_index - 1) * colors * width + column_index * colors; 
            image_buffer[output_index + 0] = rng.gen::<f64>() * 255.0;
            image_buffer[output_index + 1] = rng.gen::<f64>() * 255.0;
            image_buffer[output_index + 2] = rng.gen::<f64>() * 255.0;
        }
    }

    let window_buffer: Vec<u32> = image_buffer
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    let mut window = Window::new(
        "My Output Image - Press ESC to exit",
        width as usize,
        height as usize,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(
                &window_buffer,
                width as usize,
                height as usize,
            ).unwrap();
    }
    
}
