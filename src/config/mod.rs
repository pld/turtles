mod model;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use std::fs;
use std::path::PathBuf;

pub use model::*;

/// Get the default configuration file path
pub fn get_config_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home_dir.join(".config").join("screensage").join("config.toml")
}

/// Load configuration from file and command line arguments
pub fn load_config() -> Result<Config> {
    // Parse command line arguments
    let args = CliArgs::parse();
    
    // Determine config file path
    let config_path = args.config.clone().unwrap_or_else(get_config_path);
    info!("Using config file: {}", config_path.display());
    
    // Load config from file or use default
    let mut config = if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?
    } else {
        info!("Config file not found, using defaults");
        Config::default()
    };
    
    // Override with command line arguments
    if let Some(model) = args.model {
        config.ollama.default_model = model;
    }
    
    if let Some(api_url) = args.api_url {
        config.ollama.api_url = api_url;
    }
    
    if let Some(opacity) = args.opacity {
        config.window.opacity = opacity;
    }
    
    if let Some(log_level) = args.log_level {
        config.logging.level = log_level;
    }
    
    // Validate configuration
    config.validate()?;
    
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config, path: Option<PathBuf>) -> Result<()> {
    let config_path = path.unwrap_or_else(get_config_path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }
    
    // Serialize and save config
    let content = toml::to_string(config)
        .context("Failed to serialize config")?;
    
    fs::write(&config_path, content)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;
    
    info!("Configuration saved to {}", config_path.display());
    Ok(())
}
