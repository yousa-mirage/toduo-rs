<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import TaskList from "./components/TaskList.vue";
import AddTaskModal from "./components/AddTaskModal.vue";
import SidebarNav from "./components/SidebarNav.vue";
import SettingsModal from "./components/SettingsModal.vue";

// Types
interface Task {
  id: number;
  subject: string;
  priority: string | null;
  completed: boolean;
  create_date: string | null;
  finish_date: string | null;
  due_date: string | null;
  due_status: "Today" | "Soon" | "Overdue" | "Later" | "None";
  projects: string[];
  contexts: string[];
  raw_content: string;
}

interface CreateTaskInput {
  description: string;
  priority: string | null;
  projects: string[];
  contexts: string[];
  due_date: string | null;
}

// State
const tasks = ref<Task[]>([]);
const isInitialLoading = ref(true);
const isLoading = ref(false); // For background operations
const error = ref<string | null>(null);
const showAddModal = ref(false);
const showEditModal = ref(false);
const showSettingsModal = ref(false);
const editingTask = ref<Task | null>(null);
const existingProjects = ref<string[]>([]);
const existingContexts = ref<string[]>([]);
const todoPath = ref<string>("");
const currentFilter = ref("all");
const scrollContainer = ref<HTMLElement | null>(null);
let savedScrollPosition = 0;

// Settings State
const settingCloseToTray = ref(false);
const settingTheme = ref<"light" | "dark" | "system">("light");

// Computed: Filtered Tasks
const filteredTasks = computed(() => {
  let result = tasks.value;

  // Filter logic
  if (currentFilter.value === "all") {
    // no op
  } else if (currentFilter.value === "today") {
    const today = new Date().toISOString().split("T")[0];
    result = result.filter((t) => t.due_date === today);
  } else if (currentFilter.value === "next7") {
    const now = new Date();
    const todayStr = now.toISOString().split("T")[0];
    const next7 = new Date();
    next7.setDate(now.getDate() + 7);
    const next7Str = next7.toISOString().split("T")[0];

    result = result.filter((t) => {
      if (!t.due_date) return false;
      return t.due_date >= todayStr && t.due_date <= next7Str;
    });
  } else if (currentFilter.value.startsWith("project:")) {
    const p = currentFilter.value.replace("project:", "");
    result = result.filter((t) => t.projects.includes(p));
  } else if (currentFilter.value.startsWith("context:")) {
    const c = currentFilter.value.replace("context:", "");
    result = result.filter((t) => t.contexts.includes(c));
  } else if (currentFilter.value.startsWith("priority:")) {
    const p = currentFilter.value.replace("priority:", "");
    if (p === "none") {
      result = result.filter((t) => !t.priority);
    } else {
      result = result.filter((t) => t.priority === p);
    }
  }

  return result;
});

// Stats (unused in new layout)
// const stats = computed(() => ({
//   total: tasks.value.length,
//   completed: tasks.value.filter((t) => t.completed).length,
//   pending: tasks.value.filter((t) => !t.completed).length,
// }));

// Methods
async function loadTasks() {
  isLoading.value = true;
  error.value = null;
  try {
    tasks.value = await invoke<Task[]>("get_tasks");
    existingProjects.value = await invoke<string[]>("get_projects");
    existingContexts.value = await invoke<string[]>("get_contexts");
    todoPath.value = await invoke<string>("get_todo_path");
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
    isInitialLoading.value = false;
  }
}

// Refresh tasks without showing loading state
async function refreshTasks() {
  try {
    const [newTasks, newProjects, newContexts] = await Promise.all([
      invoke<Task[]>("get_tasks"),
      invoke<string[]>("get_projects"),
      invoke<string[]>("get_contexts"),
    ]);
    tasks.value = newTasks;
    existingProjects.value = newProjects;
    existingContexts.value = newContexts;
  } catch (e) {
    // Silently fail for refresh operations
    console.error("Failed to refresh tasks:", e);
  }
}

async function selectDirectory() {
  try {
    const success = await invoke<boolean>("select_todo_directory");
    if (success) {
      await loadTasks();
    }
  } catch (e) {
    error.value = String(e);
  }
}

async function handleToggleComplete(task: Task) {
  // Save scroll position before operation
  if (scrollContainer.value) {
    savedScrollPosition = scrollContainer.value.scrollTop;
  }

  try {
    if (task.completed) {
      await invoke("uncomplete_task", { id: task.id });
    } else {
      await invoke("complete_task", { id: task.id });
    }
    await refreshTasks();
    // Restore scroll position after update
    if (scrollContainer.value) {
      requestAnimationFrame(() => {
        if (scrollContainer.value) {
          scrollContainer.value.scrollTop = savedScrollPosition;
        }
      });
    }
  } catch (e) {
    error.value = String(e);
  }
}

async function handleDeleteTask(task: Task) {
  // Save scroll position before operation
  if (scrollContainer.value) {
    savedScrollPosition = scrollContainer.value.scrollTop;
  }

  try {
    await invoke("delete_task", { id: task.id });
    await refreshTasks();
    // Restore scroll position after update
    if (scrollContainer.value) {
      requestAnimationFrame(() => {
        if (scrollContainer.value) {
          scrollContainer.value.scrollTop = savedScrollPosition;
        }
      });
    }
  } catch (e) {
    error.value = String(e);
  }
}

async function handleSetPriority(task: Task, priority: string | null) {
  // Save scroll position before operation
  if (scrollContainer.value) {
    savedScrollPosition = scrollContainer.value.scrollTop;
  }

  try {
    await invoke("set_priority", { id: task.id, priority });
    await refreshTasks();
    // Restore scroll position after update
    if (scrollContainer.value) {
      requestAnimationFrame(() => {
        if (scrollContainer.value) {
          scrollContainer.value.scrollTop = savedScrollPosition;
        }
      });
    }
  } catch (e) {
    error.value = String(e);
  }
}

async function handleAddTask(input: CreateTaskInput) {
  // Save scroll position before operation
  if (scrollContainer.value) {
    savedScrollPosition = scrollContainer.value.scrollTop;
  }

  try {
    await invoke("add_task", { input });
    await refreshTasks();
    // Restore scroll position after update
    if (scrollContainer.value) {
      requestAnimationFrame(() => {
        if (scrollContainer.value) {
          scrollContainer.value.scrollTop = savedScrollPosition;
        }
      });
    }
    showAddModal.value = false;
  } catch (e) {
    error.value = String(e);
  }
}

async function handleEditTask(input: CreateTaskInput) {
  if (!editingTask.value) return;

  // Save scroll position before operation
  if (scrollContainer.value) {
    savedScrollPosition = scrollContainer.value.scrollTop;
  }

  try {
    await invoke("update_task", {
      id: editingTask.value.id,
      input,
    });
    await refreshTasks();
    // Restore scroll position after update
    if (scrollContainer.value) {
      requestAnimationFrame(() => {
        if (scrollContainer.value) {
          scrollContainer.value.scrollTop = savedScrollPosition;
        }
      });
    }
    showEditModal.value = false;
    editingTask.value = null;
  } catch (e) {
    error.value = String(e);
  }
}

function handleEdit(task: Task) {
  editingTask.value = task;
  showEditModal.value = true;
}

async function handleCopy(task: Task) {
  try {
    // Use frontend clipboard API to avoid thread issues in backend
    await navigator.clipboard.writeText(task.raw_content);
  } catch (e) {
    console.error("Clipboard write failed", e);
    // Fallback or show error
    error.value = "Failed to copy to clipboard: " + String(e);
  }
}

function handleFilterUpdate(newFilter: string) {
  currentFilter.value = newFilter;
}

function handleGlobalKeydown(e: KeyboardEvent) {
  // Ctrl+A (Windows/Linux) or Cmd+A (Mac) to Open Add Task
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "a") {
    e.preventDefault();
    showAddModal.value = true;
  }
}

// --- Settings Logic ---
// Apply theme
function applyTheme(theme: "light" | "dark" | "system") {
  const root = document.documentElement;
  // Clear manual class
  document.body.classList.remove("dark-theme");

  if (theme === "dark") {
    document.body.classList.add("dark-theme");
  } else if (theme === "system") {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      document.body.classList.add("dark-theme");
    }
  }
}

// Watch theme changes
watch(settingTheme, (newVal) => {
  localStorage.setItem("todo-gui-theme", newVal);
  applyTheme(newVal);
});

// Watch close to tray
watch(settingCloseToTray, (newVal) => {
  localStorage.setItem("todo-gui-close-to-tray", JSON.stringify(newVal));
});

// Initialize Settings
let unlistenClose: (() => void) | null = null;

async function initSettings() {
  const savedTheme = localStorage.getItem("todo-gui-theme");
  if (savedTheme) {
    settingTheme.value = savedTheme as any;
  }
  applyTheme(settingTheme.value);

  const savedTray = localStorage.getItem("todo-gui-close-to-tray");
  if (savedTray) {
    settingCloseToTray.value = JSON.parse(savedTray);
  }

  // System listener
  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (e) => {
      if (settingTheme.value === "system") {
        if (e.matches) {
          document.body.classList.add("dark-theme");
        } else {
          document.body.classList.remove("dark-theme");
        }
      }
    });

  // Handle Minimize to Tray
  // Note: Only works if backend supports tray hiding
  const appWindow = getCurrentWindow();
  // Remove existing listener if any (though onMounted runs once usually, in HMR it helps)
  if (unlistenClose) {
    unlistenClose();
    unlistenClose = null;
  }
  
  unlistenClose = await appWindow.onCloseRequested(async (event) => {
    // Read directly from storage to avoid stale state
    const storedVal = localStorage.getItem("todo-gui-close-to-tray");
    const shouldMin = storedVal ? JSON.parse(storedVal) : false;
    
    if (shouldMin) {
      event.preventDefault();
      try {
        await appWindow.hide();
      } catch (e) {
        console.error("Failed to hide window", e);
      }
    } else {
        // If the user wants to truly QUIT when closing (not just close window and leave tray),
        // we should call exit explicitly.
        // For a desktop text editor / utility with tray, standard is often:
        // Close -> Tray.
        // But if user disabled "Minimize to Tray", they likely expect "Quit".
        // Let's force exit if they disabled the "hide" feature.
        event.preventDefault(); // Prevent default close, use strict exit
        await invoke("exit_app");
    }
  });
}

// Initialize
onMounted(() => {
  loadTasks();
  initSettings();
  window.addEventListener("keydown", handleGlobalKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleGlobalKeydown);
  if (unlistenClose) {
    unlistenClose();
  }
});
</script>

<template>
  <div class="app-container">
    <div class="app-layout">
      <!-- Sidebar -->
      <SidebarNav
        :current-filter="currentFilter"
        :tasks="tasks"
        @update:filter="handleFilterUpdate"
        @add-task="showAddModal = true"
        @open-settings="showSettingsModal = true"
        @open-path="selectDirectory"
      />

      <!-- Main Content Area -->
      <main class="main-content">
        <!-- Header -->
        <header class="app-header">
          <div class="header-left">
            <h1 class="app-title">Todo.txt</h1>
          </div>
          <div class="header-actions">
            <!-- Actions moved to sidebar -->
          </div>
        </header>

        <div class="scrollable-content" ref="scrollContainer">
          <!-- Loading State -->
          <div v-if="isInitialLoading" class="loading-state">
            <div class="loading-spinner"></div>
          </div>

          <!-- Error State -->
          <div v-else-if="error" class="error-state">
            <p class="error-message">{{ error }}</p>
            <button class="btn btn-secondary" @click="loadTasks">Retry</button>
          </div>

          <!-- Empty State -->
          <div v-else-if="tasks.length === 0" class="empty-state">
            <div class="empty-icon">📝</div>
            <h2>No tasks</h2>
            <p>Add your first task.</p>
            <button class="btn btn-primary" @click="showAddModal = true">
              Add Task
            </button>
          </div>

          <!-- Task List -->
          <TaskList
            v-else
            :tasks="filteredTasks"
            @toggle-complete="handleToggleComplete"
            @delete="handleDeleteTask"
            @set-priority="handleSetPriority"
            @edit="handleEdit"
            @copy="handleCopy"
          />
        </div>
      </main>
    </div>

    <!-- Add Task Modal -->
    <AddTaskModal
      v-if="showAddModal"
      :existing-projects="existingProjects"
      :existing-contexts="existingContexts"
      @submit="handleAddTask"
      @close="showAddModal = false"
    />

    <!-- Edit Task Modal -->
    <AddTaskModal
      v-if="showEditModal"
      :existing-projects="existingProjects"
      :existing-contexts="existingContexts"
      :editing-task="editingTask"
      :is-editing="true"
      @submit="handleEditTask"
      @close="
        showEditModal = false;
        editingTask = null;
      "
    />

    <!-- Settings Modal -->
    <SettingsModal
      v-if="showSettingsModal"
      :current-path="todoPath"
      :close-to-tray="settingCloseToTray"
      :theme="settingTheme"
      @update:close-to-tray="(v) => (settingCloseToTray = v)"
      @update:theme="(v) => (settingTheme = v)"
      @change-folder="selectDirectory"
      @close="showSettingsModal = false"
    />
  </div>
</template>

<style>
/* Global Resets & Vars */
*,
*::before,
*::after {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

:root {
  /* Colors - Sleek Light Theme inspired */
  --color-bg: #ffffff;
  --color-bg-secondary: #f8fafc; /* Sidebar bg */
  --color-text: #334155;
  --color-text-secondary: #94a3b8;
  --color-border: #e2e8f0;
  --color-primary: #3b82f6;
  --color-primary-hover: #2563eb;
  --color-danger: #ef4444;
  --color-success: #22c55e;
  --color-priority-b: #f97316; /* Orange */
  --color-priority-c: #eab308; /* Yellow */
  --color-priority-d: #3b82f6; /* Blue */
  --color-priority-e: #22c55e; /* Green */

  --spacing-xs: 0.25rem;
  --spacing-sm: 0.5rem;
  --spacing-md: 1rem;
  --spacing-lg: 1.5rem;
  --spacing-xl: 2rem;

  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 8px;

  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-md:
    0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  --shadow-lg:
    0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);

  --font-size: 16px;

  font-family:
    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial,
    sans-serif;
  font-size: var(--font-size);
}

body {
  background-color: var(--color-bg);
  color: var(--color-text);
  user-select: none; /* Disable text selection globally */
  -webkit-user-select: none;
}

/* Dark Theme Overrides */
body.dark-theme {
    --color-bg: #1e293b;
    --color-bg-secondary: #0f172a;
    --color-text: #f1f5f9;
    --color-text-secondary: #94a3b8; /* Keep secondary text somewhat legible but dim */
    --color-border: #334155;
    
    /* Slightly Adjust Primary if needed */
    --color-primary: #60a5fa; 
    --color-primary-hover: #3b82f6;

    /* Shadows might need inversion or lightening if really needed, but defaults are often ok-ish or invisible */
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.3);
    --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
    --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.3);
}

/* Allow selection in inputs */
input,
textarea {
  user-select: text;
  -webkit-user-select: text;
}

/* Custom Scrollbar for WebKit (Chrome, Safari, newer Edge) */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.3);
}

/* Dark theme scrollbar */
body.dark-theme ::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.2);
}

body.dark-theme ::-webkit-scrollbar-thumb:hover {
  background-color: rgba(255, 255, 255, 0.3);
}
</style>

<style scoped>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: var(--color-bg);
  min-width: 0; /* Prevent flex blowout */
}

/* Header */
.app-header {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-lg);
  border-bottom: 1px solid transparent; /* Cleaner look */
}

.app-title {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--color-text);
  cursor: default;
}

.header-actions {
  display: flex;
  gap: var(--spacing-sm);
}

.scrollable-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: var(--spacing-lg);
}

/* Buttons */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 500;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-icon-only {
  padding: 0.5rem;
  background: transparent;
  font-size: 1.25rem;
}
.btn-icon-only:hover {
  background-color: var(--color-bg-secondary);
}

.btn-primary {
  background-color: var(--color-primary);
  color: white;
  padding: 0.5rem 0.75rem;
}
.btn-primary:hover {
  background-color: var(--color-primary-hover);
}

.btn-secondary {
  background-color: transparent;
  border: 1px solid var(--color-border);
  color: var(--color-text);
}

.loading-state,
.empty-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--color-text-secondary);
  gap: 1rem;
}

/* Spinner */
.loading-spinner {
  width: 2rem;
  height: 2rem;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
