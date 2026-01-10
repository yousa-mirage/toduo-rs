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

# Run specific test
cargo test -p todo-core <test_name>
cargo test -p todo-tui <test_name>

# Run with output
cargo test -p todo-core -- --nocapture

# Test specific module
cargo test -p todo-core lib::<module_name>
```

## Code Style

**In each session, when you have finished making your changes, you must run :**

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cd todo-gui
bun run format
bun run type-check || bun x vue-tsc --noEmit
bun run lint
cd ../
```

Make sure the code format is consistent and there are no errors or warnings in the code. If there are any, continue to fix them.

### Rust Conventions

**Imports**: Use `use` statements with grouping. Prefer absolute paths for workspace dependencies.

```rust
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use crate::model::Task;
```

**Naming**: Snake_case for variables/functions, PascalCase for types/traits, UPPER_SCASE for constants.

```rust
const MAX_TASK_LENGTH: usize = 500;

pub struct TaskService { ... }

fn load_tasks(path: &Path) -> Result<Vec<Task>> { ... }
```

**Error Handling**: Use `anyhow` for application errors with context. Use `thiserror` for library errors.

```rust
fn load_tasks(path: &Path) -> Result<Vec<Task>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    Ok(parse_tasks(&content)?)
}
```

**Formatting**: Run `cargo fmt` before committing. Use default Rust 2021 edition settings.

### Vue 3 + TypeScript

**Component Structure**:

```typescript
<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Props/Emits
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

**Styling**: Native CSS only. No Tailwind, no Bootstrap. Use CSS variables for theming.

## Project Structure

- **todo-core**: Library crate - business logic, todo.txt parsing, file I/O
- **todo-tui**: Terminal UI (Ratatui + Crossterm) - Nord theme
- **todo-gui**: Desktop GUI (Tauri + Vue 3) - Native CSS styling

## Key Technologies

- Rust 2021 edition, Tauri 2, Vue 3 Composition API
- Package manager: Bun (always use for GUI)
- Todo.txt format strictly followed via `todo-txt` crate
- File locking via `fs2` for atomic writes

## Communication Patterns

- GUI frontend communicates with Rust backend via Tauri invoke commands
- TUI reads/writes directly to todo.txt files in user config directory
- All crates share workspace dependencies defined in root Cargo.toml
