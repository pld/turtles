use app::App;
use iced::{Application, Settings};
use log::info;

mod app;
mod config;
mod data;
mod ollama;
mod ui;
mod utils;

fn main() -> iced::Result {
    // Set up signal handlers for clean shutdown
    setup_signal_handlers();
    
    // Load configuration first (without logging)
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            eprintln!("Using default configuration");
            config::Config::default()
        }
    };
    
    // Initialize logger with configuration
    if let Err(e) = data::logger::init_logger(&config) {
        eprintln!("Failed to set up logger: {}", e);
        // Convert error to a string error that implements std::error::Error
        return Err(iced::Error::WindowCreationFailed(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))));
    }
    
    info!("Starting ScreenSage application");
    info!("Configuration loaded successfully");
    
    // Run the application with the loaded configuration
    let result = App::run(Settings {
        window: crate::ui::window::create_window_settings(&config),
        flags: config,
        ..Default::default()
    });
    
    // Perform cleanup on exit
    info!("ScreenSage application shutting down");
    cleanup_resources();
    
    result
}

/// Set up signal handlers for clean shutdown
fn setup_signal_handlers() {
    // Use ctrlc crate to handle Ctrl+C signals
    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("Received termination signal, shutting down...");
        // The actual cleanup will be done in the main function after App::run returns
        std::process::exit(0);
    }) {
        eprintln!("Error setting Ctrl+C handler: {}", e);
    }
}

/// Clean up resources before exit
fn cleanup_resources() {
    // Any additional cleanup can be done here
    // Most resources should be automatically cleaned up when they go out of scope
}
