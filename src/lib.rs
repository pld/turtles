pub mod app;
pub mod config;
pub mod data;
pub mod ollama;
pub mod ui;
pub mod utils;

// Re-export key types for testing
pub use app::App;
pub use config::Config;
pub use data::conversation::{Conversation, Message, MessageRole};
pub use ollama::api::OllamaClient;
