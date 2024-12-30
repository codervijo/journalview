use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, process::Command};

mod jview_logs;
mod jview_search;
mod jview_selector;
mod jview_help;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let logs = jview_logs::fetch_journalctl_logs();
    let mut vertical_offset = 0;
    let mut horizontal_offset = 0;
    let mut selected_section = 0; // 0: Search, 1: Logs, 2: Selection

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

            // Draw the left column sections

            let selwidget = jview_selector::get_widget("Selectors", selected_section == 2);
            f.render_widget(selwidget, selection_chunks[0]);
            //let tbdwidget = Paragraph::new("TBD section")
            //    .block(Block::default().borders(Borders::ALL).title("TBD"));
            //  f.render_widget(tbdwidget, selection_chunks[1]);

            // Search Section
            let search_style = jview_search::get_style(selected_section == 0);
            let search_widget = jview_search::get_search_widget("Search text", search_style);
            f.render_widget(search_widget, viewer_chunks[0]);

            // Logs Section
            let mut log_items: Vec<ListItem> = jview_logs::get_log_items(vertical_offset, viewer_chunks[1].height as usize, horizontal_offset, selected_section == 1);
            let logs_widget = jview_logs::get_log_widget(log_items, selected_section == 1);
            f.render_widget(logs_widget, viewer_chunks[1]);

            // Help Section
            let help_widget = jview_help::get_widget();
            f.render_widget(help_widget, help_chunk);

        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('Q') => break,
                KeyCode::Tab => {
                    selected_section = (selected_section + 1) % 3;
                }
                KeyCode::Up => {
                    if selected_section == 1 && vertical_offset > 0 {
                        vertical_offset -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected_section == 1 && vertical_offset < logs.len() {
                        vertical_offset += 1;
                    }
                }
                KeyCode::Left => {
                    if selected_section == 1 && horizontal_offset > 0 {
                        horizontal_offset -= 1;
                    }
                }
                KeyCode::Right => {
                    if selected_section == 1 {
                        horizontal_offset += 1;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
