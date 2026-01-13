# ToDuo GUI

<div align="center">

[English](README.md) | [简体中文](README_zh.md)

</div>

The graphical interface version of **ToDuo**, built with Tauri 2 and Vue 3. It offers a modern visual experience and convenient desktop integration.

![ToDuo GUI](../.github/toduo-gui.png)

## ✨ Features

### 1. Modern UI

* **Modern Design**: Minimalism, clear layout, smooth animation
* **Dark Mode**: Supports automatic switching between Dark/Light themes based on system settings.

### 2. Convenience

* **Global Shortcut**: Default `Ctrl + Alt + Space` to quickly show the main window. Capture ideas anytime.
* **System Tray**: Supports minimizing to the system tray, running silently in the background.
* **Context Menu**: Right-click on tasks to access quick actions (Edit, Delete, Copy Content).

### 3. Task Management

* **Smart Filters**: Sidebar provides automatic categorization for "Today", "Next 7 Days", "Projects", "Contexts", etc.
* **Rich Parsing**: Automatically highlights `+project` and `@context` tags within tasks.
* **Date Picker**: Graphical date picker for adding/editing tasks.

## 🛠️ Settings

Click the **Settings** gear icon at the bottom of the sidebar:

* **Change Folder**: Switch the directory where your todo.txt file is located.
* **Theme**: Manually force Light/Dark mode or follow system.
* **Tray**: Toggle whether closing the window minimizes it to the tray.

## ⌨️ Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Ctrl + Alt + Space` | Global: Show/Hide Window |
| `Ctrl + A` | Add Task |
