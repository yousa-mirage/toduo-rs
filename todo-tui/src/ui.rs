//! UI rendering for the TUI

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, Focus, InputField, InputMode};
use crate::theme::*;

/// Main draw function
pub fn draw(f: &mut Frame, app: &mut App) {
    // 1. Sidebar | 2. Main Content | 3. Right Sidebar (Conditional)

    // Determine constraints
    let constraints = if app.input_mode == InputMode::Adding {
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
    }

    if app.input_mode == InputMode::Help {
        draw_help_modal(f);
    }
}

fn draw_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(BORDER))
        .title(" Sidebar ")
        .style(Style::default().bg(BG_DARK));

    f.render_widget(block.clone(), area);

    let inner_area = block.inner(area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Add Task Button
            Constraint::Min(1),    // Nav Items
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
    let items = vec![
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
}

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

    // Status
    let msg = app.status_message.as_deref().unwrap_or("Ready");
    let status = Paragraph::new(msg).style(Style::default().fg(TEXT_DIM));
    f.render_widget(status, layout[2]);
}

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

fn draw_add_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" Add New Task ")
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(BG_LIGHT));
    f.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Description
            Constraint::Length(3), // Priority
            Constraint::Length(3), // Projects
            Constraint::Length(3), // Contexts
            Constraint::Length(3), // Due Date
            Constraint::Min(1),    // Instructions
        ])
        .split(inner);

    // Helper to draw input field
    let draw_field = |f: &mut Frame, area: Rect, label: &str, text: &str, active: bool| {
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
    };

    draw_field(
        f,
        chunks[0],
        "Description",
        &app.description_input,
        app.input_field == InputField::Description,
    );
    draw_field(
        f,
        chunks[1],
        "Priority (A-Z)",
        &app.priority_input,
        app.input_field == InputField::Priority,
    );
    draw_field(
        f,
        chunks[2],
        "Projects (+)",
        &app.projects_input,
        app.input_field == InputField::Projects,
    );
    draw_field(
        f,
        chunks[3],
        "Contexts (@)",
        &app.contexts_input,
        app.input_field == InputField::Contexts,
    );
    draw_field(
        f,
        chunks[4],
        "Due Date (YYYY-MM-DD)",
        &app.due_date_input,
        app.input_field == InputField::DueDate,
    );

    let instructions = Paragraph::new("Tab: Next field | Enter: Submit | Esc: Cancel")
        .style(Style::default().fg(polar_night::NORD3))
        .wrap(Wrap { trim: true });
    f.render_widget(instructions, chunks[5]);
}

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

/// Helper function to create a centered rect
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
