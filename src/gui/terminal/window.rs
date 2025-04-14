use iced::{Command, Element, Rectangle, Size};
use uuid::Uuid;
use std::io::Write;
use anyhow::Result;
use portable_pty::{native_pty_system, PtySize, PtySystem, CommandBuilder, Child};
use std::sync::{Arc, Mutex};
use alacritty_terminal::{
    term::{Term, TermMode},
    event::{EventListener, Event as TermEvent},
    grid::Dimensions,
    index::{Line, Column, Point},
    vte::ansi,
};

use crate::styles::colors;
use crate::styles::theme::TerminalContainerStyle;

/// Messages that can be sent to terminal windows
#[derive(Debug, Clone)]
pub enum TerminalMessage {
    Input(Vec<u8>),
    Resize(Size),
    Output(Vec<u8>),
    ProcessExit(i32),
    Focused,
    Unfocused,
}

/// A terminal window component
pub struct TerminalWindow {
    // Unique identifier
    id: Uuid,
    
    // Title and visual state
    title: String,
    size: Size,
    is_focused: bool,
    
    // Terminal emulation
    term: Term<EventListener>,
    
    // PTY handling
    pty_master: Option<Box<dyn portable_pty::MasterPty + Send>>,
    pty_writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
    child_process: Option<Box<dyn Child + Send + Sync>>,
}

impl TerminalWindow {
    /// Create a new terminal window
    pub fn new(id: Uuid, title: &str) -> Self {
        // Create an event listener for the terminal
        let event_listener = EventListener::new();
        
        // Create a terminal with default config
        let term = Term::new(
            alacritty_terminal::term::Config::default(),
            &alacritty_terminal::grid::Dimensions::new(80, 24),
            event_listener
        );
        
        // Set up modes (e.g., enable application cursor keys)
        term.set_mode(TermMode::APP_CURSOR);
        
        Self {
            id,
            title: title.to_string(),
            size: Size::new(800.0, 600.0),
            is_focused: false,
            term,
            pty_master: None,
            pty_writer: None,
            child_process: None,
        }
    }
    
    /// Spawn a shell in the terminal
    pub fn spawn_shell(&mut self) -> Command<TerminalMessage> {
        // Get native PTY system
        let pty_system = native_pty_system();
        
        // Create a PTY pair with initial size
        let columns = 80u16;
        let rows = 24u16;
        
        match pty_system.openpty(PtySize {
            rows,
            cols: columns,
            pixel_width: 0,
            pixel_height: 0,
        }) {
            Ok(pair) => {
                self.pty_master = Some(pair.master);
                
                // Create a command builder for the shell
                // We'll use the default shell from the environment
                let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
                let mut cmd = CommandBuilder::new(shell);
                
                // Set environment variables
                cmd.env("TERM", "xterm-256color");
                if let Ok(term) = std::env::var("TERM") {
                    cmd.env("TERM", term);
                }
                if let Ok(path) = std::env::var("PATH") {
                    cmd.env("PATH", path);
                }
                if let Ok(home) = std::env::var("HOME") {
                    cmd.env("HOME", home);
                }
                
                // Spawn the process
                match pair.slave.spawn_command(cmd) {
                    Ok(child) => {
                        self.child_process = Some(child);
                        
                        // Get a writer for the PTY
                        if let Some(pty_master) = &mut self.pty_master {
                            match pty_master.take_writer() {
                                Ok(writer) => {
                                    self.pty_writer = Some(Arc::new(Mutex::new(writer)));
                                    
                                    // Now set up reading from the pty
                                    let pty_reader = pty_master.try_clone_reader().unwrap();
                                    
                                    // Return a command to start reading output
                                    return Command::perform(
                                        Self::read_pty_output(pty_reader),
                                        TerminalMessage::Output
                                    );
                                },
                                Err(e) => {
                                    log::error!("Failed to get PTY writer: {}", e);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to spawn command: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("Failed to open PTY: {}", e);
            }
        }
        
        Command::none()
    }
    
    // Read output from the PTY asynchronously
    async fn read_pty_output(mut reader: Box<dyn std::io::Read + Send>) -> Vec<u8> {
        let mut buffer = [0u8; 4096];
        let mut output = Vec::new();
        
        // This is a simplified version for the prototype
        // A real implementation would handle partial reads, errors, etc.
        if let Ok(bytes_read) = reader.read(&mut buffer) {
            if bytes_read > 0 {
                output.extend_from_slice(&buffer[..bytes_read]);
            }
        }
        
        output
    }
    
    /// Send input to the terminal
    pub fn send_input(&mut self, data: &[u8]) -> Result<()> {
        if let Some(writer) = &self.pty_writer {
            let mut writer = writer.lock().unwrap();
            writer.write_all(data)?;
            writer.flush()?;
        }
        Ok(())
    }
    
    /// Handle terminal update
    pub fn update(&mut self, message: TerminalMessage) -> Command<TerminalMessage> {
        match message {
            TerminalMessage::Input(data) => {
                if let Err(e) = self.send_input(&data) {
                    log::error!("Failed to send input: {}", e);
                }
                Command::none()
            },
            
            TerminalMessage::Output(data) => {
                // Process the received data in the terminal
                self.term.take_child().unwrap().advance_bytes(&data);
                
                // Continue reading from the PTY
                if let Some(pty_master) = &mut self.pty_master {
                    if let Ok(reader) = pty_master.try_clone_reader() {
                        return Command::perform(
                            Self::read_pty_output(reader),
                            TerminalMessage::Output
                        );
                    }
                }
                Command::none()
            },
            
            TerminalMessage::Resize(size) => {
                self.size = size;
                
                // Calculate terminal dimensions based on size
                // This is simplified and would need to be based on actual font metrics
                let cols = (size.width / 8.0) as u16;
                let rows = (size.height / 16.0) as u16;
                
                // Resize the terminal
                self.term.resize(Dimensions::new(cols, rows));
                
                // Resize the PTY
                if let Some(pty_master) = &mut self.pty_master {
                    let _ = pty_master.resize(PtySize {
                        rows,
                        cols,
                        pixel_width: 0,
                        pixel_height: 0,
                    });
                }
                
                Command::none()
            },
            
            TerminalMessage::ProcessExit(_status) => {
                // Handle process exit
                // For now, we'll just log it
                log::info!("Process exited");
                Command::none()
            },
            
            TerminalMessage::Focused => {
                self.is_focused = true;
                Command::none()
            },
            
            TerminalMessage::Unfocused => {
                self.is_focused = false;
                Command::none()
            },
        }
    }
    
    /// Update on tick (called periodically)
    pub fn update_on_tick(&mut self) -> Option<Command<TerminalMessage>> {
        // Check if there are events to process
        None
    }
    
    /// Close the terminal
    pub fn close(&mut self) -> Command<TerminalMessage> {
        // Kill the child process if it exists
        if let Some(child) = &mut self.child_process {
            let _ = child.kill();
        }
        
        // Clean up resources
        self.pty_master = None;
        self.pty_writer = None;
        self.child_process = None;
        
        Command::none()
    }
    
    /// Get the terminal title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Get the terminal ID
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    /// Render the terminal
    pub fn view(&self) -> Element<TerminalMessage> {
        // This is just a placeholder for now
        // A real implementation would render the terminal content
        iced::widget::container(
            iced::widget::text(&self.title)
                .size(14)
                .style(iced::theme::Text::Color(colors::MATRIX_GREEN))
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(
            TerminalContainerStyle { focused: self.is_focused }
        )))
        .into()
    }
}