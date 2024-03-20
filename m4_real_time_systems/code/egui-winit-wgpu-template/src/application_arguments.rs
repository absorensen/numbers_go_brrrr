use std::path::PathBuf;

use clap::{arg, command, value_parser};

use log::debug;
pub struct ApplicationArguments {
    pub no_gui: bool,
    pub config_path: Box<PathBuf>,
}

impl ApplicationArguments {
    pub fn get_from_arguments() -> Self {
        // parse input arguments and try to load config file
        // Two arguments to care about right now --ui==1 and --config-path="...."
        let matches = command!() // requires `cargo` feature
            .arg(arg!(
                --nogui ... "Turn graphical user interface off. Keyboard and mouse events will still be handled."
            ).required(false)
            .value_parser(value_parser!(u8))
            )
            .arg(
                arg!(
                    -c --config <FILE> "Sets a custom config file"
                )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            ).get_matches();


        let config_path: Box<PathBuf> = if let Some(config_path) = 
            matches.get_one::<PathBuf>("config") {
                Box::<PathBuf>::new(config_path.clone())
            } else {
                Box::<PathBuf>::new(PathBuf::new())
            };

        let no_gui: bool =
            if let Some(no_gui) = matches.get_one::<u8>("nogui") {
                0 < *no_gui
            } else { 
                false
            };

        debug!("Launching with arguments:");
        debug!("no_gui: {}", no_gui);
        debug!("config_path: {}", config_path.display());

        Self {no_gui, config_path}
    }

    pub fn has_valid_config_path(&self) -> bool {
        self.config_path.exists()
    }
}