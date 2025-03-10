use serde::{Deserialize, Serialize};

/// Request to check if a model exists
#[derive(Debug, Serialize)]
pub struct ModelInfoRequest {
    /// Name of the model to check
    pub name: String,
}

/// Response from model info request
#[derive(Debug, Deserialize)]
pub struct ModelInfoResponse {
    /// Name of the model
    pub name: String,
    /// Model metadata
    pub details: ModelDetails,
}

/// Model details
#[derive(Debug, Deserialize)]
pub struct ModelDetails {
    /// Model format
    pub format: String,
    /// Model family
    pub family: String,
    /// Model parameter size
    pub parameter_size: Option<String>,
    /// Model quantization level
    pub quantization_level: Option<String>,
}

/// Request to list available models
#[derive(Debug, Serialize)]
pub struct ListModelsRequest {}

/// Response from list models request
#[derive(Debug, Deserialize)]
pub struct ListModelsResponse {
    /// List of available models
    pub models: Vec<ModelInfo>,
}

/// Model information
#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    /// Name of the model
    pub name: String,
    /// Model size in bytes
    pub size: u64,
    /// Model modification time
    pub modified_at: String,
    /// Model digest
    pub digest: Option<String>,
}

/// Chat message for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Role of the message sender (system, user, assistant)
    pub role: String,
    /// Content of the message
    pub content: String,
}

/// Request to generate a chat completion
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    /// Model to use
    pub model: String,
    /// Messages in the conversation
    pub messages: Vec<ChatMessage>,
    /// Whether to stream the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Additional generation parameters
    #[serde(flatten)]
    pub parameters: GenerationParameters,
}

/// Parameters for text generation
#[derive(Debug, Default, Clone, Serialize)]
pub struct GenerationParameters {
    /// Temperature for sampling (higher = more random)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling (nucleus sampling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Top-k sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Maximum number of tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Stop sequences (stop generation when these are generated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

/// Response from chat completion request (non-streaming)
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    /// Model used for the response
    pub model: String,
    /// Created timestamp
    pub created_at: String,
    /// Response message
    pub message: ChatMessage,
    /// Done flag
    pub done: bool,
}

/// Streaming response chunk from chat completion
#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    /// Model used for the response
    pub model: String,
    /// Created timestamp
    pub created_at: String,
    /// Response message
    pub message: ChatMessageDelta,
    /// Done flag
    pub done: bool,
}

/// Delta of a chat message for streaming responses
#[derive(Debug, Deserialize)]
pub struct ChatMessageDelta {
    /// Role of the message sender (usually assistant)
    pub role: Option<String>,
    /// Content of the message delta
    pub content: String,
}

/// Error response from Ollama API
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}
