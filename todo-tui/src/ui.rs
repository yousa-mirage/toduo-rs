//! UI rendering for the TUI

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, InputField, InputMode};
use crate::theme::*;

/// Main draw function
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Task list
            Constraint::Length(3), // Status bar
        ])
        .split(f.area());

    draw_title(f, chunks[0]);
    draw_task_list(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    // Draw modal overlays
    match app.input_mode {
        InputMode::Adding => draw_add_modal(f, app),
        InputMode::Help => draw_help_modal(f),
        InputMode::Normal => {}
    }
}

fn draw_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Todo.txt Manager")
        .style(
            Style::default()
                .fg(TEXT_HIGHLIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG_DARK)),
        );
    f.render_widget(title, area);
}

fn draw_task_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .tasks
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
            } else {
                Style::default().fg(TEXT)
            };
            spans.push(Span::styled(&task.subject, subject_style));

            // Projects
            for proj in &task.projects {
                spans.push(Span::styled(
                    format!(" +{}", proj),
                    Style::default().fg(PROJECT),
                ));
            }

            // Contexts
            for ctx in &task.contexts {
                spans.push(Span::styled(
                    format!(" @{}", ctx),
                    Style::default().fg(frost::NORD7),
                ));
            }

            // Due date
            if let Some(ref due) = task.due_date {
                spans.push(Span::styled(
                    format!(" due:{}", due),
                    Style::default().fg(aurora::NORD12),
                ));
            }

            let style = if i == app.selected {
                Style::default().bg(SELECTION)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(spans)).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!(" Tasks ({}) ", app.tasks.len()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .style(Style::default().bg(BG_DARK)),
        )
        .highlight_style(Style::default().bg(SELECTION));

    let mut state = ListState::default();
    state.select(Some(app.selected));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (msg, style) = match &app.status_message {
        Some(msg) => (msg.clone(), Style::default().fg(aurora::NORD13)),
        None => (
            "? Help | a Add | Space Complete | d Delete | 1-3 Priority | q Quit".to_string(),
            Style::default().fg(polar_night::NORD3),
        ),
    };

    let status = Paragraph::new(msg).style(style).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER))
            .style(Style::default().bg(BG_DARK)),
    );
    f.render_widget(status, area);
}

fn draw_add_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 70, f.area());

    // Clear the area
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" Add New Task ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(BG_LIGHT));
    f.render_widget(block, area);

    let inner = Block::default().borders(Borders::ALL).inner(area);

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

        let paragraph = Paragraph::new(text).style(Style::default().fg(TEXT)).block(
            Block::default()
                .title(format!(" {} ", label))
                .borders(Borders::ALL)
                .border_style(border_style)
                .style(Style::default().bg(if active { BG_DARK } else { BG_LIGHT })),
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
        .style(Style::default().fg(polar_night::NORD3));
    f.render_widget(instructions, chunks[5]);
}

fn draw_help_modal(f: &mut Frame) {
    let area = centered_rect(50, 60, f.area());

    f.render_widget(Clear, area);

    let help_text = vec![
        "",
        "  Navigation",
        "  ──────────",
        "  j/↓     Move down",
        "  k/↑     Move up",
        "  g       Go to top",
        "  G       Go to bottom",
        "",
        "  Actions",
        "  ───────",
        "  a       Add new task",
        "  Space   Toggle complete",
        "  d       Delete task",
        "  1-3     Set priority A-C",
        "  0       Clear priority",
        "  r       Refresh list",
        "",
        "  Other",
        "  ─────",
        "  ?       Toggle help",
        "  q       Quit",
        "",
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
