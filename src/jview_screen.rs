use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::Paragraph;

use crate::jview_logs;
use crate::jview_search;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiSection {
    Search,
    Logs,
    Selector,
    Help,
}

impl UiSection {
    // Get the next variant, returning None when reaching the end
    pub fn next(&self) -> UiSection {
        match self {
            UiSection::Search => UiSection::Logs,
            UiSection::Logs => UiSection::Selector,
            UiSection::Selector => UiSection::Search,
            &UiSection::Help => todo!(),
        }
    }
}

pub struct UiScreen {
    selected: UiSection,
    search_tui: jview_search::JviewSearch,
}

impl UiScreen {
    pub fn new() -> Self {
        UiScreen {
            selected: UiSection::Logs,
            search_tui: jview_search::JviewSearch::new(),
        }
    }

    pub fn next_section(&mut self) {
        self.selected = self.selected.next();
    }

    pub fn get_selected(&self) -> UiSection {
        self.selected
    }

    pub fn get_search_widget(&self, selected: bool) -> Paragraph<'static> {
        self.search_tui.clone().get_search_widget(selected)
    }
}

pub fn screen_navigate(screen: &mut UiScreen) -> Result<bool, std::io::Error> {
    let mut vertical_offset = 0;
    let mut horizontal_offset = 0;
    let logs = jview_logs::fetch_journalctl_logs();

    if screen.get_selected() == UiSection::Search {
        let res = screen.search_tui.get_search_input()?;
        if res == KeyCode::Tab {
            screen.next_section();
            return Ok(false);
        }
    }

    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('Q') => return Ok(true),
            KeyCode::Tab => {
                //selected_section = selected_section.next();
                screen.next_section();
            }
            KeyCode::Up => {
                if screen.get_selected() == UiSection::Logs && vertical_offset > 0 {
                    vertical_offset -= 1;
                }
            }
            KeyCode::Down => {
                if screen.get_selected() == UiSection::Logs && vertical_offset < logs.len() {
                    vertical_offset += 1;
                }
            }
            KeyCode::Left => {
                if screen.get_selected() == UiSection::Logs && horizontal_offset > 0 {
                    horizontal_offset -= 1;
                }
            }
            KeyCode::Right => {
                if screen.get_selected() == UiSection::Logs {
                    horizontal_offset += 1;
                }
            }
            _ => {}
        }
    }

    Ok(false)
}