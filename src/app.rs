use iced::{
    Application, Command, Element, Length, Point, Subscription, Theme,
    executor, window as iced_window,
};
use iced::theme;
use iced::widget::{column, container, scrollable};
use log::{debug, error, info};
use futures::StreamExt;
use std::sync::{Arc, Mutex};

use crate::ui::window as ui_window;
use crate::ollama::api::OllamaClient;
use crate::data::conversation::MessageRole;

/// Main application state
pub struct App {
    /// Whether the application is being dragged
    dragging: bool,
    /// The position where the drag started
    drag_start: Option<(i32, i32)>,
    /// The current message being composed
    message: String,
    /// Application configuration
    config: crate::config::Config,
    /// Window state
    window: ui_window::Window,
    /// Current conversation
    conversation: crate::data::conversation::Conversation,
    /// Whether a message is currently being sent
    is_sending: bool,
    /// Whether to scroll to the bottom of the conversation
    scroll_to_bottom: bool,
    /// Current error message, if any
    error: Option<String>,
    /// Ollama API client
    ollama_client: Option<OllamaClient>,
    /// Current streaming response content
    streaming_content: String,
    /// Loading state with optional message
    loading_state: Option<String>,
    /// Last resize event timestamp for debouncing
    last_resize_time: std::time::Instant,
    /// Memory usage monitoring
    memory_usage: Option<u64>,
    /// Channel sender for streaming chunks
    chunk_sender: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    channel_state: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedReceiver<String>>>>,
    /// Whether streaming is active
    is_streaming: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    DragStarted(i32, i32),
    DragMoved(i32, i32),
    DragEnded,
    Close,
    InputChanged(String),
    SendMessage,
    // Window-related messages
    Resize(u32, u32),
    ResizeEnded,
    Moved(i32, i32),
    MouseDown,
    MouseUp,
    MouseMoved(Point),
    // UI-related messages
    NewLine,
    ScrollToBottom,
    // API-related messages
    OllamaConnected(OllamaClient),
    OllamaConnectionFailed(String),
    MessageChunkReceived(String),
    MessageReceived(String),
    MessageError(String),
    SaveConfig,
    // Streaming-related messages
    StartStreaming,
    StreamChunk(String),
    EndStreaming,
}

impl App {
    /// Update memory usage statistics
    pub fn update_memory_usage(&mut self) {
        // This is a simple implementation that gets the current process memory usage
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let pid = std::process::id();
            
            // Use ps command to get memory usage on macOS
            if let Ok(output) = Command::new("ps")
                .args(&["-o", "rss=", "-p", &pid.to_string()])
                .output() 
            {
                if let Ok(mem_str) = String::from_utf8(output.stdout) {
                    if let Ok(mem_kb) = mem_str.trim().parse::<u64>() {
                        // Convert KB to MB
                        self.memory_usage = Some(mem_kb / 1024);
                        debug!("Current memory usage: {} MB", self.memory_usage.unwrap());
                    }
                }
            }
        }
        
        // For other platforms, we could implement different methods
        #[cfg(not(target_os = "macos"))]
        {
            // Placeholder for other platforms
            self.memory_usage = None;
        }
    }
    
    /// Optimize conversation buffer if memory usage is high
    pub fn optimize_conversation_buffer(&mut self) {
        // If memory usage is above threshold (e.g., 100MB), optimize
        if let Some(usage) = self.memory_usage {
            if usage > 100 {
                info!("Memory usage high ({}MB), optimizing conversation buffer", usage);
                
                // Truncate conversation if it's very long
                let max_length = self.config.conversation.max_length;
                if self.conversation.messages.len() > max_length / 2 {
                    self.conversation.truncate(max_length / 2);
                }
                
                // Force garbage collection by clearing and shrinking buffers
                self.streaming_content.shrink_to_fit();
                
                // Update memory usage after optimization
                self.update_memory_usage();
            }
        }
    }
    
    /// Get the current memory usage in MB
    pub fn get_memory_usage(&self) -> Option<u64> {
        self.memory_usage
    }
    
    /// Get the current error message, if any
    pub fn error(&self) -> Option<&String> {
        self.error.as_ref()
    }
    
    /// Set the error message
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }
    
    /// Get the current message being composed
    pub fn message(&self) -> &str {
        &self.message
    }
    
    /// Update the message being composed
    pub fn update_message(&mut self, message: String) {
        self.message = message;
    }
    
    /// Clear the message being composed
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
    
    /// Check if a message is currently being sent
    pub fn is_sending(&self) -> bool {
        self.is_sending
    }
    
    /// Add a message to the conversation
    pub fn add_message(&mut self, role: MessageRole, content: &str) {
        self.conversation.add_message(role, content);
        self.scroll_to_bottom = true;
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = crate::config::Config;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        info!("Initializing App with configuration");
        
        // Try to load the most recent conversation or create a new one
        let conversation = match crate::data::conversation::Conversation::load_all() {
            Ok(conversations) if !conversations.is_empty() => {
                info!("Loaded existing conversation: {}", conversations[0].title);
                conversations[0].clone()
            }
            _ => {
                info!("Creating new conversation");
                crate::data::conversation::Conversation::new(
                    "New Conversation", 
                    &flags.ollama.default_model
                )
            }
        };

        // Create a channel for streaming chunks
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<String>();
        
        let app = Self {
            dragging: false,
            drag_start: None,
            message: String::new(),
            config: flags.clone(),
            window: ui_window::Window::new(&flags),
            conversation,
            is_sending: false,
            scroll_to_bottom: true, // Set to true initially to scroll to bottom on load
            error: None,
            ollama_client: None,
            streaming_content: String::new(),
            loading_state: Some("Connecting to Ollama API...".to_string()),
            last_resize_time: std::time::Instant::now(),
            memory_usage: None,
            chunk_sender: Some(sender),
            channel_state: Arc::new(Mutex::new(Some(receiver))),
            is_streaming: false,
        };
        
        // Initialize Ollama client
        let api_url = flags.ollama.api_url.clone();
        
        (
            app,
            Command::batch(vec![
                // Ensure we scroll to bottom after connection
                Command::perform(async {}, |_| Message::ScrollToBottom),
                Command::perform(
                    async move {
                        match OllamaClient::new(&api_url) {
                            Ok(client) => {
                                // Test connection to Ollama API
                                match client.list_models().await {
                                    Ok(_) => Ok(client),
                                    Err(e) => Err(format!("Failed to connect to Ollama API: {}", e))
                                }
                            },
                            Err(e) => Err(format!("Failed to create Ollama client: {}", e))
                        }
                    },
                    |result| match result {
                        Ok(client) => Message::OllamaConnected(client),
                        Err(e) => Message::OllamaConnectionFailed(e),
                    }
                ),
            ])
        )
    }

    fn title(&self) -> String {
        String::from("ScreenSage")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DragStarted(x, y) => {
                self.dragging = true;
                self.drag_start = Some((x, y));
                info!("Drag started at {}, {}", x, y);
                Command::none()
            }
            Message::DragMoved(x, y) => {
                if self.dragging {
                    if let Some((start_x, start_y)) = self.drag_start {
                        let delta_x = x - start_x;
                        let delta_y = y - start_y;
                        
                        let window_x = delta_x;
                        let window_y = delta_y;
                        info!("Moving window to {}, {}", window_x, window_y);
                        return iced_window::move_to(window_x, window_y);
                    }
                }
                Command::none()
            }
            Message::DragEnded => {
                self.dragging = false;
                self.drag_start = None;
                
                // Save window position to config
                if let Err(e) = self.window.save_to_config(&mut self.config) {
                    debug!("Failed to save window position: {}", e);
                }
                
                Command::none()
            }
            Message::Close => {
                // Save window position before closing
                if let Err(e) = self.window.save_to_config(&mut self.config) {
                    debug!("Failed to save window position: {}", e);
                }
                
                iced_window::close()
            }
            Message::InputChanged(value) => {
                self.message = value;
                Command::none()
            }
            Message::OllamaConnected(client) => {
                info!("Successfully connected to Ollama API");
                self.ollama_client = Some(client);
                self.error = None;
                self.loading_state = None;
                
                // Start memory usage monitoring
                self.update_memory_usage();
                
                Command::none()
            }
            Message::OllamaConnectionFailed(error) => {
                error!("Failed to connect to Ollama API: {}", error);
                self.error = Some(format!("Failed to connect to Ollama API: {}", error));
                Command::none()
            }
            Message::SendMessage => {
                if self.message.trim().is_empty() || self.is_sending {
                    return Command::none();
                }
                
                debug!("Message sent: {}", self.message);
                
                // Add the user message to the conversation
                let user_message = self.message.clone();
                self.conversation.add_message(MessageRole::User, &user_message);
                
                // Save the conversation to disk
                if let Err(e) = self.conversation.save() {
                    error!("Failed to save conversation: {}", e);
                }
                
                // Truncate the conversation if it exceeds the maximum length
                let max_length = self.config.conversation.max_length;
                if max_length > 0 {
                    self.conversation.truncate(max_length);
                }

                // Clear the input and set sending state
                self.message = String::new();
                self.is_sending = true;
                self.streaming_content = String::new();
                self.loading_state = Some("Waiting for response...".to_string());

                // Check if we have a valid Ollama client
                if let Some(client) = &self.ollama_client {
                    let client = client.clone();
                    let model = self.config.ollama.default_model.clone();
                    let messages = self.conversation.messages.clone();

                    // Convert our messages to Ollama API format
                    let ollama_messages = messages.iter().map(|msg| {
                        crate::ollama::models::ChatMessage {
                            role: msg.role.as_str().to_string(),
                            content: msg.content.clone(),
                        }
                    }).collect::<Vec<_>>();

                    // Clone the configuration values we need
                    let temperature = self.config.ollama.temperature;
                    let top_p = self.config.ollama.top_p;
                    let top_k = self.config.ollama.top_k;
                    let max_tokens = self.config.ollama.max_tokens;

                    let request = crate::ollama::models::ChatCompletionRequest {
                        model,
                        messages: ollama_messages,
                        stream: Some(true),
                        parameters: crate::ollama::models::GenerationParameters {
                            temperature: Some(temperature),
                            top_p: Some(top_p),
                            top_k: Some(top_k),
                            max_tokens: Some(max_tokens),
                            presence_penalty: None,
                            frequency_penalty: None,
                            stop: None,
                        },
                    };
                    
                    info!("Sending message to Ollama API");
                    
                    // Add an initial empty assistant message that we'll update with chunks
                    self.conversation.add_message(MessageRole::Assistant, "");
                    
                    let sender = self.chunk_sender.clone().unwrap();
                    self.is_streaming = true;
                    
                    // Create a command to start processing the stream
                    let start_stream_command = Command::perform(
                        async { }, 
                        |_| Message::StartStreaming
                    );
                    
                    // Create a command to process the stream
                    let stream_command = Command::perform(
                        async move {
                            let stream_result = client.chat_completion_stream(&request).await;
                            match stream_result {
                                Ok(mut stream) => {
                                    let mut full_content = String::new();
                                    
                                    // Process each chunk as it arrives
                                    while let Some(chunk_result) = stream.next().await {
                                        match chunk_result {
                                            Ok(chunk) => {
                                                let content = chunk.message.content.clone();
                                                if !content.is_empty() {
                                                    full_content.push_str(&content);
                                                    // Send the chunk through the channel
                                                    info!("Sending stream chunk: {}", content);
                                                    let _ = sender.send(content);
                                                }
                                                
                                                // If this is the last chunk, break
                                                if chunk.done {
                                                    break;
                                                }
                                            },
                                            Err(e) => {
                                                return Err(format!("Stream error: {}", e));
                                            }
                                        }
                                    }
                                    
                                    Ok(full_content)
                                },
                                Err(e) => Err(format!("Failed to create stream: {}", e)),
                            }
                        },
                        |result| match result {
                            Ok(content) => {
                                if content.is_empty() {
                                    Message::MessageError("Received empty response from Ollama".to_string())
                                } else {
                                    Message::EndStreaming
                                }
                            },
                            Err(e) => Message::MessageError(e),
                        }
                    );
                    
                    // Return both commands
                    Command::batch(vec![start_stream_command, stream_command])
                } else {
                    // No Ollama client available
                    self.is_sending = false;
                    self.error = Some("Ollama API client not initialized. Please check your connection.".to_string());
                    Command::none()
                }
            }

            Message::StartStreaming => {
                // Set up a subscription to the channel
                let sender = self.chunk_sender.clone();

                // Create a command to poll the channel
                Command::perform(
                    async move {
                        if let Some(_sender) = sender {
                            // This is just to keep the sender alive
                            // The actual receiving is done in the subscription
                        }
                    },
                    |_| Message::ScrollToBottom
                )
            }
            
            Message::StreamChunk(chunk) => {
                // Append the chunk to the streaming content
                self.streaming_content.push_str(&chunk);

                info!("In Message::StreamChunk: {}", chunk);
                
                // Update the last message with the new content
                if let Some(last) = self.conversation.messages.last_mut() {
                    if last.role == MessageRole::Assistant {
                        last.content = self.streaming_content.clone();
                        info!("Updated assistant message with chunk: {}", chunk);
                    }
                }
                
                // Always scroll to bottom when receiving new content                
                Command::perform(async {}, |_| Message::ScrollToBottom)
            }
            Message::EndStreaming => {
                info!("Streaming completed");
                
                // Save the conversation to disk
                if let Err(e) = self.conversation.save() {
                    error!("Failed to save conversation: {}", e);
                }

                // Reset streaming state
                self.is_streaming = false;
                self.is_sending = false;
                self.loading_state = None;
                
                // Check memory usage after receiving a message
                self.update_memory_usage();
                self.optimize_conversation_buffer();
                
                // Ensure we scroll to the bottom
                Command::perform(async {}, |_| Message::ScrollToBottom)
            }
            
            Message::MessageReceived(response) => {
                info!("Received complete response: {}", response);
                
                // Update the last message or add a new one if needed
                if let Some(last) = self.conversation.messages.last_mut() {
                    if last.role == MessageRole::Assistant && last.content.is_empty() {
                        // Update the existing empty assistant message
                        last.content = response;
                    } else {
                        // Add a new assistant message
                        self.conversation.add_message(MessageRole::Assistant, &response);
                    }
                } else {
                    // Add a new assistant message
                    self.conversation.add_message(MessageRole::Assistant, &response);
                }
                
                // Save the conversation to disk
                if let Err(e) = self.conversation.save() {
                    error!("Failed to save conversation: {}", e);
                }

                // Reset sending state
                self.is_sending = false;
                self.streaming_content = String::new();
                self.scroll_to_bottom = true;
                self.loading_state = None;

                // Check memory usage after receiving a message
                self.update_memory_usage();
                self.optimize_conversation_buffer();
                
                Command::none()
            }
            Message::MessageError(error) => {
                // Set the error message
                error!("Message error: {}", error);
                self.error = Some(error);
                
                // Reset sending state
                self.is_sending = false;
                
                Command::none()
            }
            Message::MessageChunkReceived(chunk) => {
                // This is similar to StreamChunk but kept for compatibility
                debug!("Received message chunk: {}", chunk);
                self.streaming_content.push_str(&chunk);
                
                // Update the last message with the new content
                if let Some(last) = self.conversation.messages.last_mut() {
                    if last.role == MessageRole::Assistant {
                        last.content = self.streaming_content.clone();
                    }
                }
                
                Command::perform(async {}, |_| Message::ScrollToBottom)
            }
            Message::SaveConfig => {
                // Save the current configuration
                if let Err(e) = crate::config::save_config(&self.config, None) {
                    error!("Failed to save configuration: {}", e);
                    self.error = Some(format!("Failed to save configuration: {}", e));
                } else {
                    info!("Configuration saved successfully");
                }
                
                Command::none()
            }
            Message::NewLine => {
                // Add a newline to the message
                self.message.push('\n');
                
                Command::none()
            }
            Message::ScrollToBottom => {
                info!("Scrolling to bottom of conversation");
                // Reset the scroll flag after sending the scroll command
                self.scroll_to_bottom = false;
                scrollable::scroll_to(
                    scrollable::Id::new("conversation_messages"),
                    scrollable::AbsoluteOffset { x: 0.0, y: f32::MAX }, // Use MAX to ensure we get to the bottom
                )
            }
            Message::Resize(width, height) => {
                // Debounce resize events - only process if it's been at least 100ms since last resize
                let now = std::time::Instant::now();
                let duration = now.duration_since(self.last_resize_time);
                
                if duration.as_millis() > 100 {
                    self.window.set_size(iced::Size::new(width as f32, height as f32));
                    self.last_resize_time = now;
                }
                
                Command::none()
            }
            Message::ResizeEnded => {
                // Save window size to config
                if let Err(e) = self.window.save_to_config(&mut self.config) {
                    debug!("Failed to save window size: {}", e);
                }
                
                // Check memory usage after resize operations
                self.update_memory_usage();
                self.optimize_conversation_buffer();
                
                Command::none()
            }
            Message::Moved(x, y) => {
                self.window.set_position(iced::window::Position::Specific(x, y));
                Command::none()
            }
            Message::MouseDown => {
                // Start dragging when mouse is pressed on the title bar
                if let Some(msg) = self.window.handle_mouse_press(Point::new(0.0, 0.0)) {
                    return self.update(msg);
                }
                Command::none()
            }
            Message::MouseUp => {
                // Stop dragging when mouse is released
                if let Some(msg) = self.window.handle_mouse_release() {
                    return self.update(msg);
                }
                Command::none()
            }
            Message::MouseMoved(position) => {
                // Handle mouse move for window dragging
                if let Some(msg) = self.window.handle_mouse_move(position) {
                    return self.update(msg);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // Create a title bar using the UI module
        let title_bar = ui_window::title_bar(&self.window);

        // Create the presentation area for the conversation
        let presentation = crate::ui::presentation::presentation_area(
            &self.conversation,
            &Theme::Dark, // Use the dark theme for now
        );

        // Create the input area
        let input_area = crate::ui::input::input_area(
            &self.message,
            self.is_sending,
            &Theme::Dark, // Use the dark theme for now
        );

        // Create content with error or loading indicators
        let content = if let Some(error) = &self.error {
            column![
                presentation,
                crate::ui::presentation::error_message(error, &Theme::Dark),
                input_area,
            ]
            .spacing(10)
        } else if let Some(loading_message) = &self.loading_state {
            column![
                presentation,
                crate::ui::presentation::loading_indicator(loading_message, &Theme::Dark),
                input_area,
            ]
            .spacing(10)
        } else {
            column![
                presentation,
                input_area,
            ]
            .spacing(10)
        };

        // Combine all elements into a content column
        let content_column = column![
            title_bar,
            content,
        ]
        .spacing(0);
        
        // Create the container with styling
        container(content_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Box)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = vec![
            ui_window::Window::subscription(),
            crate::ui::input::keyboard_subscription(),
        ];

        let state = self.channel_state.clone();
        
        // Add a subscription for streaming chunks if we're streaming
        if self.is_streaming {
            let chunk_stream = iced::subscription::unfold(
                "chunk_stream",
                (),
                move |_| {
                    let state_clone = state.clone();
                    async move {
                        // Try to get the receiver from the shared state
                        let mut receiver_option = None;
                        
                        // Scope for the mutex lock
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            // Take the receiver if it exists
                            if state_guard.is_some() {
                                receiver_option = state_guard.take();
                            }
                        }

                    if let Some(mut receiver) = receiver_option {
                        info!("Waiting for chunk...");

                        // Wait for a chunk
                        match receiver.recv().await {
                            Some(chunk) => {
                                info!("Received stream chunk: {}", chunk);

                                // Put the receiver back for next time
                                {
                                    let mut state_guard = state_clone.lock().unwrap();
                                    *state_guard = Some(receiver);
                                }
                            
                                 return (Message::StreamChunk(chunk), ());
                            }
                            None => {
                                // If the channel is closed, just return None
                                info!("Channel closed, ending stream");
                                return (Message::EndStreaming, ());
                            }
                        }
                    }
                    
                    // If we're not streaming or the channel is closed, just return None
                    (Message::ScrollToBottom, ())
                  }
                }
            );
            
            subscriptions.push(chunk_stream);
        }
        
        Subscription::batch(subscriptions)
    }
}
