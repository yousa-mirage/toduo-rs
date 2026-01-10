//! Path configuration management
//!
//! Stores and retrieves the last used todo.txt path in a config file.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// Configuration file name
const CONFIG_FILE_NAME: &str = "config.toml";

/// Toml representation of the config
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ConfigToml {
    /// Last used todo.txt path
    todo_path: Option<String>,
}

/// Get the config directory (~/.todo)
pub fn get_config_dir() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let config_dir = home_dir.join(".todo");

    // Ensure the directory exists
    fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

    Ok(config_dir)
}

/// Get the config file path
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(CONFIG_FILE_NAME))
}

/// Load the saved todo path from config
pub fn load_saved_todo_path() -> Result<Option<PathBuf>> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(None);
    }

    let mut content = String::new();
    File::open(&config_path)
        .with_context(|| format!("Failed to open config file: {:?}", config_path))?
        .read_to_string(&mut content)
        .with_context(|| "Failed to read config file")?;

    let config: ConfigToml = toml::from_str(&content)
        .with_context(|| "Failed to parse config file")?;

    Ok(config.todo_path.map(PathBuf::from))
}

/// Save the todo path to config
pub fn save_todo_path(path: &Path) -> Result<()> {
    let config_path = get_config_path()?;

    let config = ConfigToml {
        todo_path: Some(path.to_string_lossy().to_string()),
    };

    let toml_content = toml::to_string(&config)
        .with_context(|| "Failed to serialize config")?;

    let mut file = File::create(&config_path)
        .with_context(|| format!("Failed to create config file: {:?}", config_path))?;
    
    file.write_all(toml_content.as_bytes())
        .with_context(|| "Failed to write config file")?;

    Ok(())
}

/// Get the default todo.txt path (~/.todo/todo.txt)
pub fn get_default_todo_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    let todo_path = config_dir.join("todo.txt");

    // Create empty file if it doesn't exist
    if !todo_path.exists() {
        File::create(&todo_path)
            .with_context(|| format!("Failed to create todo.txt: {:?}", todo_path))?;
    }

    Ok(todo_path)
}

/// Get the todo path to use, preferring saved path over default
pub fn get_todo_path() -> Result<PathBuf> {
    // First try to load saved path
    if let Some(saved_path) = load_saved_todo_path()? {
        // Ensure the file exists
        if saved_path.exists() {
            return Ok(saved_path);
        }
    }

    // Fall back to default path
    get_default_todo_path()
}
