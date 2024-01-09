use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub previous_input_device: Option<String>,
    pub previous_output_device: Option<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            previous_input_device: None,
            previous_output_device: None,
        }
    }

    pub fn retrieve() -> Self {
        let path = Self::config_path();

        if path.exists() {
            let mut file = File::open(path).expect("For path to exist");
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);

            let config: Config = serde_json::from_str(&buffer).unwrap();
            config
        } else {
            let config = Self::new();
            config.save().expect("To save file");
            config
        }
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let path = Config::config_path();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let json_to_save = serde_json::to_string(self)?;

        file.write_all(json_to_save.as_bytes())?;

        Ok(())
    }

    fn config_path() -> &'static Path {
        Path::new("config.json")
    }
}
