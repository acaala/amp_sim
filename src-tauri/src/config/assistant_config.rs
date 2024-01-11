use std::path::Path;

use serde::{Deserialize, Serialize};

use super::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssistantConfig {
    pub api_key: Option<String>,
}

impl Config for AssistantConfig {
    fn default() -> Self {
        AssistantConfig { api_key: None }
    }

    fn config_path() -> &'static std::path::Path {
        Path::new("config/assistant.json")
    }
}
