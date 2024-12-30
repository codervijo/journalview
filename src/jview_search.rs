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
/// * `search_style` - The style to apply to the search widget.
///
/// # Returns
/// 
/// A `Paragraph` widget configured for the search functionality.
pub fn get_search_widget(selected: bool) -> Paragraph<'static> {
    Paragraph::new("Search text")
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(get_style(selected))
}
