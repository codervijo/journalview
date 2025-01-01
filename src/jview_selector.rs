use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List},
};
use ratatui::widgets::ListItem;
use ratatui::style;
use std::{process::Command};

pub fn fetch_systemd_units() -> Vec<String> {
    let output = Command::new("bash")
    .args(["-c", "systemctl list-units --all --no-pager --plain | awk '{print $1}'"])
    .output()
        .expect("Failed to run systemctl command");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.to_string())
            .collect()
    } else {
        vec!["<All Systemd units>".to_string()]
    }
}

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
/// * `selected` - Is this widget currently selected?
///
/// # Returns
/// 
/// A `List` widget configured for the search functionality.
pub fn get_widget(selected: bool) -> List<'static> {
    let units = fetch_systemd_units();
    let items: Vec<ListItem> = units
        .into_iter()
        .map(|unit| ListItem::new(unit))
        .collect();

    List::new(items)
        .block(Block::default()
        .borders(Borders::ALL)
        .title("Systemd Units"))
        .style(get_style(selected))
}