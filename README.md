# ScreenSage User Guide

> [!WARNING] 
> This is experimental code meant as a proof of concept and may not be maintained. There is a known serious issue with the channel subscription.

## Introduction

ScreenSage is a floating window application for macOS that allows you to chat with Ollama-powered large language models directly from your desktop. It provides a clean, minimalist interface that stays out of your way while giving you quick access to AI assistance.

<img src="https://github.com/pld/turtles/blob/main/docs/screensage.png" alt="UI convesation screenshot" width="300px">

## Installation

### Requirements

- macOS 10.13 or later
- Ollama installed and running locally (or accessible via network)

### Developer Installation Steps

1. Install rust
2. Run the app
    ```sh
    cargo run
    ```
3. Test the app
    ```sh
    cargo test
    ```
4. Benchmark the app
    ```sh
    cargo bench
    ```
5. Build a release version of the app
    ```sh
    cargo build --release
    ```

### Release Installation Steps

1. Download the latest ScreenSage.dmg from the releases page
2. Open the DMG file
3. Drag ScreenSage to your Applications folder
4. Open ScreenSage from your Applications folder

If you see a security warning when first opening the app, you may need to:
1. Go to System Preferences > Security & Privacy
2. Click "Open Anyway" for ScreenSage

## Configuration

ScreenSage can be configured through the configuration file located at:
```
~/Library/Application Support/ScreenSage/config.toml
```

### Configuration Options

```toml
[window]
width = 400           # Window width in pixels
height = 600          # Window height in pixels
opacity = 0.9         # Window opacity (0.0-1.0)
always_on_top = true  # Whether window stays on top of other windows

[ollama]
api_url = "http://localhost:11434"  # Ollama API URL
default_model = "llama3.2"          # Default model to use
temperature = 0.7                   # Temperature (0.0-1.0)
top_p = 0.9                         # Top-p sampling parameter
top_k = 40                          # Top-k sampling parameter
max_tokens = 2048                   # Maximum tokens to generate

[conversation]
max_length = 10000    # Maximum conversation length
auto_save = true      # Whether to save conversations automatically

[logging]
level = "info"        # Log level (error, warn, info, debug, trace)
log_to_file = true    # Whether to log to file
max_file_size = 10    # Maximum log file size in MB
max_files = 5         # Number of log files to keep
```

## Usage

### Basic Usage

1. Type your message in the input field at the bottom of the window
2. Press Enter to send the message
3. The AI will respond in the conversation area

### Keyboard Shortcuts

- `Enter`: Send message
- `Shift+Enter`: Add a new line in the input field
- `Esc`: Clear the input field
- `Cmd+W`: Close the window

### Window Management

- Click and drag the title bar to move the window
- Resize the window by dragging the bottom-right corner
- The window position and size are saved automatically

## Troubleshooting

### Common Issues

#### Ollama Connection Issues

If you see "Failed to connect to Ollama API" error:
1. Ensure Ollama is running (`ollama serve` in terminal)
2. Check the API URL in your configuration
3. Verify network connectivity if using a remote Ollama instance

#### Model Not Found

If you see "Model not found" error:
1. Ensure the model is installed in Ollama (`ollama list`)
2. Install the model if needed (`ollama pull modelname`)

### Logs

Logs are stored in:
```
~/Library/Logs/ScreenSage/
```

Reviewing logs can help diagnose issues.

## Support

For additional support:
- Check the GitHub repository for known issues
- Submit bug reports or feature requests on the GitHub issues page

## License

ScreenSage is licensed under the MIT License. See the LICENSE file for details.
