use iced::{
    Color, Rectangle, Size, Vector, Background, Element, Length,
    canvas::{self, Cache, Canvas, Cursor, Frame, Geometry, Path, Text},
    mouse::{self, Cursor as MouseCursor},
    widget::canvas::{self, event, Event},
    theme, Theme,
};

use alacritty_terminal::{
    term::{cell::Cell, Term, TermMode},
    grid::Dimensions,
    index::{Line, Column, Point},
    ansi,
};

use crate::terminal::TerminalEmulator;
use uuid::Uuid;
use std::time::{Duration, Instant};
use std::sync::Arc;

// Matrix colors
const MATRIX_GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.255,
    a: 1.0,
};

const DARK_GREEN: Color = Color {
    r: 0.0,
    g: 0.7,
    b: 0.176,
    a: 1.0,
};

const BACKGROUND: Color = Color::BLACK;

// Messages that can be sent from the terminal window
#[derive(Debug, Clone)]
pub enum TerminalMessage {
    Input(Vec<u8>),
    Resize(u16, u16),
    ProcessExit(i32),
    Click(mouse::Button, f32, f32),
    Key(char),
    SpecialKey(SpecialKey),
}

// Special keys that can be sent to the terminal
#[derive(Debug, Clone, Copy)]
pub enum SpecialKey {
    Enter,
    Backspace,
    Tab,
    Escape,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
}

// Terminal window component that renders a terminal
pub struct TerminalWindow {
    id: Uuid,
    emulator: TerminalEmulator,
    cache: Cache,
    last_update: Instant,
    cursor_blink_state: bool,
    cursor_blink_timer: Instant,
    cursor_blink_duration: Duration,
    focused: bool,
    cell_size: Size,
    font_size: f32,
}

impl TerminalWindow {
    // Create a new terminal window
    pub fn new(id: Uuid, title: &str) -> Self {
        let emulator = TerminalEmulator::new(id, title);
        
        Self {
            id,
            emulator,
            cache: Cache::default(),
            last_update: Instant::now(),
            cursor_blink_state: true,
            cursor_blink_timer: Instant::now(),
            cursor_blink_duration: Duration::from_millis(500),
            focused: false,
            cell_size: Size::new(8.0, 16.0), // Default cell size
            font_size: 14.0,
        }
    }
    
    // Spawn a shell in the terminal
    pub fn spawn_shell(&mut self) -> anyhow::Result<()> {
        self.emulator.spawn_shell()
    }
    
    // Send input to the terminal
    pub fn send_input(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.emulator.send_input(data)
    }
    
    // Update the terminal window
    pub fn update(&mut self, message: TerminalMessage) -> Option<TerminalMessage> {
        match message {
            TerminalMessage::Input(data) => {
                if let Err(e) = self.send_input(&data) {
                    eprintln!("Failed to send input: {}", e);
                }
                self.cache.clear();
                None
            }
            
            TerminalMessage::Resize(columns, rows) => {
                if let Err(e) = self.emulator.resize(columns, rows) {
                    eprintln!("Failed to resize terminal: {}", e);
                }
                self.cache.clear();
                None
            }
            
            TerminalMessage::Click(button, x, y) => {
                // Calculate cell position from pixel coordinates
                let column = (x / self.cell_size.width).floor() as usize;
                let row = (y / self.cell_size.height).floor() as usize;
                
                // Handle click based on position
                self.focused = true;
                self.cache.clear();
                None
            }
            
            TerminalMessage::Key(c) => {
                // Send the character to the terminal
                if let Err(e) = self.send_input(&[c as u8]) {
                    eprintln!("Failed to send key: {}", e);
                }
                self.cache.clear();
                None
            }
            
            TerminalMessage::SpecialKey(key) => {
                // Convert special key to appropriate byte sequence
                let bytes = match key {
                    SpecialKey::Enter => b"\r".to_vec(),
                    SpecialKey::Backspace => b"\x7f".to_vec(),
                    SpecialKey::Tab => b"\t".to_vec(),
                    SpecialKey::Escape => b"\x1b".to_vec(),
                    SpecialKey::Up => b"\x1b[A".to_vec(),
                    SpecialKey::Down => b"\x1b[B".to_vec(),
                    SpecialKey::Right => b"\x1b[C".to_vec(),
                    SpecialKey::Left => b"\x1b[D".to_vec(),
                    SpecialKey::Home => b"\x1b[H".to_vec(),
                    SpecialKey::End => b"\x1b[F".to_vec(),
                    SpecialKey::PageUp => b"\x1b[5~".to_vec(),
                    SpecialKey::PageDown => b"\x1b[6~".to_vec(),
                    SpecialKey::Delete => b"\x1b[3~".to_vec(),
                };
                
                if let Err(e) = self.send_input(&bytes) {
                    eprintln!("Failed to send special key: {}", e);
                }
                self.cache.clear();
                None
            }
            
            TerminalMessage::ProcessExit(status) => {
                // Terminal process has exited
                eprintln!("Terminal process exited with status: {}", status);
                None
            }
        }
    }
    
    // Check for updates from the terminal
    pub fn check_for_updates(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        // Only check for updates every 16ms (roughly 60fps)
        if elapsed < Duration::from_millis(16) {
            return false;
        }
        
        // Update cursor blink state
        if now.duration_since(self.cursor_blink_timer) > self.cursor_blink_duration {
            self.cursor_blink_state = !self.cursor_blink_state;
            self.cursor_blink_timer = now;
            self.cache.clear();
        }
        
        // Read output from the terminal
        match self.emulator.read_output() {
            Ok(true) => {
                // We received some output, so clear the cache
                self.cache.clear();
                self.last_update = now;
                true
            }
            Ok(false) => {
                // No new output
                self.last_update = now;
                false
            }
            Err(e) => {
                eprintln!("Error reading terminal output: {}", e);
                self.last_update = now;
                false
            }
        }
    }
    
    // Get the terminal's dimensions in cells
    pub fn dimensions(&self) -> &Dimensions {
        self.emulator.dimensions()
    }
    
    // Calculate the size in pixels based on the terminal dimensions
    pub fn pixel_size(&self) -> Size {
        let dimensions = self.dimensions();
        Size::new(
            dimensions.columns() as f32 * self.cell_size.width,
            dimensions.screen_lines() as f32 * self.cell_size.height,
        )
    }
    
    // Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        if self.focused != focused {
            self.focused = focused;
            self.cache.clear();
        }
    }
    
    // Check if the terminal is focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    
    // Get the terminal's ID
    pub fn id(&self) -> Uuid {
        self.id
    }
    
    // Get the terminal's title
    pub fn title(&self) -> &str {
        self.emulator.title()
    }
    
    // Render the terminal as an iced Element
    pub fn view<'a>(&'a self) -> Element<'a, TerminalMessage> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> canvas::Program<TerminalMessage> for TerminalWindow {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: MouseCursor,
    ) -> (event::Status, Option<TerminalMessage>) {
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed { button, position } => {
                        // Convert position to terminal coordinates
                        let x = position.x - bounds.x;
                        let y = position.y - bounds.y;
                        
                        return (
                            event::Status::Captured,
                            Some(TerminalMessage::Click(button, x, y)),
                        );
                    }
                    _ => {}
                }
                
                (event::Status::Ignored, None)
            }
            _ => (event::Status::Ignored, None),
        }
    }
    
    fn draw(&self, bounds: Rectangle, _cursor: MouseCursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame| {
            // Clear the frame with the background color
            frame.fill_rectangle(
                Point::new(0.0, 0.0).into(),
                bounds.size(),
                BACKGROUND,
            );
            
            // Get the terminal contents
            let term = self.emulator.term();
            let cursor_point = self.emulator.cursor_position();
            
            // Iterate through visible cells and draw them
            for row in 0..self.dimensions().screen_lines() {
                for col in 0..self.dimensions().columns() {
                    // Get the cell at this position
                    let point = Point::new(Line(row as i32), Column(col));
                    let cell = term.grid()[point];
                    
                    // Calculate pixel position
                    let x = col as f32 * self.cell_size.width;
                    let y = row as f32 * self.cell_size.height;
                    
                    // Draw the cell background if needed
                    let bg_color = if point == cursor_point && self.cursor_blink_state && self.focused {
                        // Cursor position
                        MATRIX_GREEN
                    } else if cell.bg() != alacritty_terminal::ansi::Color::Named(alacritty_terminal::term::color::NamedColor::Background) {
                        // Cell with custom background
                        DARK_GREEN
                    } else {
                        // Default background
                        continue;
                    };
                    
                    frame.fill_rectangle(
                        iced::Point::new(x, y),
                        Size::new(self.cell_size.width, self.cell_size.height),
                        bg_color,
                    );
                    
                    // Draw the cell character
                    if !cell.flags().contains(alacritty_terminal::term::cell::Flags::HIDDEN) {
                        let c = match cell.c {
                            ' ' => continue, // Don't draw spaces
                            c => c,
                        };
                        
                        // Determine text color
                        let fg_color = if point == cursor_point && self.cursor_blink_state && self.focused {
                            // Cursor position text
                            BACKGROUND
                        } else {
                            // Normal text
                            MATRIX_GREEN
                        };
                        
                        // Draw the character
                        let text = Text {
                            content: c.to_string(),
                            position: iced::Point::new(x, y + self.cell_size.height * 0.8),
                            color: fg_color,
                            size: self.font_size,
                            ..Text::default()
                        };
                        
                        frame.fill_text(text);
                    }
                }
            }
            
            // Draw cursor if not shown via cell background
            if !self.cursor_blink_state && self.focused {
                let x = cursor_point.column.0 as f32 * self.cell_size.width;
                let y = cursor_point.line.0 as f32 * self.cell_size.height;
                
                // Draw a cursor outline
                let cursor_rect = Path::rectangle(
                    iced::Point::new(x, y),
                    Size::new(self.cell_size.width, self.cell_size.height),
                );
                frame.stroke(&cursor_rect, canvas::Stroke::default().with_width(1.0).with_color(MATRIX_GREEN));
            }
            
            // Draw border if focused
            if self.focused {
                let border_rect = Path::rectangle(
                    iced::Point::new(0.0, 0.0),
                    bounds.size(),
                );
                frame.stroke(&border_rect, canvas::Stroke::default().with_width(1.0).with_color(MATRIX_GREEN));
            }
        });
        
        vec![content]
    }
}