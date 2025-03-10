use iced::{
    keyboard,
    widget::{Button, container, Container, Row, TextInput},
    Alignment, Color, Element, Event, Length, Padding, Subscription, Theme,
};

use crate::app::Message;

/// Maximum height for the input field in pixels
const MAX_INPUT_HEIGHT: u16 = 150;

/// Create an input area for the chat
pub fn input_area<'a>(
    message: &str,
    is_sending: bool,
    _theme: &Theme,
) -> Element<'a, Message> {
    // Calculate the number of lines in the input
    let line_count = message.lines().count().max(1);
    
    // Calculate the height based on line count, with a maximum
    let line_height = 24; // Approximate height per line in pixels
    let _height = (line_count as u16 * line_height).min(MAX_INPUT_HEIGHT);
    
    // Create the text input
    let input = TextInput::new("Type a message...", message)
        .on_input(Message::InputChanged)
        .padding(Padding::new(12.0))
        .size(16);
    
    // Create the send button
    let send_button_text = if is_sending { "Sending..." } else { "Send" };
    let send_button = Button::new(send_button_text)
        .padding(Padding::new(12.0))
        .style(iced::theme::Button::Primary);
    
    // Only enable the button if there's text and we're not already sending
    let send_button = if !message.trim().is_empty() && !is_sending {
        send_button.on_press(Message::SendMessage)
    } else {
        send_button
    };
    
    // Create the row with input and button
    let input_row = Row::new()
        .spacing(8)
        .align_items(Alignment::Center)
        .push(input.width(Length::Fill))
        .push(send_button);
    
    // Create the container for the input area
    Container::new(input_row)
        .width(Length::Fill)
        .padding(Padding::new(12.0))
        .style(|_theme: &Theme| {
            container::Appearance {
                background: Some(Color::from_rgb(1.0, 1.0, 1.0).into()),
                border_width: 1.0,
                border_color: Color::from_rgb(0.8, 0.8, 0.8),
                ..Default::default()
            }
        })
        .into()
}

/// Create a subscription for keyboard events
pub fn keyboard_subscription() -> Subscription<Message> {
    iced::subscription::events_with(|event, _status| {
        if let Event::Keyboard(keyboard::Event::KeyPressed { 
            key_code, 
            modifiers, 
            ..
        }) = event {
            // Handle Enter key for submission (without shift)
            if key_code == keyboard::KeyCode::Enter && !modifiers.shift() {
                return Some(Message::SendMessage);
            }
            
            // Handle Shift+Enter for new line
            if key_code == keyboard::KeyCode::Enter && modifiers.shift() {
                return Some(Message::NewLine);
            }
        }
        
        None
    })
}
