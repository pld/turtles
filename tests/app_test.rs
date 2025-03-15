use screensage::App;
use screensage::Config;
use iced::Application;

#[test]
fn test_app_initialization() {
    // Create a default configuration
    let config = Config::default();
    
    // Initialize the app
    let (app, _) = App::new(config);
    
    // Verify the app was initialized correctly
    assert_eq!(app.title(), "ScreenSage");
}

#[test]
fn test_message_handling() {
    // Create a default configuration
    let config = Config::default();
    
    // Initialize the app
    let (mut app, _) = App::new(config);
    
    // Test input message handling
    let test_message = "Test message";
    app.update_message(test_message.to_string());
    
    assert_eq!(app.message(), test_message);
    
    // Test clearing message
    app.clear_message();
    assert_eq!(app.message(), "");
}

#[test]
fn test_error_handling() {
    // Create a default configuration
    let config = Config::default();
    
    // Initialize the app
    let (mut app, _) = App::new(config);
    
    // Test setting an error
    let error_message = "Test error";
    app.set_error(Some(error_message.to_string()));
    
    assert_eq!(app.error(), Some(&error_message.to_string()));
    
    // Test clearing error
    app.set_error(None);
    assert_eq!(app.error(), None);
}
