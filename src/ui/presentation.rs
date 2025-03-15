use iced::{
    alignment, widget::{container, scrollable, Scrollable, text, Column, Container, Row},
    Alignment, Color, Element, Length, Padding, Theme
};

use crate::app::Message;
use crate::data::conversation::{Conversation, MessageRole};

/// Message display style
#[derive(Debug, Clone, Copy)]
pub enum MessageStyle {
    /// User message style (right-aligned, white on blue)
    User,
    /// LLM response style (left-aligned, dark on light grey)
    LLM,
    /// Error message style (left-aligned, white on dark red)
    Error,
}

impl MessageStyle {
    /// Get the background color for the message style
    pub fn background_color(&self, _theme: &Theme) -> Color {
        match self {
            MessageStyle::User => Color::from_rgb(0.0, 0.4, 0.8), // Blue
            MessageStyle::LLM => Color::from_rgb(0.9, 0.9, 0.9),  // Light grey
            MessageStyle::Error => Color::from_rgb(0.8, 0.0, 0.0), // Dark red
        }
    }

    /// Get the text color for the message style
    pub fn text_color(&self, _theme: &Theme) -> Color {
        match self {
            MessageStyle::User => Color::WHITE,
            MessageStyle::LLM => Color::from_rgb(0.1, 0.1, 0.1), // Dark grey
            MessageStyle::Error => Color::WHITE,
        }
    }

    /// Get the alignment for the message style
    pub fn alignment(&self) -> Alignment {
        match self {
            MessageStyle::User => Alignment::End,
            MessageStyle::LLM => Alignment::Start,
            MessageStyle::Error => Alignment::Start,
        }
    }
}

/// Create a message bubble with the given content and style
pub fn message_bubble<'a>(
    content: &str,
    style: MessageStyle,
    theme: &Theme,
) -> Element<'a, Message> {
    let max_width = 0.8; // Maximum width as a fraction of the container

    let message_text = text(content)
        .size(16)
        .style(style.text_color(theme));

    let message_container = container(message_text)
        .padding(Padding::new(12.0))
        .style(move |theme: &Theme| {
            container::Appearance {
                background: Some(style.background_color(theme).into()),
                border_radius: 12.0.into(),
                ..Default::default()
            }
        });

    let row = Row::new()
        .width(Length::Fill)
        .align_items(style.alignment())
        .push(
            Container::new(message_container)
                .width(Length::FillPortion((max_width * 10.0) as u16))
                .align_x(match style {
                    MessageStyle::User => alignment::Horizontal::Right,
                    _ => alignment::Horizontal::Left,
                }),
        );

    row.into()
}

/// Create a presentation area for the conversation
pub fn presentation_area<'a>(
    conversation: &Conversation,
    theme: &Theme,
) -> Element<'a, Message> {
    let mut messages_column = Column::new()
        .spacing(12)
        .padding(Padding::new(16.0))
        .width(Length::Fill);

    // Add messages from the conversation
    for message in &conversation.messages {
        let style = match message.role {
            MessageRole::User => MessageStyle::User,
            MessageRole::Assistant => MessageStyle::LLM,
        };
        messages_column = messages_column.push(message_bubble(&message.content, style, theme));
    }

    // Create a scrollable container for the messages with a specific ID
    let scrollable = Scrollable::new(messages_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .id(scrollable::Id::new("conversation_messages"));

    container(scrollable)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme: &Theme| {
            container::Appearance {
                background: Some(Color::from_rgb(0.95, 0.95, 0.95).into()),
                ..Default::default()
            }
        })
        .into()
}

/// Create an error message
pub fn error_message<'a>(error: &str, theme: &Theme) -> Element<'a, Message> {
    message_bubble(error, MessageStyle::Error, theme)
}

/// Create a loading indicator with a message
pub fn loading_indicator<'a>(message: &str, theme: &Theme) -> Element<'a, Message> {
    use iced::widget::{container, row, text, Space};
    use iced::{Alignment, Length};
    
    let loading_text = text(message)
        .size(14)
        .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7)));
    
    let spinner = container(
        text("⟳").size(16).style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7)))
    )
    .width(Length::Shrink)
    .height(Length::Shrink);
    
    row![
        spinner,
        Space::with_width(Length::Fixed(10.0)),
        loading_text
    ]
    .spacing(5)
    .padding(10)
    .align_items(Alignment::Center)
    .width(Length::Fill)
    .into()
}
