# ToDuo TUI (td)

<div align="center">

[English](README.md) | [简体中文](README_zh.md)

</div>

The terminal version of **ToDuo**, crafted for keyboard efficiency and geeks.

![ToDuo TUI](../.github/tui-preview.png)

## 🎮 Keybindings

TUI adopts Vim-style keybindings, keeping your hands on the home row.

| Key | Action | Description |
| :--- | :--- | :--- |
| **Navigation** | | |
| `j` / `↓` | Move Down | Select next task |
| `k` / `↑` | Move Up | Select previous task |
| `g` | Go to Top | Jump to the first item |
| `G` | Go to Bottom | Jump to the last item |
| `Tab` | Switch Focus | Toggle focus between Sidebar and Task List |
| **Actions** | | |
| `a` | Add Task | Open the "Add New Task" panel |
| `Space` | Toggle Status | Mark task as completed/uncompleted |
| `d` | Delete Task | Delete the selected task |
| `1` - `5` | Set Priority | Set priority A/B/C/D/E |
| `0` | Clear Priority | Remove task priority |
| `Enter` | Toggle Status | Same as Space |
| **Misc** | | |
| `r` | Refresh | Reload todo.txt file |
| `?` | Help | Show/Hide help modal |
| `q` | Quit | Exit the application |

## 🖱️ Mouse Support

Despite being a terminal app, `td` offers excellent mouse support:

* **Drag to Resize**: Click and drag the sidebar border to adjust width.
* **Click to Select**: Click any list item to select it.
* **Double Click to Edit**: Double-click a task to enter edit mode.
* **Click to Toggle**: Click the `[ ]` or `[x]` marker to toggle completion status.

## 🖥️ UI Layout

* **Left**: Sidebar. Contains filters (All, Today, Next 7 Days) and Priority filters.
* **Right**: Task List. Shows task details, priority colors, and due dates.
* **Bottom**: Status Bar. Shows current file path and status messages.
