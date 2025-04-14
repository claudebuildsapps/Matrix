use ratatui::layout::{Layout, Constraint, Direction, Rect};

pub fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area)
        .to_vec()
}