use crossterm::event::KeyCode;
use ratatui::widgets::Paragraph;
use ratatui::{
    widgets::{List},
};
use crate::jview_logs;
use crate::jview_search;
use crate::jview_selector;

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

#[derive(Clone)]
pub struct UiScreen {
    selected: UiSection,
    search_tui: jview_search::JviewSearch,
    logs_tui: jview_logs::JviewLogs,
    selector_tui: jview_selector::JviewSelector,
}

impl UiScreen {
    pub fn new() -> Self {
        UiScreen {
            selected: UiSection::Logs,
            search_tui: jview_search::JviewSearch::new(),
            logs_tui: jview_logs::JviewLogs::new(),
            selector_tui: jview_selector::JviewSelector::new(),
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

    pub fn get_logs_widget<'a>(&self, selected: bool) -> List<'a> {
        self.logs_tui.clone().get_logs_widget(selected)
    }

    pub fn get_selector_widget(&self, selected: bool) -> List {
        self.selector_tui.clone().get_selector_widget(selected)
    }

    pub fn set_logs_max_height(&mut self, h: usize) {
        self.logs_tui.set_max_height(h);
    }

    pub fn set_selector_max_height(&mut self, h: usize) {
        self.selector_tui.set_max_height(h);
    }
}

pub fn screen_navigate(screen: &mut UiScreen) -> Result<bool, std::io::Error> {
    if screen.get_selected() == UiSection::Search {
        let res = screen.search_tui.get_search_input()?;
        match res {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('Q') => return Ok(true),
            KeyCode::Tab => {
                screen.next_section();
                return Ok(false);
            }
            _ => {}
        }
        return Ok(false);
    }

    if screen.get_selected() == UiSection::Logs {
        let res = screen.logs_tui.logs_navigate()?;
        match res {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('Q') => return Ok(true),
            KeyCode::Tab => {
                screen.next_section();
                return Ok(false);
            }
            _ => {}
        }
        return Ok(false);
    }

    if screen.get_selected() == UiSection::Selector {
        let res = screen.selector_tui.navigate()?;
        match res {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('Q') => return Ok(true),
            KeyCode::Tab => {
                screen.next_section();
                return Ok(false);
            }
            _ => {}
        }
        return Ok(false);
    }

    Ok(false)
}