//! Application state and logic for the TUI.
//!
//! Contains the core application state machine, task operations,
//! and input handling for the terminal UI.

use anyhow::Result;

use todo_core::{AppTask, TaskService, get_todo_path};

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal mode - browsing tasks
    Normal,
    /// Adding a new task
    Adding,
    /// Editing an existing task
    Editing,
    /// Showing help
    Help,
    /// Changing todo.txt path
    ChangingPath,
}

/// Which field is being edited in the add task form
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField {
    Description,
    Priority,
    Projects,
    Contexts,
    DueDate,
}

impl InputField {
    /// Move to the next field, wrapping around to Description
    pub fn next(self) -> Self {
        match self {
            Self::Description => Self::Priority,
            Self::Priority => Self::Projects,
            Self::Projects => Self::Contexts,
            Self::Contexts => Self::DueDate,
            Self::DueDate => Self::Description,
        }
    }

    /// Move to the previous field, wrapping around to DueDate
    pub fn prev(self) -> Self {
        match self {
            Self::Description => Self::DueDate,
            Self::Priority => Self::Description,
            Self::Projects => Self::Priority,
            Self::Contexts => Self::Projects,
            Self::DueDate => Self::Contexts,
        }
    }
}

/// Filter for tasks - determines which tasks are visible in the list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Today,
    Next7Days,
    Priority(char),
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All Tasks"),
            Self::Today => write!(f, "Today"),
            Self::Next7Days => write!(f, "Next 7 Days"),
            Self::Priority(c) => write!(f, "Priority {}", c),
        }
    }
}

/// Focus determines which panel receives keyboard input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    MainList,
    RightSidebar,
}

/// Application state
///
/// Holds all application data including loaded tasks, current filter,
/// input state, and UI state.
pub struct App {
    /// Task service for file operations
    pub service: TaskService,
    /// All tasks loaded from file (before filtering)
    pub all_tasks: Vec<AppTask>,
    /// Filtered and sorted tasks for display
    pub view_tasks: Vec<AppTask>,
    /// Currently selected task index in view_tasks
    pub selected: usize,
    /// Current filter
    pub filter: Filter,
    /// Sidebar item index (0=All, 1=Today, 2=Next7, 3=A, 4=B, 5=C)
    pub sidebar_index: usize,
    /// Focus context (Sidebar, List, AddTask)
    pub focus: Focus,

    /// Current input mode
    pub input_mode: InputMode,
    /// Current input field when adding
    pub input_field: InputField,

    // Form Inputs
    pub description_input: String,
    pub priority_input: String,
    pub projects_input: String,
    pub contexts_input: String,
    pub due_date_input: String,

    /// Status message shown in footer
    pub status_message: Option<String>,
    /// Current todo.txt path for display
    pub current_todo_path: String,
    /// Input for changing path
    pub path_input: String,
    /// Cursor position (column) for cursor rendering
    pub cursor_position: usize,

    // Editing state
    /// Task ID being edited (None if not editing)
    pub editing_task_id: Option<usize>,
    /// Editing form inputs
    pub edit_description: String,
    pub edit_priority: String,
    pub edit_projects: String,
    pub edit_contexts: String,
    pub edit_due_date: String,

    // Double-click detection
    /// Last click position for double-click detection
    pub last_click_pos: Option<(u16, u16)>,
    /// Last click timestamp for double-click detection
    pub last_click_time: Option<std::time::Instant>,
}

impl App {
    /// Creates a new App instance, loading tasks from the default todo.txt path.
    pub fn new() -> Result<Self> {
        let todo_path = get_todo_path()?;
        let service = TaskService::new(&todo_path);
        let all_tasks = service.load_tasks()?;

        let mut app = Self {
            service,
            all_tasks: all_tasks.clone(),
            view_tasks: Vec::new(),
            selected: 0,
            filter: Filter::All,
            sidebar_index: 0,
            focus: Focus::MainList,
            input_mode: InputMode::Normal,
            input_field: InputField::Description,
            description_input: String::new(),
            priority_input: String::new(),
            projects_input: String::new(),
            contexts_input: String::new(),
            due_date_input: String::new(),
            status_message: None,
            current_todo_path: todo_path.to_string_lossy().to_string(),
            path_input: String::new(),
            cursor_position: 0,
            editing_task_id: None,
            edit_description: String::new(),
            edit_priority: String::new(),
            edit_projects: String::new(),
            edit_contexts: String::new(),
            edit_due_date: String::new(),
            last_click_pos: None,
            last_click_time: None,
        };
        app.apply_filter();
        Ok(app)
    }

    /// Applies the current filter to all_tasks and stores result in view_tasks.
    /// Also sorts tasks: completed last, then by priority (A-Z), then by subject.
    pub fn apply_filter(&mut self) {
        use chrono::{Days, Local};
        let today_date = Local::now().date_naive();
        let today_str = today_date.to_string();

        let next7_date = today_date.checked_add_days(Days::new(7)).unwrap_or(today_date);
        let next7_str = next7_date.to_string();

        self.view_tasks = self
            .all_tasks
            .iter()
            .filter(|t| match self.filter {
                Filter::All => true,
                Filter::Today => t.due_date.as_deref() == Some(&today_str),
                Filter::Next7Days => {
                    if let Some(d) = &t.due_date {
                        d >= &today_str && d <= &next7_str
                    } else {
                        false
                    }
                }
                Filter::Priority(p) => t.priority == Some(p),
            })
            .cloned()
            .collect();

        // Sort by priority then due date (basic)
        self.view_tasks.sort_by(|a, b| {
            // Sort completed last
            if a.completed != b.completed {
                return a.completed.cmp(&b.completed);
            }
            // Sort by priority
            match (a.priority, b.priority) {
                (Some(pa), Some(pb)) if pa != pb => pa.cmp(&pb),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => a.subject.cmp(&b.subject),
            }
        });

        // Reset selection if out of bounds
        if self.selected >= self.view_tasks.len() {
            self.selected = self.view_tasks.len().saturating_sub(1);
        }
    }

    /// Resets all input fields to empty and resets focus to Description
    fn reset_inputs(&mut self) {
        self.description_input.clear();
        self.priority_input.clear();
        self.projects_input.clear();
        self.contexts_input.clear();
        self.due_date_input.clear();
        self.input_field = InputField::Description;
    }

    /// Move selection to next item based on current focus
    pub fn next(&mut self) {
        match self.focus {
            Focus::MainList => {
                if !self.view_tasks.is_empty() {
                    self.selected = (self.selected + 1) % self.view_tasks.len();
                }
            }
            Focus::Sidebar => {
                // Mock logic for sidebar nav
                self.sidebar_index = (self.sidebar_index + 1) % 6; // 0=All, 1=Today, 2=Next7, 3=Pri A, 4=Pri B, 5=Pri C
                self.update_filter_from_sidebar();
            }
            Focus::RightSidebar => {
                // Handled in input mode mostly
            }
        }
    }

    /// Move selection to previous item based on current focus
    pub fn previous(&mut self) {
        match self.focus {
            Focus::MainList => {
                if !self.view_tasks.is_empty() {
                    self.selected = self.selected.checked_sub(1).unwrap_or(self.view_tasks.len() - 1);
                }
            }
            Focus::Sidebar => {
                self.sidebar_index = self.sidebar_index.checked_sub(1).unwrap_or(5);
                self.update_filter_from_sidebar();
            }
            _ => {}
        }
    }

    /// Updates filter based on sidebar selection index
    pub fn update_filter_from_sidebar(&mut self) {
        self.filter = match self.sidebar_index {
            0 => Filter::All,
            1 => Filter::Today,
            2 => Filter::Next7Days,
            3 => Filter::Priority('A'),
            4 => Filter::Priority('B'),
            5 => Filter::Priority('C'),
            _ => Filter::All,
        };
        self.apply_filter();
    }

    /// Go to first task in the list
    pub fn go_top(&mut self) {
        self.selected = 0;
    }

    /// Go to last task in the list
    pub fn go_bottom(&mut self) {
        if !self.view_tasks.is_empty() {
            self.selected = self.view_tasks.len() - 1;
        }
    }

    /// Switch focus between Sidebar and MainList
    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Sidebar => Focus::MainList,
            Focus::MainList => Focus::Sidebar,
            Focus::RightSidebar => Focus::MainList, // Esc from right sidebar usually cancels, but generic switch logic here
        };
    }

    /// Toggle completion status of selected task
    pub fn toggle_complete(&mut self) -> Result<()> {
        if let Some(task) = self.view_tasks.get(self.selected) {
            let id = task.id;
            if task.completed {
                self.service.uncomplete_task(id)?;
            } else {
                self.service.complete_task(id)?;
            }
            self.refresh()?;
            self.status_message = Some("Task updated".to_string());
        }
        Ok(())
    }

    /// Delete selected task from the list
    pub fn delete_selected(&mut self) -> Result<()> {
        if let Some(task) = self.view_tasks.get(self.selected) {
            let id = task.id;
            self.service.delete_task(id)?;
            self.refresh()?;
            if self.selected >= self.view_tasks.len() && !self.view_tasks.is_empty() {
                self.selected = self.view_tasks.len() - 1;
            }
            self.status_message = Some("Task deleted".to_string());
        }
        Ok(())
    }

    /// Set priority of selected task
    pub fn set_priority_selected(&mut self, priority: Option<char>) -> Result<()> {
        if let Some(task) = self.view_tasks.get(self.selected) {
            let id = task.id;
            self.service.set_priority(id, priority)?;
            self.refresh()?;
            self.status_message = Some(format!(
                "Priority set to {}",
                priority
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ));
        }
        Ok(())
    }

    /// Reload tasks from file and reapply filter
    pub fn refresh(&mut self) -> Result<()> {
        self.all_tasks = self.service.load_tasks()?;
        self.apply_filter();
        Ok(())
    }

    /// Enter add task mode and reset form inputs
    pub fn start_add_task(&mut self) {
        self.input_mode = InputMode::Adding;
        self.focus = Focus::RightSidebar;
        self.reset_inputs();
    }

    /// Cancel input and return to normal mode
    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.focus = Focus::MainList;
        self.reset_inputs();
        self.status_message = Some("Cancelled".to_string());
    }

    /// Enter edit task mode with the selected task's data
    pub fn start_edit_task(&mut self, task_id: usize) {
        if let Some(task) = self.all_tasks.iter().find(|t| t.id == task_id) {
            self.input_mode = InputMode::Editing;
            self.focus = Focus::RightSidebar;
            self.editing_task_id = Some(task_id);
            self.edit_description = task.subject.clone();
            self.edit_priority = task.priority.map(|p| p.to_string()).unwrap_or_default();
            self.edit_projects = task.projects.join(" ");
            self.edit_contexts = task.contexts.join(" ");
            self.edit_due_date = task.due_date.clone().unwrap_or_default();
            self.input_field = InputField::Description;
            self.cursor_position = self.edit_description.len();
        }
    }

    /// Cancel editing and return to normal mode
    pub fn cancel_edit(&mut self) {
        self.input_mode = InputMode::Normal;
        self.focus = Focus::MainList;
        self.editing_task_id = None;
        self.edit_description.clear();
        self.edit_priority.clear();
        self.edit_projects.clear();
        self.edit_contexts.clear();
        self.edit_due_date.clear();
        self.status_message = Some("Edit cancelled".to_string());
    }

    /// Check if a click is a double-click (same position within 300ms)
    pub fn check_double_click(&mut self, x: u16, y: u16) -> bool {
        let now = std::time::Instant::now();
        let is_double = match (self.last_click_pos, self.last_click_time) {
            (Some((last_x, last_y)), Some(last_time)) => {
                let time_diff = now.duration_since(last_time);
                x == last_x && y == last_y && time_diff.as_millis() < 300
            }
            _ => false,
        };
        self.last_click_pos = Some((x, y));
        self.last_click_time = Some(now);
        is_double
    }

    /// Move to next input field
    pub fn next_input_field(&mut self) {
        self.input_field = self.input_field.next();
        if self.input_mode == InputMode::Editing {
            self.reset_edit_cursor_position();
        } else {
            self.reset_cursor_position();
        }
    }

    /// Move to previous input field
    pub fn prev_input_field(&mut self) {
        self.input_field = self.input_field.prev();
        if self.input_mode == InputMode::Editing {
            self.reset_edit_cursor_position();
        } else {
            self.reset_cursor_position();
        }
    }

    /// Reset cursor position to end of current input
    pub fn reset_cursor_position(&mut self) {
        self.cursor_position = match self.input_field {
            InputField::Description => self.description_input.len(),
            InputField::Priority => self.priority_input.len(),
            InputField::Projects => self.projects_input.len(),
            InputField::Contexts => self.contexts_input.len(),
            InputField::DueDate => self.due_date_input.len(),
        };
    }

    /// Reset cursor position for edit inputs
    pub fn reset_edit_cursor_position(&mut self) {
        self.cursor_position = match self.input_field {
            InputField::Description => self.edit_description.len(),
            InputField::Priority => self.edit_priority.len(),
            InputField::Projects => self.edit_projects.len(),
            InputField::Contexts => self.edit_contexts.len(),
            InputField::DueDate => self.edit_due_date.len(),
        };
    }

    /// Handle character input for the active field
    pub fn handle_char(&mut self, c: char) {
        let is_priority = self.input_field == InputField::Priority;

        if is_priority {
            let c = c.to_ascii_uppercase();
            if c.is_ascii_uppercase() {
                self.set_input_string(c.to_string());
                self.cursor_position = 1;
            }
            return;
        }

        let pos = self.cursor_position;
        self.insert_char_at(pos, c);
        self.cursor_position = pos + 1;
    }

    /// Insert a character at a specific position (clamped to valid bounds)
    fn insert_char_at(&mut self, pos: usize, c: char) {
        match self.input_mode {
            InputMode::Adding => match self.input_field {
                InputField::Description => {
                    let pos = pos.min(self.description_input.len());
                    self.description_input.insert(pos, c);
                }
                InputField::Priority => {
                    let pos = pos.min(self.priority_input.len());
                    self.priority_input.insert(pos, c);
                }
                InputField::Projects => {
                    let pos = pos.min(self.projects_input.len());
                    self.projects_input.insert(pos, c);
                }
                InputField::Contexts => {
                    let pos = pos.min(self.contexts_input.len());
                    self.contexts_input.insert(pos, c);
                }
                InputField::DueDate => {
                    let pos = pos.min(self.due_date_input.len());
                    self.due_date_input.insert(pos, c);
                }
            },
            InputMode::Editing => match self.input_field {
                InputField::Description => {
                    let pos = pos.min(self.edit_description.len());
                    self.edit_description.insert(pos, c);
                }
                InputField::Priority => {
                    let pos = pos.min(self.edit_priority.len());
                    self.edit_priority.insert(pos, c);
                }
                InputField::Projects => {
                    let pos = pos.min(self.edit_projects.len());
                    self.edit_projects.insert(pos, c);
                }
                InputField::Contexts => {
                    let pos = pos.min(self.edit_contexts.len());
                    self.edit_contexts.insert(pos, c);
                }
                InputField::DueDate => {
                    let pos = pos.min(self.edit_due_date.len());
                    self.edit_due_date.insert(pos, c);
                }
            },
            _ => {}
        }
    }

    /// Set the current input string
    fn set_input_string(&mut self, s: String) {
        match self.input_mode {
            InputMode::Adding => match self.input_field {
                InputField::Description => self.description_input = s,
                InputField::Priority => self.priority_input = s,
                InputField::Projects => self.projects_input = s,
                InputField::Contexts => self.contexts_input = s,
                InputField::DueDate => self.due_date_input = s,
            },
            InputMode::Editing => match self.input_field {
                InputField::Description => self.edit_description = s,
                InputField::Priority => self.edit_priority = s,
                InputField::Projects => self.edit_projects = s,
                InputField::Contexts => self.edit_contexts = s,
                InputField::DueDate => self.edit_due_date = s,
            },
            _ => {}
        }
    }

    /// Get reference to current input field based on input_mode
    pub fn get_current_input(&self) -> &str {
        match self.input_mode {
            InputMode::Adding => match self.input_field {
                InputField::Description => &self.description_input,
                InputField::Priority => &self.priority_input,
                InputField::Projects => &self.projects_input,
                InputField::Contexts => &self.contexts_input,
                InputField::DueDate => &self.due_date_input,
            },
            InputMode::Editing => match self.input_field {
                InputField::Description => &self.edit_description,
                InputField::Priority => &self.edit_priority,
                InputField::Projects => &self.edit_projects,
                InputField::Contexts => &self.edit_contexts,
                InputField::DueDate => &self.edit_due_date,
            },
            InputMode::ChangingPath => &self.path_input,
            _ => "",
        }
    }

    // === Missing methods required by main.rs ===

    /// Toggle help mode
    pub fn toggle_help(&mut self) {
        self.input_mode = if self.input_mode == InputMode::Help {
            InputMode::Normal
        } else {
            InputMode::Help
        };
    }

    /// Submit a new task from form input
    pub fn submit_task(&mut self) -> Result<()> {
        let input = todo_core::TaskInput {
            description: self.description_input.clone(),
            priority: self
                .priority_input
                .chars()
                .next()
                .filter(|c| c.is_ascii_uppercase()),
            projects: self
                .projects_input
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            contexts: self
                .contexts_input
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            due_date: if self.due_date_input.is_empty() {
                None
            } else {
                Some(self.due_date_input.clone())
            },
        };

        self.service.add_task(input)?;
        self.refresh()?;
        self.input_mode = InputMode::Normal;
        self.focus = Focus::MainList;
        self.status_message = Some("Task added".to_string());
        self.reset_inputs();
        Ok(())
    }

    /// Submit edited task
    pub fn submit_edit(&mut self) -> Result<()> {
        let task_id = match self.editing_task_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let input = todo_core::TaskInput {
            description: self.edit_description.clone(),
            priority: self
                .edit_priority
                .chars()
                .next()
                .filter(|c| c.is_ascii_uppercase()),
            projects: self
                .edit_projects
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            contexts: self
                .edit_contexts
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            due_date: if self.edit_due_date.is_empty() {
                None
            } else {
                Some(self.edit_due_date.clone())
            },
        };

        self.service.update_task(task_id, input)?;
        self.refresh()?;
        self.input_mode = InputMode::Normal;
        self.focus = Focus::MainList;
        self.editing_task_id = None;
        self.status_message = Some("Task updated".to_string());
        Ok(())
    }

    /// Move cursor left in current input field
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right in current input field
    pub fn move_cursor_right(&mut self) {
        let input_len = self.get_current_input().len();
        if self.cursor_position < input_len {
            self.cursor_position += 1;
        }
    }

    /// Move cursor to start of current input field
    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to end of current input field
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.get_current_input().len();
    }

    /// Handle backspace in current input field
    pub fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            match self.input_mode {
                InputMode::Adding => match self.input_field {
                    InputField::Description => {
                        self.description_input.pop();
                    }
                    InputField::Priority => {
                        self.priority_input.clear();
                    }
                    InputField::Projects => {
                        self.projects_input.pop();
                    }
                    InputField::Contexts => {
                        self.contexts_input.pop();
                    }
                    InputField::DueDate => {
                        self.due_date_input.pop();
                    }
                },
                InputMode::Editing => match self.input_field {
                    InputField::Description => {
                        self.edit_description.pop();
                    }
                    InputField::Priority => {
                        self.edit_priority.clear();
                    }
                    InputField::Projects => {
                        self.edit_projects.pop();
                    }
                    InputField::Contexts => {
                        self.edit_contexts.pop();
                    }
                    InputField::DueDate => {
                        self.edit_due_date.pop();
                    }
                },
                InputMode::ChangingPath => {
                    self.path_input.pop();
                }
                _ => {}
            }
            self.cursor_position = self.cursor_position.saturating_sub(1);
        }
    }

    /// Start changing the todo.txt path
    /// Shows the folder path (parent directory of todo.txt) for better UX
    pub fn start_change_path(&mut self) {
        self.input_mode = InputMode::ChangingPath;
        self.focus = Focus::MainList;
        // Show folder path instead of todo.txt path
        self.path_input = self
            .service
            .todo_path()
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        self.cursor_position = self.path_input.len();
    }

    /// Cancel path change and return to normal mode
    pub fn cancel_change_path(&mut self) {
        self.input_mode = InputMode::Normal;
        self.path_input.clear();
        self.status_message = Some("Path change cancelled".to_string());
    }

    /// Submit the path change
    /// Accepts either:
    /// - A folder path: checks if it contains todo.txt, creates if not exists
    /// - A direct todo.txt file path: uses it directly
    pub fn submit_path_change(&mut self) -> Result<()> {
        let input_path = std::path::PathBuf::from(&self.path_input);

        // Determine the actual todo.txt path
        let todo_path = if input_path.is_file() {
            // User entered a direct file path
            input_path
        } else if input_path.is_dir() || !input_path.exists() {
            // User entered a folder path (or a new path)
            // If path doesn't exist, treat as folder and create todo.txt inside
            let folder = if input_path.exists() {
                input_path
            } else {
                // User might be typing a new path, try to create it
                std::fs::create_dir_all(&input_path)
                    .map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))?;
                input_path
            };
            folder.join("todo.txt")
        } else {
            // Path exists but is neither a file nor a directory
            self.status_message = Some("Invalid path".to_string());
            return Ok(());
        };

        // Ensure todo.txt exists (create if needed)
        if !todo_path.exists() {
            std::fs::File::create(&todo_path).map_err(|e| {
                self.status_message = Some(format!("Failed to create todo.txt: {}", e));
                anyhow::anyhow!("Failed to create todo.txt: {}", e)
            })?;
        }

        // Save path and reload
        todo_core::save_todo_path(&todo_path)?;
        self.service = TaskService::new(&todo_path);
        self.current_todo_path = todo_path.to_string_lossy().to_string();
        self.refresh()?;
        self.input_mode = InputMode::Normal;
        self.path_input.clear();
        self.status_message = Some("Path updated".to_string());
        Ok(())
    }

    /// Handle character input for path change mode
    pub fn handle_path_char(&mut self, c: char) {
        self.path_input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    /// Handle backspace in path change mode
    pub fn handle_path_backspace(&mut self) {
        self.handle_backspace();
    }
}
