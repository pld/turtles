pub mod api;
pub mod models;

pub use api::OllamaClient;
pub use models::{
    ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
    GenerationParameters,
};
