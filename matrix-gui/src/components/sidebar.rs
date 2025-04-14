use iced::{Element, Point, Rectangle, Size, Color, Vector};
use iced::widget::{Container, Text};
use iced::mouse::Cursor;

use crate::styles::colors;

/// Sidebar icons
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SidebarIcon {
    NewWindow,
    SplitHorizontal,
    SplitVertical,
    GridLayout,
    HorizontalLayout,
    VerticalLayout,
    MainLayout,
    Zoom,
    CloseWindow,
    Help,
}

/// Messages sent from the sidebar
#[derive(Debug, Clone)]
pub enum SidebarMessage {
    IconClicked(SidebarIcon),
}

/// Definition of a sidebar icon
struct SidebarIconDef {
    icon: SidebarIcon,
    symbol: &'static str,
    tooltip: &'static str,
    shortcut: &'static str,
}

/// The sidebar component
pub struct Sidebar {
    // Configuration
    width: f32,
    visible: bool,
    
    // Interactive state
    hovered_icon: Option<SidebarIcon>,
    
    // Icons
    icons: Vec<SidebarIconDef>,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        let icons = vec![
            SidebarIconDef {
                icon: SidebarIcon::NewWindow,
                symbol: "N",
                tooltip: "New Window",
                shortcut: "Ctrl+N",
            },
            SidebarIconDef {
                icon: SidebarIcon::SplitHorizontal,
                symbol: "H",
                tooltip: "Split Horizontal",
                shortcut: "Ctrl+H",
            },
            SidebarIconDef {
                icon: SidebarIcon::SplitVertical,
                symbol: "V",
                tooltip: "Split Vertical",
                shortcut: "Ctrl+V",
            },
            SidebarIconDef {
                icon: SidebarIcon::GridLayout,
                symbol: "G",
                tooltip: "Grid Layout",
                shortcut: "Ctrl+G",
            },
            SidebarIconDef {
                icon: SidebarIcon::HorizontalLayout,
                symbol: "=",
                tooltip: "Horizontal Layout",
                shortcut: "Ctrl+Shift+H",
            },
            SidebarIconDef {
                icon: SidebarIcon::VerticalLayout,
                symbol: "‖",
                tooltip: "Vertical Layout",
                shortcut: "Ctrl+Shift+V",
            },
            SidebarIconDef {
                icon: SidebarIcon::MainLayout,
                symbol: "M",
                tooltip: "Main Layout",
                shortcut: "Ctrl+M",
            },
            SidebarIconDef {
                icon: SidebarIcon::Zoom,
                symbol: "Z",
                tooltip: "Zoom Window",
                shortcut: "Ctrl+Z",
            },
            SidebarIconDef {
                icon: SidebarIcon::CloseWindow,
                symbol: "X",
                tooltip: "Close Window",
                shortcut: "Ctrl+W",
            },
            SidebarIconDef {
                icon: SidebarIcon::Help,
                symbol: "?",
                tooltip: "Help",
                shortcut: "F1",
            },
        ];
        
        Self {
            width: 30.0,
            visible: true,
            hovered_icon: None,
            icons,
        }
    }
    
    /// Get the width of the sidebar
    pub fn width(&self) -> f32 {
        if self.visible {
            self.width
        } else {
            0.0
        }
    }
    
    /// Toggle sidebar visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
    
    /// Handle hover events
    pub fn handle_hover(&mut self, position: Point) {
        if !self.visible || position.x > self.width {
            self.hovered_icon = None;
            return;
        }
        
        // Calculate which icon was hovered
        let icon_height = 30.0;
        let icon_index = (position.y / icon_height) as usize;
        
        self.hovered_icon = if icon_index < self.icons.len() {
            Some(self.icons[icon_index].icon)
        } else {
            None
        };
    }
    
    /// Render the sidebar
    pub fn view<'a>(&self) -> Element<'a, SidebarMessage> {
        if !self.visible {
            // Return an empty element if not visible
            return Container::new(Text::new(""))
                .width(iced::Length::Fixed(0.0))
                .height(iced::Length::Fill)
                .into();
        }
        
        // For now, this is just a placeholder
        // A real implementation would render each icon and handle interactions
        Container::new(Text::new(""))
            .width(iced::Length::Fixed(self.width))
            .height(iced::Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(|_theme| {
                iced::widget::container::Appearance {
                    text_color: None,
                    background: Some(iced::Background::Color(colors::BACKGROUND)),
                    border_radius: 0.0,
                    border_width: 1.0,
                    border_color: colors::BORDER,
                }
            })))
            .into()
    }
}

/// Custom sidebar rendering (will be implemented with iced canvas in a full implementation)
struct SidebarRenderer {
    // State for rendering
    icons: Vec<SidebarIconDef>,
    hovered: Option<SidebarIcon>,
    width: f32,
    icon_height: f32,
}

impl SidebarRenderer {
    fn new(icons: Vec<SidebarIconDef>, hovered: Option<SidebarIcon>, width: f32) -> Self {
        Self {
            icons,
            hovered,
            width,
            icon_height: 30.0,
        }
    }
    
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<iced::widget::canvas::Geometry> {
        // This would be used in a full implementation with iced::widget::canvas
        // For now it's just a sketch of what would be included
        vec![]
    }
}