use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List},
};
use ratatui::widgets::ListItem;
use ratatui::style;
use std::{process::Command};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct JviewSelector {
    vertical_offset: usize,
    horizontal_offset: usize,
    max_viewer_height: usize,
}

impl JviewSelector {
    pub fn new() -> Self {
        JviewSelector {
            vertical_offset: 0,
            horizontal_offset: 0,
            max_viewer_height: 25,
        }
    }

    pub fn set_max_height(&mut self, h: usize) {
        self.max_viewer_height = h;
    }
}

fn fetch_systemd_units() -> Vec<String> {
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

impl JviewSelector {
    /// Creates a search widget for the application.
    ///
    /// # Arguments
    ///
    /// * `selected` - Is this widget currently selected?
    ///
    /// # Returns
    ///
    /// A `List` widget configured for the search functionality.
    pub fn get_selector_widget(&self, selected: bool) -> List<'static> {
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
}