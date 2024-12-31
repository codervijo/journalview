use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph},
};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use ratatui::style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JviewSearch {
    input: String,
}

impl JviewSearch {
    pub fn new() -> Self {
        JviewSearch {
            input: "Type to start searching...".to_string(),
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

impl JviewSearch {
    /// Creates a search widget for the application.
    ///
    /// # Arguments
    ///
    /// * `selected` - Is this widget currently selected?
    ///
    /// # Returns
    ///
    /// A `Paragraph` widget configured for the search functionality.
    pub fn get_search_widget(self, selected: bool) -> Paragraph<'static> {
        let intext = format!("\u{1F50D} {}", self.input);
        Paragraph::new(intext)
            .block(Block::default().borders(Borders::ALL).title("Search"))
            .style(get_style(selected))
    }

    pub fn get_search_input(&mut self) -> Result<String, std::io::Error> {
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
        self.input = input.clone();
        Ok(input)
    }
}

pub fn navigate() {

}