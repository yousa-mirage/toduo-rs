<script setup lang="ts">
import { ref, computed, watch } from "vue";
import DatePicker from "./DatePicker.vue";

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

const props = defineProps<{
  existingProjects: string[];
  existingContexts: string[];
  editingTask?: Task | null;
  isEditing?: boolean;
}>();

const emit = defineEmits<{
  (e: "submit", input: CreateTaskInput): void;
  (e: "close"): void;
}>();

// Form state
const description = ref("");
const priority = ref<string>("");
const projectsInput = ref("");
const contextsInput = ref("");
const dueDate = ref<string | null>(null);

// Validation
const isValid = computed(() => description.value.trim().length > 0);

// Pre-fill form when editing
watch(
  () => props.editingTask,
  (task) => {
    if (task) {
      description.value = task.subject;
      priority.value = task.priority || "";
      projectsInput.value = task.projects.map((p) => `+${p}`).join(" ");
      contextsInput.value = task.contexts.map((c) => `@${c}`).join(" ");
      dueDate.value = task.due_date;
    }
  },
  { immediate: true },
);

// Parse tags from input
function parseTags(input: string): string[] {
  return input
    .split(/[\s,]+/)
    .map((s) => s.replace(/^[+@]/, "").trim())
    .filter((s) => s.length > 0);
}

function handleSubmit() {
  if (!isValid.value) return;

  emit("submit", {
    description: description.value.trim(),
    priority: priority.value || null,
    projects: parseTags(projectsInput.value),
    contexts: parseTags(contextsInput.value),
    due_date: dueDate.value || null,
  });
}

function handleOverlayClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains("modal-overlay")) {
    emit("close");
  }
}
</script>

<template>
  <div class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content">
      <div class="modal-header">
        <h2>{{ isEditing ? "Edit Task" : "Add New Task" }}</h2>
        <button class="modal-close" @click="emit('close')" aria-label="Close">
          ×
        </button>
      </div>

      <form @submit.prevent="handleSubmit" class="modal-form">
        <!-- Description -->
        <div class="form-group">
          <label for="description">Description *</label>
          <input
            id="description"
            v-model="description"
            type="text"
            placeholder="What needs to be done?"
            autofocus
            class="form-input"
          />
        </div>

        <!-- Priority -->
        <div class="form-group">
          <label for="priority">Priority</label>
          <select id="priority" v-model="priority" class="form-select">
            <option value="">None</option>
            <option value="A">A (Highest)</option>
            <option value="B">B</option>
            <option value="C">C</option>
            <option value="D">D</option>
            <option value="E">E (Lowest)</option>
          </select>
        </div>

        <!-- Projects -->
        <div class="form-group">
          <label for="projects">Projects</label>
          <input
            id="projects"
            v-model="projectsInput"
            type="text"
            placeholder="project1 project2 (space separated)"
            class="form-input"
            list="project-suggestions"
          />
          <datalist id="project-suggestions">
            <option v-for="p in existingProjects" :key="p" :value="p" />
          </datalist>
          <div class="form-hint" v-if="existingProjects.length > 0">
            Existing: {{ existingProjects.join(", ") }}
          </div>
        </div>

        <!-- Contexts -->
        <div class="form-group">
          <label for="contexts">Contexts</label>
          <input
            id="contexts"
            v-model="contextsInput"
            type="text"
            placeholder="home work (space separated)"
            class="form-input"
            list="context-suggestions"
          />
          <datalist id="context-suggestions">
            <option v-for="c in existingContexts" :key="c" :value="c" />
          </datalist>
          <div class="form-hint" v-if="existingContexts.length > 0">
            Existing: {{ existingContexts.join(", ") }}
          </div>
        </div>

        <!-- Due Date -->
        <div class="form-group">
          <label for="due-date">Due Date</label>
          <DatePicker v-model="dueDate" placeholder="yyyy-mm-dd" />
          <div class="form-hint">Format: yyyy-mm-dd</div>
        </div>

        <!-- Actions -->
        <div class="modal-actions">
          <button
            type="button"
            class="btn btn-secondary"
            @click="emit('close')"
          >
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" :disabled="!isValid">
            {{ isEditing ? "Update Task" : "Add Task" }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  padding: var(--spacing-md);
}

.modal-content {
  width: 100%;
  max-width: 480px;
  background-color: var(--color-bg-secondary);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--color-border);
}

.modal-header h2 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
}

.modal-close {
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  color: var(--color-text-secondary);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
}

.modal-close:hover {
  background-color: var(--color-border);
  color: var(--color-text);
}

.modal-form {
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
}

.form-input,
.form-select {
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: 1rem;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background-color: var(--color-bg);
  color: var(--color-text);
  font-family: inherit;
  height: 42px;
  line-height: 1.5;
  transition: border-color 0.2s ease;
}

.form-input::placeholder {
  color: var(--color-text-secondary);
  opacity: 0.7;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

.form-hint {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  margin-top: var(--spacing-xs);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-sm);
}

/* Button Styles */
.btn {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: 0.875rem;
  font-weight: 500;
  border: none;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background-color: var(--color-primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--color-primary-hover);
}

.btn-secondary {
  background-color: var(--color-bg);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.btn-secondary:hover {
  background-color: var(--color-border);
}
</style>
