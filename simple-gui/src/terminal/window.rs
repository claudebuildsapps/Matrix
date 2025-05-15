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
    // Optimization: Track visible cells range
    visible_start_row: usize,
    visible_end_row: usize,
    visible_start_col: usize,
    visible_end_col: usize,
    // Optimization: Cache cell data for faster access
    cell_cache: Vec<Vec<Option<(char, Color, Color)>>>,
    // Optimization: Track if cells need redraw
    cells_dirty: bool,
}

impl TerminalWindow {
    // Create a new terminal window
    pub fn new(id: Uuid, title: &str) -> Self {
        let emulator = TerminalEmulator::new(id, title);
        let dimensions = emulator.dimensions().clone();
        
        // Initialize cell cache with terminal dimensions
        let cols = dimensions.columns();
        let rows = dimensions.screen_lines();
        let mut cell_cache = Vec::with_capacity(rows);
        for _ in 0..rows {
            let row = vec![None; cols];
            cell_cache.push(row);
        }
        
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
            visible_start_row: 0,
            visible_end_row: rows,
            visible_start_col: 0,
            visible_end_col: cols,
            cell_cache,
            cells_dirty: true,
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
                self.cells_dirty = true;
                self.cache.clear();
                None
            }
            
            TerminalMessage::Resize(columns, rows) => {
                if let Err(e) = self.emulator.resize(columns, rows) {
                    eprintln!("Failed to resize terminal: {}", e);
                }
                
                // Resize cell cache
                self.cell_cache = Vec::with_capacity(rows as usize);
                for _ in 0..(rows as usize) {
                    let row = vec![None; columns as usize];
                    self.cell_cache.push(row);
                }
                
                self.cells_dirty = true;
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
                self.cells_dirty = true;
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
                self.cells_dirty = true;
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
                // We received some output, so mark cells as dirty
                self.cells_dirty = true;
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
    
    // Calculate which cells are visible in the viewport
    fn update_visible_cell_range(&mut self, bounds: Rectangle) {
        let width = bounds.width;
        let height = bounds.height;
        
        let cols = self.dimensions().columns();
        let rows = self.dimensions().screen_lines();
        
        // Calculate visible range - add 1 to ensure we render cells that are partially visible
        let visible_cols = (width / self.cell_size.width).ceil() as usize + 1;
        let visible_rows = (height / self.cell_size.height).ceil() as usize + 1;
        
        // Ensure we don't exceed terminal dimensions
        self.visible_end_col = visible_cols.min(cols);
        self.visible_end_row = visible_rows.min(rows);
        self.visible_start_col = 0;
        self.visible_start_row = 0;
    }
    
    // Update the cell cache with current terminal state
    fn update_cell_cache(&mut self) {
        if !self.cells_dirty {
            return;
        }
        
        let term = self.emulator.term();
        
        // Only update visible cells to save time
        for row in self.visible_start_row..self.visible_end_row {
            for col in self.visible_start_col..self.visible_end_col {
                let point = Point::new(Line(row as i32), Column(col as u16));
                let cell = term.grid()[point];
                
                // Skip empty/spaces if background is default
                if cell.c == ' ' && cell.bg() == alacritty_terminal::ansi::Color::Named(alacritty_terminal::term::color::NamedColor::Background) {
                    self.cell_cache[row][col] = None;
                    continue;
                }
                
                // Cache the cell character and colors
                let fg_color = MATRIX_GREEN; // Simplified - in a real implementation we'd convert from alacritty colors
                let bg_color = if cell.bg() != alacritty_terminal::ansi::Color::Named(alacritty_terminal::term::color::NamedColor::Background) {
                    DARK_GREEN
                } else {
                    BACKGROUND
                };
                
                self.cell_cache[row][col] = Some((cell.c, fg_color, bg_color));
            }
        }
        
        self.cells_dirty = false;
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
            // Update visible cell range
            let mut mutable_self = unsafe { &mut *(self as *const Self as *mut Self) };
            mutable_self.update_visible_cell_range(bounds);
            mutable_self.update_cell_cache();
            
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
            for row in self.visible_start_row..self.visible_end_row {
                for col in self.visible_start_col..self.visible_end_col {
                    // Get the cell at this position
                    let point = Point::new(Line(row as i32), Column(col as u16));
                    
                    // Skip cells that don't need drawing (from cache)
                    if self.cell_cache[row][col].is_none() {
                        continue;
                    }
                    
                    // Calculate pixel position
                    let x = col as f32 * self.cell_size.width;
                    let y = row as f32 * self.cell_size.height;
                    
                    // Check if this is the cursor position
                    let is_cursor = point == cursor_point && self.cursor_blink_state && self.focused;
                    
                    // Get cached cell info
                    let (c, mut fg_color, mut bg_color) = self.cell_cache[row][col].unwrap();
                    
                    // Override colors for cursor
                    if is_cursor {
                        bg_color = MATRIX_GREEN;
                        fg_color = BACKGROUND;
                    }
                    
                    // Draw the cell background if needed
                    if bg_color != BACKGROUND {
                        frame.fill_rectangle(
                            iced::Point::new(x, y),
                            Size::new(self.cell_size.width, self.cell_size.height),
                            bg_color,
                        );
                    }
                    
                    // Draw the cell character
                    if c != ' ' {
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