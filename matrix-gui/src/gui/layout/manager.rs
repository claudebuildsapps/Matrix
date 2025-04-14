use uuid::Uuid;
use iced::Rectangle;
use std::collections::HashMap;

/// Direction to split a window
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Horizontal, // Split horizontally (side by side)
    Vertical,   // Split vertically (one above the other)
}

/// A node in the layout tree
#[derive(Debug, Clone)]
pub enum LayoutNode {
    Window(Uuid),
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
}

/// Layout manager that organizes windows in a tree structure
pub struct LayoutManager {
    // The root of the layout tree
    root: Option<LayoutNode>,
    
    // Pre-zoom state for when we zoom a window
    pre_zoom_layout: Option<LayoutNode>,
    
    // The currently zoomed window, if any
    zoomed_window: Option<Uuid>,
    
    // Calculated rectangles for each window
    window_rects: HashMap<Uuid, Rectangle>,
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new() -> Self {
        Self {
            root: None,
            pre_zoom_layout: None,
            zoomed_window: None,
            window_rects: HashMap::new(),
        }
    }
    
    /// Add a window to the layout
    pub fn add_window(&mut self, window_id: Uuid) {
        // If there's no root, this becomes the root
        if self.root.is_none() {
            self.root = Some(LayoutNode::Window(window_id));
            return;
        }
        
        // Otherwise, we need to find a place to add this window
        // For now, we'll just replace the root with a split
        // containing the old root and the new window
        if let Some(old_root) = self.root.take() {
            self.root = Some(LayoutNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.5,
                first: Box::new(old_root),
                second: Box::new(LayoutNode::Window(window_id)),
            });
        }
    }
    
    /// Remove a window from the layout
    pub fn remove_window(&mut self, window_id: &Uuid) {
        if let Some(root) = &self.root {
            // If we're removing the root and it's a window, just clear the root
            if let LayoutNode::Window(id) = root {
                if id == window_id {
                    self.root = None;
                    return;
                }
            }
            
            // Otherwise, we need to restructure the tree
            if let Some(new_root) = self.remove_from_node(root, window_id) {
                self.root = Some(new_root);
            }
        }
    }
    
    /// Helper to remove a window from a node
    fn remove_from_node(&self, node: &LayoutNode, window_id: &Uuid) -> Option<LayoutNode> {
        match node {
            LayoutNode::Window(id) => {
                if id == window_id {
                    // This is the window to remove, return None
                    None
                } else {
                    // Not the window to remove, keep it
                    Some(node.clone())
                }
            },
            LayoutNode::Split { direction, ratio, first, second, .. } => {
                // Try to remove from the first child
                let new_first = self.remove_from_node(first, window_id);
                
                // Try to remove from the second child
                let new_second = self.remove_from_node(second, window_id);
                
                match (new_first, new_second) {
                    // Both children remain, create a new split with them
                    (Some(first), Some(second)) => {
                        Some(LayoutNode::Split {
                            direction: *direction,
                            ratio: *ratio,
                            first: Box::new(first),
                            second: Box::new(second),
                        })
                    },
                    // Only the first child remains, return it directly
                    (Some(first), None) => Some(first),
                    // Only the second child remains, return it directly
                    (None, Some(second)) => Some(second),
                    // Both children were removed, return None
                    (None, None) => None,
                }
            }
        }
    }
    
    /// Split a window
    pub fn split_window(&mut self, window_id: &Uuid, direction: SplitDirection, 
                      new_window_id: Uuid, ratio: f32) {
        if let Some(root) = &self.root {
            if let Some(new_root) = self.split_in_node(root, window_id, 
                                                     direction, new_window_id, ratio) {
                self.root = Some(new_root);
            }
        }
    }
    
    /// Helper to split a window in a node
    fn split_in_node(&self, node: &LayoutNode, window_id: &Uuid, 
                   direction: SplitDirection, new_window_id: Uuid, 
                   ratio: f32) -> Option<LayoutNode> {
        match node {
            LayoutNode::Window(id) => {
                if id == window_id {
                    // This is the window to split
                    Some(LayoutNode::Split {
                        direction,
                        ratio,
                        first: Box::new(LayoutNode::Window(*id)),
                        second: Box::new(LayoutNode::Window(new_window_id)),
                    })
                } else {
                    // Not the window to split, keep it
                    Some(node.clone())
                }
            },
            LayoutNode::Split { direction: split_dir, ratio: split_ratio, 
                             first, second, .. } => {
                // Try to split in the first child
                let new_first = self.split_in_node(first, window_id, 
                                                 direction, new_window_id, ratio);
                
                // If first child was split, create a new split with it
                if let Some(new_first) = new_first {
                    if !matches!(&**first, LayoutNode::Window(id) if id == window_id) {
                        return Some(LayoutNode::Split {
                            direction: *split_dir,
                            ratio: *split_ratio,
                            first: Box::new(new_first),
                            second: second.clone(),
                        });
                    }
                }
                
                // Try to split in the second child
                let new_second = self.split_in_node(second, window_id, 
                                                  direction, new_window_id, ratio);
                
                // If second child was split, create a new split with it
                if let Some(new_second) = new_second {
                    Some(LayoutNode::Split {
                        direction: *split_dir,
                        ratio: *split_ratio,
                        first: first.clone(),
                        second: Box::new(new_second),
                    })
                } else {
                    // Neither child was split, keep the original
                    Some(node.clone())
                }
            }
        }
    }
    
    /// Zoom in on a window
    pub fn zoom_window(&mut self, window_id: &Uuid) {
        // Save the current layout
        if let Some(root) = &self.root {
            self.pre_zoom_layout = Some(root.clone());
            self.zoomed_window = Some(*window_id);
            
            // Set the root to just the zoomed window
            self.root = Some(LayoutNode::Window(*window_id));
        }
    }
    
    /// Restore from zoom
    pub fn unzoom(&mut self) {
        if let Some(layout) = self.pre_zoom_layout.take() {
            self.root = Some(layout);
            self.zoomed_window = None;
        }
    }
    
    /// Calculate layouts for all windows
    pub fn calculate_layout(&mut self, area: Rectangle) {
        // Clear the current layout
        self.window_rects.clear();
        
        // Calculate the layout if we have a root
        if let Some(root) = &self.root {
            self.calculate_node_layout(root, area);
        }
    }
    
    /// Helper to calculate layout for a node
    fn calculate_node_layout(&mut self, node: &LayoutNode, area: Rectangle) {
        match node {
            LayoutNode::Window(id) => {
                // Store the rectangle for this window
                self.window_rects.insert(*id, area);
            },
            LayoutNode::Split { direction, ratio, first, second, .. } => {
                // Split the area according to the direction and ratio
                let (first_area, second_area) = match direction {
                    SplitDirection::Horizontal => {
                        // Split horizontally (side by side)
                        let width = area.width;
                        let first_width = (width * ratio).round();
                        let second_width = width - first_width;
                        
                        (
                            Rectangle {
                                x: area.x,
                                y: area.y,
                                width: first_width,
                                height: area.height,
                            },
                            Rectangle {
                                x: area.x + first_width,
                                y: area.y,
                                width: second_width,
                                height: area.height,
                            },
                        )
                    },
                    SplitDirection::Vertical => {
                        // Split vertically (one above the other)
                        let height = area.height;
                        let first_height = (height * ratio).round();
                        let second_height = height - first_height;
                        
                        (
                            Rectangle {
                                x: area.x,
                                y: area.y,
                                width: area.width,
                                height: first_height,
                            },
                            Rectangle {
                                x: area.x,
                                y: area.y + first_height,
                                width: area.width,
                                height: second_height,
                            },
                        )
                    },
                };
                
                // Calculate layout for the children
                self.calculate_node_layout(first, first_area);
                self.calculate_node_layout(second, second_area);
            }
        }
    }
    
    /// Get the rectangle for a window
    pub fn get_window_rect(&self, window_id: &Uuid) -> Option<Rectangle> {
        self.window_rects.get(window_id).copied()
    }
    
    /// Get all window rectangles
    pub fn get_window_rects(&self) -> &HashMap<Uuid, Rectangle> {
        &self.window_rects
    }
    
    /// Check if a window is zoomed
    pub fn is_zoomed(&self, window_id: &Uuid) -> bool {
        self.zoomed_window == Some(*window_id)
    }
}