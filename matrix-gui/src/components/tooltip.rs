use iced::{Element, Rectangle, Color};
use iced::widget::{Container, Text};

use crate::styles::colors;

/// A tooltip component for the Matrix Terminal
pub struct Tooltip {
    title: String,
    description: String,
    shortcut: String,
}

impl Tooltip {
    /// Create a new tooltip
    pub fn new(title: &str, description: &str, shortcut: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            shortcut: shortcut.to_string(),
        }
    }
    
    /// Render the tooltip
    pub fn view<'a, Message>(&self) -> Element<'a, Message> {
        // Create the tooltip content
        let title = Text::new(&self.title)
            .size(14)
            .style(iced::theme::Text::Color(colors::MATRIX_GREEN));
            
        let description = Text::new(&self.description)
            .size(12)
            .style(iced::theme::Text::Color(colors::MATRIX_GREEN));
            
        let shortcut = Text::new(format!("Shortcut: {}", self.shortcut))
            .size(12)
            .style(iced::theme::Text::Color(colors::MATRIX_GREEN));
            
        // Arrange the content vertically
        let content = iced::widget::column![
            title,
            iced::widget::vertical_space(5),
            description,
            iced::widget::vertical_space(5),
            shortcut,
        ]
        .spacing(2)
        .padding(10);
        
        // Create the tooltip container with a Matrix-style border
        Container::new(content)
            .style(iced::theme::Container::Custom(Box::new(|_theme| {
                iced::widget::container::Appearance {
                    text_color: None,
                    background: Some(iced::Background::Color(Color::BLACK)),
                    border_radius: 0.0, // Square borders, not rounded
                    border_width: 1.0,
                    border_color: colors::MATRIX_GREEN,
                }
            })))
            .into()
    }
}