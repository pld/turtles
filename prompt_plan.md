# Step-by-Step Implementation Plan for macOS Floating LLM Chat Window

After analyzing the specification, I've created a detailed implementation plan that breaks down this project into manageable steps. This approach prioritizes incremental development with strong testing at each stage.

## Overall Blueprint

The implementation plan is organized into these major phases:

1. **Project Foundation** - Basic setup, configuration, and logging
2. **Window & UI** - Building the core interface components
3. **Ollama Integration** - Creating the LLM communication layer
4. **Conversation Management** - Managing chat history and persistence
5. **Integration & Refinement** - Connecting components and polishing
6. **Testing & Packaging** - Final verification and distribution

Each phase is further broken down into small, testable steps that build upon each other.

## Implementation Prompts

These prompts guide a code-generation LLM through incremental implementation using test-driven development. Each prompt builds on previous work and ensures all components are properly integrated.

### Prompt 1: Project Setup & Structure

```
# Context
We're building a macOS app with a floating window for LLM chat using Rust. This is the initial setup phase.

# Objective
Create the basic project structure and foundation for a floating chat window application.

# Requirements
- Initialize a Rust project with Cargo
- Set up the project directory structure
- Add essential dependencies for macOS GUI, serialization, and networking
- Create a simple "Hello World" window to verify setup

# Steps
1. Initialize a new Rust project named "screensage"
2. Add necessary dependencies to Cargo.toml:
   - A Rust macOS GUI library (research and select one that supports translucent/floating windows)
   - serde and serde_derive for configuration serialization
   - toml for configuration parsing
   - reqwest for HTTP requests
   - log and fern for logging
3. Create the following directory structure:
   - src/
     - main.rs (entry point)
     - app.rs (application state)
     - config/ (configuration handling)
     - ui/ (user interface components)
     - ollama/ (Ollama API integration)
     - data/ (conversation storage)
     - utils/ (utility functions)
4. Implement a basic App struct in app.rs
5. Create a simple "Hello World" window in main.rs to verify the setup

# Testing
- Ensure the project builds without errors
- Verify the window appears on screen
- Add a simple test to verify App initialization

# Deliverables
- Complete Cargo.toml with dependencies
- Project directory structure
- Initial main.rs with window creation
- Basic app.rs with App struct
- A simple test for App initialization

Checkoff the appropriate boxes in todo.md
```

### Prompt 2: Configuration System

```
# Context
Building on our project structure, we need a configuration system that can load from files and command-line arguments.

# Objective
Implement a complete configuration system that manages application settings from files and command-line arguments.

# Requirements
- Define configuration data structures
- Load configuration from TOML/JSON files
- Override configuration from command-line arguments
- Provide default values for all settings
- Save configuration changes back to file

# Steps
1. Create config/mod.rs and config/model.rs files
2. Define a Config struct with fields for:
   - model: String (Ollama model name)
   - api_url: String (URL to Ollama API)
   - window.opacity: Float (Window transparency)
   - window.position: (x,y) (Last window position)
   - window.size: (width,height) (Last window size)
   - conversation.max_length: Integer (Maximum in-memory characters)
   - logging.level: String (Log verbosity)
3. Implement the Default trait for Config
4. Add serde derives for serialization
5. Create functions to:
   - Determine the configuration file path
   - Load configuration from file
   - Save configuration to file
   - Merge with command-line arguments
6. Use clap for command-line argument parsing
7. Implement validation for configuration values

# Testing
- Create tests for default configuration
- Test loading configuration from a file
- Test saving configuration to a file
- Test command-line overrides
- Test validation of configuration values

# Integration
- Modify app.rs to use the configuration system
- Update main.rs to load configuration at startup

# Deliverables
- Complete config/mod.rs and config/model.rs files
- Configuration loading and saving functions
- Command-line argument parsing
- Tests for configuration functionality

Checkoff the appropriate boxes in todo.md
```

### Prompt 3: Logging System

```
# Context
Our application needs structured logging for debugging and troubleshooting.

# Objective
Implement a comprehensive logging system that writes to console and files with rotation.

# Requirements
- Initialize logging based on configuration
- Support different log levels
- Write logs to console and file
- Implement log rotation
- Include context information in logs

# Steps
1. Create data/logger.rs file
2. Implement a Logger struct to manage logging state
3. Create an initialization function that:
   - Sets up console logging
   - Configures file logging with path from configuration
   - Sets log level from configuration
   - Implements log formatting with timestamp, level, and component
4. Add log rotation with daily rotation and 7-day retention
5. Implement helper macros for logging
6. Add error handling for logging failures

# Testing
- Test logger initialization
- Verify logs are written to console
- Test file logging functionality
- Verify log rotation works correctly
- Test different log levels

# Integration
- Update main.rs to initialize logging early
- Use logging in configuration and window modules

# Deliverables
- Complete data/logger.rs implementation
- Log initialization function
- Log rotation functionality
- Tests for logging system

Checkoff the appropriate boxes in todo.md
```

### Prompt 4: Basic Window Creation

```
# Context
The core of our application is a floating, always-on-top window with custom appearance.

# Objective
Create a macOS window with custom appearance that stays on top of other applications.

# Requirements
- Create a window without standard chrome
- Implement always-on-top functionality
- Add semi-transparency/opacity
- Allow window dragging from any point
- Support resizing with minimum constraints
- Save and restore window position

# Steps
1. Create ui/window.rs file
2. Define a Window struct to manage window state
3. Implement window creation with:
   - Custom appearance (no standard title bar)
   - Rounded corners (10px radius)
   - Configurable opacity
   - Always-on-top property
4. Add mouse event handling for window dragging
5. Implement window resizing with minimum dimensions
6. Create functions to:
   - Save window position to configuration
   - Restore window position from configuration
7. Handle window close events

# Testing
- Test window creation with custom appearance
- Verify window stays on top of other applications
- Test window dragging functionality
- Verify resize constraints work properly
- Test position saving and restoring

# Integration
- Update app.rs to manage window state
- Modify main.rs to create the window with proper configuration

# Deliverables
- Complete ui/window.rs implementation
- Window creation and management functions
- Window position persistence
- Tests for window functionality

Checkoff the appropriate boxes in todo.md
```

### Prompt 5: UI Layout Components

```
# Context
Our window needs a two-part layout with a conversation display area and an input field.

# Objective
Implement the UI layout with presentation and input areas according to the specification.

# Requirements
- Create a two-part layout with conversation history and input field
- Style messages according to sender (user/LLM/error)
- Implement scrolling for conversation history
- Create an expanding text input field
- Handle keyboard events for submission and multiline input

# Steps
1. Create ui/presentation.rs for the conversation display area
2. Create ui/input.rs for the text input field
3. Implement the presentation area with:
   - Scrollable container for messages
   - Different styling for user messages, LLM responses, and errors
   - Automatic scrolling to latest message
4. Create the input area with:
   - Expandable text field
   - Maximum height constraint
   - Scrolling when maximum height is reached
5. Implement key event handling:
   - Enter key for submission
   - Shift+Enter for new line
6. Add styling according to specification
7. Implement clipboard operations

# Testing
- Test layout rendering
- Verify message styling for different types
- Test scrolling behavior
- Verify input field expansion
- Test key event handling

# Integration
- Update ui/window.rs to incorporate the layout
- Modify app.rs to manage UI state

# Deliverables
- Complete ui/presentation.rs implementation
- Complete ui/input.rs implementation
- Layout integration with window
- Tests for UI components

Checkoff the appropriate boxes in todo.md
```

### Prompt 6: Ollama API Integration

```
# Context
Our application needs to communicate with the local Ollama service to access language models.

# Objective
Implement Ollama API integration for model verification and message exchange.

# Requirements
- Create an API client for Ollama
- Verify the Ollama service is running
- Check if the specified model exists
- Send messages to the model
- Handle streaming responses
- Implement error handling and retries

# Steps
1. Create ollama/mod.rs, ollama/api.rs, and ollama/models.rs files
2. Define API request and response structures in models.rs
3. Implement an OllamaClient struct in api.rs with:
   - Initialization with API URL from configuration
   - Connection verification function
   - Model verification function
   - Message sending function with streaming support
4. Add error types for API communication
5. Implement retry mechanism with exponential backoff
6. Create a response processor for streaming chunks

# Testing
- Test API client initialization
- Verify connection checking works
- Test model verification
- Test message sending and response handling
- Verify error handling and retries

# Integration
- Update app.rs to initialize the Ollama client
- Prepare for integration with conversation management

# Deliverables
- Complete ollama module implementation
- API client with connection and model verification
- Message sending with streaming support
- Error handling and retry mechanism
- Tests for API functionality

Checkoff the appropriate boxes in todo.md
```

### Prompt 7: Conversation Management

```
# Context
We need to manage the conversation state, including persistence to disk and loading on startup.

# Objective
Implement conversation management with storage, retrieval, and formatting.

# Requirements
- Define conversation data structures
- Store conversation messages in memory
- Save conversations to disk
- Load conversations on startup
- Implement conversation truncation for performance
- Format messages for display

# Steps
1. Create data/conversation.rs file
2. Define Message and Conversation structs
3. Implement message formatting for different types (user/LLM/error)
4. Create functions for:
   - Adding messages to conversation
   - Saving conversation to file
   - Loading conversation from file
   - Truncating conversation when it exceeds maximum length
5. Add serialization for conversation persistence
6. Implement file management for conversation storage

# Testing
- Test adding messages to conversation
- Verify conversation persistence to disk
- Test loading conversation from file
- Verify truncation functionality
- Test message formatting

# Integration
- Update app.rs to manage conversation state
- Prepare for integration with UI components

# Deliverables
- Complete data/conversation.rs implementation
- Conversation state management
- Persistence and loading functionality
- Truncation mechanism
- Tests for conversation operations

Checkoff the appropriate boxes in todo.md
```

### Prompt 8: Application Integration

```
# Context
Now we need to connect all the components to create a functional application.

# Objective
Integrate all components to create a complete functioning application.

# Requirements
- Connect configuration, window, UI, API, and conversation components
- Implement the complete message flow
- Add error handling and display
- Ensure proper application startup and shutdown
- Implement resource cleanup

# Steps
1. Update app.rs to integrate all components:
   - Initialize configuration
   - Set up logging
   - Create window and UI components
   - Initialize Ollama client
   - Load conversation state
2. Implement the message submission flow:
   - Capture input from UI
   - Send to Ollama API
   - Display user message and LLM response
   - Handle errors
3. Add visual indicators for processing state
4. Implement proper error display in UI
5. Ensure clean shutdown with resource release
6. Add signal handling for termination

# Testing
- Test the complete message flow
- Verify error handling and display
- Test application startup and shutdown
- Verify resource cleanup

# Integration
- Update main.rs for complete application lifecycle
- Ensure all components work together

# Deliverables
- Updated app.rs with full integration
- Complete message flow implementation
- Error handling and display
- Application lifecycle management
- Tests for integrated functionality

Checkoff the appropriate boxes in todo.md
```

### Prompt 9: Polish & Optimization

```
# Context
With the basic functionality working, we need to optimize performance and refine the user experience.

# Objective
Optimize resource usage and improve the user experience with visual polish.

# Requirements
- Minimize CPU and memory usage
- Implement efficient rendering
- Add visual indicators for better UX
- Optimize conversation buffer management
- Enhance error recovery strategies

# Steps
1. Add loading indicators for API requests
2. Implement debouncing for resize events
3. Optimize rendering for conversation history
4. Add memory usage monitoring and optimization
5. Implement conversation buffer management
6. Enhance error recovery with graceful degradation
7. Refine UI styling according to specification

# Testing
- Measure CPU and memory usage
- Test performance under load
- Verify visual indicators work correctly
- Test error recovery scenarios

# Integration
- Update relevant components with optimizations
- Ensure all refinements are properly integrated

# Deliverables
- Performance optimizations
- Visual refinements
- Resource usage monitoring
- Enhanced error recovery
- Test results for performance metrics

Checkoff the appropriate boxes in todo.md
```

### Prompt 10: Testing & Packaging

```
# Context
Finally, we need comprehensive testing and packaging for distribution.

# Objective
Create a test suite and package the application for macOS.

# Requirements
- Implement unit and integration tests
- Add performance tests
- Create a macOS application bundle
- Generate user documentation

# Steps
1. Expand the test suite with:
   - Unit tests for all components
   - Integration tests for key flows
   - Performance tests for resource usage
2. Create a build script for macOS application bundling
3. Configure the application bundle with:
   - Proper application icon
   - Required permissions
   - File associations
4. Generate user documentation with:
   - Installation instructions
   - Configuration guide
   - Troubleshooting information

# Testing
- Run the complete test suite
- Verify the application bundle works correctly
- Test documentation accuracy

# Deliverables
- Comprehensive test suite
- macOS application bundle
- User documentation
- Build and packaging scripts
```

## Implementation Strategy Notes

1. **Test-Driven Development**: Each prompt emphasizes writing tests first, then implementing features.

2. **Incremental Progress**: Steps build naturally on each other, avoiding large complexity jumps.

3. **Integration Focus**: Components are designed to be connected from the start, preventing orphaned code.

4. **Error Handling**: Error cases are addressed throughout, not as an afterthought.

5. **Performance Awareness**: Resource usage is considered early in the design process.

This approach ensures a solid foundation with early testing while making steady progress toward the complete application. Each implementation stage is manageable and testable, with clear goals and deliverables.