use alacritty_terminal::{
    event::{Event as TermEvent, EventListener},
    term::{cell::Cell, Term, TermMode},
    grid::Dimensions,
    index::{Column, Line, Point},
    ansi,
};

use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use anyhow::{Result, Context};
use log::{error, info};

/// Manages a terminal emulation and its connection to a PTY
pub struct TerminalEmulator {
    id: Uuid,
    term: Term<EventListener>,
    pty_master: Option<Box<dyn portable_pty::MasterPty + Send>>,
    pty_writer: Option<Arc<Mutex<Box<dyn Write + Send>>>>,
    child_process: Option<Box<dyn portable_pty::Child + Send + Sync>>,
    dimensions: Dimensions,
    cursor_position: Point,
    title: String,
}

impl TerminalEmulator {
    /// Create a new terminal emulator
    pub fn new(id: Uuid, title: &str) -> Self {
        // Create event listener and terminal
        let event_listener = EventListener::new();
        
        // Set initial dimensions (80x24 is a common default)
        let dimensions = Dimensions::new(80, 24);
        
        // Create a terminal with default config
        let term = Term::new(
            alacritty_terminal::term::Config::default(),
            &dimensions,
            event_listener,
        );
        
        Self {
            id,
            term,
            pty_master: None,
            pty_writer: None,
            child_process: None,
            dimensions,
            cursor_position: Point::new(Line(0), Column(0)),
            title: title.to_string(),
        }
    }
    
    /// Spawn a shell in the terminal
    pub fn spawn_shell(&mut self) -> Result<()> {
        // Get the native PTY system
        let pty_system = native_pty_system();
        
        // Create a PTY with initial size
        let columns = self.dimensions.columns() as u16;
        let rows = self.dimensions.screen_lines() as u16;
        
        let pair = pty_system.openpty(PtySize {
            rows,
            cols: columns,
            pixel_width: 0,
            pixel_height: 0,
        }).context("Failed to open PTY")?;
        
        // Store the master side
        self.pty_master = Some(pair.master);
        
        // Get the default shell from the environment or use a fallback
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        
        // Create a command to run the shell
        let mut cmd = CommandBuilder::new(shell);
        
        // Set up environment variables
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
        let child = pair.slave.spawn_command(cmd)
            .context("Failed to spawn shell process")?;
        
        self.child_process = Some(child);
        
        // Get a writer for the PTY
        if let Some(pty_master) = &mut self.pty_master {
            let writer = pty_master.take_writer()
                .context("Failed to get PTY writer")?;
            
            self.pty_writer = Some(Arc::new(Mutex::new(writer)));
        }
        
        info!("Shell process spawned for terminal {}", self.id);
        Ok(())
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
    
    /// Read output from the PTY and feed it to the terminal
    pub fn read_output(&mut self) -> Result<bool> {
        if let Some(pty_master) = &mut self.pty_master {
            // Try to get a reader
            let mut reader = pty_master.try_clone_reader()
                .context("Failed to clone PTY reader")?;
            
            // Read data from the PTY
            let mut buf = [0u8; 4096];
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    // Process the received data in the terminal
                    let data = &buf[..n];
                    self.term.take_child().unwrap().advance_bytes(data);
                    
                    // Update cursor position
                    self.cursor_position = self.term.grid().cursor.point;
                    
                    return Ok(true); // We read some data
                }
                Ok(_) => {
                    // No data available
                    return Ok(false);
                }
                Err(e) => {
                    error!("Error reading from PTY: {}", e);
                    return Err(e.into());
                }
            }
        }
        
        Ok(false)
    }
    
    /// Resize the terminal
    pub fn resize(&mut self, columns: u16, rows: u16) -> Result<()> {
        // Update dimensions
        self.dimensions = Dimensions::new(columns as usize, rows as usize);
        
        // Resize the terminal
        self.term.resize(self.dimensions.clone());
        
        // Resize the PTY
        if let Some(pty_master) = &mut self.pty_master {
            pty_master.resize(PtySize {
                rows,
                cols: columns,
                pixel_width: 0,
                pixel_height: 0,
            })?;
        }
        
        Ok(())
    }
    
    /// Check if the terminal process has exited
    pub fn check_exit_status(&mut self) -> Option<i32> {
        if let Some(child) = &mut self.child_process {
            match child.try_wait() {
                Ok(Some(status)) => {
                    // Process has exited
                    #[cfg(windows)]
                    let code = status.code().unwrap_or(0);
                    #[cfg(not(windows))]
                    let code = status.code().unwrap_or(0);
                    
                    return Some(code);
                }
                _ => {}
            }
        }
        
        None
    }
    
    /// Close the terminal
    pub fn close(&mut self) -> Result<()> {
        // Kill the child process if it exists
        if let Some(child) = &mut self.child_process {
            let _ = child.kill();
        }
        
        // Clean up resources
        self.pty_master = None;
        self.pty_writer = None;
        self.child_process = None;
        
        Ok(())
    }
    
    /// Get the terminal's UUID
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    /// Get the terminal's title
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// Set the terminal's title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    
    /// Get the terminal's dimensions
    pub fn dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
    
    /// Get the terminal's cursor position
    pub fn cursor_position(&self) -> Point {
        self.cursor_position
    }
    
    /// Get a reference to the terminal for rendering
    pub fn term(&self) -> &Term<EventListener> {
        &self.term
    }
}