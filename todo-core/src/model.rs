//! Data models for todo-core

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use todo_txt::Priority;
use todo_txt::task::Simple as RawTask;

/// Due date status for UI display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DueStatus {
    /// Due date is today - red
    Today,
    /// Due date is within 7 days - orange
    Soon,
    /// Due date is in the past - brown
    Overdue,
    /// Due date is more than 7 days away - gray
    Later,
    /// No due date set
    None,
}

impl DueStatus {
    /// Calculate DueStatus from a due date string (YYYY-MM-DD format)
    pub fn from_due_date(due_date: Option<&str>) -> Self {
        let due_date = match due_date {
            Some(d) => d,
            None => return Self::None,
        };

        // Parse the due date
        let due = match NaiveDate::parse_from_str(due_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return Self::None,
        };

        // Get today's date (local time)
        let today = chrono::Local::now().date_naive();

        // Calculate dates for comparison
        let next_7_days = today + chrono::Duration::days(7);

        if due < today {
            Self::Overdue
        } else if due == today {
            Self::Today
        } else if due <= next_7_days {
            Self::Soon
        } else {
            Self::Later
        }
    }
}

/// Application-level task wrapper
///
/// Wraps the raw `todo-txt` Task with additional metadata needed for UI operations.
/// The `id` corresponds to the physical line number (1-based) in the todo.txt file.
#[derive(Debug, Clone, Serialize)]
pub struct AppTask {
    /// Line number in the file (1-based index)
    pub id: usize,
    /// Original raw text content
    pub raw_content: String,
    /// Parsed task data from todo-txt crate
    #[serde(skip)]
    pub parsed: RawTask,
    // Serializable fields extracted from parsed task
    /// Task description/subject
    pub subject: String,
    /// Priority (A-Z) or None
    pub priority: Option<char>,
    /// Whether the task is completed
    pub completed: bool,
    /// Creation date
    pub create_date: Option<String>,
    /// Completion date (only if completed)
    pub finish_date: Option<String>,
    /// Due date from due:YYYY-MM-DD tag
    pub due_date: Option<String>,
    /// Calculated due status for UI display
    pub due_status: DueStatus,
    /// Project tags (+project)
    pub projects: Vec<String>,
    /// Context tags (@context)
    pub contexts: Vec<String>,
}

impl AppTask {
    /// Create an AppTask from a raw task and its line number
    pub fn from_raw(id: usize, raw_content: String, parsed: RawTask) -> Self {
        let subject = parsed.subject.clone();
        let priority = if parsed.priority.is_lowest() {
            None
        } else {
            Some((b'A' + parsed.priority.get_value()) as char)
        };
        let completed = parsed.finished;
        let create_date = parsed.create_date.map(|d| d.format("%Y-%m-%d").to_string());
        let finish_date = parsed.finish_date.map(|d| d.format("%Y-%m-%d").to_string());
        let due_date = parsed.due_date.map(|d| d.format("%Y-%m-%d").to_string());
        let due_status = DueStatus::from_due_date(due_date.as_deref());
        let projects = parsed.projects.clone();
        let contexts = parsed.contexts.clone();

        let clean_subject = subject
            .replace(['\r', '\n'], "")
            .split_whitespace()
            .filter(|word| !word.starts_with('@') && !word.starts_with('+'))
            .collect::<Vec<_>>()
            .join(" ");

        Self {
            id,
            raw_content,
            parsed,
            subject: clean_subject,
            priority,
            completed,
            create_date,
            finish_date,
            due_date,
            due_status,
            projects,
            contexts,
        }
    }

    /// Convert the task back to todo.txt format string
    pub fn to_todo_txt(&self) -> String {
        self.parsed.to_string()
    }
}

/// Input data for creating a new task
///
/// This is the structured input from the UI that will be compiled
/// into a valid todo.txt format string.
#[derive(Debug, Clone, Deserialize)]
pub struct TaskInput {
    /// Main task description
    pub description: String,
    /// Priority (A-Z) or None
    pub priority: Option<char>,
    /// Project tags (without + prefix)
    pub projects: Vec<String>,
    /// Context tags (without @ prefix)
    pub contexts: Vec<String>,
    /// Due date in YYYY-MM-DD format
    pub due_date: Option<String>,
}

impl TaskInput {
    /// Build a todo.txt format string from the input
    pub fn to_todo_txt(&self) -> String {
        let mut parts = Vec::new();

        // Priority
        if let Some(p) = self.priority {
            parts.push(format!("({}) ", p.to_ascii_uppercase()));
        }

        // Creation date (today)
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        parts.push(format!("{} ", today));

        // Description
        parts.push(self.description.clone());

        // Projects
        for proj in &self.projects {
            parts.push(format!(" +{}", proj));
        }

        // Contexts
        for ctx in &self.contexts {
            parts.push(format!(" @{}", ctx));
        }

        // Due date
        if let Some(ref due) = self.due_date {
            parts.push(format!(" due:{}", due));
        }

        parts.concat()
    }

    /// Validate the input
    pub fn validate(&self) -> Result<(), String> {
        if self.description.trim().is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        if let Some(p) = self.priority {
            if !p.is_ascii_uppercase() {
                return Err("Priority must be A-Z".to_string());
            }
        }

        if let Some(ref due) = self.due_date {
            if NaiveDate::parse_from_str(due, "%Y-%m-%d").is_err() {
                return Err("Due date must be in YYYY-MM-DD format".to_string());
            }
        }

        Ok(())
    }
}

/// Helper trait for Priority conversion
pub trait PriorityExt {
    fn get_value(&self) -> u8;
}

impl PriorityExt for Priority {
    fn get_value(&self) -> u8 {
        // Priority displays as (A), (B), etc. We need to extract the letter offset
        let s = self.to_string();
        if s.len() == 1 {
            if let Some(c) = s.chars().next() {
                if c.is_ascii_uppercase() {
                    return c as u8 - b'A';
                }
            }
        }
        26 // lowest priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_input_to_todo_txt() {
        let input = TaskInput {
            description: "Write documentation".to_string(),
            priority: Some('A'),
            projects: vec!["todo-app".to_string()],
            contexts: vec!["work".to_string()],
            due_date: Some("2026-01-15".to_string()),
        };

        let result = input.to_todo_txt();
        assert!(result.contains("(A)"));
        assert!(result.contains("Write documentation"));
        assert!(result.contains("+todo-app"));
        assert!(result.contains("@work"));
        assert!(result.contains("due:2026-01-15"));
    }

    #[test]
    fn test_due_status() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let yesterday = {
            let d = chrono::Local::now().date_naive() - chrono::Duration::days(1);
            d.format("%Y-%m-%d").to_string()
        };
        let tomorrow = {
            let d = chrono::Local::now().date_naive() + chrono::Duration::days(1);
            d.format("%Y-%m-%d").to_string()
        };
        // Use 8 days to be safely in the "Later" category (boundary is 7 days)
        let next_week = {
            let d = chrono::Local::now().date_naive() + chrono::Duration::days(8);
            d.format("%Y-%m-%d").to_string()
        };

        assert_eq!(
            DueStatus::from_due_date(Some(&yesterday)),
            DueStatus::Overdue
        );
        assert_eq!(DueStatus::from_due_date(Some(&today)), DueStatus::Today);
        assert_eq!(DueStatus::from_due_date(Some(&tomorrow)), DueStatus::Soon);
        assert_eq!(DueStatus::from_due_date(Some(&next_week)), DueStatus::Later);
        assert_eq!(DueStatus::from_due_date(None), DueStatus::None);
    }

    #[test]
    fn test_task_parsing_subject_no_tags() {
        use std::str::FromStr;
        let raw = "(A) 2026-01-10 Test task +project @context due:2026-01-15";
        let parsed = RawTask::from_str(raw).unwrap();

        let app_task = AppTask::from_raw(1, raw.to_string(), parsed);

        assert!(!app_task.subject.contains("@context"));
        assert!(!app_task.subject.contains("+project"));

        assert_eq!(app_task.contexts, vec!["context"]);
        assert_eq!(app_task.projects, vec!["project"]);
    }

    #[test]
    fn test_task_input_validation() {
        let valid_input = TaskInput {
            description: "Valid task".to_string(),
            priority: Some('B'),
            projects: vec![],
            contexts: vec![],
            due_date: None,
        };
        assert!(valid_input.validate().is_ok());

        let empty_desc = TaskInput {
            description: "   ".to_string(),
            priority: None,
            projects: vec![],
            contexts: vec![],
            due_date: None,
        };
        assert!(empty_desc.validate().is_err());

        let invalid_priority = TaskInput {
            description: "Task".to_string(),
            priority: Some('a'),
            projects: vec![],
            contexts: vec![],
            due_date: None,
        };
        assert!(invalid_priority.validate().is_err());
    }

    #[test]
    fn test_priority_ext_trait() {
        use todo_txt::Priority;

        let priority_a = Priority::from(0u8);
        let priority_z = Priority::from(25u8);
        let lowest = Priority::lowest();

        assert_eq!(priority_a.get_value(), 0);
        assert_eq!(priority_z.get_value(), 25);
        assert_eq!(lowest.get_value(), 26);
    }

    #[test]
    fn test_due_status_boundary_dates() {
        let today = chrono::Local::now().date_naive();
        let day_after_next = today + chrono::Duration::days(8);
        let day_before_next = today + chrono::Duration::days(7);

        assert_eq!(
            DueStatus::from_due_date(Some(&day_after_next.format("%Y-%m-%d").to_string())),
            DueStatus::Later
        );
        assert_eq!(
            DueStatus::from_due_date(Some(&day_before_next.format("%Y-%m-%d").to_string())),
            DueStatus::Soon
        );
    }

    #[test]
    fn test_task_input_empty_projects_contexts() {
        let input = TaskInput {
            description: "Simple task".to_string(),
            priority: None,
            projects: vec![],
            contexts: vec![],
            due_date: None,
        };

        let result = input.to_todo_txt();
        assert!(result.contains("Simple task"));
        assert!(!result.contains('+'));
        assert!(!result.contains('@'));
    }

    #[test]
    fn test_task_input_multiple_projects_contexts() {
        let input = TaskInput {
            description: "Multi task".to_string(),
            priority: Some('B'),
            projects: vec!["project1".to_string(), "project2".to_string()],
            contexts: vec!["home".to_string(), "work".to_string()],
            due_date: Some("2026-02-01".to_string()),
        };

        let result = input.to_todo_txt();
        assert!(result.contains("(B)"));
        assert!(result.contains("+project1"));
        assert!(result.contains("+project2"));
        assert!(result.contains("@home"));
        assert!(result.contains("@work"));
        assert!(result.contains("due:2026-02-01"));
    }
}
