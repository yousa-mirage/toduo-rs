//! todo-tui: Terminal UI for todo.txt task manager
//!
//! Nord-themed TUI with Vim-style keybindings

mod app;
mod theme;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
};
use std::io::{self, Stdout};

use app::{App, Focus, InputMode};

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

        let evt = event::read()?;
        match evt {
            Event::Key(key) => {
                // Only process key press events (ignore Release and Repeat for now if desired, or handle Repeat)
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                if app.input_mode == InputMode::Adding {
                    match key.code {
                        KeyCode::Esc => app.cancel_input(),
                        KeyCode::Enter => app.submit_task()?,
                        KeyCode::Tab => app.next_input_field(),
                        KeyCode::BackTab => app.prev_input_field(),
                        KeyCode::Char(c) if key.modifiers.is_empty() => app.handle_char(c),
                        KeyCode::Backspace => app.handle_backspace(),
                        _ => {}
                    }
                } else if app.input_mode == InputMode::Help {
                    if matches!(key.code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')) {
                        app.toggle_help();
                    }
                } else {
                    // Normal Mode
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Tab => app.switch_focus(),
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Char('g') => app.go_top(),
                        KeyCode::Char('G') => app.go_bottom(),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if app.focus == Focus::MainList {
                                app.toggle_complete()?;
                            } else if app.focus == Focus::Sidebar {
                                // Enter on sidebar could select too (next/prev selects immediately currently)
                            }
                        }
                        KeyCode::Char('a') => app.start_add_task(),
                        KeyCode::Char('d') => app.delete_selected()?,
                        KeyCode::Char('1') => app.set_priority_selected(Some('A'))?,
                        KeyCode::Char('2') => app.set_priority_selected(Some('B'))?,
                        KeyCode::Char('3') => app.set_priority_selected(Some('C'))?,
                        KeyCode::Char('0') => app.set_priority_selected(None)?,
                        KeyCode::Char('r') => app.refresh()?,
                        KeyCode::Char('?') => app.toggle_help(),
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    let (x, y) = (mouse.column, mouse.row);
                    let area = terminal.get_frame().area();

                    // Re-calculate layout to find where click happened
                    let constraints = if app.input_mode == InputMode::Adding {
                        vec![
                            Constraint::Length(25),
                            Constraint::Min(40),
                            Constraint::Length(40),
                        ]
                    } else {
                        vec![Constraint::Length(25), Constraint::Min(40)]
                    };

                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(constraints)
                        .split(area);

                    if chunks[0].contains(ratatui::layout::Position { x, y }) {
                        // Clicked Sidebar
                        app.focus = Focus::Sidebar;
                        // Determine which item
                        // Layout inside sidebar: [Title(1), AddBtn(3), List(Min)]
                        // Just approximating roughly:
                        // Add Button is roughly row 1-3 inside sidebar (if borders included)
                        // Actually block takes borders, inner is smaller.
                        // Let's assume standard density.
                        let relative_y = y.saturating_sub(chunks[0].y + 1); // Remove top border

                        if relative_y < 3 {
                            // Add Task Button
                            app.start_add_task();
                        } else {
                            // Filter List
                            // List starts after add button (3 height)
                            let list_y = relative_y.saturating_sub(3);
                            // Indices corresponds to list items
                            if list_y < 6 {
                                // 6 items
                                app.sidebar_index = list_y as usize;
                                app.update_filter_from_sidebar();
                            }
                        }
                    } else if chunks[1].contains(ratatui::layout::Position { x, y }) {
                        // Clicked Main List
                        app.focus = Focus::MainList;
                        // Calculate list item index
                        let relative_y = y.saturating_sub(chunks[1].y + 3 + 1); // Title(3) + Border(1)
                        // This is rough.
                        // App has scrolled state? 'selected' is absolute index.
                        // List widget handles scrolling, but we don't know the offset easily in main.
                        // For MVP without scrolling state tracking:
                        // Assume list starts at top.
                        if (relative_y as usize) < app.view_tasks.len() {
                            app.selected = relative_y as usize;
                            // Optionally toggle complete on double click?
                            // Just select for now.
                        }
                    } else if chunks.len() > 2 && chunks[2].contains(ratatui::layout::Position { x, y })
                    {
                        // Clicked form
                        app.focus = Focus::RightSidebar;
                        // Focus field based on Y
                    }
                }
            }
            _ => {}
        }
    }
}
