pub mod window;
pub mod presentation;
pub mod input;

use iced::{
    widget::{button, row, text, text_input},
    Element, Length,
};

use crate::app::Message;

/// Build the title bar for the application window
pub fn title_bar<'a>() -> Element<'a, Message> {
    row![
        text("ScreenSage").size(20),
        button("×").on_press(Message::Close)
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fill)
    .into()
}

/// Build the chat input area (legacy version, use input::input_area instead)
pub fn chat_input<'a>(message: &str) -> Element<'a, Message> {
    row![
        text_input("Type a message...", message)
            .on_input(Message::InputChanged)
            .padding(10),
        button("Send").on_press(Message::SendMessage)
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fill)
    .into()
}
