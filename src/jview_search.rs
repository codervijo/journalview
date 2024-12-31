use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph},
};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use ratatui::style;

pub struct JviewSearch {
    input: String,
}

impl JviewSearch {
    pub fn new() -> Self {
        JviewSearch {
            input: "".to_string(),
        }
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
/// A `Paragraph` widget configured for the search functionality.
pub fn get_search_widget(selected: bool) -> Paragraph<'static> {
    Paragraph::new("\u{1F50D} Type to start searching...")
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(get_style(selected))
}

impl JviewSearch {
    pub fn get_search_input(&self) -> Result<String, std::io::Error> {
        let mut input = String::new();
        loop {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break, // Escape to stop input
                    KeyCode::Backspace => {
                        input.pop(); // Remove last character
                    }
                    KeyCode::Enter => break, // Enter to submit input
                    KeyCode::Char(c) => {
                        input.push(c); // Add character to input string
                    }
                    KeyCode::Tab => {
                        return Ok(input);
                    }
                    _ => {}
                }
            }
        }
        Ok(input)
    }
}

pub fn navigate() {

}