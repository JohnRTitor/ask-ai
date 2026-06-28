pub mod types;

use anyhow::{Context, Result};
use std::path::PathBuf;
use types::AppConfig;

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("ask-ai");
    path
}

pub fn get_config_path() -> PathBuf {
    let mut path = get_config_dir();
    path.push("config.toml");
    path
}

pub fn load_config() -> Result<AppConfig> {
    let path = get_config_path();
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file at {:?}", path))?;

    let config: AppConfig = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse config file at {:?}", path))?;

    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = get_config_path();
    let dir = get_config_dir();

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config directory {:?}", dir))?;
    }

    let toml = toml::to_string_pretty(config).with_context(|| "Failed to serialize config")?;

    std::fs::write(&path, toml)
        .with_context(|| format!("Failed to write config file at {:?}", path))?;

    Ok(())
}
