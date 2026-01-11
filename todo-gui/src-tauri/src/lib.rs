//! Tauri backend for todo-gui
//!
//! Provides Tauri commands to interact with todo-core

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use todo_core::{AppTask, DueStatus, TaskInput, TaskService, save_todo_path};

/// Application state managed by Tauri
pub struct AppState {
    pub service: Mutex<Option<TaskService>>,
    pub todo_path: Mutex<PathBuf>,
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
    pub due_status: DueStatus,
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
            due_status: task.due_status,
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

#[tauri::command]
fn update_task(state: State<AppState>, id: usize, input: CreateTaskInput) -> Result<Task, String> {
    let mut service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_mut().ok_or("No todo file selected")?;
    service
        .update_task(id, input.into())
        .map(Task::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_task(state: State<AppState>, id: usize) -> Result<Task, String> {
    let service = state.service.lock().map_err(|e| e.to_string())?;
    let service = service.as_ref().ok_or("No todo file selected")?;
    service
        .load_tasks()
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|t| t.id == id)
        .map(Task::from)
        .ok_or_else(|| format!("Task with id {} not found", id))
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
fn get_todo_path(state: State<AppState>) -> Result<String, String> {
    let path = state.todo_path.lock().map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

/// Write text to clipboard
#[tauri::command]
fn write_text_to_clipboard(text: String) -> Result<(), String> {
    use clipboard_win::Unicode;
    use clipboard_win::set;
    set(Unicode, text).map_err(|e| format!("Failed to write to clipboard: {}", e))
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
        let path_buf = path
            .as_path()
            .ok_or_else(|| String::from("Invalid path"))?
            .to_path_buf();
        let todo_path = path_buf.join("todo.txt");

        if !todo_path.exists() {
            std::fs::File::create(&todo_path)
                .map_err(|e| format!("Failed to create todo.txt: {}", e))?;
        }

        save_todo_path(&todo_path).map_err(|e| format!("Failed to save todo path: {}", e))?;

        let mut state_guard = state.service.lock().map_err(|e| e.to_string())?;
        *state_guard = Some(TaskService::new(&todo_path));

        drop(state_guard);
        let mut path_guard = state.todo_path.lock().map_err(|e| e.to_string())?;
        *path_guard = todo_path.clone();

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

    // Save the path to config
    save_todo_path(&todo_path).map_err(|e| format!("Failed to save todo path: {}", e))?;

    let service = TaskService::new(&todo_path);
    let mut svc = state.service.lock().map_err(|e| e.to_string())?;
    *svc = Some(service);

    Ok(())
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Use saved path if available, otherwise use default (~/.todo/todo.txt)
    let todo_path = todo_core::get_todo_path().unwrap_or_else(|_| {
        dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
            .join(".todo")
            .join("todo.txt")
    });

    if let Some(parent) = todo_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let service = TaskService::new(&todo_path);

    if let Err(e) = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::Manager;
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Tasks", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let icon = if let Some(icon_ref) = app.default_window_icon() {
                icon_ref.clone()
            } else {
                tauri::image::Image::<'static>::new_owned(Vec::new(), 1, 1)
            };
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .manage(AppState {
            service: Mutex::new(Some(service)),
            todo_path: Mutex::new(todo_path),
        })
        .invoke_handler(tauri::generate_handler![
            get_tasks,
            get_task,
            add_task,
            complete_task,
            uncomplete_task,
            set_priority,
            update_task,
            delete_task,
            get_projects,
            get_contexts,
            get_todo_path,
            write_text_to_clipboard,
            select_todo_directory,
            init_todo_directory,
            exit_app,
        ])
        .run(tauri::generate_context!())
    {
        eprintln!("Error while running tauri application: {}", e);
        std::process::exit(1);
    }
}
