use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::text::Text;
use ratatui::style::Style;

pub fn create_block(title: &str, style: Style) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(style)
}

pub fn create_paragraph<'a>(content: &'a str, style: Style) -> Paragraph<'a> {
    let text = Text::from(content);
    Paragraph::new(text)
        .style(style)
        .wrap(Wrap { trim: true })
}
