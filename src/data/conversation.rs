use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Role of a message sender
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// User message
    User,
    /// Assistant (LLM) message
    Assistant,
}

impl MessageRole {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
        }
    }

    /// Convert from string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "user" => Some(MessageRole::User),
            "assistant" => Some(MessageRole::Assistant),
            _ => None,
        }
    }
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender
    pub role: MessageRole,
    /// Content of the message
    pub content: String,
    /// Timestamp when the message was created
    pub timestamp: DateTime<Utc>,
}

impl Message {
    /// Create a new message
    pub fn new(role: MessageRole, content: &str) -> Self {
        Self {
            role,
            content: content.to_string(),
            timestamp: Utc::now(),
        }
    }

    /// Format the message for display
    pub fn format(&self) -> String {
        format!("{}: {}", self.role.as_str(), self.content)
    }
}

/// A conversation between a user and an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// The ID of the conversation
    pub id: String,
    /// The title of the conversation
    pub title: String,
    /// The messages in the conversation
    pub messages: Vec<Message>,
    /// The model used for the conversation
    pub model: String,
    /// The creation timestamp
    pub created_at: DateTime<Utc>,
    /// The last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    /// Create a new conversation
    pub fn new(title: &str, model: &str) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            messages: Vec::new(),
            model: model.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a message to the conversation
    pub fn add_message(&mut self, role: MessageRole, content: &str) {
        let message = Message::new(role, content);
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Get the conversation directory path
    pub fn get_conversations_dir() -> PathBuf {
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("screensage");
        path.push("conversations");
        path
    }

    /// Get the file path for this conversation
    pub fn get_file_path(&self) -> PathBuf {
        let mut path = Self::get_conversations_dir();
        path.push(format!("{}.json", self.id));
        path
    }

    /// Save the conversation to a file
    pub fn save(&self) -> Result<()> {
        let path = self.get_file_path();
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        
        // Serialize and save conversation
        let file = File::create(&path)
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
        
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)
            .with_context(|| format!("Failed to write conversation to file: {}", path.display()))?;
        
        debug!("Saved conversation {} to {}", self.id, path.display());
        Ok(())
    }

    /// Load a conversation from a file
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        
        let reader = BufReader::new(file);
        let conversation: Self = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse conversation from file: {}", path.display()))?;
        
        debug!("Loaded conversation {} from {}", conversation.id, path.display());
        Ok(conversation)
    }

    /// Load all conversations from the conversations directory
    pub fn load_all() -> Result<Vec<Self>> {
        let dir = Self::get_conversations_dir();
        
        // Create directory if it doesn't exist
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
            return Ok(Vec::new());
        }
        
        let mut conversations = Vec::new();
        
        // Read all JSON files in the directory
        for entry in fs::read_dir(&dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))? {
            
            let entry = entry?;
            let path = entry.path();
            
            // Skip non-JSON files
            if path.extension().is_some_and(|ext| ext == "json") {
                match Self::load(&path) {
                    Ok(conversation) => conversations.push(conversation),
                    Err(e) => {
                        error!("Failed to load conversation from {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        // Sort conversations by updated_at (newest first)
        conversations.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        info!("Loaded {} conversations from {}", conversations.len(), dir.display());
        Ok(conversations)
    }

    /// Truncate the conversation to the specified maximum number of messages
    pub fn truncate(&mut self, max_messages: usize) {
        if self.messages.len() > max_messages {
            let to_remove = self.messages.len() - max_messages;
            self.messages.drain(0..to_remove);
            debug!("Truncated conversation {} to {} messages", self.id, max_messages);
        }
    }

    /// Get the last message in the conversation
    pub fn last_message(&self) -> Option<&Message> {
        self.messages.last()
    }

    /// Get the number of messages in the conversation
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Check if the conversation is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Get a summary of the conversation
    pub fn summary(&self) -> String {
        if self.messages.is_empty() {
            return format!("{} (empty)", self.title);
        }
        
        let last_message = self.last_message().unwrap();
        let preview = if last_message.content.len() > 50 {
            format!("{}...", &last_message.content[..47])
        } else {
            last_message.content.clone()
        };
        
        format!(
            "{} - {} messages - Last: {}",
            self.title,
            self.messages.len(),
            preview
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_message_role_conversion() {
        assert_eq!(MessageRole::User.as_str(), "user");
        assert_eq!(MessageRole::Assistant.as_str(), "assistant");
        
        assert_eq!(MessageRole::from_str("user"), Some(MessageRole::User));
        assert_eq!(MessageRole::from_str("USER"), Some(MessageRole::User));
        assert_eq!(MessageRole::from_str("assistant"), Some(MessageRole::Assistant));
        assert_eq!(MessageRole::from_str("ASSISTANT"), Some(MessageRole::Assistant));
        assert_eq!(MessageRole::from_str("unknown"), None);
    }

    #[test]
    fn test_message_creation() {
        let message = Message::new(MessageRole::User, "Hello");
        assert_eq!(message.role, MessageRole::User);
        assert_eq!(message.content, "Hello");
        assert!(message.timestamp <= Utc::now());
    }

    #[test]
    fn test_conversation_creation() {
        let conversation = Conversation::new("Test Conversation", "gpt-3.5-turbo");
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.model, "gpt-3.5-turbo");
        assert!(conversation.messages.is_empty());
        assert!(conversation.created_at <= Utc::now());
        assert_eq!(conversation.created_at, conversation.updated_at);
    }

    #[test]
    fn test_add_message() {
        let mut conversation = Conversation::new("Test", "model");
        let before_update = conversation.updated_at;
        
        // Wait a moment to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        conversation.add_message(MessageRole::User, "Hello");
        assert_eq!(conversation.messages.len(), 1);
        assert_eq!(conversation.messages[0].role, MessageRole::User);
        assert_eq!(conversation.messages[0].content, "Hello");
        assert!(conversation.updated_at > before_update);
        
        conversation.add_message(MessageRole::Assistant, "Hi there");
        assert_eq!(conversation.messages.len(), 2);
        assert_eq!(conversation.messages[1].role, MessageRole::Assistant);
        assert_eq!(conversation.messages[1].content, "Hi there");
    }

    #[test]
    fn test_truncate() {
        let mut conversation = Conversation::new("Test", "model");
        
        // Add 5 messages
        for i in 0..5 {
            conversation.add_message(MessageRole::User, &format!("Message {}", i));
        }
        
        assert_eq!(conversation.messages.len(), 5);
        
        // Truncate to 3 messages
        conversation.truncate(3);
        assert_eq!(conversation.messages.len(), 3);
        
        // Check that the oldest messages were removed
        assert_eq!(conversation.messages[0].content, "Message 2");
        assert_eq!(conversation.messages[1].content, "Message 3");
        assert_eq!(conversation.messages[2].content, "Message 4");
    }

    #[test]
    fn test_save_and_load() {
        // Create a temporary directory for the test
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("conversation.json");
        
        // Create a conversation with messages
        let mut conversation = Conversation::new("Test Save Load", "test-model");
        conversation.add_message(MessageRole::User, "Hello");
        conversation.add_message(MessageRole::Assistant, "Hi there");
        
        // Override the file path method for testing
        let original_get_file_path = conversation.get_file_path();
        let conversation_id = conversation.id.clone();
        
        // Save the conversation to the temporary file
        let save_result = conversation.save();
        assert!(save_result.is_ok());
        assert!(original_get_file_path.exists());
        
        // Load the conversation from the file
        let loaded = Conversation::load(&original_get_file_path);
        assert!(loaded.is_ok());
        
        let loaded_conversation = loaded.unwrap();
        assert_eq!(loaded_conversation.id, conversation_id);
        assert_eq!(loaded_conversation.title, "Test Save Load");
        assert_eq!(loaded_conversation.model, "test-model");
        assert_eq!(loaded_conversation.messages.len(), 2);
        assert_eq!(loaded_conversation.messages[0].role, MessageRole::User);
        assert_eq!(loaded_conversation.messages[0].content, "Hello");
        assert_eq!(loaded_conversation.messages[1].role, MessageRole::Assistant);
        assert_eq!(loaded_conversation.messages[1].content, "Hi there");
    }

    #[test]
    fn test_summary() {
        let mut conversation = Conversation::new("Test Summary", "model");
        
        // Empty conversation
        assert_eq!(conversation.summary(), "Test Summary (empty)");
        
        // Add a short message
        conversation.add_message(MessageRole::User, "Hello");
        assert_eq!(conversation.summary(), "Test Summary - 1 messages - Last: Hello");
        
        // Add a long message
        let long_message = "This is a very long message that should be truncated in the summary because it exceeds the maximum length allowed for previews.";
        conversation.add_message(MessageRole::Assistant, long_message);
        assert_eq!(conversation.summary(), "Test Summary - 2 messages - Last: This is a very long message that should be trun...");
    }
}
