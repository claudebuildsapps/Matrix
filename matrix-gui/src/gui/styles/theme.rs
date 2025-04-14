use iced::{Theme, Application};
use crate::styles::colors;

/// Creates a Matrix-themed iced theme
pub fn matrix_theme() -> Theme {
    let mut theme = Theme::Dark;
    
    // Set custom colors for the theme
    // This customizes various widgets like buttons, text, etc.
    // The exact API depends on the version of iced being used
    
    // For iced 0.10+, we would customize the palette
    if let Theme::Dark = &mut theme {
        // Some customization would go here
        // This is a simplified version and would need to be expanded
        // based on the actual version of iced being used
    }
    
    theme
}

/// Custom container style for terminal windows with non-rounded borders
pub struct TerminalContainerStyle {
    pub focused: bool,
}

impl iced::widget::container::StyleSheet for TerminalContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &Self::Style) -> iced::widget::container::Appearance {
        let border_color = if self.focused {
            colors::BORDER_FOCUSED
        } else {
            colors::BORDER
        };

        iced::widget::container::Appearance {
            text_color: None,
            background: Some(iced::Background::Color(colors::BACKGROUND)),
            border_radius: 0.0, // Square borders, not rounded
            border_width: 1.0,
            border_color,
        }
    }
}