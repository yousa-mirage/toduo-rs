//! todo-tui: Terminal UI for todo.txt task manager
//!
//! Nord-themed TUI with Vim-style keybindings

mod app;
mod theme;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};

use app::{App, InputMode};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Skip key repeat events (when key is being held down)
            if key.kind == event::KeyEventKind::Repeat {
                continue;
            }

            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('g') => app.go_top(),
                    KeyCode::Char('G') => app.go_bottom(),
                    KeyCode::Char(' ') | KeyCode::Enter => app.toggle_complete()?,
                    KeyCode::Char('a') => app.start_add_task(),
                    KeyCode::Char('d') => app.delete_selected()?,
                    KeyCode::Char('1') => app.set_priority_selected(Some('A'))?,
                    KeyCode::Char('2') => app.set_priority_selected(Some('B'))?,
                    KeyCode::Char('3') => app.set_priority_selected(Some('C'))?,
                    KeyCode::Char('0') => app.set_priority_selected(None)?,
                    KeyCode::Char('r') => app.refresh()?,
                    KeyCode::Char('?') => app.toggle_help(),
                    _ => {}
                },
                InputMode::Adding => match key.code {
                    KeyCode::Esc => app.cancel_input(),
                    KeyCode::Enter => app.submit_task()?,
                    KeyCode::Tab => app.next_input_field(),
                    KeyCode::BackTab => app.prev_input_field(),
                    KeyCode::Char(c) if key.modifiers.is_empty() => app.handle_char(c),
                    KeyCode::Backspace => app.handle_backspace(),
                    _ => {}
                },
                InputMode::Help => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => app.toggle_help(),
                    _ => {}
                },
            }
        }
    }
}
