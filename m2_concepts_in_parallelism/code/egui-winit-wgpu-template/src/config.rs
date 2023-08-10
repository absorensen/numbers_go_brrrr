use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub image_height: usize,
    pub image_scale: f32,
    pub output_path: String,
    pub subpixels_per_pixel: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub scene_index: usize,
    pub seed: usize,
    pub use_loop_rendering: bool,
}

impl Config {
    pub fn update_derived_values(&mut self) {
        self.aspect_ratio = (self.image_height as f32) / (self.image_width as f32);
        self.image_scale = 1.0 / (self.samples_per_pixel as f32);
    }
}

impl ::std::default::Default for Config {
    fn default() -> 
        Self { 
            Self { 
                aspect_ratio: 16.0 / 9.0, 
                image_width: 500, 
                image_height: 500,
                image_scale: 0.0,
                output_path: "output.png".to_string(), 
                subpixels_per_pixel: 2,
                samples_per_pixel: 5, 
                max_depth: 10, 
                scene_index: 7,
                seed: 1337,
                use_loop_rendering: true,
            } 
        }
}