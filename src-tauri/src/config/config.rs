use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub trait Config {
    fn default() -> Self;
    fn config_path() -> &'static Path;
    fn retrieve() -> Self
    where
        Self: Sized + for<'a> Deserialize<'a> + Serialize,
    {
        let path = Self::config_path();

        if path.exists() {
            let mut file = File::open(path).expect("For path to exist");
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);

            let config: Self = serde_json::from_str(&buffer).unwrap();
            config
        } else {
            let config = Self::default();
            config.save().expect("To save file");
            config
        }
    }
    fn save(&self) -> Result<(), anyhow::Error>
    where
        Self: Serialize,
    {
        let path = Self::config_path();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let json_to_save = serde_json::to_string(self)?;

        file.write_all(json_to_save.as_bytes())?;

        Ok(())
    }
}
