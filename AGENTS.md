# AGENTS.md

This document provides guidelines for agentic coding assistants working on the Todo.txt project.

## Build Commands

### Rust (Workspace)

```bash
# Build all crates
cargo build

# Build specific crate
cargo build -p todo-core
cargo build -p todo-tui
cargo build -p todo-gui

# Release build
cargo build --release

# Run TUI application
cargo run -p todo-tui
```

### GUI (Tauri + Vue)

```bash
cd todo-gui

# Install dependencies (Bun)
bun install

# Development (with hot reload)
bun run dev

# Type check & build
bun run build

# Tauri CLI
bun run tauri build
bun run tauri dev
```

### Tests

```bash
# Run all tests
cargo test

# Run specific test (exact name)
cargo test -p todo-core test_load_tasks
cargo test -p todo-tui test_add_task

# Run with output (for debugging)
cargo test -p todo-core -- --nocapture

# Test specific module
cargo test -p todo-core lib::config
cargo test -p todo-core lib::model::tests

# Run doc tests
cargo test --doc
```

### Linting & Formatting

```bash
# Format all code
cargo fmt --all

# Clippy check (strict -D warnings)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# GUI linting
cd todo-gui
bun run format
bun run type-check || bun x vue-tsc --noEmit
bun run lint
```

## Code Style

**Before committing, always run:**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cd todo-gui && bun run format && bun run type-check && bun run lint
```

### Rust Conventions

**Imports**: Group by category. Use absolute paths for workspace deps.

```rust
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::model::Task;
use crate::error::TodoError;
```

**Naming**: `snake_case` for vars/functions, `PascalCase` for types/traits, `UPPER_SNAKE_CASE` for consts.

```rust
const MAX_TASK_LENGTH: usize = 500;

pub struct TaskService { ... }

fn load_tasks(path: &Path) -> Result<Vec<Task>> { ... }
```

**Error Handling**: Use `anyhow` with `.with_context()` for app errors. Use `thiserror` for library errors.

```rust
// Application error with context
fn load_tasks(path: &Path) -> Result<Vec<Task>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    Ok(parse_tasks(&content)?)
}

// Library error with thiserror
#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Task not found: {0}")]
    TaskNotFound(usize),
}
```

**Avoid**: `unwrap()`, `expect()`, `panic!`, `unwrap_err()`, `unwrap_ok()`. Use `?` or proper error mapping.

**Early Returns**: Use guard clauses. Return `None`/`Err` early when possible.

```rust
pub fn from_due_date(due_date: Option<&str>) -> DueStatus {
    let Some(due_str) = due_date else { return DueStatus::None };
    let Ok(due) = NaiveDate::parse_from_str(due_str, "%Y-%m-%d") else {
        return DueStatus::None;
    };
    // ... rest of logic
}
```

**Option/Result**: Prefer `if let Some(x) = ...` or `let Some(x) = ... else { return ... }` over nested matches.

### Vue 3 + TypeScript

**Component Structure**:

```typescript
<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

defineProps<{ tasks: Task[] }>();
defineEmits<{ (e: "update", task: Task): void }>();
</script>

<template>
  <div class="task-list">...</div>
</template>

<style scoped>
.task-list { ... }
</style>
```

**Styling**: Native CSS only. No Tailwind. Use CSS variables for theming.

## Project Structure

- **todo-core**: Library crate - business logic, todo.txt parsing, file I/O
- **todo-tui**: Terminal UI (Ratatui + Crossterm) - Nord theme
- **todo-gui**: Desktop GUI (Tauri + Vue 3) - Native CSS styling

## Key Technologies

- Rust 2024 edition, Tauri 2, Vue 3 Composition API
- Package manager: Bun (always use for GUI)
- Todo.txt format via `todo-txt` crate
- `anyhow` for errors, `thiserror` for library errors
- `toml` for config, `serde` for serialization

## Communication Patterns

- GUI frontend communicates with Rust backend via Tauri invoke commands
- TUI reads/writes directly to todo.txt files in user config directory
- All crates share workspace dependencies defined in root Cargo.toml
