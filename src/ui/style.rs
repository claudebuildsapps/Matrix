use ratatui::style::{Color, Style, Modifier};

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selected: Color,
    pub border: Color,
    pub title: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            cursor: Color::Cyan,
            selected: Color::LightCyan,
            border: Color::Gray,
            title: Color::Green,
        }
    }
}

pub fn default_title_style() -> Style {
    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
}

pub fn default_border_style() -> Style {
    Style::default().fg(Color::Gray)
}
