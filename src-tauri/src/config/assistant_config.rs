use std::path::Path;

use serde::{Deserialize, Serialize};

use super::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssistantConfig {
    pub api_key: Option<String>,
    pub thread_id: Option<String>,
}

impl Config for AssistantConfig {
    fn default() -> Self {
        AssistantConfig {
            api_key: None,
            thread_id: None,
        }
    }

    fn config_path() -> &'static std::path::Path {
        Path::new("assistant.json")
    }
}
