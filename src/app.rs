use crate::terminal::terminal::Terminal;
use crate::terminal::events::{EventHandler, AppEvent};
use crate::config::settings::Settings;
use crate::ui::{style, widgets, window_manager::{WindowManager, SplitDirection, Direction}, sidebar::{Sidebar, SidebarIcon}};
use anyhow::Result;
use std::time::Duration;
use ratatui::prelude::*;
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent, MouseEvent, MouseEventKind, MouseButton};
use std::collections::HashMap;
use uuid::Uuid;

// Application state
pub enum AppState {
    Normal,
    Command,
}

pub struct App {
    // Terminal backend
    terminal: Terminal,
    // Application settings
    settings: Settings,
    // Is the application running
    running: bool,
    // Event handler for input
    events: EventHandler,
    // Window manager
    window_manager: WindowManager,
    // Application state
    state: AppState,
    // Command input buffer
    command_buffer: String,
    // Command history
    command_history: Vec<String>,
    // Sidebar for icon-based controls
    sidebar: Sidebar,
}

impl App {
    pub fn new() -> Result<Self> {
        let settings = Settings::load()?;
        let terminal = Terminal::new()?;
        let tick_rate = Duration::from_millis(settings.general.tick_rate_ms);
        let events = EventHandler::new(tick_rate);
        
        // Create the window manager with an initial size
        let window_manager = WindowManager::new(Rect::new(0, 0, 80, 24));
        
        // Create the sidebar
        let sidebar = Sidebar::new();
        
        // Create the app
        let mut app = Self {
            terminal,
            settings,
            running: true,
            events,
            window_manager,
            state: AppState::Normal,
            command_buffer: String::new(),
            command_history: Vec::new(),
            sidebar,
        };
        
        // Create an initial window
        app.create_window("Matrix Terminal")?;
        
        Ok(app)
    }
    
    // Handle keyboard shortcuts
    fn handle_shortcut(&mut self, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
        match (key_code, modifiers) {
            // Create a new window (Ctrl+N)
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                if let Err(e) = self.create_window("New Terminal") {
                    eprintln!("Error creating window: {}", e);
                }
                true
            },
            
            // Split window horizontally (Ctrl+H)
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                if let Err(e) = self.split_window(SplitDirection::Horizontal) {
                    eprintln!("Error splitting window: {}", e);
                }
                true
            },
            
            // Split window vertically (Ctrl+V)
            (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                if let Err(e) = self.split_window(SplitDirection::Vertical) {
                    eprintln!("Error splitting window: {}", e);
                }
                true
            },
            
            // Switch to next window (Ctrl+Tab)
            (KeyCode::Tab, KeyModifiers::CONTROL) => {
                if let Err(e) = self.focus_next_window() {
                    eprintln!("Error focusing next window: {}", e);
                }
                true
            },
            
            // Switch to previous window (Ctrl+Shift+Tab)
            (KeyCode::BackTab, KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                if let Err(e) = self.window_manager.focus_prev_window() {
                    eprintln!("Error focusing previous window: {}", e);
                }
                true
            },
            
            // Navigate up (Ctrl+Up)
            (KeyCode::Up, KeyModifiers::CONTROL) => {
                if let Err(e) = self.window_manager.focus_direction(Direction::Up) {
                    eprintln!("Error focusing window: {}", e);
                }
                true
            },
            
            // Navigate down (Ctrl+Down)
            (KeyCode::Down, KeyModifiers::CONTROL) => {
                if let Err(e) = self.window_manager.focus_direction(Direction::Down) {
                    eprintln!("Error focusing window: {}", e);
                }
                true
            },
            
            // Navigate left (Ctrl+Left)
            (KeyCode::Left, KeyModifiers::CONTROL) => {
                if let Err(e) = self.window_manager.focus_direction(Direction::Left) {
                    eprintln!("Error focusing window: {}", e);
                }
                true
            },
            
            // Navigate right (Ctrl+Right)
            (KeyCode::Right, KeyModifiers::CONTROL) => {
                if let Err(e) = self.window_manager.focus_direction(Direction::Right) {
                    eprintln!("Error focusing window: {}", e);
                }
                true
            },
            
            // Zoom toggle (Ctrl+Z)
            (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
                if let Some(id) = self.window_manager.focused_window().map(|w| w.id()) {
                    if let Err(e) = self.window_manager.zoom_window(Some(id)) {
                        eprintln!("Error zooming window: {}", e);
                    }
                }
                true
            },
            
            // Grid layout (Ctrl+G)
            (KeyCode::Char('g'), KeyModifiers::CONTROL) => {
                // Get all window IDs
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_grid_layout(&window_ids) {
                        eprintln!("Error applying grid layout: {}", e);
                    }
                }
                true
            },
            
            // Horizontal layout (Ctrl+Shift+H)
            (KeyCode::Char('H'), KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                // Get all window IDs
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_horizontal_layout(&window_ids) {
                        eprintln!("Error applying horizontal layout: {}", e);
                    }
                }
                true
            },
            
            // Vertical layout (Ctrl+Shift+V)
            (KeyCode::Char('V'), KeyModifiers::CONTROL | KeyModifiers::SHIFT) => {
                // Get all window IDs
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_vertical_layout(&window_ids) {
                        eprintln!("Error applying vertical layout: {}", e);
                    }
                }
                true
            },
            
            // Main and stack layout (Ctrl+M)
            (KeyCode::Char('m'), KeyModifiers::CONTROL) => {
                // Get the current window as main, and the rest as stack
                if let Some(main_id) = self.window_manager.focused_window().map(|w| w.id()) {
                    let stack_ids: Vec<Uuid> = self.window_manager.windows().keys()
                        .filter(|&&id| id != main_id)
                        .cloned()
                        .collect();
                    
                    if let Err(e) = self.window_manager.apply_main_and_stack_layout(main_id, &stack_ids) {
                        eprintln!("Error applying main and stack layout: {}", e);
                    }
                }
                true
            },
            
            // Close current window (Ctrl+W)
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                if let Err(e) = self.close_current_window() {
                    eprintln!("Error closing window: {}", e);
                }
                true
            },
            
            // Toggle sidebar (Ctrl+B)
            (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
                self.sidebar.toggle();
                true
            },
            
            // No shortcut found
            _ => false,
        }
    }
    
    // Create a new window
    fn create_window(&mut self, title: &str) -> Result<()> {
        // Create the window
        let window_id = self.window_manager.create_window(title)?;
        
        // Start a shell in the window
        if let Some(window) = self.window_manager.windows_mut().get_mut(&window_id) {
            // Use the default shell from settings
            let shell = &self.settings.general.default_shell;
            window.spawn_process(shell, None)?;
        }
        
        Ok(())
    }
    
    // Split the current window
    fn split_window(&mut self, direction: SplitDirection) -> Result<()> {
        if let Some(window_id) = self.window_manager.focused_window().map(|w| w.id()) {
            let new_id = self.window_manager.split_window(window_id, direction, 0.5)?;
            
            // Start a shell in the new window
            if let Some(window) = self.window_manager.windows_mut().get_mut(&new_id) {
                // Use the default shell from settings
                let shell = &self.settings.general.default_shell;
                window.spawn_process(shell, None)?;
            }
            
            // Focus the new window
            self.window_manager.focus_window(new_id)?;
        }
        
        Ok(())
    }
    
    // Focus the next window
    fn focus_next_window(&mut self) -> Result<()> {
        let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
        
        if window_ids.is_empty() {
            return Ok(());
        }
        
        let current_id = self.window_manager.focused_window().map(|w| w.id());
        
        if let Some(current_id) = current_id {
            // Find the index of the current window
            if let Some(index) = window_ids.iter().position(|id| *id == current_id) {
                // Get the next window index
                let next_index = (index + 1) % window_ids.len();
                let next_id = window_ids[next_index];
                
                // Focus the next window
                self.window_manager.focus_window(next_id)?;
            }
        }
        
        Ok(())
    }
    
    // Close the current window
    fn close_current_window(&mut self) -> Result<()> {
        if let Some(window_id) = self.window_manager.focused_window().map(|w| w.id()) {
            self.window_manager.close_window(window_id)?;
        }
        
        Ok(())
    }
    
    pub fn run(&mut self) -> Result<()> {
        // Main application loop
        while self.running {
            // Update window states
            for window in self.window_manager.windows_mut().values_mut() {
                window.update()?;
            }
            
            // Draw UI
            self.terminal.draw(|f| {
                // Get terminal size
                let size = f.size();
                
                // Create a layout with sidebar and main area
                let sidebar_width = self.sidebar.width();
                
                // If sidebar is active, reserve space for it
                let main_area = if self.sidebar.is_active() {
                    Rect::new(
                        sidebar_width, // X position after sidebar
                        size.y,
                        size.width.saturating_sub(sidebar_width), // Width minus sidebar
                        size.height
                    )
                } else {
                    size
                };
                
                // Resize the window manager to fit the main area
                let _ = self.window_manager.resize(main_area);
                
                // Render the windows
                for window in self.window_manager.windows().values() {
                    let paragraph = window.render();
                    f.render_widget(paragraph, window.size());
                }
                
                // Render the sidebar if active
                if self.sidebar.is_active() {
                    self.sidebar.render(f, size);
                }
                
                // Render the command line if in command mode
                if let AppState::Command = self.state {
                    // Create a command line at the bottom
                    let command_height = 1;
                    let command_rect = Rect::new(
                        0,
                        size.height.saturating_sub(command_height),
                        size.width,
                        command_height,
                    );
                    
                    let command_text = format!(": {}", self.command_buffer);
                    let command_paragraph = widgets::create_paragraph(&command_text, Style::default().fg(Color::Yellow));
                    f.render_widget(command_paragraph, command_rect);
                }
            })?;
            
            // Handle events
            match self.events.next()? {
                AppEvent::Key(key) => self.handle_key_event(key),
                AppEvent::Mouse(event) => self.handle_mouse_event(event),
                AppEvent::Tick => self.update_on_tick(),
                AppEvent::Quit => self.running = false,
            }
        }
        Ok(())
    }
    
    fn handle_key_event(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Normal => {
                // Check for keyboard shortcuts
                if self.handle_shortcut(key.code, key.modifiers) {
                    // Shortcut was handled
                    return;
                } else if key.code == KeyCode::Char(':') {
                    // Enter command mode
                    self.state = AppState::Command;
                    self.command_buffer.clear();
                } else {
                    // Pass the key to the focused window
                    if let Some(window) = self.window_manager.focused_window_mut() {
                        // Convert the key to bytes
                        let mut bytes = Vec::new();
                        
                        match key.code {
                            KeyCode::Char(c) => {
                                bytes.push(c as u8);
                            }
                            KeyCode::Enter => {
                                bytes.push(b'\n');
                            }
                            KeyCode::Tab => {
                                bytes.push(b'\t');
                            }
                            KeyCode::Backspace => {
                                bytes.push(8); // ASCII backspace
                            }
                            KeyCode::Esc => {
                                bytes.push(27); // ASCII escape
                            }
                            // Add other key conversions as needed
                            _ => {}
                        }
                        
                        // Send the input to the process
                        if !bytes.is_empty() {
                            if let Err(e) = window.send_input(&bytes) {
                                // Handle error
                                eprintln!("Error sending input: {}", e);
                            }
                        }
                    }
                }
            }
            AppState::Command => {
                match key.code {
                    KeyCode::Char(c) => {
                        // Add the character to the command buffer
                        self.command_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        // Remove the last character
                        self.command_buffer.pop();
                    }
                    KeyCode::Enter => {
                        // Execute the command
                        self.execute_command();
                        
                        // Return to normal mode
                        self.state = AppState::Normal;
                    }
                    KeyCode::Esc => {
                        // Cancel command mode
                        self.state = AppState::Normal;
                    }
                    _ => {}
                }
            }
        }
    }
    
    // Handle mouse events
    fn handle_mouse_event(&mut self, event: MouseEvent) {
        // Only process mouse moves and clicks
        match event.kind {
            MouseEventKind::Moved => {
                // If the mouse is in the sidebar area, determine which icon is being hovered
                if self.sidebar.is_active() && event.column < self.sidebar.width() {
                    let hovered_icon = self.sidebar.icon_at_position(event.row);
                    self.sidebar.set_hover(hovered_icon);
                } else {
                    // Clear hover state when mouse leaves sidebar
                    self.sidebar.set_hover(None);
                }
            },
            MouseEventKind::Down(MouseButton::Left) => {
                // Handle clicks on the sidebar
                if self.sidebar.is_active() && event.column < self.sidebar.width() {
                    if let Some(icon) = self.sidebar.icon_at_position(event.row) {
                        self.handle_sidebar_click(icon);
                    }
                }
            },
            _ => {}
        }
    }
    
    // Handle clicks on sidebar icons
    fn handle_sidebar_click(&mut self, icon: SidebarIcon) {
        match icon {
            SidebarIcon::NewWindow => {
                if let Err(e) = self.create_window("New Terminal") {
                    eprintln!("Error creating window: {}", e);
                }
            },
            SidebarIcon::SplitHorizontal => {
                if let Err(e) = self.split_window(SplitDirection::Horizontal) {
                    eprintln!("Error splitting window: {}", e);
                }
            },
            SidebarIcon::SplitVertical => {
                if let Err(e) = self.split_window(SplitDirection::Vertical) {
                    eprintln!("Error splitting window: {}", e);
                }
            },
            SidebarIcon::GridLayout => {
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_grid_layout(&window_ids) {
                        eprintln!("Error applying grid layout: {}", e);
                    }
                }
            },
            SidebarIcon::HorizontalLayout => {
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_horizontal_layout(&window_ids) {
                        eprintln!("Error applying horizontal layout: {}", e);
                    }
                }
            },
            SidebarIcon::VerticalLayout => {
                let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                if !window_ids.is_empty() {
                    if let Err(e) = self.window_manager.apply_vertical_layout(&window_ids) {
                        eprintln!("Error applying vertical layout: {}", e);
                    }
                }
            },
            SidebarIcon::MainLayout => {
                if let Some(main_id) = self.window_manager.focused_window().map(|w| w.id()) {
                    let stack_ids: Vec<Uuid> = self.window_manager.windows().keys()
                        .filter(|&&id| id != main_id)
                        .cloned()
                        .collect();
                    
                    if let Err(e) = self.window_manager.apply_main_and_stack_layout(main_id, &stack_ids) {
                        eprintln!("Error applying main and stack layout: {}", e);
                    }
                }
            },
            SidebarIcon::Zoom => {
                if let Some(id) = self.window_manager.focused_window().map(|w| w.id()) {
                    if let Err(e) = self.window_manager.zoom_window(Some(id)) {
                        eprintln!("Error zooming window: {}", e);
                    }
                }
            },
            SidebarIcon::CloseWindow => {
                if let Err(e) = self.close_current_window() {
                    eprintln!("Error closing window: {}", e);
                }
            },
            SidebarIcon::Help => {
                self.display_help();
            },
        }
    }
    
    fn execute_command(&mut self) {
        // Add the command to history
        if !self.command_buffer.is_empty() {
            self.command_history.push(self.command_buffer.clone());
        }
        
        // Clone the command buffer
        let command = self.command_buffer.clone();
        
        // Parse and execute the command
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if let Some(cmd) = parts.first() {
            match *cmd {
                "q" | "quit" => {
                    // Quit the application
                    self.running = false;
                }
                "new" => {
                    // Create a new window
                    let title = if parts.len() > 1 {
                        parts[1]
                    } else {
                        "New Terminal"
                    };
                    
                    if let Err(e) = self.create_window(title) {
                        // Handle error
                        eprintln!("Error creating window: {}", e);
                    }
                }
                "split" => {
                    // Split the current window
                    let direction = if parts.len() > 1 && parts[1] == "h" {
                        SplitDirection::Horizontal
                    } else {
                        SplitDirection::Vertical
                    };
                    
                    if let Err(e) = self.split_window(direction) {
                        // Handle error
                        eprintln!("Error splitting window: {}", e);
                    }
                }
                "layout" => {
                    // Apply a layout
                    if parts.len() > 1 {
                        let layout_type = parts[1];
                        let window_ids: Vec<Uuid> = self.window_manager.windows().keys().cloned().collect();
                        
                        if window_ids.is_empty() {
                            eprintln!("No windows to arrange");
                            return;
                        }
                        
                        let result = match layout_type {
                            "grid" => {
                                self.window_manager.apply_grid_layout(&window_ids)
                            },
                            "horizontal" | "h" => {
                                self.window_manager.apply_horizontal_layout(&window_ids)
                            },
                            "vertical" | "v" => {
                                self.window_manager.apply_vertical_layout(&window_ids)
                            },
                            "main" | "m" => {
                                if let Some(main_id) = self.window_manager.focused_window().map(|w| w.id()) {
                                    let stack_ids: Vec<Uuid> = window_ids.into_iter()
                                        .filter(|&id| id != main_id)
                                        .collect();
                                    self.window_manager.apply_main_and_stack_layout(main_id, &stack_ids)
                                } else {
                                    Ok(()) // No focused window
                                }
                            },
                            _ => {
                                eprintln!("Unknown layout: {}", layout_type);
                                Ok(())
                            }
                        };
                        
                        if let Err(e) = result {
                            eprintln!("Error applying layout: {}", e);
                        }
                    } else {
                        eprintln!("Usage: layout [grid|horizontal|vertical|main]");
                    }
                }
                "zoom" => {
                    // Zoom the current window
                    if let Some(id) = self.window_manager.focused_window().map(|w| w.id()) {
                        if let Err(e) = self.window_manager.zoom_window(Some(id)) {
                            eprintln!("Error zooming window: {}", e);
                        }
                    } else {
                        eprintln!("No window to zoom");
                    }
                }
                "close" => {
                    // Close the current window
                    if let Err(e) = self.close_current_window() {
                        // Handle error
                        eprintln!("Error closing window: {}", e);
                    }
                }
                "help" => {
                    // Display help information
                    self.display_help();
                }
                "sidebar" => {
                    // Toggle sidebar
                    self.sidebar.toggle();
                }
                // Add more commands as needed
                _ => {
                    // Unknown command
                    eprintln!("Unknown command: {}", cmd);
                }
            }
        }
    }
    
    fn update_on_tick(&mut self) {
        // Update state on tick
        // Nothing to do yet
    }

    fn display_help(&mut self) {
        // Create a help window with information about commands and features
        let help_text = "
╔══════════════════════════════════════════════════════════════════════════════╗
║                             Matrix Terminal Help                             ║
╚══════════════════════════════════════════════════════════════════════════════╝

WINDOW MANAGEMENT:
  Ctrl+N        Create a new terminal window
  Ctrl+H        Split the current window horizontally
  Ctrl+V        Split the current window vertically
  Ctrl+W        Close the current window

NAVIGATION:
  Ctrl+Tab            Move to the next window
  Ctrl+Shift+Tab      Move to the previous window
  Ctrl+Up/Down/Left/Right  Navigate between windows in the specified direction
  Ctrl+Z              Toggle zoom on the current window

LAYOUTS:
  Ctrl+G              Apply grid layout to all windows
  Ctrl+Shift+H        Apply horizontal layout to all windows
  Ctrl+Shift+V        Apply vertical layout to all windows
  Ctrl+M              Apply main and stack layout (current window as main)

INTERFACE:
  Ctrl+B              Toggle sidebar

COMMAND MODE:
  :                   Enter command mode
  :new [title]        Create a new window with optional title
  :split [h]          Split window (vertically by default, horizontally with 'h')
  :close              Close the current window
  :layout [type]      Apply layout (grid, horizontal, vertical, main)
  :sidebar            Toggle sidebar
  :help               Show this help information
  :quit               Exit the application
  q                   Exit the application (when in command mode)

TERMINAL:
  Ctrl+C              Send SIGINT to the current process
  Other keys          Passed to the terminal process

FEATURES:
  • Advanced window management with flexible layouts
  • Multiple navigation methods (sequential, directional, etc.)
  • Customizable layout presets (grid, horizontal, vertical, main+stack)
  • Window zooming for focused work
  • Intelligent window closing that preserves layout structure
  • Matrix-style sidebar with hover tooltips for easy access to functionality
  • Command execution in terminals
  • User-friendly command interface

For more information, visit the project repository.
";

        // Create a new window for the help text
        if let Ok(window_id) = self.window_manager.create_window("Help") {
            // Get the window
            if let Some(window) = self.window_manager.windows_mut().get_mut(&window_id) {
                // Write the help text to the buffer
                if let Err(e) = window.buffer.write(help_text.as_bytes()) {
                    eprintln!("Error displaying help: {}", e);
                }
                
                // Focus the help window
                self.window_manager.focus_window(window_id).ok();
            }
        }
    }
}

pub fn run() -> Result<()> {
    // Simple direct initialization with better error handling
    println!("Starting Matrix Terminal...");
    match App::new() {
        Ok(mut app) => {
            println!("Matrix Terminal initialized.");
            return app.run();
        }
        Err(e) => {
            eprintln!("Error initializing Matrix Terminal: {}", e);
            eprintln!("Detailed error: {:?}", e);
            
            // If we're not in a TTY, we might be running from a launcher
            if !atty::is(atty::Stream::Stdout) {
                eprintln!("Not running in interactive terminal. Press Enter to continue...");
                let mut s = String::new();
                let _ = std::io::stdin().read_line(&mut s);
            }
            
            return Err(e);
        }
    }
}