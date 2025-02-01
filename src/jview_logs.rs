use std::{process::Command};
use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem},
};
use ratatui::style;
use crossterm::event::{self, Event, KeyCode};
#[allow(unused_imports)]
use crate::jview_config;
use crate::jview_config::settings;
use crate::jview_debug;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct JviewLogs {
    vertical_start: usize,
    horizontal_start: usize,
    max_viewer_height: usize,
    max_viewer_width: usize,
}

impl JviewLogs {
    pub fn new() -> Self {
        JviewLogs {
            vertical_start: 0,
            horizontal_start: 0,
            max_viewer_height: 25,
            max_viewer_width: 25,
        }
    }

    pub fn set_max_height(&mut self, h: usize) {
        self.max_viewer_height = h;
    }
}

pub fn fetch_journalctl_logs() -> Vec<String> {
    let chosen = settings::get_unit();
    let mut jargs = vec![];

    if !chosen.is_empty() {
        jview_debug::log_debug_info("Found Selected Unit to filter:", format_args!("{}", chosen));
        jargs.push("-u".to_string());
        jargs.push(chosen);
    } else {
        jargs.push("--since=yesterday".to_string());
    }

    jview_debug::log_debug_info("Doing command:", format_args!("{:?}", jargs));
    let output = Command::new("journalctl")
        .args(&jargs)
        .output()
        .expect("Failed to run journalctl command");

    if output.status.success() {
        //jview_debug::log_debug_info("Command output: ", format_args!("{}", String::from_utf8_lossy(&output.stdout)));
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

impl JviewLogs {
    fn get_log_items(self, selected: bool) -> Vec<ListItem<'static>> {
        let logs = fetch_journalctl_logs();
        let mut log_items: Vec<ListItem> = Vec::new(); // Viewable

        for (i, line) in logs.iter().enumerate() {
            if i < self.vertical_start {
                continue; // Skip lines until the vertical offset
            }

            if log_items.len() >= self.max_viewer_height {
                break; // Stop if we've taken enough lines to fit the section
            }

            let visible_line = if line.len() > self.horizontal_start {
                &line[self.horizontal_start..]
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
                KeyCode::Char('q') => return Ok(KeyCode::Char('q')),
                KeyCode::Char('Q') => return Ok(KeyCode::Char('q')),
                KeyCode::Tab => {
                    return Ok(KeyCode::Tab);
                }
                KeyCode::Up => {
                    if self.vertical_start > 0 {
                        self.vertical_start -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.vertical_start < logs.len() {
                        self.vertical_start += 1;
                    }
                }
                KeyCode::Left => {
                    if self.horizontal_start > 0 {
                        self.horizontal_start -= 1;
                    }
                }
                KeyCode::Right => {
                    self.horizontal_start += 1;
                }
                _ => {}
            }
        }

        Ok(KeyCode::Enter)
    }
}