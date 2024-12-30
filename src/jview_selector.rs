use std::{io, process::Command};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use ratatui::style;

fn get_style(selected: bool) -> style::Style {
    if selected {
        Style::default()
            .fg(Color::Cyan)
            .bg(Color::Black)
    } else {
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Blue)
    }
}

/// Creates a search widget for the application.
/// 
/// # Arguments
/// 
/// * `search_text` - Text to display in the search widget.
/// * `search_style` - The style to apply to the search widget.
///
/// # Returns
/// 
/// A `Paragraph` widget configured for the search functionality.
pub fn get_widget(text: &str, selected: bool) -> Paragraph<'_> {
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .style(get_style(selected))
}