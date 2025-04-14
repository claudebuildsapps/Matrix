use anyhow::Result;
use std::collections::VecDeque;
use std::cmp::{min, max};
use std::ops::Range;

// Terminal buffer to store and manage terminal output
pub struct TerminalBuffer {
    lines: VecDeque<String>,
    // Current cursor position (row, column)
    cursor_pos: (usize, usize),
    // Maximum number of lines to keep in history
    max_lines: usize,
    // Current viewport scroll position (0 = bottom/newest)
    scroll_offset: usize,
    // Viewport size (rows, columns)
    viewport_size: (usize, usize),
}

impl TerminalBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            cursor_pos: (0, 0),
            max_lines,
            scroll_offset: 0,
            viewport_size: (24, 80), // Default terminal size
        }
    }
    
    // Write raw data to the buffer (handles basic terminal control sequences)
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        // Ensure there's at least one line
        if self.lines.is_empty() {
            self.lines.push_back(String::new());
        }
        
        // Current cursor position
        let (mut row, mut col) = self.cursor_pos;
        
        // Process each byte
        let mut i = 0;
        while i < data.len() {
            match data[i] {
                // Newline
                b'\n' => {
                    row += 1;
                    col = 0;
                    
                    // Add a new line if needed
                    if row >= self.lines.len() {
                        self.lines.push_back(String::new());
                        
                        // Trim history if needed
                        if self.lines.len() > self.max_lines {
                            self.lines.pop_front();
                            row = self.lines.len() - 1;
                        }
                    }
                }
                
                // Carriage return
                b'\r' => {
                    col = 0;
                }
                
                // Tab
                b'\t' => {
                    // Replace tab with spaces (every 8 columns)
                    let spaces = 8 - (col % 8);
                    for _ in 0..spaces {
                        if col < self.viewport_size.1 {
                            // Extend current line if needed
                            let current_line = &mut self.lines[row];
                            while current_line.len() <= col {
                                current_line.push(' ');
                            }
                            current_line.replace_range(col..col+1, " ");
                            col += 1;
                        }
                    }
                }
                
                // ESC - basic ANSI escape sequence handling (very simplified)
                b'\x1b' => {
                    // Check if we have an escape sequence
                    if i + 1 < data.len() && data[i + 1] == b'[' {
                        i += 2; // Skip ESC [
                        
                        // Parse sequence
                        let mut seq = String::new();
                        while i < data.len() && !data[i].is_ascii_alphabetic() {
                            seq.push(data[i] as char);
                            i += 1;
                        }
                        
                        // Process command if we have one
                        if i < data.len() {
                            let cmd = data[i] as char;
                            match cmd {
                                // Clear screen
                                'J' => {
                                    if seq == "2" {
                                        self.lines.clear();
                                        self.lines.push_back(String::new());
                                        row = 0;
                                        col = 0;
                                    }
                                }
                                // Cursor position
                                'H' => {
                                    let parts: Vec<&str> = seq.split(';').collect();
                                    if parts.len() == 2 {
                                        if let (Ok(new_row), Ok(new_col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                                            row = min(new_row.saturating_sub(1), self.lines.len().saturating_sub(1));
                                            col = min(new_col.saturating_sub(1), self.viewport_size.1.saturating_sub(1));
                                        }
                                    }
                                }
                                // We ignore other escape sequences for now
                                _ => {}
                            }
                        }
                    }
                }
                
                // Normal character
                _ => {
                    // Make sure we have enough lines
                    while self.lines.len() <= row {
                        self.lines.push_back(String::new());
                    }
                    
                    // Get current line and make sure it's long enough
                    let current_line = &mut self.lines[row];
                    while current_line.len() <= col {
                        current_line.push(' ');
                    }
                    
                    // Replace character at current position
                    if col < current_line.len() {
                        // This is safe because we're indexing within a valid char boundary
                        // (we're only handling ASCII for now)
                        current_line.replace_range(col..col+1, &(data[i] as char).to_string());
                    } else {
                        current_line.push(data[i] as char);
                    }
                    
                    // Advance cursor
                    col += 1;
                }
            }
            
            i += 1;
        }
        
        // Update cursor position
        self.cursor_pos = (row, col);
        
        Ok(())
    }
    
    // Get visible lines based on current scroll position and viewport height
    pub fn visible_lines(&self) -> Vec<&str> {
        let buffer_size = self.lines.len();
        
        // Calculate visible range
        let visible_rows = min(self.viewport_size.0, buffer_size);
        
        // Adjust scroll offset if needed
        let max_scroll = buffer_size.saturating_sub(visible_rows);
        let scroll = min(self.scroll_offset, max_scroll);
        
        // Calculate range of lines to display
        let start = buffer_size.saturating_sub(visible_rows).saturating_sub(scroll);
        let end = start + visible_rows;
        let range = start..min(end, buffer_size);
        
        // Extract lines from the buffer
        range.map(|i| self.lines[i].as_str()).collect()
    }
    
    // Scroll the view up (toward older content)
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = min(self.scroll_offset + lines, self.lines.len().saturating_sub(1));
    }
    
    // Scroll the view down (toward newer content)
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }
    
    // Move to the bottom (most recent content)
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }
    
    // Resize the viewport
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.viewport_size = (rows, cols);
        
        // Adjust cursor position if needed
        let (row, col) = self.cursor_pos;
        self.cursor_pos = (min(row, self.lines.len().saturating_sub(1)), min(col, cols));
    }
    
    // Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        self.cursor_pos
    }
    
    // Search for text in the buffer
    pub fn search(&self, query: &str, case_sensitive: bool) -> Vec<(usize, Range<usize>)> {
        let mut results = Vec::new();
        
        if query.is_empty() {
            return results;
        }
        
        // Process each line
        for (line_idx, line) in self.lines.iter().enumerate() {
            let mut start_idx = 0;
            
            while start_idx < line.len() {
                let search_line = if case_sensitive {
                    line[start_idx..].to_string()
                } else {
                    line[start_idx..].to_lowercase()
                };
                
                let search_query = if case_sensitive {
                    query.to_string()
                } else {
                    query.to_lowercase()
                };
                
                if let Some(pos) = search_line.find(&search_query) {
                    let match_start = start_idx + pos;
                    let match_end = match_start + query.len();
                    results.push((line_idx, match_start..match_end));
                    
                    // Move past this match
                    start_idx = match_end;
                } else {
                    break;
                }
            }
        }
        
        results
    }
    
    // Clear the buffer
    pub fn clear(&mut self) {
        self.lines.clear();
        self.lines.push_back(String::new());
        self.cursor_pos = (0, 0);
        self.scroll_offset = 0;
    }
}