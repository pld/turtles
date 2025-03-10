# macOS Desktop App Specification: Floating LLM Chat Window

## 1. Overview

A lightweight, always-visible macOS desktop application providing a floating window for interacting with local large language models via Ollama. The application features a clean, two-part interface with conversation history and an expanding text input field for an unobtrusive yet powerful user experience.

## 2. Technical Requirements

### 2.1 Technology Stack
- **Programming Language**: Rust
- **LLM Backend**: Ollama with quantized models
- **UI Framework**: Appropriate Rust macOS GUI library (iced, druid with macOS bindings, or rust-objc)
- **API Communication**: HTTP requests to Ollama API
- **Build System**: Cargo
- **Packaging**: macOS app bundle

### 2.2 Hardware & Software Requirements
- **OS Support**: macOS 12 (Monterey) or newer
- **Hardware**: Apple Silicon (M-series) Mac with minimum 16GB RAM
- **Dependencies**: Ollama installed and configured separately
- **Disk Space**: Minimal (<50MB for application, excluding Ollama and models)

## 3. Interface Design Specifications

### 3.1 Window Properties
- **Position**: Floating (always on top of other applications)
- **Opacity**: Semi-transparent background (configurable alpha value: default 0.85)
- **Shape**: Rounded corners (10px radius)
- **Behavior**: 
  - Movable via drag-and-drop anywhere on screen
  - Resizable with minimum dimensions (width: 300px, height: 400px)
  - No dock icon or menu bar presence
  - No standard window chrome (custom implementation)

### 3.2 Layout Components

#### 3.2.1 Presentation Area (Upper Section)
- **Purpose**: Display conversation history between user and LLM
- **Default Height**: 75% of total window height
- **Content Styling**:
  - User messages: Right-aligned, white text on blue background (#007AFF)
  - LLM responses: Left-aligned, black/dark text on light grey background (#E9E9EB)
  - Error messages: Left-aligned, white text on dark red background (#CC0000)
  - Message padding: 8px
  - Message margin: 4px
  - Message border radius: 8px
  - Font: System font, size 13pt
- **Navigation**:
  - Automatic scrolling to latest message when new content appears
  - Vertical scrollbar visible when content exceeds viewport
  - Mouse wheel scrolling supported
- **Visual Indicators**:
  - Animated dots or spinner while waiting for LLM response
  - Clear visual separation between conversation turns

#### 3.2.2 Input Area (Lower Section)
- **Purpose**: Accept user text input for submission to LLM
- **Default Height**: 25% of total window height (minimum)
- **Input Field Properties**:
  - Initial state: Single-line text entry field
  - Dynamic behavior: Expands downward when input exceeds width
  - Maximum height: 50% of window (after which scrolling is enabled)
  - Font: System font, size 13pt
  - Padding: 8px
  - Clear visual distinction from presentation area (border or color difference)

## 4. Functionality & Behavior

### 4.1 User Interaction
- **Text Submission**: Enter key submits text to LLM
- **Multi-line Input**: Shift+Enter creates a new line in input field
- **Text Selection**: Standard macOS text selection behavior
- **Copy/Paste**: Standard macOS clipboard operations supported
- **Launch Method**: Command-line execution only
- **Termination Method**: 
  - Ctrl+C in terminal where launched
  - Standard process termination signals

### 4.2 Application Lifecycle
- **Launch Process**: 
  - Launched manually via command line
  - No auto-start capability in current scope
  - Command format: `./app_name [optional_config_path]`
- **Initialization**:
  - Load configuration from file
  - Verify Ollama availability
  - Load previous conversation if available
  - Display window at last known position or center of primary screen
- **Shutdown**:
  - Save conversation state
  - Flush logs
  - Release all resources
  - Terminate cleanly with 0 exit code

## 5. Ollama Integration

### 5.1 API Communication
- **Protocol**: HTTP REST API
- **Endpoint**: `http://localhost:11434/api`
- **Methods**:
  - `/generate` for streaming responses
  - `/chat` for handling conversation context
- **Authentication**: None (local only)
- **Error Handling**: Proper HTTP status code processing with retries

### 5.2 Model Configuration
- **Selection Method**: Configuration file only
- **Parameters**:
  - Model name: String (e.g., "llama2:7b-chat-q4_0")
  - Temperature: Float (0.0-1.0)
  - Max tokens: Integer
  - System prompt: String (optional)

### 5.3 Response Processing
- **Streaming**: Process and display chunks as they arrive
- **Parsing**: JSON decoding with proper error handling
- **Formatting**: Convert markdown/special formatting if present

## 6. Data Management

### 6.1 Conversation Persistence
- **Storage Format**: Plain text or JSON
- **Save Location**: User's application data directory
  - macOS: `~/Library/Application Support/[AppName]/conversations/`
- **Save Frequency**: 
  - Incrementally after each message (user or LLM)
  - Complete save on application close
- **Load Behavior**: 
  - Automatic on startup
  - Most recent conversation loaded by default
- **Retention Policy**:
  - In-memory: Maximum 100,000 characters (configurable)
  - On-disk: No limit (disk space constrained)

### 6.2 Logging
- **Destinations**:
  - STDOUT/STDERR for CLI output
  - Log file at `~/Library/Logs/[AppName]/app.log`
- **Log Levels**:
  - ERROR: Critical failures
  - WARN: Non-critical issues
  - INFO: General operation information
  - DEBUG: Detailed debugging (enabled via config)
- **Format**: `[TIMESTAMP] [LEVEL] [COMPONENT] Message`
- **Rotation**: Daily with 7-day retention

### 6.3 Configuration
- **Format**: TOML or JSON
- **Location**: 
  - Default: `~/.config/[AppName]/config.toml`
  - Overridable via command line argument
- **Parameters**:
  - `model`: String - Ollama model name
  - `api_url`: String - URL to Ollama API
  - `window.opacity`: Float - Window transparency
  - `window.position`: (x,y) - Last window position
  - `window.size`: (width,height) - Last window size
  - `conversation.max_length`: Integer - Maximum in-memory characters
  - `logging.level`: String - Log verbosity

## 7. Error Handling

### 7.1 Error Types & Responses
- **Connection Errors**:
  - Ollama not running: "Ollama service not detected. Please start Ollama and try again."
  - Network unavailable: "Cannot connect to Ollama API. Check network configuration."
- **Model Errors**:
  - Model not found: "Model '[model_name]' not found in Ollama. Please verify it's installed."
  - Model loading failure: "Failed to load model '[model_name]'. Check Ollama logs for details."
- **API Errors**:
  - Response timeout: "Ollama response timed out. The model may be processing a complex query."
  - Invalid response: "Received invalid response from Ollama. Please check compatibility."
- **Application Errors**:
  - Configuration invalid: "Invalid configuration. Using defaults."
  - File I/O errors: "Could not read/write [file_path]. Check permissions."

### 7.2 Recovery Strategies
- **Auto-retry**: Implement exponential backoff for transient API errors
- **Graceful Degradation**: Fall back to simpler operations when advanced features fail
- **User Notification**: Display actionable error messages in the conversation area
- **Logging**: Detailed error information written to logs for troubleshooting

## 8. Performance Considerations

### 8.1 Resource Management
- **Memory Usage**: 
  - Target: Less than 100MB for application (excluding Ollama)
  - Conversation buffer management to prevent leaks
- **CPU Usage**:
  - Idle: <1% CPU
  - Active: <5% CPU (excluding Ollama processing)
- **Startup Time**: Target <500ms to window visible

### 8.2 Optimization Strategies
- **Efficient Rendering**: Minimize redraws and layout calculations
- **Asynchronous Processing**: Use Rust's async/await for API calls
- **Throttling**: Implement debouncing for resize events
- **Lazy Loading**: Only load necessary components on startup

## 9. Implementation Plan

### 9.1 Project Structure
```
src/
├── main.rs               # Entry point, initialization
├── app.rs                # Application state and lifecycle
├── config/               # Configuration handling
│   ├── mod.rs
│   └── model.rs
├── ui/                   # User interface components
│   ├── mod.rs
│   ├── window.rs         # Window management
│   ├── presentation.rs   # Conversation display area
│   └── input.rs          # Text input field
├── ollama/               # Ollama API integration
│   ├── mod.rs
│   ├── api.rs            # API client
│   └── models.rs         # Response models
├── data/                 # Data handling
│   ├── mod.rs
│   ├── conversation.rs   # Conversation storage
│   └── logger.rs         # Logging implementation
└── utils/                # Utility functions
    ├── mod.rs
    └── error.rs          # Error types and handling
```

### 9.2 Development Phases
1. **Setup Phase** (1-2 days)
   - Project initialization
   - Basic window creation
   - Configuration loading

2. **Core Functionality** (3-5 days)
   - UI components implementation
   - Ollama API integration
   - Basic conversation flow

3. **Data Management** (2-3 days)
   - Conversation persistence
   - Logging system
   - Error handling

4. **Polish & Performance** (2-3 days)
   - UI refinement
   - Performance optimization
   - Edge case handling

5. **Testing & Packaging** (1-2 days)
   - Test suite execution
   - Bug fixes
   - App bundling

## 10. Testing Strategy

### 10.1 Unit Tests
- **Coverage Target**: 70%+ of code base
- **Focus Areas**:
  - Configuration parsing
  - Data persistence
  - API client
  - Error handling

### 10.2 Integration Tests
- **Ollama API Integration**: Mock server for API response testing
- **UI Component Tests**: Verify layout and behavior
- **End-to-End Flow**: Simulate complete conversation cycles

### 10.3 Performance Tests
- **Memory Usage**: Monitor for leaks during extended use
- **Stress Testing**: Rapid input/response cycles
- **Load Testing**: Large conversation history handling

### 10.4 Manual Testing Checklist
- Window behavior (move, resize, transparency)
- Text input handling (single/multi line)
- Conversation display formatting
- Error message presentation
- Configuration changes
- Application startup/shutdown

## 11. Security Considerations

### 11.1 Data Security
- **Conversation Storage**: Local only, no cloud transmission
- **Plain Text**: Be aware conversations are stored as plain text
- **API Limitation**: Local network only, no external API exposure

### 11.2 Application Security
- **Input Validation**: Sanitize all user input
- **Resource Limits**: Implement safeguards against resource exhaustion
- **Dependency Audit**: Regular security review of dependencies

## 12. Limitations & Future Enhancements

### 12.1 Current Limitations
- Command-line launch only
- Single conversation context
- No model switching in UI
- No conversation export features
- No theme customization

### 12.2 Potential Future Enhancements
- Dock icon and menu bar integration
- Multiple conversation tabs/windows
- In-app model selection and parameter adjustment
- Markdown rendering for responses
- System-wide keyboard shortcut activation
- Conversation search functionality

## 13. Documentation

### 13.1 Developer Documentation
- Code comments following Rust documentation standards
- README with setup instructions
- Architecture overview diagram
- API documentation with examples

### 13.2 User Documentation
- Installation instructions
- Configuration file format
- Command-line parameters
- Troubleshooting guide

---

This specification is considered complete and ready for development. Changes to this specification should follow proper change management procedures with stakeholder approval.
