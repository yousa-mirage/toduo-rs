# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### TUI

- feat: Use `[ ]` and `[✓]` for task completion status in TUI.

### GUI

- feat: Add the Auto-Launch function to GUI.
- feat: Each time the GUI application is launched from the tray, the window is displayed in the center of the screen.
- feat: Add settings at Right-click menu for GUI.

## [v0.1.1] - 2026-01-13

### Fixed

- Fixed issue where GUI does not show date selector correctly.

## [v0.1.0] - 2026-01-13

### Added

- **Dual Interface Support**
  - TUI (Terminal User Interface) with Ratatui and Nord theme
  - GUI (Graphical User Interface) with Tauri 2 and Vue 3

- **Core Features**
  - Full todo.txt format support
  - Task CRUD operations (Create, Read, Update, Delete)
  - Priority management (A-E)
  - Due date tracking with status indicators (Today/Soon/Overdue/Later)
  - Project tags (`+project`) and context tags (`@context`)
  - Task completion tracking

- **TUI Features**
  - Vim-style keybindings (`j`/`k` navigation, `g`/`G` for top/bottom)
  - Mouse support (click to select, double-click to edit)
  - Multiple filter views (All, Today, Next 7 Days, Priority A-E)
  - Help modal with all shortcuts
  - Customizable todo.txt path

- **GUI Features**
  - Native CSS styling
  - Light/Dark/System theme support
  - System tray integration
  - Keyboard shortcuts (Ctrl+A to add task)

### Technical

- Rust 2024 edition
- Optimized release builds with LTO and stripping
- Cross-platform support (Windows, macOS, Linux)

[0.1.0]: https://github.com/yourusername/todo-app/releases/tag/v0.1.0
