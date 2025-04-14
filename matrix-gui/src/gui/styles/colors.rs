use iced::Color;

// Core Matrix theme colors
pub const MATRIX_GREEN: Color = Color::from_rgb(
    0.0,   // R
    1.0,   // G
    0.255  // B
);

pub const DARK_GREEN: Color = Color::from_rgb(
    0.0,   // R
    0.7,   // G
    0.176  // B
);

pub const DARKER_GREEN: Color = Color::from_rgb(
    0.0,   // R
    0.4,   // G
    0.1    // B
);

// Background colors
pub const BACKGROUND: Color = Color::BLACK;
pub const BACKGROUND_LIGHT: Color = Color::from_rgb(0.1, 0.1, 0.1);

// Border colors
pub const BORDER: Color = DARK_GREEN;
pub const BORDER_FOCUSED: Color = MATRIX_GREEN;