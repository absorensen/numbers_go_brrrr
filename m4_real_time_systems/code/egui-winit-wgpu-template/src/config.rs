use std::path::PathBuf;
use std::{fs::File, io::Write};
use std::io::BufReader;
use std::io::prelude::*;

use log::debug;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rotate_triangle: bool,
    pub triangle_speed: f32,
    pub config_save_path: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            rotate_triangle: true,
            triangle_speed: 0.5,
            config_save_path: "".to_string(),
        }
    }

    pub fn deserialize_from_path(path: &PathBuf) -> Result<Config, std::io::Error> {
        // Does file exist in path?
        let file: File = File::open(path)?;
        let mut buf_reader: BufReader<File> = BufReader::new(file);
        let mut contents: String = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let app_state: Result<Config, toml::de::Error> = toml::from_str(contents.as_str());
        if app_state.is_err() {
            debug!("Loaded config file, but failed to parse into Config struct");
        }
        let app_state: Config = app_state.expect("Failed to get Config from toml::from_str()");
        debug!("Successfully loaded config file from: {}", path.display());
        Ok(app_state)
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        let toml: String = toml::to_string(&self).expect("Failed to create .toml string form AppState");
        let mut file: File = File::create(&self.config_save_path)?;
        file.write_all(toml.as_bytes())?;
        debug!("Successfully wrote config file to: {}", &self.config_save_path);
        Ok(())
    }
}