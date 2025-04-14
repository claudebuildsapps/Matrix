use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{EnableMouseCapture, DisableMouseCapture},
    execute,
};
use std::io::{self, Stdout};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal as TuiTerminal;
use anyhow::Result;

pub struct Terminal {
    terminal: TuiTerminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        // Initialize terminal with robust error handling
        // First enable raw mode
        match enable_raw_mode() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to enable raw mode: {}", e);
                eprintln!("Matrix Terminal requires an interactive terminal.");
                return Err(anyhow::anyhow!("Failed to enable raw mode: {}", e));
            }
        };
        
        // Setup terminal enhancements
        match execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture) {
            Ok(_) => {},
            Err(e) => {
                // Try to restore terminal
                let _ = disable_raw_mode();
                eprintln!("Failed to setup terminal: {}", e);
                return Err(anyhow::anyhow!("Failed to setup terminal: {}", e));
            }
        };
        
        // Create the terminal backend
        let backend = CrosstermBackend::new(io::stdout());
        
        // Create the terminal
        let mut terminal = match TuiTerminal::new(backend) {
            Ok(term) => term,
            Err(e) => {
                // Try to restore terminal
                let _ = disable_raw_mode();
                let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
                eprintln!("Failed to create terminal: {}", e);
                return Err(anyhow::anyhow!("Failed to create terminal: {}", e));
            }
        };
        
        // Clear the terminal
        if let Err(e) = terminal.clear() {
            // Non-fatal error, just log it
            eprintln!("Warning: Failed to clear terminal: {}", e);
        }
        
        // Return the initialized terminal
        Ok(Self { terminal })
    }
    
    pub fn draw<F>(&mut self, render_fn: F) -> Result<()>
    where
        F: FnOnce(&mut ratatui::Frame),
    {
        self.terminal.draw(render_fn)?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Restore terminal
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}
