# Project Rules & Context

## 1. Project Overview

We are building a **Todo.txt** task manager.

- **Root**: Rust Cargo Workspace (`todo-app`).
- **Core**: `todo-core` (Library, handles business logic & file I/O).
- **TUI**: `todo-tui` (Ratatui + Crossterm).
- **GUI**: `todo-gui` (Tauri + Vue3 + Native CSS).

## 2. Tech Stack & Tools

- **Language**: Rust (2021 edition), TypeScript.
- **Package Manager**: **Bun** (Always use `bun install`, `bun run`).
- **GUI Framework**: Vue 3 (Composition API `<script setup>`).
- **GUI Styling**: **Native CSS**. Use `<style scoped>` in Vue components.
- **TUI Framework**: Ratatui (Nord Theme).

## 3. Coding Guidelines

### GUI (Vue 3)

- **Styling**: Do NOT use Tailwind or Bootstrap. Write standard CSS in the `<style scoped>` block of each component.
- **Layout**: Use CSS Flexbox and Grid for layout.
- **Design**: Keep it clean and modern. Use consistent padding and margins.
- **Components**: Create reusable components in `src/components/` (e.g., `TaskItem.vue`, `AppButton.vue`).

### todo-core (The Truth Source)

- **Protocol**: Strictly adhere to `todo.txt` format using the `todo-txt` crate.
- **Storage**: The `.txt` file is the database. Use `fs2` for atomic locking.

### TUI (Ratatui)

- **Theme**: **Nord Theme** (Polar Night background, Snow Storm text, Frost/Aurora accents).
- **Input**: Use `tui-textarea` for input fields.

## 4. Initialization Instruction

When writing the Vue frontend, verify that `src/style.css` contains basic resets (e.g., `* { box-sizing: border-box; }`) before implementing complex layouts.
