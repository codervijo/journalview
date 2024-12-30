use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph},
};
use ratatui::style;

fn get_style() -> style::Style {
    Style::default().fg(Color::Green)
}

/// Creates a help widget for the application.
/// 
/// # Arguments
/// 
///
/// # Returns
/// 
/// A `Paragraph` widget configured for the search functionality.
pub fn get_widget() -> Paragraph<'static> {
    Paragraph::new("Help [Tab]: Switch Sections | [Up/Down/Left/Right]: Scroll | [q]: Quit")
        .block(Block::default().borders(Borders::ALL))
        .style(get_style())
}