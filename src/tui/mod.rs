use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub mod app;
pub mod ui;

use app::App;
use ui::ui;

use crate::types::RecordSet;

pub fn run_tui(rs: RecordSet) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(rs);

    let res = run_app(&mut terminal, &mut app);

    // Cleanup terminal mode
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if app.should_quit {
            return Ok(());
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if app.search_active {
                    match key.code {
                        KeyCode::Esc => {
                            app.search_active = false;
                        }
                        KeyCode::Enter => {
                            app.search_active = false;
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.update_search();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.update_search();
                        }
                        _ => {}
                    }
                } else if app.show_detail {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.show_detail = false;
                        }
                        _ => {}
                    }
                } else if app.show_stats {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('s') | KeyCode::Char('q') => {
                            app.show_stats = false;
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.move_up();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.move_down();
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            app.move_left();
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            app.move_right();
                        }
                        KeyCode::PageUp | KeyCode::Char('u')
                            if key.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            app.page_up(15);
                        }
                        KeyCode::PageDown | KeyCode::Char('d')
                            if key.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            app.page_down(15);
                        }
                        KeyCode::Char('g') => {
                            app.move_to_top();
                        }
                        KeyCode::Char('G') => {
                            app.move_to_bottom();
                        }
                        KeyCode::Char('/') => {
                            app.search_active = true;
                        }
                        KeyCode::Char('n') => {
                            app.next_match();
                        }
                        KeyCode::Char('N') => {
                            app.prev_match();
                        }
                        KeyCode::Enter => {
                            app.show_detail = true;
                        }
                        KeyCode::Char('s') => {
                            app.show_stats = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
