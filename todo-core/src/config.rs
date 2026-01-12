//! Path configuration management
//!
//! Stores and retrieves the last used todo.txt path in a config file.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Configuration file name
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GuiConfig {
    pub theme: Option<String>,
    pub close_to_tray: Option<bool>,
}

/// Toml representation of the config
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppConfig {
    /// Last used todo.txt path
    pub todo_path: Option<String>,
    /// GUI specific configuration
    pub gui: Option<GuiConfig>,
}

/// Get the config directory (~/.todo)
pub fn get_config_dir() -> Result<PathBuf> {
    // Allow overriding config directory via environment variable (useful for testing)
    if let Ok(env_dir) = std::env::var("TODO_APP_CONFIG_DIR") {
        let dir = PathBuf::from(env_dir);
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config directory from env: {:?}", dir))?;
        return Ok(dir);
    }

    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".todo");

    fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

    Ok(config_dir)
}

/// Get the config file path
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join(CONFIG_FILE_NAME))
}

/// Load the full configuration
pub fn load_config() -> Result<AppConfig> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

    toml::from_str(&content).with_context(|| "Failed to parse config file")
}

/// Save the full configuration
pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_path = get_config_path()?;
    let toml_content = toml::to_string_pretty(config)?;

    File::create(&config_path)?.write_all(toml_content.as_bytes())?;

    Ok(())
}

/// Load the saved todo path from config
pub fn load_saved_todo_path() -> Result<Option<PathBuf>> {
    Ok(load_config()?.todo_path.map(PathBuf::from))
}

/// Save the todo path to config
pub fn save_todo_path(path: &Path) -> Result<()> {
    let mut config = load_config().unwrap_or_default();
    config.todo_path = Some(path.to_string_lossy().to_string());
    save_config(&config)
}

/// Save the GUI configuration
pub fn save_gui_config(gui_config: GuiConfig) -> Result<()> {
    let mut config = load_config().unwrap_or_default();
    config.gui = Some(gui_config);
    save_config(&config)
}

/// Get the default todo.txt path (~/.todo/todo.txt)
pub fn get_default_todo_path() -> Result<PathBuf> {
    let todo_path = get_config_dir()?.join("todo.txt");

    if !todo_path.exists() {
        File::create(&todo_path)?;
    }

    Ok(todo_path)
}

/// Get the todo path to use, preferring saved path over default
pub fn get_todo_path() -> Result<PathBuf> {
    // Try to load saved path, ignoring errors (e.g., corrupted config)
    let saved_path = load_saved_todo_path().unwrap_or_default();
    if let Some(path) = saved_path.filter(|p| p.exists()) {
        Ok(path)
    } else {
        get_default_todo_path()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::TempDir;

    // Mutex to ensure tests manipulating environment variables run serially
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_save_and_load_todo_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_dir = TempDir::new().unwrap();

        // Redirect config to temp dir to avoid overwriting real user config
        unsafe {
            std::env::set_var("TODO_APP_CONFIG_DIR", temp_dir.path());
        }

        // Use a closure to ensure cleanup runs even if assertions fail (though panic still unwinds)
        let result = std::panic::catch_unwind(|| {
            let test_path = temp_dir.path().join("test_todo.txt");

            save_todo_path(&test_path).unwrap();

            let loaded = load_saved_todo_path().unwrap();
            assert!(loaded.is_some());
            assert_eq!(loaded.unwrap(), test_path);
        });

        // Cleanup
        unsafe {
            std::env::remove_var("TODO_APP_CONFIG_DIR");
        }

        if let Err(e) = result {
            std::panic::resume_unwind(e);
        }
    }

    #[test]
    fn test_get_config_dir() {
        // Lock to ensure we are testing default behavior without env var interference
        let _guard = ENV_LOCK.lock().unwrap();
        // Ensure env var is cleared (in case a previous test panic left it set)
        unsafe {
            std::env::remove_var("TODO_APP_CONFIG_DIR");
        }

        let config_dir = get_config_dir().unwrap();
        assert!(config_dir.to_string_lossy().contains(".todo"));
    }

    #[test]
    fn test_get_default_todo_path() {
        let _guard = ENV_LOCK.lock().unwrap();
        unsafe {
            std::env::remove_var("TODO_APP_CONFIG_DIR");
        }

        let default_path = get_default_todo_path().unwrap();
        assert!(default_path.to_string_lossy().contains("todo.txt"));
    }
}
