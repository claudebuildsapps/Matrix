use ratatui::layout::{Rect, Layout, Direction, Constraint};
use ratatui::style::{Style, Color, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph, BorderType};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::Frame;

// Define the possible icons for the sidebar
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

// Define the sidebar structure
pub struct Sidebar {
    // Currently hovered icon
    hovered: Option<SidebarIcon>,
    // Width of the sidebar
    width: u16,
    // Is sidebar active
    active: bool,
}

impl Sidebar {
    // Create a new sidebar
    pub fn new() -> Self {
        Self {
            hovered: None,
            width: 3, // Very narrow sidebar
            active: true,
        }
    }
    
    // Set the hovered icon
    pub fn set_hover(&mut self, icon: Option<SidebarIcon>) {
        self.hovered = icon;
    }
    
    // Get the icon at a particular position
    pub fn icon_at_position(&self, y: u16) -> Option<SidebarIcon> {
        // Calculate which icon was hovered based on y position
        // Each icon takes 3 rows (icon + small gap)
        let icon_index = y / 3;
        
        match icon_index {
            0 => Some(SidebarIcon::NewWindow),
            1 => Some(SidebarIcon::SplitHorizontal),
            2 => Some(SidebarIcon::SplitVertical),
            3 => Some(SidebarIcon::GridLayout),
            4 => Some(SidebarIcon::HorizontalLayout),
            5 => Some(SidebarIcon::VerticalLayout),
            6 => Some(SidebarIcon::MainLayout),
            7 => Some(SidebarIcon::Zoom),
            8 => Some(SidebarIcon::CloseWindow),
            9 => Some(SidebarIcon::Help),
            _ => None,
        }
    }
    
    // Get the width of the sidebar
    pub fn width(&self) -> u16 {
        self.width
    }
    
    // Toggle the sidebar
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
    
    // Is sidebar active
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    // Render the sidebar
    pub fn render(&self, f: &mut Frame, area: Rect) {
        if !self.active {
            return;
        }
        
        // Create a thin vertical area for the sidebar
        let sidebar_area = Rect::new(area.x, area.y, self.width, area.height);
        
        // Draw the sidebar background
        let block = Block::default()
            .borders(Borders::RIGHT)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray));
        
        f.render_widget(block, sidebar_area);
        
        // Render each icon
        let icon_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3), // NewWindow
                Constraint::Length(3), // SplitHorizontal
                Constraint::Length(3), // SplitVertical
                Constraint::Length(3), // GridLayout
                Constraint::Length(3), // HorizontalLayout
                Constraint::Length(3), // VerticalLayout
                Constraint::Length(3), // MainLayout
                Constraint::Length(3), // Zoom
                Constraint::Length(3), // CloseWindow
                Constraint::Length(3), // Help
                Constraint::Min(0),   // Remaining space
            ])
            .split(sidebar_area);
        
        // Render the icons
        self.render_icon(f, icon_areas[0], "N", SidebarIcon::NewWindow);
        self.render_icon(f, icon_areas[1], "H", SidebarIcon::SplitHorizontal);
        self.render_icon(f, icon_areas[2], "V", SidebarIcon::SplitVertical);
        self.render_icon(f, icon_areas[3], "G", SidebarIcon::GridLayout);
        self.render_icon(f, icon_areas[4], "=", SidebarIcon::HorizontalLayout);
        self.render_icon(f, icon_areas[5], "‖", SidebarIcon::VerticalLayout);
        self.render_icon(f, icon_areas[6], "M", SidebarIcon::MainLayout);
        self.render_icon(f, icon_areas[7], "Z", SidebarIcon::Zoom);
        self.render_icon(f, icon_areas[8], "X", SidebarIcon::CloseWindow);
        self.render_icon(f, icon_areas[9], "?", SidebarIcon::Help);
        
        // If an icon is hovered, show the tooltip
        if let Some(hovered_icon) = self.hovered {
            self.render_tooltip(f, area, hovered_icon);
        }
    }
    
    // Render an individual icon
    fn render_icon(&self, f: &mut Frame, area: Rect, icon: &str, icon_type: SidebarIcon) {
        // Define the Matrix green color
        let matrix_green = Color::Rgb(0, 255, 65);
        let darker_green = Color::Rgb(0, 180, 45);
        
        // Set the style based on whether this icon is hovered
        let style = if self.hovered == Some(icon_type) {
            Style::default().fg(matrix_green).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(darker_green)
        };
        
        // Create the icon text
        let text = Line::from(vec![
            Span::styled(icon, style),
        ]);
        
        // Render the icon
        let icon_widget = Paragraph::new(vec![text])
            .alignment(ratatui::layout::Alignment::Center);
        
        f.render_widget(icon_widget, area);
    }
    
    // Render the tooltip for a hovered icon
    fn render_tooltip(&self, f: &mut Frame, area: Rect, icon: SidebarIcon) {
        // Matrix green
        let matrix_green = Color::Rgb(0, 255, 65);
        
        // Define tooltip content based on the icon
        let (title, description, shortcut) = match icon {
            SidebarIcon::NewWindow => (
                "New Window",
                "Create a new terminal window",
                "Ctrl+N or :new"
            ),
            SidebarIcon::SplitHorizontal => (
                "Split Horizontal",
                "Split current window horizontally",
                "Ctrl+H or :split h"
            ),
            SidebarIcon::SplitVertical => (
                "Split Vertical",
                "Split current window vertically",
                "Ctrl+V or :split"
            ),
            SidebarIcon::GridLayout => (
                "Grid Layout",
                "Arrange windows in a grid pattern",
                "Ctrl+G or :layout grid"
            ),
            SidebarIcon::HorizontalLayout => (
                "Horizontal Layout",
                "Arrange windows horizontally",
                "Ctrl+Shift+H or :layout h"
            ),
            SidebarIcon::VerticalLayout => (
                "Vertical Layout",
                "Arrange windows vertically",
                "Ctrl+Shift+V or :layout v"
            ),
            SidebarIcon::MainLayout => (
                "Main Layout",
                "Show current window as main with others stacked",
                "Ctrl+M or :layout main"
            ),
            SidebarIcon::Zoom => (
                "Zoom Window",
                "Toggle zoom on current window",
                "Ctrl+Z or :zoom"
            ),
            SidebarIcon::CloseWindow => (
                "Close Window",
                "Close the current window",
                "Ctrl+W or :close"
            ),
            SidebarIcon::Help => (
                "Help",
                "Show help information",
                ":help"
            ),
        };
        
        // Create an area for the tooltip - right next to the sidebar
        let tooltip_area = Rect::new(
            area.x + self.width + 1,
            area.y + 1,
            40, // Width of tooltip
            7,  // Height of tooltip
        );
        
        // Create a block for the tooltip
        let tooltip_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(matrix_green))
            .style(Style::default().bg(Color::Black));
        
        // Create styled text
        let title_line = Line::from(vec![
            Span::styled(title, Style::default().fg(matrix_green).add_modifier(Modifier::BOLD))
        ]);
        
        let description_line = Line::from(vec![
            Span::styled(description, Style::default().fg(matrix_green))
        ]);
        
        let shortcut_line = Line::from(vec![
            Span::styled("Shortcut: ", Style::default().fg(matrix_green).add_modifier(Modifier::BOLD)),
            Span::styled(shortcut, Style::default().fg(matrix_green))
        ]);
        
        // Combine text into a paragraph
        let tooltip_text = vec![
            Line::from(""),  // Empty line for padding
            title_line,
            Line::from(""),  // Empty line for spacing
            description_line, 
            Line::from(""),  // Empty line for spacing
            shortcut_line,
        ];
        
        // Create the paragraph
        let tooltip = Paragraph::new(tooltip_text)
            .block(tooltip_block)
            .alignment(ratatui::layout::Alignment::Left);
        
        // Render the tooltip
        f.render_widget(tooltip, tooltip_area);
    }
}