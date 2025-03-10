use anyhow::{Context, Result};
use iced::{
    event, keyboard, mouse, subscription, window,
    Event, Point, Rectangle, Size, Subscription,
};
use iced::window::Position;
use log::{debug, info};
use std::collections::HashMap;

use crate::app::Message;
use crate::config::{Config, save_config};

/// Window state
#[derive(Debug)]
pub struct Window {
    /// Window ID
    id: String,
    /// Window title
    title: String,
    /// Window size
    size: Size,
    /// Window position
    position: Position,
    /// Window minimum size
    min_size: Size,
    /// Window opacity
    opacity: f32,
    /// Whether the window is always on top
    always_on_top: bool,
    /// Whether the window is being dragged
    dragging: bool,
    /// The position where the drag started
    drag_start: Option<Point>,
    /// The window position when the drag started
    window_start_pos: Option<(i32, i32)>,
    /// Whether the window is resizing
    resizing: bool,
    /// The resize direction
    resize_direction: ResizeDirection,
    /// The resize start position
    resize_start: Option<Point>,
    /// The window size when the resize started
    window_start_size: Option<Size>,
}

/// Resize direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDirection {
    /// No resize
    None,
    /// Top-left resize
    TopLeft,
    /// Top-right resize
    TopRight,
    /// Bottom-left resize
    BottomLeft,
    /// Bottom-right resize
    BottomRight,
}

impl Window {
    /// Create a new window
    pub fn new(config: &Config) -> Self {
        let id = format!("screensage-{}", uuid::Uuid::new_v4());
        let title = "ScreenSage".to_string();
        let size = Size::new(config.window.width as f32, config.window.height as f32);
        let position = match (config.window.position_x, config.window.position_y) {
            (Some(x), Some(y)) => Position::Specific(x, y),
            _ => Position::Centered,
        };
        let min_size = Size::new(300.0, 400.0);
        let opacity = config.window.opacity;
        let always_on_top = config.window.always_on_top;

        Self {
            id,
            title,
            size,
            position,
            min_size,
            opacity,
            always_on_top,
            dragging: false,
            drag_start: None,
            window_start_pos: None,
            resizing: false,
            resize_direction: ResizeDirection::None,
            resize_start: None,
            window_start_size: None,
        }
    }

    /// Get the window ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the window size
    pub fn size(&self) -> Size {
        self.size
    }

    /// Get the window position
    pub fn position(&self) -> Position {
        self.position
    }

    /// Get the window minimum size
    pub fn min_size(&self) -> Size {
        self.min_size
    }

    /// Get the window opacity
    pub fn opacity(&self) -> f32 {
        self.opacity
    }

    /// Get whether the window is always on top
    pub fn always_on_top(&self) -> bool {
        self.always_on_top
    }

    /// Set the window position
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Set the window size
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Set the window opacity
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.max(0.0).min(1.0);
    }

    /// Set whether the window is always on top
    pub fn set_always_on_top(&mut self, always_on_top: bool) {
        self.always_on_top = always_on_top;
    }

    /// Handle a mouse press event
    pub fn handle_mouse_press(&mut self, position: Point) -> Option<Message> {
        // Check if we're in a resize area
        let resize_area_size = 10.0;
        let window_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.size.width,
            height: self.size.height,
        };

        // Check corners for resize
        if position.x <= resize_area_size && position.y <= resize_area_size {
            // Top-left corner
            self.resizing = true;
            self.resize_direction = ResizeDirection::TopLeft;
            self.resize_start = Some(position);
            self.window_start_size = Some(self.size);
            return None;
        } else if position.x >= window_bounds.width - resize_area_size && position.y <= resize_area_size {
            // Top-right corner
            self.resizing = true;
            self.resize_direction = ResizeDirection::TopRight;
            self.resize_start = Some(position);
            self.window_start_size = Some(self.size);
            return None;
        } else if position.x <= resize_area_size && position.y >= window_bounds.height - resize_area_size {
            // Bottom-left corner
            self.resizing = true;
            self.resize_direction = ResizeDirection::BottomLeft;
            self.resize_start = Some(position);
            self.window_start_size = Some(self.size);
            return None;
        } else if position.x >= window_bounds.width - resize_area_size && position.y >= window_bounds.height - resize_area_size {
            // Bottom-right corner
            self.resizing = true;
            self.resize_direction = ResizeDirection::BottomRight;
            self.resize_start = Some(position);
            self.window_start_size = Some(self.size);
            return None;
        }

        // If not resizing, start dragging
        self.dragging = true;
        self.drag_start = Some(position);
        
        // Get current window position
        match self.position {
            Position::Specific(x, y) => {
                self.window_start_pos = Some((x, y));
            }
            _ => {
                // If the window is not at a specific position, set it to (0,0) so we can drag from there
                self.window_start_pos = Some((0, 0));
                self.position = Position::Specific(0, 0);
            }
        }
        
        Some(Message::DragStarted(position.x as i32, position.y as i32))
    }

    /// Handle a mouse move event
    pub fn handle_mouse_move(&mut self, position: Point) -> Option<Message> {
        if self.resizing {
            if let (Some(start), Some(start_size)) = (self.resize_start, self.window_start_size) {
                let delta_x = position.x - start.x;
                let delta_y = position.y - start.y;
                
                let mut new_width = start_size.width;
                let mut new_height = start_size.height;
                
                match self.resize_direction {
                    ResizeDirection::TopLeft => {
                        new_width = (start_size.width - delta_x).max(self.min_size.width);
                        new_height = (start_size.height - delta_y).max(self.min_size.height);
                    }
                    ResizeDirection::TopRight => {
                        new_width = (start_size.width + delta_x).max(self.min_size.width);
                        new_height = (start_size.height - delta_y).max(self.min_size.height);
                    }
                    ResizeDirection::BottomLeft => {
                        new_width = (start_size.width - delta_x).max(self.min_size.width);
                        new_height = (start_size.height + delta_y).max(self.min_size.height);
                    }
                    ResizeDirection::BottomRight => {
                        new_width = (start_size.width + delta_x).max(self.min_size.width);
                        new_height = (start_size.height + delta_y).max(self.min_size.height);
                    }
                    ResizeDirection::None => {}
                }
                
                self.size = Size::new(new_width, new_height);
                return Some(Message::Resize(new_width as u32, new_height as u32));
            }
            return None;
        }
        
        if self.dragging {
            if let (Some(start), Some((start_x, start_y))) = (self.drag_start, self.window_start_pos) {
                let delta_x = position.x - start.x;
                let delta_y = position.y - start.y;
                
                let new_x = start_x + delta_x as i32;
                let new_y = start_y + delta_y as i32;
                
                self.position = Position::Specific(new_x, new_y);
                return Some(Message::DragMoved(position.x as i32, position.y as i32));
            }
        }
        
        None
    }

    /// Handle a mouse release event
    pub fn handle_mouse_release(&mut self) -> Option<Message> {
        if self.resizing {
            self.resizing = false;
            self.resize_direction = ResizeDirection::None;
            self.resize_start = None;
            self.window_start_size = None;
            return Some(Message::ResizeEnded);
        }
        
        if self.dragging {
            self.dragging = false;
            self.drag_start = None;
            self.window_start_pos = None;
            return Some(Message::DragEnded);
        }
        
        None
    }

    /// Save the window position and size to configuration
    pub fn save_to_config(&self, config: &mut Config) -> Result<()> {
        match self.position {
            Position::Specific(x, y) => {
                config.window.position_x = Some(x);
                config.window.position_y = Some(y);
            }
            _ => {
                config.window.position_x = None;
                config.window.position_y = None;
            }
        }
        
        config.window.width = self.size.width as u32;
        config.window.height = self.size.height as u32;
        config.window.opacity = self.opacity;
        config.window.always_on_top = self.always_on_top;
        
        save_config(config, None)?;
        
        Ok(())
    }

    /// Create a subscription for window events
    pub fn subscription() -> Subscription<Message> {
        subscription::events_with(|event, _status| {
            match event {
                Event::Window(window::Event::Resized { width, height, .. }) => {
                    Some(Message::Resize(width, height))
                }
                Event::Window(window::Event::Moved { x, y, .. }) => {
                    Some(Message::Moved(x, y))
                }
                Event::Window(window::Event::CloseRequested { .. }) => {
                    Some(Message::Close)
                }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    Some(Message::MouseDown)
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::MouseUp)
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::MouseMoved(position))
                }
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if key_code == keyboard::KeyCode::Escape {
                        Some(Message::Close)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
    }
}

/// Create window settings from configuration
pub fn create_window_settings(config: &Config) -> window::Settings {
    window::Settings {
        size: (config.window.width, config.window.height),
        position: match (config.window.position_x, config.window.position_y) {
            (Some(x), Some(y)) => Position::Specific(x, y),
            _ => Position::Centered,
        },
        min_size: Some((300, 400)),
        max_size: None,
        resizable: true,
        decorations: false, // No window decorations for floating effect
        transparent: true,  // Transparent background
        // Note: always_on_top is not directly supported in iced::window::Settings
        // We'll need to implement this differently later
        icon: None,
        ..Default::default()
    }
}

/// Build the title bar for the application window
pub fn title_bar<'a>(window: &Window) -> iced::Element<'a, crate::app::Message> {
    use iced::widget::{button, container, row, text};
    use iced::{Alignment, Length};
    
    let title = text(window.title())
        .size(20);
    
    let close_button = button(text("×").size(20))
        .on_press(crate::app::Message::Close)
        .padding(5);
    
    let row_content = row![
        title,
        iced::widget::Space::with_width(Length::Fill),
        close_button
    ]
    .spacing(10)
    .align_items(Alignment::Center);
    
    // Use a button as the container to make it interactive for dragging
    iced::widget::button(
        container(row_content)
            .padding(10)
            .width(Length::Fill)
    )
    .style(iced::theme::Button::Text)
    .on_press(crate::app::Message::MouseDown)
    .width(Length::Fill)
    .into()
}

/// Build the chat input area
pub fn chat_input<'a>(message: &str) -> iced::Element<'a, crate::app::Message> {
    use iced::widget::{button, container, row, text, text_input};
    use iced::{Alignment, Length};
    
    let input = text_input("Type a message...", message)
        .on_input(crate::app::Message::InputChanged)
        .padding(10);
    
    let send_button = button(text("Send"))
        .on_press(crate::app::Message::SendMessage)
        .padding(10);
    
    container(
        row![
            input,
            send_button
        ]
        .spacing(10)
        .align_items(Alignment::Center)
    )
    .padding(10)
    .width(Length::Fill)
    .into()
}
