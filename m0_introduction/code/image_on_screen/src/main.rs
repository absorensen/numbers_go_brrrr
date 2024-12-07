use minifb::{ScaleMode, Window, WindowOptions, Key};
use rand::{rngs::ThreadRng, Rng};

struct PixelColor{
    red: f64,
    blue: f64,
    green: f64,
}

// Put your color generation in here and replace this random noise.
// Also feel free to replace the f64's with whatever type you want to generate your
// colors in.
fn generate_pixel(rng: &mut ThreadRng, _row_index: usize, _column_index: usize) -> PixelColor {
    PixelColor{red: rng.gen::<f64>() * 255.0, blue: rng.gen::<f64>() * 255.0, green: rng.gen::<f64>() * 255.0}
}

fn main() {

    let width: usize = 512;
    let height: usize = 384;
    let colors: usize = 3;

    let mut image_buffer: Vec<f64> = vec![0.0; (width * height * colors) as usize];

    // If multithreading, make this thread local.
    let mut rng: ThreadRng = rand::thread_rng();

    for row_index in 0..height {
        for column_index in 0..width {
            let pixel: PixelColor = generate_pixel(&mut rng, row_index, column_index);

            let output_index: usize = (height - row_index - 1) * colors * width + column_index * colors; 
            image_buffer[output_index + 0] = pixel.red;
            image_buffer[output_index + 1] = pixel.blue;
            image_buffer[output_index + 2] = pixel.green;
        }
    }

    // Map the floating point values to a single quantized integer with 3 8-bit colors.
    let window_buffer: Vec<u32> = image_buffer
        .chunks(3)
        .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
        .collect();

    // Create the window to use below.
    let mut window: Window = Window::new(
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

    // Show the image until the user escapes.
    // You could also make this update conditional on changes, but
    // this is just a small demo.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(
                &window_buffer,
                width as usize,
                height as usize,
            ).unwrap();
    }
    
}
