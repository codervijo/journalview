use std::{io, process::Command};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use ratatui::style;

fn get_style() -> style::Style {
    Style::default().fg(Color::Green)
}

/// Creates a help widget for the application.
/// 
/// # Arguments
/// 
/// * `search_text` - Text to display in the search widget.
/// * `search_style` - The style to apply to the search widget.
///
/// # Returns
/// 
/// A `Paragraph` widget configured for the search functionality.
pub fn get_widget() -> Paragraph<'static> {
    Paragraph::new("Help [Tab]: Switch Sections | [Up/Down/Left/Right]: Scroll | [q]: Quit")
        .block(Block::default().borders(Borders::ALL))
        .style(get_style())
}