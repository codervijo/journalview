use ratatui::{
    style::{Style, Color},
    widgets::{Block, Borders, Paragraph},
};
use crossterm::event::{self, KeyCode};
use ratatui::style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JviewSearch {
    input: String,
    help: String,
    inited: bool,
}

impl JviewSearch {
    pub fn new() -> Self {
        JviewSearch {
            input: "".to_string(),
            help: "Type to start searching...".to_string(),
            inited: false,
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
        let intext;
        if self.inited == true {
            intext = format!("\u{1F50D} {}", self.input);
        } else {
            intext = format!("\u{1F50D} {}", self.help);
        }
        Paragraph::new(intext)
            .block(Block::default().borders(Borders::ALL).title("Search"))
            .style(get_style(selected))
    }

    pub fn get_search_input(&mut self) -> Result<KeyCode, std::io::Error> {
        let mut input = String::new();
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    return Ok(KeyCode::Tab); // Escape to stop input
                }
                KeyCode::Backspace => {
                    input.pop(); // Remove last character
                }
                KeyCode::Enter => {
                    return Ok(KeyCode::Tab); // Enter to submit input
                }
                KeyCode::Char(c) => {
                    self.inited = true;
                    input.push(c); // Add character to input string
                }
                KeyCode::Tab => {
                    return Ok(KeyCode::Tab);
                }
                _ => {}
            }
        }
        self.input += &input.clone();
        Ok(KeyCode::Enter)
    }
}
