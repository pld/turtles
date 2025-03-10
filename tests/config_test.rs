use screensage::Config;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_default_config() {
    let config = Config::default();
    
    // Verify default values
    assert_eq!(config.window.width, 400);
    assert_eq!(config.window.height, 600);
    assert_eq!(config.window.opacity, 0.9);
    assert!(config.window.always_on_top);
    
    assert_eq!(config.ollama.api_url, "http://localhost:11434");
    
    assert!(config.conversation.max_length > 0);
    assert!(config.conversation.auto_save);
    
    assert_eq!(config.logging.level, "info");
    assert!(config.logging.log_to_file);
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    
    // Test valid configuration
    assert!(config.validate().is_ok());
    
    // Test invalid opacity
    config.window.opacity = 1.5;
    assert!(config.validate().is_err());
    config.window.opacity = 0.9; // Reset to valid value
    
    // Test invalid window dimensions
    config.window.width = 100;
    assert!(config.validate().is_err());
    config.window.width = 400; // Reset to valid value
    
    // Test invalid log level
    config.logging.level = "invalid".to_string();
    assert!(config.validate().is_err());
    config.logging.level = "info".to_string(); // Reset to valid value
    
    // Test invalid API URL
    config.ollama.api_url = "invalid-url".to_string();
    assert!(config.validate().is_err());
}
