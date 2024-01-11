use std::path::Path;

use serde::{Deserialize, Serialize};

use super::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioConfig {
    pub previous_input_device: Option<String>,
    pub previous_output_device: Option<String>,
}

impl Config for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            previous_input_device: None,
            previous_output_device: None,
        }
    }

    fn config_path() -> &'static Path {
        Path::new("config/audio_device.json")
    }
}
