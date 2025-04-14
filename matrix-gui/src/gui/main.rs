use iced::{window, Application, Settings};
use matrix_gui::MatrixApp;

pub fn main() -> iced::Result {
    // Initialize logger
    env_logger::init();
    
    // Set up and run the application
    MatrixApp::run(Settings {
        window: window::Settings {
            size: (1024, 768),
            min_size: Some((400, 300)),
            position: window::Position::Centered,
            decorations: true,  // Window decorations (title bar, etc)
            transparent: false,
            resizable: true,
            title: String::from("Matrix Terminal"),
            ..window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}