use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

mod jview_screen;
mod jview_logs;
mod jview_search;
mod jview_selector;
mod jview_help;

use crate::jview_screen::UiScreen;
use crate::jview_screen::UiSection::Search;
use crate::jview_screen::UiSection::Logs;
use crate::jview_screen::UiSection::Selector;
//use crate::jview_screen::UiSection::Help;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    //let mut selected_section = Search; // 0: Search, 1: Logs, 2: Selection
    let mut screen = UiScreen::new(); // Persistent screen state

    loop {
        terminal.draw(|f| {
            // Define the layout with two main sections: Left and Right
            let overall_layout = Layout::default()
                .direction(Direction::Vertical) // Split horizontally into left and right columns
                .constraints([
                    Constraint::Length(1), // Top margin
                    Constraint::Min(5), 
                    Constraint::Length(3),
                ])
                .split(f.size());
            
            let active_layout = Layout::default()
                .direction(Direction::Horizontal) // Split horizontally into left and right columns
                .constraints([
                    Constraint::Percentage(20), // Left column takes up 20% of the width
                    Constraint::Percentage(80), // Right column takes up 80% of the width
                ])
                .split(overall_layout[1]);

            let selection_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(100),
                ])
                .split(active_layout[0]);

            let viewer_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Search section
                    Constraint::Min(5),   // Logs section
                ])
                .split(active_layout[1]);

            // Define the layout for the Help section, which takes up the entire width of the terminal
            let help_chunk = Layout::default()
                .direction(Direction::Horizontal) // Single row that spans the entire width
                .constraints([Constraint::Min(1)]) // Only one section (Help) that takes the entire space
                .split(overall_layout[2])[0]; // Apply to the whole width

            // Draw the left column selector
            let selwidget = jview_selector::get_widget(screen.get_selected() == Selector);
            f.render_widget(selwidget, selection_chunks[0]);

            // Search Section
            let search_widget = screen.get_search_widget(screen.get_selected() == Search);
            f.render_widget(search_widget, viewer_chunks[0]);

            // Logs Section
            let mut log_items: Vec<ListItem> = jview_logs::get_log_items(0, viewer_chunks[1].height as usize, 0, screen.get_selected() == Logs);
            //let mut log_items: Vec<ListItem> = jview_logs::get_log_items(vertical_offset, viewer_chunks[1].height as usize, horizontal_offset, screen.get_selected() == Logs);
            let logs_widget = screen.get_logs_widget(log_items, screen.get_selected() == Logs);
            f.render_widget(logs_widget, viewer_chunks[1]);

            // Help Section
            let help_widget = jview_help::get_widget();
            f.render_widget(help_widget, help_chunk);

        })?;
        if jview_screen::screen_navigate(&mut screen)? == true {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
