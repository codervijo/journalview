use std::{process::Command};
use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem},
};
use ratatui::style;
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct JviewLogs {
    vertical_offset: usize,
    horizontal_offset: usize,
    max_viewer_height: usize,
}

impl JviewLogs {
    pub fn new() -> Self {
        JviewLogs {
            vertical_offset: 0,
            horizontal_offset: 0,
            max_viewer_height: 25,
        }
    }

    pub fn set_max_height(&mut self, h: usize) {
        self.max_viewer_height = h;
    }
}

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

impl JviewLogs {
    pub fn get_log_items(self, selected: bool) -> Vec<ListItem<'static>> {
        let logs = fetch_journalctl_logs();
        let mut log_items: Vec<ListItem> = Vec::new();

        for (i, line) in logs.iter().enumerate() {
            if i < self.vertical_offset {
                continue; // Skip lines until the vertical offset
            }

            if log_items.len() >= self.max_viewer_height {
                break; // Stop if we've taken enough lines to fit the section
            }

            let visible_line = if line.len() > self.horizontal_offset {
                &line[self.horizontal_offset..]
            } else {
                ""
            };

            let style = get_style(selected);

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
    pub fn get_logs_widget<'b>(&self, selected: bool) -> List<'b> {
        let logitems: Vec<ListItem> = (*self.get_log_items(selected)).to_vec();

        List::new(logitems)
            .block(Block::default().borders(Borders::ALL).title("Logs"))
            .style(get_style(selected))
    }


    pub fn logs_navigate(&mut self) -> Result<KeyCode, std::io::Error> {
        let logs = fetch_journalctl_logs();

        if let Event::Key(key) = event::read()? {
            match key.code {
                //KeyCode::Char('q') => return Ok(true),
                //KeyCode::Char('Q') => return Ok(true),
                KeyCode::Tab => {
                    return Ok(KeyCode::Tab);
                }
                KeyCode::Up => {
                    if self.vertical_offset > 0 {
                        self.vertical_offset -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.vertical_offset < logs.len() {
                        self.vertical_offset += 1;
                    }
                }
                KeyCode::Left => {
                    if self.horizontal_offset > 0 {
                        self.horizontal_offset -= 1;
                    }
                }
                KeyCode::Right => {
                    self.horizontal_offset += 1;
                }
                _ => {}
            }
        }

        Ok(KeyCode::Enter)
    }
}