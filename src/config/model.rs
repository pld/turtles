use anyhow::{bail, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about = "A floating window for LLM chat on macOS")]
pub struct CliArgs {
    /// Path to configuration file
    #[clap(short, long)]
    pub config: Option<PathBuf>,
    
    /// Ollama model to use
    #[clap(short, long)]
    pub model: Option<String>,
    
    /// Ollama API URL
    #[clap(long)]
    pub api_url: Option<String>,
    
    /// Window opacity (0.0-1.0)
    #[clap(short, long)]
    pub opacity: Option<f32>,
    
    /// Log level (error, warn, info, debug, trace)
    #[clap(long)]
    pub log_level: Option<String>,
}

/// Application configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Window configuration
    pub window: WindowConfig,
    /// Ollama API configuration
    pub ollama: OllamaConfig,
    /// Conversation configuration
    pub conversation: ConversationConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Window configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Window opacity
    pub opacity: f32,
    /// Window always on top
    pub always_on_top: bool,
    /// Window position X coordinate
    pub position_x: Option<i32>,
    /// Window position Y coordinate
    pub position_y: Option<i32>,
}

/// Ollama API configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaConfig {
    /// API base URL
    pub api_url: String,
    /// Default model
    pub default_model: String,
    /// Temperature for sampling (higher = more random)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Top-p sampling (nucleus sampling)
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    /// Top-k sampling
    #[serde(default = "default_top_k")]
    pub top_k: u32,
    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

/// Conversation configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationConfig {
    /// Maximum conversation length in characters
    pub max_length: usize,
    /// Whether to save conversations automatically
    pub auto_save: bool,
}

/// Logging configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Maximum log file size in MB
    pub max_file_size: u32,
    /// Number of log files to keep
    pub max_files: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            ollama: OllamaConfig::default(),
            conversation: ConversationConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 400,
            height: 600,
            opacity: 0.9,
            always_on_top: true,
            position_x: None,
            position_y: None,
        }
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:11434".to_string(),
            default_model: "llama3.2".to_string(),
            temperature: default_temperature(),
            top_p: default_top_p(),
            top_k: default_top_k(),
            max_tokens: default_max_tokens(),
        }
    }
}

/// Default temperature value
fn default_temperature() -> f32 {
    0.7
}

/// Default top-p value
fn default_top_p() -> f32 {
    0.9
}

/// Default top-k value
fn default_top_k() -> u32 {
    40
}

/// Default max tokens value
fn default_max_tokens() -> u32 {
    2048
}

impl Default for ConversationConfig {
    fn default() -> Self {
        Self {
            max_length: 10000,
            auto_save: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_to_file: true,
            max_file_size: 10,
            max_files: 5,
        }
    }
}

impl Config {
    /// Validate configuration values
    pub fn validate(&self) -> Result<()> {
        // Validate window opacity
        if self.window.opacity < 0.0 || self.window.opacity > 1.0 {
            bail!("Window opacity must be between 0.0 and 1.0");
        }
        
        // Validate window dimensions
        if self.window.width < 200 || self.window.height < 200 {
            bail!("Window dimensions must be at least 200x200");
        }
        
        // Validate log level
        match self.logging.level.to_lowercase().as_str() {
            "error" | "warn" | "info" | "debug" | "trace" => {}
            _ => bail!("Invalid log level: {}", self.logging.level),
        }
        
        // Validate max conversation length
        if self.conversation.max_length < 1000 {
            bail!("Maximum conversation length must be at least 1000 characters");
        }
        
        // Validate Ollama API URL
        if !self.ollama.api_url.starts_with("http://") && !self.ollama.api_url.starts_with("https://") {
            bail!("Ollama API URL must start with http:// or https://");
        }
        
        Ok(())
    }
}
