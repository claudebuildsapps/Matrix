/// Font utilities for the terminal renderer
pub struct FontMetrics {
    pub width: f32,
    pub height: f32,
    pub descender: f32,
    pub line_height: f32,
}

impl FontMetrics {
    /// Create default font metrics for a given font size
    pub fn new(font_size: f32) -> Self {
        // These are approximate values that work well for monospace fonts
        let width = font_size * 0.6;         // Character width (approximate for monospace)
        let height = font_size;              // Base character height
        let descender = font_size * 0.2;     // Space below the baseline
        let line_height = font_size * 1.2;   // Total height including line spacing
        
        Self {
            width,
            height,
            descender,
            line_height,
        }
    }
    
    /// Calculate the size needed for a terminal with the given dimensions
    pub fn calculate_terminal_size(&self, columns: u16, rows: u16) -> (f32, f32) {
        let width = self.width * columns as f32;
        let height = self.line_height * rows as f32;
        
        (width, height)
    }
    
    /// Convert pixel coordinates to cell coordinates
    pub fn pixel_to_cell(&self, x: f32, y: f32) -> (u16, u16) {
        let col = (x / self.width).floor() as u16;
        let row = (y / self.line_height).floor() as u16;
        
        (col, row)
    }
    
    /// Convert cell coordinates to pixel coordinates (top-left of cell)
    pub fn cell_to_pixel(&self, col: u16, row: u16) -> (f32, f32) {
        let x = col as f32 * self.width;
        let y = row as f32 * self.line_height;
        
        (x, y)
    }
}