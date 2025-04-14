use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub ui: UiSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub tick_rate_ms: u64,
    pub default_shell: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                tick_rate_ms: 250,
                default_shell: std::env::var("SHELL").unwrap_or_else(|_| String::from("/bin/bash")),
            },
            ui: UiSettings {
                theme: String::from("default"),
            },
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load() -> Result<Self> {
        // TODO: Implement loading from config file
        // For now, just return default settings
        Ok(Self::default())
    }
    
    pub fn save(&self, _path: &PathBuf) -> Result<()> {
        // TODO: Implement saving to config file
        Ok(())
    }
}
