//! Custom error types for todo-core

use thiserror::Error;

/// Custom error type for todo-core operations
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Failed to read todo file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse task at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Task not found with id: {0}")]
    TaskNotFound(usize),

    #[error("Invalid task input: {0}")]
    InvalidInput(String),
}
