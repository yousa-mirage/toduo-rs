//! TaskService: Main service for managing todo.txt files

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::Local;
use todo_txt::task::Simple as RawTask;

use crate::error::TodoError;
use crate::model::{AppTask, TaskInput};

/// Service for managing todo.txt file operations
#[derive(Debug, Clone)]
pub struct TaskService {
    /// Path to the todo.txt file
    todo_path: PathBuf,
    /// Path to the done.txt file (for archived completed tasks)
    done_path: PathBuf,
}

impl TaskService {
    /// Create a new TaskService with the specified todo.txt path
    pub fn new<P: AsRef<Path>>(todo_path: P) -> Self {
        let todo_path = todo_path.as_ref().to_path_buf();
        let done_path = todo_path.with_file_name("done.txt");

        Self {
            todo_path,
            done_path,
        }
    }

    /// Create a TaskService using the default config directory
    pub fn with_default_path() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        let todo_dir = home_dir.join(".todo");

        // Ensure the directory exists
        fs::create_dir_all(&todo_dir)
            .with_context(|| format!("Failed to create todo directory: {:?}", todo_dir))?;

        let todo_path = todo_dir.join("todo.txt");

        // Create empty file if it doesn't exist
        if !todo_path.exists() {
            File::create(&todo_path)
                .with_context(|| format!("Failed to create todo.txt: {:?}", todo_path))?;
        }

        Ok(Self::new(todo_path))
    }

    /// Get the path to the todo.txt file
    pub fn todo_path(&self) -> &Path {
        &self.todo_path
    }

    /// Get the path to the done.txt file
    pub fn done_path(&self) -> &Path {
        &self.done_path
    }

    /// Load all tasks from the todo.txt file
    pub fn load_tasks(&self) -> Result<Vec<AppTask>> {
        if !self.todo_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.todo_path)
            .with_context(|| format!("Failed to open {:?}", self.todo_path))?;

        let reader = BufReader::new(&file);
        let mut tasks = Vec::new();

        for (idx, line) in reader.lines().enumerate() {
            let line = line.with_context(|| format!("Failed to read line {}", idx + 1))?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let raw_task = RawTask::from_str(line).map_err(|_| TodoError::ParseError {
                line: idx + 1,
                message: "Failed to parse task".to_string(),
            })?;

            tasks.push(AppTask::from_raw(idx + 1, line.to_string(), raw_task));
        }

        Ok(tasks)
    }

    /// Add a new task to the todo.txt file
    pub fn add_task(&self, input: TaskInput) -> Result<AppTask> {
        input.validate().map_err(TodoError::InvalidInput)?;

        let todo_txt_line = input.to_todo_txt();

        let raw_task = RawTask::from_str(&todo_txt_line).map_err(|_| TodoError::ParseError {
            line: 0,
            message: "Failed to parse generated task".to_string(),
        })?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .with_context(|| format!("Failed to open {:?} for writing", self.todo_path))?;

        let new_id = self.count_lines()?;
        writeln!(&file, "{}", todo_txt_line).with_context(|| "Failed to write task to file")?;

        Ok(AppTask::from_raw(new_id, todo_txt_line, raw_task))
    }

    /// Mark a task as complete by ID
    pub fn complete_task(&self, id: usize) -> Result<AppTask> {
        let mut tasks = self.load_tasks()?;

        let task = tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        // Update the parsed task
        task.parsed.complete();
        task.completed = true;
        task.finish_date = Some(Local::now().format("%Y-%m-%d").to_string());
        task.raw_content = task.parsed.to_string();

        // Save all tasks back to file
        self.save_tasks(&tasks)?;

        // Return the updated task
        let updated_task = tasks
            .into_iter()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        Ok(updated_task)
    }

    /// Uncomplete a task by ID
    pub fn uncomplete_task(&self, id: usize) -> Result<AppTask> {
        let mut tasks = self.load_tasks()?;

        let task = tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        // Update the parsed task
        task.parsed.uncomplete();
        task.completed = false;
        task.finish_date = None;
        task.raw_content = task.parsed.to_string();

        // Save all tasks back to file
        self.save_tasks(&tasks)?;

        let updated_task = tasks
            .into_iter()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        Ok(updated_task)
    }

    /// Update a task's priority
    pub fn set_priority(&self, id: usize, priority: Option<char>) -> Result<AppTask> {
        if let Some(p) = priority {
            if !p.is_ascii_uppercase() {
                return Err(TodoError::InvalidInput("Priority must be A-Z".to_string()).into());
            }
        }

        let mut tasks = self.load_tasks()?;

        let task = tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        task.parsed.priority = match priority {
            Some(c) => todo_txt::Priority::from(c as u8 - b'A'),
            None => todo_txt::Priority::lowest(),
        };
        task.priority = priority;
        task.raw_content = task.parsed.to_string();

        self.save_tasks(&tasks)?;

        let updated_task = tasks
            .into_iter()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;
        Ok(updated_task)
    }

    /// Delete a task by ID
    pub fn delete_task(&self, id: usize) -> Result<()> {
        let tasks = self.load_tasks()?;

        if !tasks.iter().any(|t| t.id == id) {
            return Err(TodoError::TaskNotFound(id).into());
        }

        let remaining: Vec<_> = tasks.into_iter().filter(|t| t.id != id).collect();
        self.save_tasks(&remaining)?;

        Ok(())
    }

    /// Save tasks to the todo.txt file (overwrites entire file)
    fn save_tasks(&self, tasks: &[AppTask]) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.todo_path)
            .with_context(|| format!("Failed to open {:?} for writing", self.todo_path))?;

        let mut writer = std::io::BufWriter::new(&file);

        for task in tasks {
            writeln!(writer, "{}", task.parsed).with_context(|| "Failed to write task")?;
        }

        writer.flush()?;

        Ok(())
    }

    /// Count non-empty lines in the file
    fn count_lines(&self) -> Result<usize> {
        if !self.todo_path.exists() {
            return Ok(0);
        }

        let content = fs::read_to_string(&self.todo_path)
            .with_context(|| format!("Failed to read {:?}", self.todo_path))?;

        Ok(content.lines().filter(|l| !l.trim().is_empty()).count())
    }

    /// Get all unique projects from current tasks
    pub fn get_all_projects(&self) -> Result<Vec<String>> {
        let tasks = self.load_tasks()?;
        let mut projects: Vec<String> = tasks.iter().flat_map(|t| t.projects.clone()).collect();

        projects.sort();
        projects.dedup();
        Ok(projects)
    }

    /// Get all unique contexts from current tasks
    pub fn get_all_contexts(&self) -> Result<Vec<String>> {
        let tasks = self.load_tasks()?;
        let mut contexts: Vec<String> = tasks.iter().flat_map(|t| t.contexts.clone()).collect();

        contexts.sort();
        contexts.dedup();
        Ok(contexts)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_todo_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_load_tasks() {
        let file = create_temp_todo_file(
            "(A) 2026-01-10 Important task +project @context\n\
             (B) 2026-01-09 Another task\n\
             x 2026-01-08 2026-01-05 Completed task\n",
        );

        let service = TaskService::new(file.path());
        let tasks = service.load_tasks().unwrap();

        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks[0].priority, Some('A'));
        assert!(!tasks[0].completed);
        assert!(tasks[2].completed);
    }

    #[test]
    fn test_add_task() {
        let file = create_temp_todo_file("");
        let service = TaskService::new(file.path());

        let input = TaskInput {
            description: "New task".to_string(),
            priority: Some('A'),
            projects: vec!["test".to_string()],
            contexts: vec!["home".to_string()],
            due_date: None,
        };

        let task = service.add_task(input).unwrap();
        assert_eq!(task.priority, Some('A'));
        assert!(task.raw_content.contains("New task"));
        assert!(task.raw_content.contains("+test"));
        assert!(task.raw_content.contains("@home"));
    }

    #[test]
    fn test_complete_task() {
        let file = create_temp_todo_file("(A) 2026-01-10 Task to complete\n");
        let service = TaskService::new(file.path());

        let task = service.complete_task(1).unwrap();
        assert!(task.completed);
        assert!(task.finish_date.is_some());
    }

    #[test]
    fn test_uncomplete_task() {
        let file = create_temp_todo_file("x 2026-01-10 2026-01-08 Completed task\n");
        let service = TaskService::new(file.path());

        let task = service.uncomplete_task(1).unwrap();
        assert!(!task.completed);
        assert!(task.finish_date.is_none());
    }

    #[test]
    fn test_set_priority() {
        let file = create_temp_todo_file("(A) 2026-01-10 Task without priority\n");
        let service = TaskService::new(file.path());

        let task = service.set_priority(1, Some('B')).unwrap();
        assert_eq!(task.priority, Some('B'));
        assert!(task.raw_content.contains("(B)"));
    }

    #[test]
    fn test_remove_priority() {
        let file = create_temp_todo_file("(A) 2026-01-10 Task with priority\n");
        let service = TaskService::new(file.path());

        let task = service.set_priority(1, None).unwrap();
        assert!(task.priority.is_none());
    }

    #[test]
    fn test_delete_task() {
        let file = create_temp_todo_file("Task 1\nTask 2\nTask 3\n");
        let service = TaskService::new(file.path());

        service.delete_task(2).unwrap();

        let tasks = service.load_tasks().unwrap();
        assert_eq!(tasks.len(), 2);
        // After deletion, remaining tasks get new IDs based on their new line numbers
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[test]
    fn test_get_all_projects() {
        let file = create_temp_todo_file(
            "(A) Task +project1 @context1\n\
             (B) Task +project2\n\
             Task +project1 +project3\n",
        );
        let service = TaskService::new(file.path());

        let projects = service.get_all_projects().unwrap();
        assert_eq!(projects, vec!["project1", "project2", "project3"]);
    }

    #[test]
    fn test_get_all_contexts() {
        let file = create_temp_todo_file(
            "(A) Task +project1 @context1\n\
             (B) Task @context2 @context1\n\
             Task @context3\n",
        );
        let service = TaskService::new(file.path());

        let contexts = service.get_all_contexts().unwrap();
        assert_eq!(contexts, vec!["context1", "context2", "context3"]);
    }

    #[test]
    fn test_task_with_due_date() {
        let file = create_temp_todo_file("(A) 2026-01-10 Task with due date due:2026-01-20\n");
        let service = TaskService::new(file.path());

        let tasks = service.load_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].due_date, Some("2026-01-20".to_string()));
    }

    #[test]
    fn test_load_empty_file() {
        let file = create_temp_todo_file("");
        let service = TaskService::new(file.path());

        let tasks = service.load_tasks().unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_complete_nonexistent_task() {
        let file = create_temp_todo_file("Task 1\n");
        let service = TaskService::new(file.path());

        let result = service.complete_task(99);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_nonexistent_task() {
        let file = create_temp_todo_file("Task 1\n");
        let service = TaskService::new(file.path());

        let result = service.delete_task(99);
        assert!(result.is_err());
    }
}
