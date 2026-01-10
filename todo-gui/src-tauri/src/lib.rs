//! Tauri backend for todo-gui
//!
//! Provides Tauri commands to interact with todo-core

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use todo_core::{AppTask, TaskInput, TaskService};

/// Application state managed by Tauri
pub struct AppState {
    pub service: Mutex<Option<TaskService>>,
    pub todo_path: PathBuf,
}

/// Serializable task for frontend
#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub id: usize,
    pub subject: String,
    pub priority: Option<char>,
    pub completed: bool,
    pub create_date: Option<String>,
    pub finish_date: Option<String>,
    pub due_date: Option<String>,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub raw_content: String,
}

impl From<AppTask> for Task {
    fn from(task: AppTask) -> Self {
        Self {
            id: task.id,
            subject: task.subject,
            priority: task.priority,
            completed: task.completed,
            create_date: task.create_date,
            finish_date: task.finish_date,
            due_date: task.due_date,
            projects: task.projects,
            contexts: task.contexts,
            raw_content: task.raw_content,
        }
    }
}

/// Input for creating a new task
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskInput {
    pub description: String,
    pub priority: Option<char>,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub due_date: Option<String>,
}

impl From<CreateTaskInput> for TaskInput {
    fn from(input: CreateTaskInput) -> Self {
        Self {
            description: input.description,
            priority: input.priority,
            projects: input.projects,
            contexts: input.contexts,
            due_date: input.due_date,
        }
    }
}

/// Get all tasks
#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_ref().ok_or("No todo file selected")?;
    service
        .load_tasks()
        .map(|tasks| tasks.into_iter().map(Task::from).collect())
        .map_err(|e| e.to_string())
}

/// Add a new task
#[tauri::command]
fn add_task(state: State<AppState>, input: CreateTaskInput) -> Result<Task, String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service
        .add_task(input.into())
        .map(Task::from)
        .map_err(|e| e.to_string())
}

/// Complete a task
#[tauri::command]
fn complete_task(state: State<AppState>, id: usize) -> Result<Task, String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service
        .complete_task(id)
        .map(Task::from)
        .map_err(|e| e.to_string())
}

/// Uncomplete a task
#[tauri::command]
fn uncomplete_task(state: State<AppState>, id: usize) -> Result<Task, String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service
        .uncomplete_task(id)
        .map(Task::from)
        .map_err(|e| e.to_string())
}

/// Set task priority
#[tauri::command]
fn set_priority(state: State<AppState>, id: usize, priority: Option<char>) -> Result<Task, String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service
        .set_priority(id, priority)
        .map(Task::from)
        .map_err(|e| e.to_string())
}

/// Delete a task
#[tauri::command]
fn delete_task(state: State<AppState>, id: usize) -> Result<(), String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service.delete_task(id).map_err(|e| e.to_string())
}

/// Get all projects
#[tauri::command]
fn get_projects(state: State<AppState>) -> Result<Vec<String>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_ref().ok_or("No todo file selected")?;
    service.get_all_projects().map_err(|e| e.to_string())
}

/// Get all contexts
#[tauri::command]
fn get_contexts(state: State<AppState>) -> Result<Vec<String>, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_ref().ok_or("No todo file selected")?;
    service.get_all_contexts().map_err(|e| e.to_string())
}

/// Get current todo file path
#[tauri::command]
fn get_todo_path(state: State<AppState>) -> String {
    state.todo_path.to_string_lossy().to_string()
}

/// Select todo file directory
#[tauri::command]
async fn select_todo_directory(
    state: State<'_, AppState>,
    app: tauri::Window,
) -> Result<bool, String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_directory(std::path::PathBuf::from("."))
        .set_title("Select todo.txt directory")
        .pick_folder(move |path| {
            tx.send(path).ok();
        });

    let path = rx.recv_timeout(std::time::Duration::from_secs(60));

    if let Ok(Some(path)) = path {
        // Convert FilePath to PathBuf
        let path_buf = path
            .as_path()
            .ok_or_else(|| String::from("Invalid path"))?
            .to_path_buf();
        let todo_path = path_buf.join("todo.txt");

        if !todo_path.exists() {
            std::fs::File::create(&todo_path)
                .map_err(|e| format!("Failed to create todo.txt: {}", e))?;
        }

        let mut service = state.service.lock().map_err(|e| e.to_string())?;
        *service = Some(TaskService::new(&todo_path));

        return Ok(true);
    }

    Ok(false)
}

/// Initialize todo service with the given directory
#[tauri::command]
fn init_todo_directory(state: State<AppState>, directory: String) -> Result<(), String> {
    let todo_path = PathBuf::from(directory).join("todo.txt");

    if !todo_path.exists() {
        std::fs::File::create(&todo_path)
            .map_err(|e| format!("Failed to create todo.txt: {}", e))?;
    }

    let service = TaskService::new(&todo_path);
    let mut svc = state.service.lock().map_err(|e| e.to_string())?;
    *svc = Some(service);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let todo_path = dirs::home_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(".todo")
        .join("todo.txt");

    if let Some(parent) = todo_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let service = TaskService::new(&todo_path);

    if let Err(e) = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            service: Mutex::new(Some(service)),
            todo_path,
        })
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            add_task,
            complete_task,
            uncomplete_task,
            set_priority,
            delete_task,
            get_projects,
            get_contexts,
            get_todo_path,
            select_todo_directory,
            init_todo_directory,
        ])
        .run(tauri::generate_context!())
    {
        eprintln!("Error while running tauri application: {}", e);
        std::process::exit(1);
    }
}
