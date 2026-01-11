//! todo-tui: Terminal UI for todo.txt task manager
//!
//! A Nord-themed terminal user interface for managing todo.txt files.
//! Features Vim-style keybindings, mouse support, and a clean layout.
//!
//! # Keybindings
//!
//! | Key | Action |
//! |-----|--------|
//! | `j` / `↓` | Move selection down |
//! | `k` / `↑` | Move selection up |
//! | `g` | Go to top |
//! | `G` | Go to bottom |
//! | `a` | Add new task |
//! | `Space` | Toggle task completion |
//! | `d` | Delete selected task |
//! | `1-3` | Set priority (A/B/C) |
//! | `0` | Clear priority |
//! | `?` | Toggle help |
//! | `q` | Quit |

use std::io::{self, Stdout, Write};

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
    layout::{Constraint, Direction, Layout, Rect},
};

mod app;
mod theme;
mod ui;

use app::{App, Focus, InputField, InputMode};

/// Application entry point.
///
/// Sets up the terminal in raw mode, initializes the TUI backend,
/// and starts the main event loop.
fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // Enable block cursor (CSI ? 12 h / CSI 2 q)
    print!("\x1b[2 q");
    io::stdout().flush()?;
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

/// Main event loop.
///
/// Handles keyboard and mouse events, dispatches to appropriate handlers,
/// and renders the UI on each iteration.
fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let evt = event::read()?;
        match evt {
            Event::Key(key) => {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                // Ctrl+C always exits, regardless of input mode
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                match app.input_mode {
                    InputMode::Adding | InputMode::Editing => handle_input_mode_key(app, key),
                    InputMode::Help => {
                        if matches!(key.code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')) {
                            app.toggle_help();
                        }
                    }
                    InputMode::ChangingPath => handle_path_change_key(app, key),
                    InputMode::Normal => handle_normal_mode_key(app, key)?,
                }
            }
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    handle_mouse_click(app, mouse.column, mouse.row, terminal.get_frame().area());
                }
            }
            _ => {}
        }
    }
}

/// Handle keyboard input in Adding/Editing mode
fn handle_input_mode_key(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            if app.input_mode == InputMode::Adding {
                app.cancel_input();
            } else {
                app.cancel_edit();
            }
        }
        KeyCode::Enter => {
            if app.input_mode == InputMode::Adding {
                let _ = app.submit_task();
            } else {
                let _ = app.submit_edit();
            }
        }
        KeyCode::Tab => app.next_input_field(),
        KeyCode::BackTab => app.prev_input_field(),
        KeyCode::Left => app.move_cursor_left(),
        KeyCode::Right => app.move_cursor_right(),
        KeyCode::Home => app.move_cursor_to_start(),
        KeyCode::End => app.move_cursor_to_end(),
        KeyCode::Char(c)
            if key.modifiers.is_empty() || key.modifiers.intersects(KeyModifiers::SHIFT) =>
        {
            app.handle_char(c)
        }
        KeyCode::Backspace => app.handle_backspace(),
        _ => {}
    }
}

/// Handle keyboard input in Path Change mode
fn handle_path_change_key(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.cancel_change_path(),
        KeyCode::Enter => {
            let _ = app.submit_path_change();
        }
        KeyCode::Left => app.move_cursor_left(),
        KeyCode::Right => app.move_cursor_right(),
        KeyCode::Home => app.move_cursor_to_start(),
        KeyCode::End => app.move_cursor_to_end(),
        KeyCode::Char(c)
            if key.modifiers.is_empty() || key.modifiers.intersects(KeyModifiers::SHIFT) =>
        {
            app.handle_path_char(c)
        }
        KeyCode::Backspace => app.handle_path_backspace(),
        _ => {}
    }
}

/// Handle keyboard input in Normal mode
fn handle_normal_mode_key(app: &mut App, key: event::KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => return Ok(()),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
        KeyCode::Tab => app.switch_focus(),
        KeyCode::Char('j') | KeyCode::Down => app.next(),
        KeyCode::Char('k') | KeyCode::Up => app.previous(),
        KeyCode::Char('g') => app.go_top(),
        KeyCode::Char('G') => app.go_bottom(),
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.focus == Focus::MainList {
                app.toggle_complete()?;
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
    Ok(())
}

/// Handle mouse click events
fn handle_mouse_click(app: &mut App, x: u16, y: u16, area: Rect) {
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
        let sidebar_inner_y = y - chunks[0].y;
        if sidebar_inner_y < 3 {
            app.start_add_task();
        } else if sidebar_inner_y < 9 {
            let list_y = sidebar_inner_y - 3;
            app.sidebar_index = list_y as usize;
            app.update_filter_from_sidebar();
        } else {
            app.start_change_path();
        }
    } else if chunks[1].contains(ratatui::layout::Position { x, y }) {
        // Clicked Main List
        app.focus = Focus::MainList;
        if y > chunks[1].y + 2 {
            let item_index = y - (chunks[1].y + 3);
            if usize::from(item_index) < app.view_tasks.len() {
                app.selected = usize::from(item_index);
                if app.check_double_click(x, y) {
                    let task_id = app.view_tasks[usize::from(item_index)].id;
                    app.start_edit_task(task_id);
                }
            }
        }
    } else if chunks.len() > 2 && chunks[2].contains(ratatui::layout::Position { x, y }) {
        // Clicked Right Sidebar
        app.focus = Focus::RightSidebar;
        let sidebar_y = y - chunks[2].y;
        app.input_field = match sidebar_y {
            1..=3 => InputField::Description,
            4..=6 => InputField::Priority,
            7..=9 => InputField::Projects,
            10..=12 => InputField::Contexts,
            13..=15 => InputField::DueDate,
            _ => InputField::Description,
        };
        app.cursor_position = app.get_current_input().len();
    }
}
