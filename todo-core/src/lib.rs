//! todo-core: Core library for todo.txt task management
//!
//! This crate provides:
//! - Task parsing using the `todo-txt` crate
//! - File I/O operations
//! - TaskService for managing todo.txt files
//! - Path configuration management for remembering user preferences

mod config;
mod error;
mod model;
mod service;

pub use config::{get_todo_path, save_todo_path};
pub use error::TodoError;
pub use model::{AppTask, TaskInput};
pub use service::TaskService;

// Re-export commonly used types from todo-txt
pub use todo_txt::task::Simple as RawTask;
pub use todo_txt::Priority;
