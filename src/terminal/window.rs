use anyhow::Result;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};
use std::time::Duration;
use uuid::Uuid;
use tokio::task::JoinHandle;
use tokio::sync::mpsc;

use crate::terminal::buffer::TerminalBuffer;
use crate::terminal::process::{Process, ProcessController, ProcessEvent};

// The different states a terminal window can be in
#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    // Ready to start a process
    Ready,
    // Running a process
    Running,
    // Process has exited
    Exited(i32),
    // An error occurred
    Error(String),
}

// Events that can happen in a terminal window
#[derive(Debug, Clone)]
pub enum WindowEvent {
    // Process has output new data
    Output(Vec<u8>),
    // Process has exited
    Exit(i32),
    // An error occurred
    Error(String),
    // Resize event
    Resize(u16, u16),
    // Request to focus this window
    Focus,
}

pub struct TerminalWindow {
    // Unique ID for this window
    id: Uuid,
    // Window title
    pub title: String,
    // Terminal buffer to store content
    pub buffer: TerminalBuffer,
    // Current state of the window
    state: WindowState,
    // Process controller for interacting with the subprocess
    process: Option<Box<dyn ProcessController + Send>>,
    // Window size
    size: Rect,
    // Is this window focused
    focused: bool,
    // Event receiver from process
    event_rx: Option<mpsc::UnboundedReceiver<WindowEvent>>,
    // Process monitor task
    _process_task: Option<JoinHandle<()>>,
}

impl TerminalWindow {
    // Create a new terminal window
    pub fn new(title: &str, size: Rect) -> Self {
        let buffer_size = 10000; // Store up to 10000 lines of history
        
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            buffer: TerminalBuffer::new(buffer_size),
            state: WindowState::Ready,
            process: None,
            size,
            focused: false,
            event_rx: None,
            _process_task: None,
        }
    }
    
    // Start a new process in this window
    pub fn spawn_process(&mut self, command: &str, working_dir: Option<&str>) -> Result<()> {
        // Create a new process
        let process = Process::new(
            command,
            working_dir,
            self.size.width,
            self.size.height.saturating_sub(2), // Subtract border height
        )?;
        
        // Set up channel for process events
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Clone what we need for the background task
        let tx_clone = tx.clone();
        
        // Start a background task to monitor the process
        let process_task = tokio::spawn(async move {
            // Simulate some output
            let output = b"Welcome to Matrix Terminal!\nThis is a simulated shell.\n> ";
            let _ = tx_clone.send(WindowEvent::Output(output.to_vec()));
            
            // Keep the task alive to simulate a running process
            loop {
                tokio::time::sleep(Duration::from_secs(1000)).await;
            }
        });
        
        // Create a process controller
        let process_controller = Box::new(process);
        
        self.process = Some(process_controller);
        self.event_rx = Some(rx);
        self._process_task = Some(process_task);
        self.state = WindowState::Running;
        
        Ok(())
    }
    
    // Update the window state based on events
    pub fn update(&mut self) -> Result<()> {
        if let Some(rx) = &mut self.event_rx {
            // Try to receive events without blocking
            if let Ok(event) = rx.try_recv() {
                match event {
                    WindowEvent::Output(data) => {
                        self.buffer.write(&data)?;
                    }
                    WindowEvent::Exit(code) => {
                        self.state = WindowState::Exited(code);
                    }
                    WindowEvent::Error(err) => {
                        self.state = WindowState::Error(err);
                    }
                    WindowEvent::Resize(rows, cols) => {
                        if let Some(process) = &mut self.process {
                            process.resize(rows, cols)?;
                        }
                        self.buffer.resize(rows as usize, cols as usize);
                    }
                    WindowEvent::Focus => {
                        self.focused = true;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Send input to the process
    pub fn send_input(&mut self, data: &[u8]) -> Result<()> {
        if let Some(process) = &mut self.process {
            process.write(data)?;
        }
        Ok(())
    }
    
    // Render the window to a ratatui frame
    pub fn render<'a>(&self) -> Paragraph<'a> {
        // Get the visible content from the buffer
        let content = self.buffer.visible_lines();
        let content_text = content.join("\n");
        
        // Create a styled block for the window
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let status_indicator = match &self.state {
            WindowState::Ready => "[Ready]",
            WindowState::Running => "[Running]",
            WindowState::Exited(code) => if *code == 0 { "[Exited:0]" } else { "[Exited!]" },
            WindowState::Error(_) => "[Error]",
        };
        
        let title = format!("{} {}", self.title, status_indicator);
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Plain) // Use plain borders for square edges
            .title(title)
            .border_style(border_style);
        
        // Create paragraph with the content
        Paragraph::new(Text::from(content_text))
            .block(block)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false })
    }
    
    // Resize the window
    pub fn resize(&mut self, size: Rect) -> Result<()> {
        self.size = size;
        
        // Calculate effective terminal size (accounting for borders)
        let terminal_rows = size.height.saturating_sub(2) as usize; // Subtract 2 for borders
        let terminal_cols = size.width.saturating_sub(2) as usize; // Subtract 2 for borders
        
        // Resize buffer
        self.buffer.resize(terminal_rows, terminal_cols);
        
        // Resize the process terminal
        if let Some(process) = &mut self.process {
            process.resize(terminal_rows as u16, terminal_cols as u16)?;
        }
        
        Ok(())
    }
    
    // Focus this window
    pub fn focus(&mut self) {
        self.focused = true;
    }
    
    // Unfocus this window
    pub fn unfocus(&mut self) {
        self.focused = false;
    }
    
    // Get window ID
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    // Check if window is focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    
    // Get window size
    pub fn size(&self) -> Rect {
        self.size
    }
    
    // Get window state
    pub fn state(&self) -> &WindowState {
        &self.state
    }
    
    // Close the window, killing any running process
    pub fn close(&mut self) -> Result<()> {
        if let Some(process) = &mut self.process {
            process.kill()?;
        }
        
        self.process = None;
        self._process_task = None;
        self.event_rx = None;
        self.state = WindowState::Exited(-1);
        
        Ok(())
    }
}