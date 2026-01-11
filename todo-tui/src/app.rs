//! Application state and logic

use anyhow::Result;

use todo_core::{AppTask, TaskInput, TaskService, get_todo_path, save_todo_path};

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal mode - browsing tasks
    Normal,
    /// Adding a new task
    Adding,
    /// Showing help
    Help,
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
    pub fn next(self) -> Self {
        match self {
            Self::Description => Self::Priority,
            Self::Priority => Self::Projects,
            Self::Projects => Self::Contexts,
            Self::Contexts => Self::DueDate,
            Self::DueDate => Self::Description,
        }
    }

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

/// Filter for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Today,
    Next7Days,
    Priority(char),
    #[allow(dead_code)]
    NoPriority,
    #[allow(dead_code)]
    Projects, // Just a placeholder for "By Project" if needed, or we list projects
    #[allow(dead_code)]
    Contexts, // Placeholder
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All Tasks"),
            Self::Today => write!(f, "Today"),
            Self::Next7Days => write!(f, "Next 7 Days"),
            Self::Priority(c) => write!(f, "Priority {}", c),
            Self::NoPriority => write!(f, "No Priority"),
            Self::Projects => write!(f, "Projects"),
            Self::Contexts => write!(f, "Contexts"),
        }
    }
}

/// Application state
pub struct App {
    /// Task service for file operations
    pub service: TaskService,
    /// All tasks loaded from file
    pub all_tasks: Vec<AppTask>,
    /// Filtered and sorted tasks for display
    pub view_tasks: Vec<AppTask>,
    /// Currently selected task index in view_tasks
    pub selected: usize,
    /// Current filter
    pub filter: Filter,
    /// Sidebar item index
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

    /// Status message
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    MainList,
    RightSidebar,
}

impl App {
    /// Create a new App instance
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
        };
        app.apply_filter();
        Ok(app)
    }

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
                Filter::NoPriority => t.priority.is_none(),
                _ => true,
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

    /// Reset all input fields
    fn reset_inputs(&mut self) {
        self.description_input.clear();
        self.priority_input.clear();
        self.projects_input.clear();
        self.contexts_input.clear();
        self.due_date_input.clear();
        self.input_field = InputField::Description;
    }

    /// Move selection to next task
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

    /// Move selection to previous task
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

    /// Go to first task
    pub fn go_top(&mut self) {
        self.selected = 0;
    }

    /// Go to last task
    pub fn go_bottom(&mut self) {
        if !self.view_tasks.is_empty() {
            self.selected = self.view_tasks.len() - 1;
        }
    }

    pub fn switch_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Sidebar => Focus::MainList,
            Focus::MainList => Focus::Sidebar,
            Focus::RightSidebar => Focus::MainList, // Esc from right sidebar usually cancels, but generic switch logic here
        };
    }

    /// Toggle completion of selected task
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

    /// Delete selected task
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

    /// Refresh tasks from file
    pub fn refresh(&mut self) -> Result<()> {
        self.all_tasks = self.service.load_tasks()?;
        self.apply_filter();
        Ok(())
    }

    /// Start adding a new task
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

    /// Move to next input field
    pub fn next_input_field(&mut self) {
        self.input_field = self.input_field.next();
    }

    /// Move to previous input field
    pub fn prev_input_field(&mut self) {
        self.input_field = self.input_field.prev();
    }

    /// Handle character input
    pub fn handle_char(&mut self, c: char) {
        let input = match self.input_field {
            InputField::Description => &mut self.description_input,
            InputField::Priority => &mut self.priority_input,
            InputField::Projects => &mut self.projects_input,
            InputField::Contexts => &mut self.contexts_input,
            InputField::DueDate => &mut self.due_date_input,
        };

        if self.input_field == InputField::Priority {
            let c = c.to_ascii_uppercase();
            if c.is_ascii_uppercase() {
                *input = c.to_string();
            }
            return;
        }

        input.push(c);
    }

    /// Handle backspace
    pub fn handle_backspace(&mut self) {
        let input = match self.input_field {
            InputField::Description => &mut self.description_input,
            InputField::Priority => &mut self.priority_input,
            InputField::Projects => &mut self.projects_input,
            InputField::Contexts => &mut self.contexts_input,
            InputField::DueDate => &mut self.due_date_input,
        };

        if self.input_field == InputField::Priority {
            input.clear();
        } else {
            input.pop();
        }
    }

    /// Submit the new task
    pub fn submit_task(&mut self) -> Result<()> {
        let description = self.description_input.trim().to_string();

        if description.is_empty() {
            self.status_message = Some("Description cannot be empty".to_string());
            return Ok(());
        }

        let priority = self
            .priority_input
            .chars()
            .next()
            .filter(|c| c.is_ascii_uppercase());

        let projects: Vec<String> = self
            .projects_input
            .split_whitespace()
            .map(|s| s.trim_start_matches('+').to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let contexts: Vec<String> = self
            .contexts_input
            .split_whitespace()
            .map(|s| s.trim_start_matches('@').to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let due_date = if self.due_date_input.trim().is_empty() {
            None
        } else {
            Some(self.due_date_input.trim().to_string())
        };

        let input = TaskInput {
            description,
            priority,
            projects,
            contexts,
            due_date,
        };

        match self.service.add_task(input) {
            Ok(_) => {
                self.refresh()?;
                self.input_mode = InputMode::Normal;
                self.focus = Focus::MainList;
                self.status_message = Some("Task added".to_string());
                // Select the newly added task
                if !self.view_tasks.is_empty() {
                    self.selected = self.view_tasks.len() - 1;
                }
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }

        Ok(())
    }

    /// Toggle help display
    pub fn toggle_help(&mut self) {
        self.input_mode = match self.input_mode {
            InputMode::Help => InputMode::Normal,
            _ => InputMode::Help,
        };
    }

    /// Change the todo directory and save the new path
    #[allow(dead_code)]
    pub fn change_todo_directory(&mut self, path: &str) -> Result<()> {
        let new_path = std::path::PathBuf::from(path);

        // Save the new path
        save_todo_path(&new_path)?;

        // Create new service with the new path
        self.service = TaskService::new(&new_path);

        // Load tasks from the new path
        self.refresh()?;
        self.selected = 0;
        self.status_message = Some(format!("Changed to: {}", path));

        Ok(())
    }
}
