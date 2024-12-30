use std::{io, process::Command};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use ratatui::style;

pub fn fetch_journalctl_logs() -> Vec<String> {
    let output = Command::new("journalctl")
        .arg("--since=yesterday")
        .output()
        .expect("Failed to run journalctl command");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect()
    } else {
        vec!["Error fetching logs".to_string()]
    }
}

pub fn get_style(selected: bool) -> style::Style {
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

pub fn get_section_style(selected: bool) -> style::Style {
    if selected {
        Style::default()
            .fg(Color::Cyan)
            .bg(Color::Black)
    } else {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
    }
}

pub fn get_log_items(vertical_offset: usize, maxviewer: usize, horizontal_offset: usize, selected_section: bool) -> Vec<ListItem<'static>> {
    let logs = fetch_journalctl_logs();
    let mut log_items: Vec<ListItem> = Vec::new();

    for (i, line) in logs.iter().enumerate() {
        if i < vertical_offset {
            continue; // Skip lines until the vertical offset
        }
    
        if log_items.len() >= maxviewer {
            break; // Stop if we've taken enough lines to fit the section
        }

        let visible_line = if line.len() > horizontal_offset {
            &line[horizontal_offset..]
        } else {
            ""
        };

        let style = get_style(selected_section);

        log_items.push(ListItem::new(visible_line.to_string()).style(style));
    }

    log_items
}

/// Creates a configurable widget for displaying a list of items.
/// 
/// # Arguments
/// 
/// * `items` - A list of items to display in the widget.
/// * `selected` - 
/// 
/// # Returns
/// 
/// A `List` widget configured with the provided parameters.
pub fn get_log_widget<'a>(items: Vec<ListItem<'a>>, selected: bool) -> List<'a> {
    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Journalctl Logs"))
        .style(get_section_style(selected))
}
