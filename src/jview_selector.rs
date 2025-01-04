use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem},
};
use crossterm::event::{self, Event, KeyCode};
use std::process::Command;
use crate::jview_config;
use crate::jview_config::settings;
use crate::jview_debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JviewSelector {
    selected_idx: usize,
    vertical_start: usize,
    horizontal_start: usize,
    max_viewer_height: usize,
    units: Vec<String>,
}

impl JviewSelector {
    pub fn new() -> Self {
        JviewSelector {
            selected_idx: 0,
            vertical_start: 0,
            horizontal_start: 0,
            max_viewer_height: 15,
            units: fetch_systemd_units(),
        }
    }

    pub fn set_max_height(&mut self, h: usize) {
        self.max_viewer_height = h;
    }

    fn get_visible_units(&self) -> Vec<String> {
        let mut vitems: Vec<String> = Vec::new(); // Viewable units

        for (i, line) in self.units.iter().enumerate() {
            if i < self.vertical_start {
                continue; // Skip lines until the vertical offset
            }

            if vitems.len() >= self.max_viewer_height {
                break; // Stop if we've taken enough lines to fit the section
            }

            let visible_line = if line.len() > self.horizontal_start {
                &line[self.horizontal_start..]
            } else {
                ""
            };

            vitems.push(visible_line.to_string());
        }

        vitems
    }


    /// Navigate the list vertically based on user input.
    ///
    /// # Arguments
    ///
    pub fn navigate(&mut self) -> Result<KeyCode, std::io::Error> {

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(KeyCode::Char('q')),
                KeyCode::Char('Q') => return Ok(KeyCode::Char('q')),
                KeyCode::Tab => {
                    return Ok(KeyCode::Tab);
                }
                KeyCode::Enter => {
                    settings::set_unit(&self.units[self.selected_idx]);
                    jview_debug::log_debug_info("Selected Unit ID to filter: ", format_args!("{}", self.selected_idx));
                    jview_debug::log_debug_info("Selected Unit to filter:", format_args!("{}", self.units[self.selected_idx]));
                    return Ok(KeyCode::Tab);
                }
                KeyCode::Up => {
                    if self.vertical_start > 0 {
                        self.vertical_start -= 1;
                    }
                    if self.selected_idx > 0 {
                        self.selected_idx -= 1;
                    }
                    jview_debug::log_debug_info("Clearing unit 1", format_args!("{:?}", key.code));
                    settings::clear_unit();
                }
                KeyCode::Down => {
                    self.selected_idx += 1;
                    if self.selected_idx >= self.max_viewer_height-4 {
                        self.vertical_start += 1;
                    }
                    jview_debug::log_debug_info("Clearing unit 2", format_args!("{:?}", key.code));
                    settings::clear_unit();
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

    /// Creates a selector widget for the application.
    ///
    /// # Arguments
    ///
    /// * `selected` - Is this widget currently selected?
    ///
    /// # Returns
    ///
    /// A `List` widget configured for the systemd units.
    pub fn get_selector_widget(&self, selected: bool) -> List<'static> {
        let vunits = self.get_visible_units();
        let items: Vec<ListItem> = vunits
            .into_iter()
            .enumerate()
            .map(|(i, unit)| {
                let style = if (i + self.vertical_start) == self.selected_idx {
                    Style::default().fg(Color::Black).bg(Color::Cyan)
                } else {
                    get_style(selected)
                };
                if settings::get_unit() == unit {
                    let su = format!("\u{2714} {}", unit);
                    ListItem::new(su).style(style)
                } else if (i+self.vertical_start) == self.selected_idx {
                    let su = format!("\u{2713} {}", unit);
                    ListItem::new(su).style(style)
                } else {
                    ListItem::new(unit).style(style)
                }
            })
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Systemd Units"),
            )
            .style(get_style(selected))
    }
}

fn fetch_systemd_units() -> Vec<String> {
    let output = Command::new("bash")
        .args(["-c", "systemctl list-units --all --no-pager --plain | awk '{print $1}'|grep '.service'"])
        .output()
        .expect("Failed to run systemctl command");
    /*
        Use this in the future and use json output
        systemctl list-units --all --plain --no-legend --no-pager --output=json
    */
    if output.status.success() {
        let mut result = Vec::new();

        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.contains(".service") {
                result.push(line.replace(".service", ""));
            } else {
                result.push(line.to_string())
            }
        }
        result
    } else {
        vec!["<All Systemd units>".to_string()]
    }
}

fn get_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan).bg(Color::Black)
    } else {
        Style::default().fg(Color::Yellow).bg(Color::Blue)
    }
}
