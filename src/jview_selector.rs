use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem},
};
use crossterm::event::{self, Event, KeyCode};
use std::{cmp, process::Command};

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

    /// Navigate the list vertically based on user input.
    ///
    /// # Arguments
    ///
    /// * `direction` - Movement direction, `1` for down, `-1` for up.
    /// * `list_len` - Total number of items in the list.
    pub fn navigate(&mut self) -> Result<KeyCode, std::io::Error> {
        let list_len = 20;//list_len as isize;
        //let new_offset = self.vertical_offset as isize + direction;

        // Wrap around or clamp the offset within the valid range
       // self.vertical_offset = cmp::max(0, cmp::min(list_len - 1, new_offset)) as usize;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(KeyCode::Char('q')),
                KeyCode::Char('Q') => return Ok(KeyCode::Char('q')),
                KeyCode::Tab => {
                    return Ok(KeyCode::Tab);
                }
                KeyCode::Up => {
                    if self.vertical_offset > 0 {
                        self.vertical_offset -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.vertical_offset < list_len {
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
        let units = fetch_systemd_units();
        let items: Vec<ListItem> = units
            .into_iter()
            .enumerate()
            .map(|(i, unit)| {
                let style = if i == self.vertical_offset {
                    Style::default().fg(Color::Black).bg(Color::Cyan)
                } else {
                    get_style(selected)
                };
                ListItem::new(unit).style(style)
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

fn get_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan).bg(Color::Black)
    } else {
        Style::default().fg(Color::Yellow).bg(Color::Blue)
    }
}
