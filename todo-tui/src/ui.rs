//! UI rendering for the TUI using ratatui.
//!
//! Provides functions to render the sidebar, task list, add task form,
//! and help modal.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, Focus, InputField, InputMode};
use crate::theme::*;

/// Main draw function - orchestrates rendering of all UI components
pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. Sidebar | 2. Main Content | 3. Right Sidebar (Conditional)

    // Determine constraints
    let constraints = if app.input_mode == InputMode::Adding || app.input_mode == InputMode::Editing {
        vec![
            Constraint::Length(25), // Sidebar
            Constraint::Min(40),    // Main
            Constraint::Length(40), // Right Sidebar
        ]
    } else {
        vec![
            Constraint::Length(25), // Sidebar
            Constraint::Min(40),    // Main
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.area());

    draw_sidebar(f, app, chunks[0]);
    draw_main_area(f, app, chunks[1]);

    if app.input_mode == InputMode::Adding {
        draw_add_sidebar(f, app, chunks[2]);
    } else if app.input_mode == InputMode::Editing {
        draw_edit_sidebar(f, app, chunks[2]);
    } else if app.input_mode == InputMode::ChangingPath {
        draw_path_change_modal(f, app);
    }

    if app.input_mode == InputMode::Help {
        draw_help_modal(f);
    }
}

/// Renders the left sidebar with filter navigation and add task button
fn draw_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(BG_DARK));

    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Add Task Button
            Constraint::Min(1),    // Nav Items
            Constraint::Length(3), // Change Path Button
        ])
        .split(inner_area);

    // Add Task Button
    let btn_style = if app.input_mode == InputMode::Adding {
        Style::default()
            .bg(ACCENT)
            .fg(BG_DARK)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(ACCENT)
    };

    let add_btn = Paragraph::new("+ Add Task").style(btn_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER)),
    );
    f.render_widget(add_btn, layout[0]);

    // Navigation Items
    let items = [
        "Tasks",
        "Today",
        "Next 7 Days",
        "High Priority (A)",
        "Medium Priority (B)",
        "Low Priority (C)",
    ];

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, &text)| {
            let style = if app.focus == Focus::Sidebar && i == app.sidebar_index {
                Style::default().bg(SELECTION).fg(TEXT_HIGHLIGHT)
            } else if i == app.sidebar_index {
                // Active filter but not focused?
                Style::default().fg(TEXT)
            } else {
                Style::default().fg(TEXT_DIM)
            };

            // Add icons or markers?
            let prefix = match i {
                0 => "📝 ",
                1 => "📅 ",
                2 => "🗓️ ",
                3 => "🔴 ",
                4 => "🟡 ",
                5 => "🔵 ",
                _ => "  ",
            };

            ListItem::new(Line::from(vec![Span::raw(prefix), Span::raw(text)])).style(style)
        })
        .collect();

    // Highlight currently active filter if focus is NOT sidebar
    // Actually, app.sidebar_index IS the source of truth for current filter in this simple implementation

    let list = List::new(list_items).style(Style::default().bg(BG_DARK));

    f.render_widget(list, layout[1]);

    // Change Path Button
    let path_btn_style = if app.input_mode == InputMode::ChangingPath {
        Style::default()
            .bg(ACCENT)
            .fg(BG_DARK)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(ACCENT)
    };

    let path_btn = Paragraph::new("  ⚙  Change Path").style(path_btn_style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER)),
    );
    f.render_widget(path_btn, layout[2]);
}

/// Renders the main task list area with header and task items
fn draw_main_area(f: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title/Header
            Constraint::Min(1),    // List
            Constraint::Length(1), // Footer/Status
        ])
        .split(area);

    // Header
    let filter_name = app.filter.to_string();
    let header = Paragraph::new(format!(" {} ({})", filter_name, app.view_tasks.len()))
        .style(Style::default().fg(TEXT_HIGHLIGHT).add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER)),
        );
    f.render_widget(header, layout[0]);

    // Task List
    draw_task_list(f, app, layout[1]);

    // Status bar with path on the right
    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),     // Status message (left)
            Constraint::Length(40), // Path (right)
        ])
        .split(layout[2]);

    let msg = app.status_message.as_deref().unwrap_or("Ready");
    let status = Paragraph::new(msg).style(Style::default().fg(TEXT_DIM));
    f.render_widget(status, footer_layout[0]);

    // Truncate path if too long
    let display_path = if app.current_todo_path.len() > 38 {
        "...".to_string() + &app.current_todo_path[app.current_todo_path.len().saturating_sub(38)..]
    } else {
        app.current_todo_path.clone()
    };
    let path_display = Paragraph::new(format!("📁 {}", display_path))
        .style(Style::default().fg(TEXT_DIM))
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(path_display, footer_layout[1]);
}

/// Renders the scrollable task list with completion status and priority indicators
fn draw_task_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .view_tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let mut spans = Vec::new();

            // Completion marker
            if task.completed {
                spans.push(Span::styled(
                    "✓ ",
                    Style::default().fg(COMPLETED).add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::raw("  "));
            }

            // Priority
            if let Some(p) = task.priority {
                let color = match p {
                    'A' => PRIORITY_A,
                    'B' => PRIORITY_B,
                    'C' => PRIORITY_C,
                    _ => TEXT,
                };
                spans.push(Span::styled(
                    format!("({}) ", p),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            }

            // Subject
            let subject_style = if task.completed {
                Style::default()
                    .fg(polar_night::NORD3)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else if app.focus == Focus::MainList && i == app.selected {
                Style::default().fg(TEXT_HIGHLIGHT)
            } else {
                Style::default().fg(TEXT)
            };
            spans.push(Span::styled(&task.subject, subject_style));

            // Projects
            for proj in &task.projects {
                spans.push(Span::styled(format!(" +{}", proj), Style::default().fg(PROJECT)));
            }

            // Due date
            if let Some(ref due) = task.due_date {
                spans.push(Span::styled(
                    format!(" due:{}", due),
                    Style::default().fg(aurora::NORD12),
                ));
            }

            let style = if app.focus == Focus::MainList && i == app.selected {
                Style::default().bg(SELECTION)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(spans)).style(style)
        })
        .collect();

    let list = List::new(items).highlight_style(Style::default().bg(SELECTION));

    let mut state = ListState::default();
    state.select(Some(app.selected));

    f.render_stateful_widget(list, area, &mut state);
}

/// Renders the add/edit task form sidebar with input fields
fn draw_task_form(f: &mut Frame, app: &mut App, area: Rect, title: &str, instructions: &str) {
    let block = Block::default()
        .title(title)
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(BG_LIGHT));
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(inner);

    // Get current inputs based on mode
    let (desc, pri, proj, ctx, due) = if app.input_mode == InputMode::Adding {
        (
            &app.description_input,
            &app.priority_input,
            &app.projects_input,
            &app.contexts_input,
            &app.due_date_input,
        )
    } else {
        (
            &app.edit_description,
            &app.edit_priority,
            &app.edit_projects,
            &app.edit_contexts,
            &app.edit_due_date,
        )
    };

    draw_input_field(
        f,
        chunks[0],
        "Description",
        desc,
        app.input_field == InputField::Description,
    );
    draw_input_field(
        f,
        chunks[1],
        "Priority (A-Z)",
        pri,
        app.input_field == InputField::Priority,
    );
    draw_input_field(
        f,
        chunks[2],
        "Projects (+)",
        proj,
        app.input_field == InputField::Projects,
    );
    draw_input_field(
        f,
        chunks[3],
        "Contexts (@)",
        ctx,
        app.input_field == InputField::Contexts,
    );
    draw_input_field(
        f,
        chunks[4],
        "Due Date (YYYY-MM-DD)",
        due,
        app.input_field == InputField::DueDate,
    );

    let instructions = Paragraph::new(instructions)
        .style(Style::default().fg(polar_night::NORD3))
        .wrap(Wrap { trim: true });
    f.render_widget(instructions, chunks[5]);

    // Set native cursor position
    f.set_cursor_position(calculate_cursor_position(app, &chunks));
}

/// Calculate cursor position based on current input field
fn calculate_cursor_position(app: &App, chunks: &[Rect]) -> Position {
    let text = app.get_current_input();

    let chunk_index = match app.input_field {
        InputField::Description => 0,
        InputField::Priority => 1,
        InputField::Projects => 2,
        InputField::Contexts => 3,
        InputField::DueDate => 4,
    };

    // Convert byte offset to char offset for proper display
    let char_offset = text
        .char_indices()
        .enumerate()
        .take_while(|(_, (byte_idx, _))| *byte_idx < app.cursor_position)
        .count();

    let x = (chunks[chunk_index].x + 1 + char_offset as u16)
        .min(chunks[chunk_index].x + chunks[chunk_index].width - 2);
    let y = chunks[chunk_index].y + 1;

    Position::new(x, y)
}

/// Helper to draw an input field
fn draw_input_field(f: &mut Frame, area: Rect, label: &str, text: &str, active: bool) {
    let border_style = if active {
        Style::default().fg(ACCENT)
    } else {
        Style::default().fg(BORDER)
    };

    let bg_color = if active { BG_DARK } else { BG_LIGHT };

    let paragraph = Paragraph::new(text).style(Style::default().fg(TEXT)).block(
        Block::default()
            .title(format!(" {} ", label))
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(Style::default().bg(bg_color)),
    );

    f.render_widget(paragraph, area);
}

/// Renders the add task form sidebar with input fields
fn draw_add_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    draw_task_form(
        f,
        app,
        area,
        " Add New Task ",
        "Tab: Next field | Enter: Submit | Esc: Cancel",
    );
}

/// Renders the edit task form sidebar with input fields
fn draw_edit_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    draw_task_form(
        f,
        app,
        area,
        " Edit Task ",
        "Tab: Next field | Enter: Save | Esc: Cancel",
    );
}

/// Renders the help modal with keyboard shortcuts
fn draw_help_modal(f: &mut Frame) {
    let area = centered_rect(50, 60, f.area());

    f.render_widget(Clear, area);
    // ... existing help code ...
    let help_text = vec![
        "",
        "  Navigation",
        "  ──────────",
        "  Tab     Switch Focus (Sidebar/List)",
        "  j/↓     Move down",
        "  k/↑     Move up",
        "",
        "  Actions",
        "  ───────",
        "  a       Add new task (Open Sidebar)",
        "  Space   Toggle complete",
        "  Click   Select item / Filter",
        "",
        "  q       Quit",
    ];

    let help = Paragraph::new(help_text.join("\n"))
        .style(Style::default().fg(TEXT))
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ACCENT))
                .style(Style::default().bg(BG_LIGHT)),
        );

    f.render_widget(help, area);
}

/// Renders the path change modal
fn draw_path_change_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(50, 30, f.area());

    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Change Path ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(BG_LIGHT));
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Path input
            Constraint::Min(1),    // Instructions
        ])
        .split(inner);

    // Render path input
    let path_input = Paragraph::new(&*app.path_input)
        .style(Style::default().fg(TEXT))
        .block(
            Block::default()
                .title(" Directory Path ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ACCENT))
                .style(Style::default().bg(BG_DARK)),
        );
    f.render_widget(path_input, chunks[0]);

    let instructions =
        Paragraph::new("Enter folder path or todo.txt file path | Enter: Submit | Esc: Cancel")
            .style(Style::default().fg(polar_night::NORD3))
            .wrap(Wrap { trim: true });
    f.render_widget(instructions, chunks[1]);

    // Set native cursor position
    let x = (chunks[0].x + 1 + app.cursor_position as u16).min(chunks[0].x + chunks[0].width - 2);
    let y = chunks[0].y + 1;
    f.set_cursor_position(Position::new(x, y));
}

/// Helper function to create a centered rectangular area for modals
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
