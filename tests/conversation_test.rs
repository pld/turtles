use screensage::{Conversation, Message, MessageRole};
use chrono::Utc;

#[test]
fn test_message_creation() {
    let content = "Test message";
    let message = Message::new(MessageRole::User, content);
    
    assert_eq!(message.role, MessageRole::User);
    assert_eq!(message.content, content);
    assert!(message.timestamp <= Utc::now());
}

#[test]
fn test_conversation_creation() {
    let title = "Test Conversation";
    let model = "test-model";
    let conversation = Conversation::new(title, model);
    
    assert_eq!(conversation.title, title);
    assert_eq!(conversation.model, model);
    assert!(conversation.messages.is_empty());
    assert!(conversation.created_at <= Utc::now());
    assert_eq!(conversation.created_at, conversation.updated_at);
}

#[test]
fn test_add_message() {
    let mut conversation = Conversation::new("Test", "test-model");
    let before_update = conversation.updated_at;
    
    // Wait a moment to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    // Add a message
    conversation.add_message(MessageRole::User, "Test message");
    
    assert_eq!(conversation.messages.len(), 1);
    assert_eq!(conversation.messages[0].role, MessageRole::User);
    assert_eq!(conversation.messages[0].content, "Test message");
    assert!(conversation.updated_at > before_update);
}

#[test]
fn test_truncate() {
    let mut conversation = Conversation::new("Test", "test-model");
    
    // Add 5 messages
    for i in 0..5 {
        conversation.add_message(MessageRole::User, &format!("Message {}", i));
    }
    
    assert_eq!(conversation.messages.len(), 5);
    
    // Truncate to 3 messages
    conversation.truncate(3);
    
    assert_eq!(conversation.messages.len(), 3);
    // Verify we kept the most recent messages
    assert_eq!(conversation.messages[0].content, "Message 2");
    assert_eq!(conversation.messages[1].content, "Message 3");
    assert_eq!(conversation.messages[2].content, "Message 4");
}
