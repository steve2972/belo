use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use dirs::home_dir;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

/// Global configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub active_project: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            return Ok(Self {
                active_project: None,
            });
        }

        let mut file = File::open(&config_path)
            .with_context(|| format!("Failed to open config file at '{:?}'", config_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)
            .with_context(|| "Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = serde_json::to_string_pretty(self)?;
        let mut file = File::create(config_path.clone())
            .with_context(|| format!("Failed to create config file at '{:?}'", config_path))?;

        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let home = home_dir().ok_or_else(|| anyhow!("Failed to get home directory"))?;
        Ok(home.join(".belo").join("config.json"))
    }
    
}