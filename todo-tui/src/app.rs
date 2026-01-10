//! Application state and logic

use anyhow::Result;

use todo_core::{get_todo_path, save_todo_path, AppTask, TaskInput, TaskService};

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

/// Application state
pub struct App {
    /// Task service for file operations
    pub service: TaskService,
    /// Current list of tasks
    pub tasks: Vec<AppTask>,
    /// Currently selected task index
    pub selected: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input field when adding
    pub input_field: InputField,
    /// Input strings
    pub description_input: String,
    pub priority_input: String,
    pub projects_input: String,
    pub contexts_input: String,
    pub due_date_input: String,
    /// Status message
    pub status_message: Option<String>,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Result<Self> {
        let todo_path = get_todo_path()?;
        let service = TaskService::new(&todo_path);
        let tasks = service.load_tasks()?;

        Ok(Self {
            service,
            tasks,
            selected: 0,
            input_mode: InputMode::Normal,
            input_field: InputField::Description,
            description_input: String::new(),
            priority_input: String::new(),
            projects_input: String::new(),
            contexts_input: String::new(),
            due_date_input: String::new(),
            status_message: None,
        })
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
        if !self.tasks.is_empty() {
            self.selected = (self.selected + 1) % self.tasks.len();
        }
    }

    /// Move selection to previous task
    pub fn previous(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = self.selected.checked_sub(1).unwrap_or(self.tasks.len() - 1);
        }
    }

    /// Go to first task
    pub fn go_top(&mut self) {
        self.selected = 0;
    }

    /// Go to last task
    pub fn go_bottom(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = self.tasks.len() - 1;
        }
    }

    /// Toggle completion of selected task
    pub fn toggle_complete(&mut self) -> Result<()> {
        if let Some(task) = self.tasks.get(self.selected) {
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
        if let Some(task) = self.tasks.get(self.selected) {
            let id = task.id;
            self.service.delete_task(id)?;
            self.refresh()?;
            if self.selected >= self.tasks.len() && !self.tasks.is_empty() {
                self.selected = self.tasks.len() - 1;
            }
            self.status_message = Some("Task deleted".to_string());
        }
        Ok(())
    }

    /// Set priority of selected task
    pub fn set_priority_selected(&mut self, priority: Option<char>) -> Result<()> {
        if let Some(task) = self.tasks.get(self.selected) {
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
        self.tasks = self.service.load_tasks()?;
        if self.selected >= self.tasks.len() && !self.tasks.is_empty() {
            self.selected = self.tasks.len() - 1;
        }
        Ok(())
    }

    /// Start adding a new task
    pub fn start_add_task(&mut self) {
        self.input_mode = InputMode::Adding;
        self.reset_inputs();
    }

    /// Cancel input and return to normal mode
    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
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
                self.status_message = Some("Task added".to_string());
                // Select the newly added task
                if !self.tasks.is_empty() {
                    self.selected = self.tasks.len() - 1;
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
