#[cfg(test)]
mod tests {
    use crate::config::{Config, save_config};
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        // Test default values
        assert_eq!(config.window.width, 400);
        assert_eq!(config.window.height, 600);
        assert_eq!(config.window.opacity, 0.9);
        assert!(config.window.always_on_top);
        assert_eq!(config.ollama.api_url, "http://localhost:11434");
        assert_eq!(config.ollama.default_model, "llama3.2");
        assert_eq!(config.conversation.max_length, 10000);
        assert_eq!(config.logging.level, "info");
        
        // Test validation
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        // Test invalid opacity
        let mut config = Config::default();
        config.window.opacity = 1.5;
        assert!(config.validate().is_err());
        
        // Test invalid window dimensions
        config = Config::default();
        config.window.width = 100;
        assert!(config.validate().is_err());
        
        // Test invalid log level
        config = Config::default();
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Test invalid max conversation length
        config = Config::default();
        config.conversation.max_length = 500;
        assert!(config.validate().is_err());
        
        // Test invalid API URL
        config = Config::default();
        config.ollama.api_url = "localhost:11434".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_save_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        // Create a config with non-default values
        let mut config = Config::default();
        config.window.width = 500;
        config.window.height = 700;
        config.ollama.default_model = "mistral".to_string();
        
        // Save the config
        save_config(&config, Some(config_path.clone())).unwrap();
        
        // Verify the file exists
        assert!(config_path.exists());
        
        // Load the config using the module function instead of a method
        let loaded_config = toml::from_str::<Config>(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
        
        // Verify the loaded values match the saved values
        assert_eq!(loaded_config.window.width, 500);
        assert_eq!(loaded_config.window.height, 700);
        assert_eq!(loaded_config.ollama.default_model, "mistral");
    }
}
