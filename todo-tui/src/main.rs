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

use std::io::{self, Stdout};

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

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Enable block cursor (blinking) - use execute through terminal
    execute!(
        terminal.backend_mut(),
        crossterm::cursor::SetCursorStyle::BlinkingBlock
    )?;

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

                // Ctrl+C always exits
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                // q exits only in Normal mode with proper focus
                if key.code == KeyCode::Char('q')
                    && app.input_mode == InputMode::Normal
                    && (app.focus == Focus::Sidebar || app.focus == Focus::MainList)
                {
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
        KeyCode::Char('4') => app.set_priority_selected(Some('D'))?,
        KeyCode::Char('5') => app.set_priority_selected(Some('E'))?,
        KeyCode::Char('0') => app.set_priority_selected(None)?,
        KeyCode::Char('r') => app.refresh()?,
        KeyCode::Char('?') => app.toggle_help(),
        _ => {}
    }
    Ok(())
}

/// Handle mouse click events
fn handle_mouse_click(app: &mut App, x: u16, y: u16, area: Rect) {
    let mouse_pos = ratatui::layout::Position { x, y };

    // 1. Handle Modal Overlays (Help, Change Path)
    // Clicking outside closes them.
    if app.input_mode == InputMode::Help {
        let help_area = ui::centered_rect(50, 60, area);
        if !help_area.contains(mouse_pos) {
            app.toggle_help();
        }
        return;
    }

    if app.input_mode == InputMode::ChangingPath {
        let path_area = ui::centered_rect(50, 30, area);
        if !path_area.contains(mouse_pos) {
            app.cancel_change_path();
        }
        return;
    }

    // 2. Handle Three-Pane Layout (Normal, Adding, Editing)
    // Reconstruct layout to match ui.rs
    let constraints = vec![Constraint::Length(25), Constraint::Min(40)];

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    match app.input_mode {
        InputMode::Adding | InputMode::Editing => {
            // Updated to use centered_rect coordinates like Help modal
            // In ui.rs we used 60% width/height
            let modal_area = ui::centered_rect(60, 60, area);

            if modal_area.contains(mouse_pos) {
                app.focus = Focus::RightSidebar;
                // Calculate relative position within modal
                // The modal has a block + inner area.
                // Inner area is roughly modal_area minus 1 for borders.
                let inner_y = y.saturating_sub(modal_area.y).saturating_sub(1);

                // Layout inside modal:
                // [0] Desc: 3 (y=0..2)
                // [1] Pri: 3 (y=3..5)
                // [2] Proj: 3 (y=6..8)
                // [3] Ctx: 3 (y=9..11)
                // [4] Due: 3 (y=12..14)

                // We added margin(1) in layout, so everything is shifted down by 1 more line?
                // Let's check ui.rs: inner.split with margin(1)
                let inner_y_adjusted = inner_y.saturating_sub(1);

                if inner_y_adjusted < 3 {
                    app.input_field = InputField::Description;
                } else if inner_y_adjusted < 6 {
                    app.input_field = InputField::Priority;
                } else if inner_y_adjusted < 9 {
                    app.input_field = InputField::Projects;
                } else if inner_y_adjusted < 12 {
                    app.input_field = InputField::Contexts;
                } else if inner_y_adjusted < 15 {
                    app.input_field = InputField::DueDate;
                }

                // Only reset cursor position if we actually changed field?
                // Or updating is fine.
                app.cursor_position = app.get_current_input().chars().count();
            } else {
                // Clicking outside closes the modal
                if app.input_mode == InputMode::Adding {
                    app.cancel_input();
                } else {
                    app.cancel_edit();
                }
            }
        }
        InputMode::Normal => {
            // Normal interaction with Sidebar and Main List
            if chunks[0].contains(mouse_pos) {
                // Clicked Sidebar
                app.focus = Focus::Sidebar;
                let sidebar_inner_y = y - chunks[0].y;
                let sidebar_height = chunks[0].height;

                if sidebar_inner_y < 3 {
                    app.start_add_task();
                } else if sidebar_inner_y >= sidebar_height.saturating_sub(3) {
                    app.start_change_path();
                } else {
                    // Check if clicked exactly on a list item
                    let list_y = sidebar_inner_y.saturating_sub(3);
                    if list_y < 6 {
                        app.sidebar_index = list_y as usize;
                        app.update_filter_from_sidebar();
                    }
                }
            } else if chunks[1].contains(mouse_pos) {
                // Clicked Main List
                app.focus = Focus::MainList;

                // Layout: Header (1 line) + List Top Border (1 line) = 2 lines offset
                let list_start_y = chunks[1].y + 2;

                if y >= list_start_y {
                    let visual_index = (y - list_start_y) as usize;
                    let real_index = visual_index + app.list_state.offset();

                    if real_index < app.view_tasks.len() {
                        app.selected = real_index;
                        app.list_state.select(Some(real_index));

                        if app.check_double_click(x, y) {
                            let task_id = app.view_tasks[real_index].id;
                            app.start_edit_task(task_id);
                        }
                    }
                }
            }
        }
        _ => {} // Handled above
    }
}
