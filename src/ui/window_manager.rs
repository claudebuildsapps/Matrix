use anyhow::Result;
use ratatui::layout::{Layout, Direction as TuiDirection, Constraint, Rect};
use uuid::Uuid;
use std::collections::HashMap;

use crate::terminal::window::TerminalWindow;

// Navigation directions for window focus
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// The different types of window layouts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

// A node in the window layout tree
#[derive(Debug, Clone)]
pub enum LayoutNode {
    // A leaf node containing a terminal window
    Window {
        id: Uuid,
        rect: Rect,
    },
    // A split node with two children
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
        rect: Rect,
    },
}

impl LayoutNode {
    // Create a new window node
    pub fn window(id: Uuid, rect: Rect) -> Self {
        LayoutNode::Window { id, rect }
    }
    
    // Create a new split node
    pub fn split(direction: SplitDirection, ratio: f32, first: LayoutNode, second: LayoutNode, rect: Rect) -> Self {
        LayoutNode::Split {
            direction,
            ratio,
            first: Box::new(first),
            second: Box::new(second),
            rect,
            }
    }
    
    // Calculate the layout of child nodes
    pub fn calculate_layout(&mut self, outer_rect: Rect) {
        match self {
            LayoutNode::Window { rect, .. } => {
                *rect = outer_rect;
            }
            LayoutNode::Split { direction, ratio, first, second, rect } => {
                *rect = outer_rect;
                
                let constraints = match direction {
                    SplitDirection::Horizontal => {
                        let left_width = (outer_rect.width as f32 * *ratio).floor() as u16;
                        let right_width = outer_rect.width - left_width;
                        [Constraint::Length(left_width), Constraint::Length(right_width)]
                    }
                    SplitDirection::Vertical => {
                        let top_height = (outer_rect.height as f32 * *ratio).floor() as u16;
                        let bottom_height = outer_rect.height - top_height;
                        [Constraint::Length(top_height), Constraint::Length(bottom_height)]
                    }
                };
                
                let layout_direction = match direction {
                    SplitDirection::Horizontal => TuiDirection::Horizontal,
                    SplitDirection::Vertical => TuiDirection::Vertical,
                };
                
                let areas = Layout::default()
                    .direction(layout_direction)
                    .constraints(constraints)
                    .split(outer_rect);
                
                first.calculate_layout(areas[0]);
                second.calculate_layout(areas[1]);
            }
        }
    }
    
    // Get all window IDs in this layout
    pub fn window_ids(&self) -> Vec<Uuid> {
        match self {
            LayoutNode::Window { id, .. } => vec![*id],
            LayoutNode::Split { first, second, .. } => {
                let mut ids = first.window_ids();
                ids.extend(second.window_ids());
                ids
            }
        }
    }
    
    // Get the window rectangle for a given ID
    pub fn window_rect(&self, id: &Uuid) -> Option<Rect> {
        match self {
            LayoutNode::Window { id: window_id, rect } => {
                if window_id == id {
                    Some(*rect)
                } else {
                    None
                }
            }
            LayoutNode::Split { first, second, .. } => {
                first.window_rect(id).or_else(|| second.window_rect(id))
            }
        }
    }
    
    // Find the node containing a window
    pub fn find_window_node(&mut self, id: &Uuid) -> Option<&mut LayoutNode> {
        match self {
            LayoutNode::Window { id: window_id, .. } => {
                if window_id == id {
                    Some(self)
                } else {
                    None
                }
            }
            LayoutNode::Split { first, second, .. } => {
                first.find_window_node(id).or_else(|| second.find_window_node(id))
            }
        }
    }
    
    // Split a window in the layout
    pub fn split_window(&mut self, id: &Uuid, direction: SplitDirection, new_id: Uuid, ratio: f32) -> Result<()> {
        if let Some(node) = self.find_window_node(id) {
            match node {
                LayoutNode::Window { id, rect } => {
                    // Create a new window node for the second window
                    let original_id = *id;
                    let original_rect = *rect;
                    
                    // Replace this node with a split node
                    *node = LayoutNode::Split {
                        direction,
                        ratio,
                        first: Box::new(LayoutNode::Window { id: original_id, rect: original_rect }),
                        second: Box::new(LayoutNode::Window { id: new_id, rect: original_rect }),
                        rect: original_rect,
                    };
                    
                    // Recalculate the layout
                    node.calculate_layout(original_rect);
                    
                    Ok(())
                }
                _ => unreachable!(),
            }
        } else {
            anyhow::bail!("Window not found in layout")
        }
    }
}

// The window manager handles the layout and interactions between windows
pub struct WindowManager {
    // The layout tree
    layout: Option<LayoutNode>,
    // The windows
    windows: HashMap<Uuid, TerminalWindow>,
    // The focused window
    focused_window: Option<Uuid>,
    // The available space
    area: Rect,
    // Zoomed window (if any)
    zoomed_window: Option<Uuid>,
    // Original layout before zooming
    pre_zoom_layout: Option<LayoutNode>,
}

impl WindowManager {
    // Create a new window manager
    pub fn new(area: Rect) -> Self {
        Self {
            layout: None,
            windows: HashMap::new(),
            focused_window: None,
            area,
            zoomed_window: None,
            pre_zoom_layout: None,
        }
    }
    
    // Create a new window
    pub fn create_window(&mut self, title: &str) -> Result<Uuid> {
        // Create the window
        let mut window = TerminalWindow::new(title, self.area);
        let window_id = window.id();
        
        // Add the window to our collection
        self.windows.insert(window_id, window);
        
        // If this is the first window, create the layout
        if self.layout.is_none() {
            self.layout = Some(LayoutNode::window(window_id, self.area));
            self.focused_window = Some(window_id);
            self.windows.get_mut(&window_id).unwrap().focus();
        }
        
        Ok(window_id)
    }
    
    // Split a window
    pub fn split_window(&mut self, id: Uuid, direction: SplitDirection, ratio: f32) -> Result<Uuid> {
        // Make sure the window exists
        if !self.windows.contains_key(&id) {
            anyhow::bail!("Window not found");
        }
        
        // Create a new window for the split
        let title = if let Some(window) = self.windows.get(&id) {
            format!("{} (Split)", window.title)
        } else {
            "New Window".to_string()
        };
        
        let mut new_window = TerminalWindow::new(&title, self.area);
        let new_id = new_window.id();
        
        // Add the new window to our collection
        self.windows.insert(new_id, new_window);
        
        // Update the layout
        if let Some(layout) = &mut self.layout {
            layout.split_window(&id, direction, new_id, ratio)?;
            
            // Recalculate the layout
            layout.calculate_layout(self.area);
            
            // Apply the calculated rectangles to windows
            self.apply_layout()?;
        }
        
        Ok(new_id)
    }
    
    // Apply the layout to the windows
    fn apply_layout(&mut self) -> Result<()> {
        if let Some(layout) = &self.layout {
            // Get all window IDs from the layout
            let window_ids = layout.window_ids();
            
            // Apply the calculated rectangles to each window
            for id in window_ids {
                if let Some(rect) = layout.window_rect(&id) {
                    if let Some(window) = self.windows.get_mut(&id) {
                        window.resize(rect)?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Resize the window manager
    pub fn resize(&mut self, area: Rect) -> Result<()> {
        self.area = area;
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Focus a window
    pub fn focus_window(&mut self, id: Uuid) -> Result<()> {
        // Make sure the window exists
        if !self.windows.contains_key(&id) {
            anyhow::bail!("Window not found");
        }
        
        // Unfocus the current window
        if let Some(focused_id) = self.focused_window {
            if let Some(window) = self.windows.get_mut(&focused_id) {
                window.unfocus();
            }
        }
        
        // Focus the new window
        if let Some(window) = self.windows.get_mut(&id) {
            window.focus();
            self.focused_window = Some(id);
            Ok(())
        } else {
            anyhow::bail!("Window not found");
        }
    }
    
    // Get the focused window
    pub fn focused_window(&self) -> Option<&TerminalWindow> {
        self.focused_window.and_then(|id| self.windows.get(&id))
    }
    
    // Get a mutable reference to the focused window
    pub fn focused_window_mut(&mut self) -> Option<&mut TerminalWindow> {
        self.focused_window.and_then(|id| self.windows.get_mut(&id))
    }
    
    // Focus the next window in sequence
    pub fn focus_next_window(&mut self) -> Result<()> {
        let window_ids: Vec<Uuid> = self.windows.keys().cloned().collect();
        
        if window_ids.is_empty() {
            return Ok(());
        }
        
        let current_id = self.focused_window;
        
        if let Some(current_id) = current_id {
            // Find the index of the current window
            if let Some(index) = window_ids.iter().position(|id| *id == current_id) {
                // Get the next window index (wrap around if needed)
                let next_index = (index + 1) % window_ids.len();
                let next_id = window_ids[next_index];
                
                // Focus the next window
                return self.focus_window(next_id);
            }
        }
        
        // If no window is focused or the current window is not found,
        // focus the first window
        if !window_ids.is_empty() {
            self.focus_window(window_ids[0])
        } else {
            Ok(())
        }
    }
    
    // Focus the previous window in sequence
    pub fn focus_prev_window(&mut self) -> Result<()> {
        let window_ids: Vec<Uuid> = self.windows.keys().cloned().collect();
        
        if window_ids.is_empty() {
            return Ok(());
        }
        
        let current_id = self.focused_window;
        
        if let Some(current_id) = current_id {
            // Find the index of the current window
            if let Some(index) = window_ids.iter().position(|id| *id == current_id) {
                // Get the previous window index (wrap around if needed)
                let prev_index = if index == 0 {
                    window_ids.len() - 1
                } else {
                    index - 1
                };
                let prev_id = window_ids[prev_index];
                
                // Focus the previous window
                return self.focus_window(prev_id);
            }
        }
        
        // If no window is focused or the current window is not found,
        // focus the last window
        if !window_ids.is_empty() {
            self.focus_window(window_ids[window_ids.len() - 1])
        } else {
            Ok(())
        }
    }
    
    // Navigate in a specific direction (if possible)
    pub fn focus_direction(&mut self, direction: Direction) -> Result<()> {
        // If we're zoomed, direction navigation doesn't make sense
        if self.zoomed_window.is_some() {
            return Ok(());
        }
        
        let current_id = if let Some(id) = self.focused_window {
            id
        } else {
            // If no window is focused, nothing to do
            return Ok(());
        };
        
        // Get the current window's rectangle
        let current_rect = if let Some(layout) = &self.layout {
            if let Some(rect) = layout.window_rect(&current_id) {
                rect
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };
        
        // Find the nearest window in the specified direction
        let mut best_candidate = None;
        let mut best_distance = f32::MAX;
        
        // Get center points of current window
        let current_center_x = current_rect.x as f32 + current_rect.width as f32 / 2.0;
        let current_center_y = current_rect.y as f32 + current_rect.height as f32 / 2.0;
        
        // Check all other windows to find the best candidate
        for (&id, window) in &self.windows {
            if id == current_id {
                continue;
            }
            
            let candidate_rect = window.size();
            let candidate_center_x = candidate_rect.x as f32 + candidate_rect.width as f32 / 2.0;
            let candidate_center_y = candidate_rect.y as f32 + candidate_rect.height as f32 / 2.0;
            
            // Calculate directional vectors
            let dx = candidate_center_x - current_center_x;
            let dy = candidate_center_y - current_center_y;
            
            // Check if the window is in the specified direction
            let is_in_direction = match direction {
                Direction::Up => dy < 0.0 && dy.abs() > dx.abs(),
                Direction::Down => dy > 0.0 && dy.abs() > dx.abs(),
                Direction::Left => dx < 0.0 && dx.abs() > dy.abs(),
                Direction::Right => dx > 0.0 && dx.abs() > dy.abs(),
            };
            
            if is_in_direction {
                // Calculate distance (squared for efficiency)
                let distance = dx * dx + dy * dy;
                
                if distance < best_distance {
                    best_distance = distance;
                    best_candidate = Some(id);
                }
            }
        }
        
        // Focus the best candidate if found
        if let Some(id) = best_candidate {
            self.focus_window(id)?;
        }
        
        Ok(())
    }
    
    // Zoom in on a window (or the focused window if none specified)
    pub fn zoom_window(&mut self, id: Option<Uuid>) -> Result<()> {
        // If already zoomed, first unzoom
        if self.zoomed_window.is_some() {
            self.unzoom()?;
            return Ok(());
        }
        
        // Get the ID of the window to zoom
        let zoom_id = if let Some(id) = id {
            id
        } else if let Some(id) = self.focused_window {
            id
        } else {
            anyhow::bail!("No window to zoom");
        };
        
        // Make sure the window exists
        if !self.windows.contains_key(&zoom_id) {
            anyhow::bail!("Window not found");
        }
        
        // Save the current layout
        if let Some(layout) = &self.layout {
            self.pre_zoom_layout = Some(layout.clone());
        }
        
        // Create a new layout with just the zoomed window
        self.layout = Some(LayoutNode::window(zoom_id, self.area));
        
        // Focus the zoomed window
        self.focus_window(zoom_id)?;
        
        // Remember which window is zoomed
        self.zoomed_window = Some(zoom_id);
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Restore the layout after zooming
    pub fn unzoom(&mut self) -> Result<()> {
        // Make sure we're zoomed
        if self.zoomed_window.is_none() {
            return Ok(());
        }
        
        // Restore the original layout
        if let Some(layout) = &self.pre_zoom_layout {
            self.layout = Some(layout.clone());
        }
        
        // Clear the zoom state
        self.zoomed_window = None;
        self.pre_zoom_layout = None;
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Check if a window is currently zoomed
    pub fn is_zoomed(&self) -> bool {
        self.zoomed_window.is_some()
    }
    
    // Get the zoomed window ID if any
    pub fn zoomed_window(&self) -> Option<Uuid> {
        self.zoomed_window
    }
    
    // Get all windows
    pub fn windows(&self) -> &HashMap<Uuid, TerminalWindow> {
        &self.windows
    }
    
    // Get a mutable reference to all windows
    pub fn windows_mut(&mut self) -> &mut HashMap<Uuid, TerminalWindow> {
        &mut self.windows
    }
    
    // Close a window and restructure the layout
    pub fn close_window(&mut self, id: Uuid) -> Result<()> {
        // Make sure the window exists
        if !self.windows.contains_key(&id) {
            anyhow::bail!("Window not found");
        }
        
        // If this is the only window, remove it completely
        if self.windows.len() == 1 {
            if let Some(mut window) = self.windows.remove(&id) {
                window.close()?;
            }
            self.layout = None;
            self.focused_window = None;
            return Ok(());
        }
        
        // Find a new window to focus if we're closing the focused window
        if self.focused_window == Some(id) {
            let other_window = self.windows.keys()
                .find(|&&window_id| window_id != id)
                .cloned();
                
            if let Some(other_id) = other_window {
                self.focused_window = Some(other_id);
                if let Some(window) = self.windows.get_mut(&other_id) {
                    window.focus();
                }
            }
        }
        
        // Close the window
        if let Some(mut window) = self.windows.remove(&id) {
            window.close()?;
        }
        
        // Restructure the layout
        self.restructure_layout(&id)?;
        
        Ok(())
    }
    
    // Restructure the layout after removing a window
    fn restructure_layout(&mut self, removed_id: &Uuid) -> Result<()> {
        if self.windows.is_empty() {
            self.layout = None;
            self.focused_window = None;
            return Ok(());
        }
        
        // Create a new layout, preserving the structure as much as possible
        if let Some(old_layout) = &self.layout {
            self.layout = Some(self.create_new_layout_without(old_layout, removed_id)?);
        } else {
            // If there was no layout, create one with the first window
            let window_ids: Vec<Uuid> = self.windows.keys().cloned().collect();
            self.layout = Some(LayoutNode::window(window_ids[0], self.area));
        }
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Create a new layout without the specified window
    fn create_new_layout_without(&self, old_layout: &LayoutNode, removed_id: &Uuid) -> Result<LayoutNode> {
        match old_layout {
            LayoutNode::Window { id, rect } => {
                if id == removed_id {
                    // This is the window to remove, but we need to replace it
                    // with something. For now, just grab the first available window.
                    let window_ids: Vec<Uuid> = self.windows.keys().cloned().collect();
                    if window_ids.is_empty() {
                        anyhow::bail!("No windows available");
                    }
                    Ok(LayoutNode::window(window_ids[0], *rect))
                } else {
                    // This window stays as is
                    Ok(LayoutNode::window(*id, *rect))
                }
            }
            LayoutNode::Split { direction, ratio, first, second, rect } => {
                // First check if the removed window is in either branch
                let first_contains = self.layout_contains_window(first, removed_id);
                let second_contains = self.layout_contains_window(second, removed_id);
                
                match (first_contains, second_contains) {
                    (true, false) => {
                        // The removed window is in the first branch
                        let new_first = self.create_new_layout_without(first, removed_id)?;
                        
                        // If the first branch now has only one window, we can simplify
                        if let LayoutNode::Window { id: first_id, .. } = new_first {
                            // Just return the second branch, which becomes parent
                            Ok(LayoutNode::window(first_id, *rect))
                        } else {
                            // Keep the split structure with the new first branch
                            Ok(LayoutNode::Split {
                                direction: *direction,
                                ratio: *ratio,
                                first: Box::new(new_first),
                                second: Box::new(second.as_ref().clone()),
                                rect: *rect,
                            })
                        }
                    }
                    (false, true) => {
                        // The removed window is in the second branch
                        let new_second = self.create_new_layout_without(second, removed_id)?;
                        
                        // If the second branch now has only one window, we can simplify
                        if let LayoutNode::Window { id: second_id, .. } = new_second {
                            // Just return the first branch, which becomes parent
                            Ok(LayoutNode::window(second_id, *rect))
                        } else {
                            // Keep the split structure with the new second branch
                            Ok(LayoutNode::Split {
                                direction: *direction,
                                ratio: *ratio,
                                first: Box::new(first.as_ref().clone()),
                                second: Box::new(new_second),
                                rect: *rect,
                            })
                        }
                    }
                    (true, true) => {
                        // Both branches contain windows to be removed (unlikely)
                        // Recursively handle both sides
                        let new_first = self.create_new_layout_without(first, removed_id)?;
                        let new_second = self.create_new_layout_without(second, removed_id)?;
                        
                        Ok(LayoutNode::Split {
                            direction: *direction,
                            ratio: *ratio,
                            first: Box::new(new_first),
                            second: Box::new(new_second),
                            rect: *rect,
                        })
                    }
                    (false, false) => {
                        // The split doesn't contain the window to be removed
                        Ok(LayoutNode::Split {
                            direction: *direction,
                            ratio: *ratio,
                            first: Box::new(first.as_ref().clone()),
                            second: Box::new(second.as_ref().clone()),
                            rect: *rect,
                        })
                    }
                }
            }
        }
    }
    
    // Check if a layout contains a window
    fn layout_contains_window(&self, layout: &LayoutNode, window_id: &Uuid) -> bool {
        match layout {
            LayoutNode::Window { id, .. } => id == window_id,
            LayoutNode::Split { first, second, .. } => {
                self.layout_contains_window(first, window_id) || self.layout_contains_window(second, window_id)
            }
        }
    }
    
    // Layout presets for common window arrangements
    
    // Apply a horizontal split layout with the given windows
    pub fn apply_horizontal_layout(&mut self, window_ids: &[Uuid]) -> Result<()> {
        if window_ids.is_empty() {
            anyhow::bail!("No windows provided");
        }
        
        // Make sure all windows exist
        for &id in window_ids {
            if !self.windows.contains_key(&id) {
                anyhow::bail!("Window not found: {}", id);
            }
        }
        
        // For a single window, just set it as the layout
        if window_ids.len() == 1 {
            self.layout = Some(LayoutNode::window(window_ids[0], self.area));
        } else {
            // Build a horizontal layout tree from right to left
            let mut layout = LayoutNode::window(window_ids[window_ids.len() - 1], self.area);
            
            // Build the layout tree from right to left
            for i in (0..window_ids.len() - 1).rev() {
                let id = window_ids[i];
                
                // Calculate how much of the remaining space this window gets
                let ratio = 1.0 / (i + 2) as f32;
                
                layout = LayoutNode::Split {
                    direction: SplitDirection::Horizontal,
                    ratio,
                    first: Box::new(LayoutNode::window(id, self.area)),
                    second: Box::new(layout),
                    rect: self.area,
                };
            }
            
            self.layout = Some(layout);
        }
        
        // Focus the first window
        if !window_ids.is_empty() {
            self.focus_window(window_ids[0])?;
        }
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Apply a vertical split layout with the given windows
    pub fn apply_vertical_layout(&mut self, window_ids: &[Uuid]) -> Result<()> {
        if window_ids.is_empty() {
            anyhow::bail!("No windows provided");
        }
        
        // Make sure all windows exist
        for &id in window_ids {
            if !self.windows.contains_key(&id) {
                anyhow::bail!("Window not found: {}", id);
            }
        }
        
        // For a single window, just set it as the layout
        if window_ids.len() == 1 {
            self.layout = Some(LayoutNode::window(window_ids[0], self.area));
        } else {
            // Build a vertical layout tree from bottom to top
            let mut layout = LayoutNode::window(window_ids[window_ids.len() - 1], self.area);
            
            // Build the layout tree from bottom to top
            for i in (0..window_ids.len() - 1).rev() {
                let id = window_ids[i];
                
                // Calculate how much of the remaining space this window gets
                let ratio = 1.0 / (i + 2) as f32;
                
                layout = LayoutNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio,
                    first: Box::new(LayoutNode::window(id, self.area)),
                    second: Box::new(layout),
                    rect: self.area,
                };
            }
            
            self.layout = Some(layout);
        }
        
        // Focus the first window
        if !window_ids.is_empty() {
            self.focus_window(window_ids[0])?;
        }
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Apply a grid layout with the given windows
    pub fn apply_grid_layout(&mut self, window_ids: &[Uuid]) -> Result<()> {
        if window_ids.is_empty() {
            anyhow::bail!("No windows provided");
        }
        
        // Make sure all windows exist
        for &id in window_ids {
            if !self.windows.contains_key(&id) {
                anyhow::bail!("Window not found: {}", id);
            }
        }
        
        // For a single window, just set it as the layout
        if window_ids.len() == 1 {
            self.layout = Some(LayoutNode::window(window_ids[0], self.area));
        } else if window_ids.len() == 2 {
            // For two windows, create a horizontal split
            self.layout = Some(LayoutNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                first: Box::new(LayoutNode::window(window_ids[0], self.area)),
                second: Box::new(LayoutNode::window(window_ids[1], self.area)),
                rect: self.area,
            });
        } else if window_ids.len() == 3 {
            // For three windows, create a left panel and vertically split right panel
            self.layout = Some(LayoutNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                first: Box::new(LayoutNode::window(window_ids[0], self.area)),
                second: Box::new(LayoutNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio: 0.5,
                    first: Box::new(LayoutNode::window(window_ids[1], self.area)),
                    second: Box::new(LayoutNode::window(window_ids[2], self.area)),
                    rect: self.area,
                }),
                rect: self.area,
            });
        } else {
            // For four or more windows, create a 2x2 grid or larger
            
            // Calculate grid dimensions
            let num_windows = window_ids.len();
            let rows = (num_windows as f64).sqrt().ceil() as usize;
            let cols = (num_windows + rows - 1) / rows; // Ceiling division
            
            // Create a grid of windows
            let mut row_layouts = Vec::new();
            
            for row in 0..rows {
                let mut col_layouts = Vec::new();
                
                for col in 0..cols {
                    let index = row * cols + col;
                    
                    if index < num_windows {
                        col_layouts.push(LayoutNode::window(window_ids[index], self.area));
                    }
                }
                
                // If we have multiple columns in this row, create a horizontal split
                if col_layouts.len() > 1 {
                    let mut row_layout = col_layouts.pop().unwrap();
                    
                    for layout in col_layouts.into_iter().rev() {
                        row_layout = LayoutNode::Split {
                            direction: SplitDirection::Horizontal,
                            ratio: 1.0 / 2.0, // Equal split
                            first: Box::new(layout),
                            second: Box::new(row_layout),
                            rect: self.area,
                        };
                    }
                    
                    row_layouts.push(row_layout);
                } else if !col_layouts.is_empty() {
                    // Just a single column in this row
                    row_layouts.push(col_layouts.pop().unwrap());
                }
            }
            
            // If we have multiple rows, create a vertical split
            if row_layouts.len() > 1 {
                let mut layout = row_layouts.pop().unwrap();
                
                for row_layout in row_layouts.into_iter().rev() {
                    layout = LayoutNode::Split {
                        direction: SplitDirection::Vertical,
                        ratio: 1.0 / 2.0, // Equal split
                        first: Box::new(row_layout),
                        second: Box::new(layout),
                        rect: self.area,
                    };
                }
                
                self.layout = Some(layout);
            } else if !row_layouts.is_empty() {
                // Just a single row
                self.layout = Some(row_layouts.pop().unwrap());
            }
        }
        
        // Focus the first window
        if !window_ids.is_empty() {
            self.focus_window(window_ids[0])?;
        }
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
    
    // Apply a layout that maximizes the main window with smaller windows to the side
    pub fn apply_main_and_stack_layout(&mut self, main_window_id: Uuid, stack_window_ids: &[Uuid]) -> Result<()> {
        // Make sure the main window exists
        if !self.windows.contains_key(&main_window_id) {
            anyhow::bail!("Main window not found: {}", main_window_id);
        }
        
        // Make sure all stack windows exist
        for &id in stack_window_ids {
            if !self.windows.contains_key(&id) {
                anyhow::bail!("Window not found: {}", id);
            }
        }
        
        // If there are no stack windows, just use the main window
        if stack_window_ids.is_empty() {
            self.layout = Some(LayoutNode::window(main_window_id, self.area));
        } else {
            // Create the stack layout (a vertical column of windows)
            let mut stack_layout = LayoutNode::window(stack_window_ids[stack_window_ids.len() - 1], self.area);
            
            // Build the stack from bottom to top
            for i in (0..stack_window_ids.len() - 1).rev() {
                let id = stack_window_ids[i];
                
                stack_layout = LayoutNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio: 1.0 / (stack_window_ids.len() - i) as f32,
                    first: Box::new(LayoutNode::window(id, self.area)),
                    second: Box::new(stack_layout),
                    rect: self.area,
                };
            }
            
            // Create the main layout with the main window taking up 2/3 of the space
            self.layout = Some(LayoutNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.7, // Main window gets 70% of the width
                first: Box::new(LayoutNode::window(main_window_id, self.area)),
                second: Box::new(stack_layout),
                rect: self.area,
            });
        }
        
        // Focus the main window
        self.focus_window(main_window_id)?;
        
        // Recalculate the layout
        if let Some(layout) = &mut self.layout {
            layout.calculate_layout(self.area);
            self.apply_layout()?;
        }
        
        Ok(())
    }
}