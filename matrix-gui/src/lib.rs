pub mod components;
pub mod terminal;
pub mod layout;
pub mod styles;
pub mod utils;

use iced::{
    executor, keyboard, mouse, Application, Color, Command, Element, Event, Length, 
    Renderer, Subscription, Theme
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::components::sidebar::{Sidebar, SidebarMessage};
use crate::terminal::window::{TerminalWindow, TerminalMessage};
use crate::layout::manager::{LayoutManager, LayoutNode, SplitDirection};
use crate::styles::theme::matrix_theme;

/// Main application state
pub struct MatrixApp {
    // Window management
    windows: HashMap<Uuid, TerminalWindow>,
    layout_manager: LayoutManager,
    
    // UI components
    sidebar: Sidebar,
    
    // Application state
    focused_window: Option<Uuid>,
    is_zoomed: bool,
    
    // Theming
    theme: Theme,
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    // Event handling
    Event(Event),
    
    // Terminal messages
    Terminal(Uuid, TerminalMessage),
    
    // Window management
    CreateWindow,
    CloseWindow(Uuid),
    FocusWindow(Uuid),
    
    // Layout messages
    SplitWindow(Uuid, SplitDirection),
    ZoomToggle(Uuid),
    
    // UI component messages
    Sidebar(SidebarMessage),
    
    // System messages
    Tick,
}

impl Application for MatrixApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // Create a default layout manager
        let layout_manager = LayoutManager::new();
        
        // Create a sidebar
        let sidebar = Sidebar::new();
        
        // Create the initial application state
        let app = MatrixApp {
            windows: HashMap::new(),
            layout_manager,
            sidebar,
            focused_window: None,
            is_zoomed: false,
            theme: matrix_theme(),
        };
        
        // Command to create an initial window
        let command = Command::perform(
            async { },
            |_| Message::CreateWindow
        );
        
        (app, command)
    }
    
    fn title(&self) -> String {
        let title = "Matrix Terminal";
        
        if let Some(id) = self.focused_window {
            if let Some(window) = self.windows.get(&id) {
                return format!("{} - {}", title, window.title());
            }
        }
        
        title.to_string()
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Event(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                        // Handle keyboard shortcuts
                        // TODO: Implement keyboard shortcuts
                        Command::none()
                    },
                    Event::Mouse(mouse::Event::CursorMoved { position, .. }) => {
                        // Handle mouse movement
                        self.sidebar.handle_hover(position);
                        Command::none()
                    },
                    _ => Command::none(),
                }
            },
            
            Message::CreateWindow => {
                let window_id = Uuid::new_v4();
                let mut terminal_window = TerminalWindow::new(window_id, "New Terminal");
                
                // Start the terminal process
                let command = terminal_window.spawn_shell();
                
                // Add window to our collection
                self.windows.insert(window_id, terminal_window);
                
                // Focus the new window
                self.focused_window = Some(window_id);
                
                // Update the layout
                self.layout_manager.add_window(window_id);
                
                command.map(move |msg| Message::Terminal(window_id, msg))
            },
            
            Message::Terminal(id, terminal_message) => {
                if let Some(terminal) = self.windows.get_mut(&id) {
                    terminal.update(terminal_message)
                        .map(move |msg| Message::Terminal(id, msg))
                } else {
                    Command::none()
                }
            },
            
            Message::CloseWindow(id) => {
                if let Some(mut terminal) = self.windows.remove(&id) {
                    // Restructure the layout
                    self.layout_manager.remove_window(&id);
                    
                    // Update focus if needed
                    if self.focused_window == Some(id) {
                        self.focused_window = self.windows.keys().next().cloned();
                    }
                    
                    // Close the terminal
                    terminal.close()
                } else {
                    Command::none()
                }
            },
            
            Message::FocusWindow(id) => {
                if self.windows.contains_key(&id) {
                    self.focused_window = Some(id);
                }
                Command::none()
            },
            
            Message::SplitWindow(id, direction) => {
                if self.windows.contains_key(&id) {
                    // Create a new window
                    let new_id = Uuid::new_v4();
                    let mut terminal_window = TerminalWindow::new(new_id, "Split Terminal");
                    
                    // Start the terminal process
                    let command = terminal_window.spawn_shell();
                    
                    // Add window to our collection
                    self.windows.insert(new_id, terminal_window);
                    
                    // Update the layout
                    self.layout_manager.split_window(&id, direction, new_id, 0.5);
                    
                    // Focus the new window
                    self.focused_window = Some(new_id);
                    
                    command.map(move |msg| Message::Terminal(new_id, msg))
                } else {
                    Command::none()
                }
            },
            
            Message::ZoomToggle(id) => {
                if self.windows.contains_key(&id) {
                    if self.is_zoomed {
                        // Unzoom
                        self.is_zoomed = false;
                        self.layout_manager.unzoom();
                    } else {
                        // Zoom
                        self.is_zoomed = true;
                        self.layout_manager.zoom_window(&id);
                        self.focused_window = Some(id);
                    }
                }
                Command::none()
            },
            
            Message::Sidebar(sidebar_message) => {
                match sidebar_message {
                    SidebarMessage::IconClicked(icon) => {
                        // Handle sidebar icon clicks
                        // TODO: Implement sidebar icon handling
                        Command::none()
                    }
                }
            },
            
            Message::Tick => {
                // Update terminal windows
                let mut commands = Vec::new();
                
                for (&id, window) in &mut self.windows {
                    if let Some(cmd) = window.update_on_tick() {
                        commands.push(cmd.map(move |msg| Message::Terminal(id, msg)));
                    }
                }
                
                Command::batch(commands)
            }
        }
    }
    
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            // Listen for system events (keyboard, mouse, etc.)
            iced::subscription::events().map(Message::Event),
            
            // Add a tick subscription for terminal updates
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Tick),
        ])
    }
    
    fn view(&self) -> Element<Message> {
        // TODO: Implement the view function
        // This will render the sidebar and terminal windows according to layout
        iced::widget::container(
            iced::widget::text("Matrix Terminal")
                .size(24)
                .style(iced::theme::Text::Color(Color::from_rgb(0.0, 1.0, 0.25)))
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(|_theme| {
            iced::widget::container::Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::BLACK)),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }
        })))
        .into()
    }
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}