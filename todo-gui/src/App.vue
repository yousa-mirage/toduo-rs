<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TaskList from "./components/TaskList.vue";
import AddTaskModal from "./components/AddTaskModal.vue";
import Sidebar from "./components/Sidebar.vue";

// Types
interface Task {
  id: number;
  subject: string;
  priority: string | null;
  completed: boolean;
  create_date: string | null;
  finish_date: string | null;
  due_date: string | null;
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
const isLoading = ref(true);
const error = ref<string | null>(null);
const showAddModal = ref(false);
const existingProjects = ref<string[]>([]);
const existingContexts = ref<string[]>([]);
const todoPath = ref<string>("");
const currentFilter = ref("all");

// Computed: Filtered Tasks
const filteredTasks = computed(() => {
  let result = tasks.value;
  
  // Filter logic
  if (currentFilter.value === 'all') {
      // no op
  } else if (currentFilter.value === 'today') {
      const today = new Date().toISOString().split('T')[0];
      result = result.filter(t => t.due_date === today);
  } else if (currentFilter.value.startsWith('project:')) {
      const p = currentFilter.value.replace('project:', '');
      result = result.filter(t => t.projects.includes(p));
  } else if (currentFilter.value.startsWith('context:')) {
      const c = currentFilter.value.replace('context:', '');
      result = result.filter(t => t.contexts.includes(c));
  }
  
  // Always sort completed to bottom, usually handled by TaskList grouping, 
  // but if we want strict sorting inside groups, we can do it here or let TaskList handle it.
  // TaskList groups by priority.
  // We should just pass filtered result.
  return result;
});

// Stats
const stats = computed(() => ({
  total: tasks.value.length,
  completed: tasks.value.filter((t) => t.completed).length,
  pending: tasks.value.filter((t) => !t.completed).length,
}));

// Methods
async function loadTasks() {
  try {
    isLoading.value = true;
    error.value = null;
    tasks.value = await invoke<Task[]>("get_tasks");
    existingProjects.value = await invoke<string[]>("get_projects");
    existingContexts.value = await invoke<string[]>("get_contexts");
    todoPath.value = await invoke<string>("get_todo_path");
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
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
  try {
    if (task.completed) {
      await invoke("uncomplete_task", { id: task.id });
    } else {
      await invoke("complete_task", { id: task.id });
    }
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function handleDeleteTask(task: Task) {
  try {
    await invoke("delete_task", { id: task.id });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function handleSetPriority(task: Task, priority: string | null) {
  try {
    await invoke("set_priority", { id: task.id, priority });
    await loadTasks();
  } catch (e) {
    error.value = String(e);
  }
}

async function handleAddTask(input: CreateTaskInput) {
  try {
    await invoke("add_task", { input });
    await loadTasks();
    showAddModal.value = false;
  } catch (e) {
    error.value = String(e);
  }
}

function handleFilterUpdate(newFilter: string) {
    currentFilter.value = newFilter;
}

// Initialize
onMounted(() => {
  loadTasks();
});
</script>

<template>
  <div class="app-container">
    <div class="app-layout">
        <!-- Sidebar -->
        <Sidebar 
            :current-filter="currentFilter"
            :projects="existingProjects"
            :contexts="existingContexts"
            :tasks="tasks"
            @update:filter="handleFilterUpdate"
        />

        <!-- Main Content Area -->
        <main class="main-content">
            <!-- Header -->
            <header class="app-header">
              <div class="header-left">
                <h1 class="app-title">Todo.txt</h1>
                <!-- Optional: Breadcrumb or filter name -->
              </div>
              <div class="header-actions">
                 <button class="btn btn-icon-only" @click="selectDirectory" title="Change directory">
                    📁
                 </button>
                 <button class="btn btn-primary" @click="showAddModal = true">
                  <span class="btn-icon">+</span>
                </button>
              </div>
            </header>
            
            <div class="scrollable-content">
                  <!-- Loading State -->
                  <div v-if="isLoading" class="loading-state">
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
  </div>
</template>

<style>
/* Global Resets & Vars */
*, *::before, *::after {
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
  
  --spacing-xs: 0.25rem;
  --spacing-sm: 0.5rem;
  --spacing-md: 1rem;
  --spacing-lg: 1.5rem;
  --spacing-xl: 2rem;
  
  --radius-sm: 4px;
  --radius-md: 6px;
  --radius-lg: 8px;

  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  font-size: 14px;
}

body {
    background-color: var(--color-bg);
    color: var(--color-text);
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
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--color-text);
}

.header-actions {
    display: flex;
    gap: var(--spacing-sm);
}

.scrollable-content {
    flex: 1;
    overflow-y: auto;
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

.loading-state, .empty-state, .error-state {
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
  to { transform: rotate(360deg); }
}

</style>
