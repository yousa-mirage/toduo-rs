<script setup lang="ts">
import { computed } from "vue";

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

const props = defineProps<{
  tasks: Task[];
}>();

const emit = defineEmits<{
  (e: "toggle-complete", task: Task): void;
  (e: "delete", task: Task): void;
  (e: "set-priority", task: Task, priority: string | null): void;
}>();

function getPriorityClass(priority: string | null): string {
  if (!priority) return "";
  switch (priority) {
    case "A":
      return "priority-a";
    case "B":
      return "priority-b";
    case "C":
      return "priority-c";
    default:
      return "priority-other";
  }
}

function getDueDateClass(status: string | null): string {
  if (!status) return "";
  switch (status) {
    case "Today":
      return "due-today";
    case "Soon":
      return "due-soon";
    case "Overdue":
      return "due-overdue";
    default:
      return "";
  }
}

// Group tasks by priority, with completed tasks at the bottom of each group
const groupedTasks = computed(() => {
  const groups: Record<string, { pending: Task[]; completed: Task[] }> = {};
  const noPriority = { pending: [] as Task[], completed: [] as Task[] };

  props.tasks.forEach((task) => {
    if (task.priority) {
      // Create group if it doesn't exist
      if (!groups[task.priority]) {
        groups[task.priority] = { pending: [], completed: [] };
      }
      // Add to appropriate array
      if (task.completed) {
        groups[task.priority].completed.push(task);
      } else {
        groups[task.priority].pending.push(task);
      }
    } else {
      // No priority
      if (task.completed) {
        noPriority.completed.push(task);
      } else {
        noPriority.pending.push(task);
      }
    }
  });

  // Convert to array of groups, sorted by key A-Z
  const result = Object.keys(groups)
    .sort()
    .map((key) => ({
      key,
      tasks: [...groups[key].pending, ...groups[key].completed],
    }));

  if (noPriority.pending.length > 0 || noPriority.completed.length > 0) {
    result.push({
      key: "",
      tasks: [...noPriority.pending, ...noPriority.completed],
    });
  }

  return result;
});
</script>

<template>
  <div class="task-list-container">
    <TransitionGroup name="task-group" tag="div">
      <div
        v-for="group in groupedTasks"
        :key="group.key || 'no-priority'"
        class="task-group"
      >
        <!-- Group Header -->
        <div v-if="group.key" class="group-header">
          <span class="group-badge" :class="getPriorityClass(group.key)">
            {{ group.key }}
          </span>
          <div class="group-line"></div>
        </div>

        <!-- Group Content -->
        <div class="group-items">
          <TransitionGroup name="task-row" tag="div">
            <div
              v-for="task in group.tasks"
              :key="task.id"
              class="task-row"
              :class="{ 'task-completed': task.completed }"
            >
              <!-- Custom Radio-style Checkbox -->
              <div
                class="checkbox-wrapper"
                @click="emit('toggle-complete', task)"
              >
                <div
                  class="custom-checkbox"
                  :class="{ checked: task.completed }"
                ></div>
              </div>

              <!-- Task Body -->
              <div class="task-body">
                <div class="task-main-line">
                  <span
                    class="task-text"
                    :class="{ completed: task.completed }"
                  >
                    {{ task.subject }}
                  </span>

                  <!-- Inline Metadata Badges -->
                  <span
                    v-if="task.due_status !== 'None'"
                    class="meta-badge meta-due"
                    :class="getDueDateClass(task.due_status)"
                  >
                    due: {{ task.due_date }}
                  </span>
                </div>

                <div
                  class="task-sub-line"
                  v-if="task.projects.length || task.contexts.length"
                >
                  <span v-for="p in task.projects" :key="p" class="text-project"
                    >+{{ p }}</span
                  >
                  <span v-for="c in task.contexts" :key="c" class="text-context"
                    >@{{ c }}</span
                  >
                </div>
              </div>

              <!-- Actions (Hover only) -->
              <div class="task-actions">
                <button
                  class="action-btn delete-btn"
                  @click.stop="emit('delete', task)"
                  title="Delete"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path
                      d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                    ></path>
                    <line x1="10" y1="11" x2="10" y2="17"></line>
                    <line x1="14" y1="11" x2="14" y2="17"></line>
                  </svg>
                </button>
              </div>
            </div>
          </TransitionGroup>
        </div>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.task-list-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.task-group {
  display: flex;
  flex-direction: column;
}

.group-header {
  display: flex;
  align-items: center;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
}

.group-badge {
  font-size: 1.2rem;
  font-weight: 800;
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  color: white;
  margin-right: 0.5rem;
  flex-shrink: 0;
  /* Fix vertical alignment */
  line-height: 1;
  padding-bottom: 2px; /* Push text up visually */
}

.group-line {
  height: 1px;
  background-color: var(--color-border); /* Dashed in sleek? */
  flex: 1;
  border-top: 1px dashed var(--color-border);
  background: none;
  opacity: 0.5;
}

/* Priority Colors */
.priority-a {
  background-color: #ef4444;
}
.priority-b {
  background-color: #f97316;
}
.priority-c {
  background-color: #eab308;
}
.priority-other {
  background-color: #94a3b8;
}

/* Item Row */
.task-row {
  display: flex;
  align-items: center;
  padding: 0.5rem var(--spacing-lg);
  margin: 0 calc(-1 * var(--spacing-lg));
  border-radius: 4px;
  transition: background-color 0.1s;
  min-height: 48px;
  will-change: transform, opacity;
}

.task-row:hover {
  background-color: rgba(0, 0, 0, 0.04);
  box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.08);
}

.task-row:hover .task-actions {
  opacity: 1;
}

.task-row.task-completed {
  background-color: rgba(0, 0, 0, 0.02);
  padding: 0.5rem var(--spacing-lg);
  margin: 0 calc(-1 * var(--spacing-lg));
  transition:
    background-color 0.3s ease,
    opacity 0.3s ease;
}

.task-row.task-completed .task-text {
  color: var(--color-text-secondary);
  text-decoration: line-through;
  text-decoration-color: rgba(148, 163, 184, 0.5);
  text-decoration-thickness: 2px;
  transition:
    color 0.3s ease,
    text-decoration-color 0.3s ease;
}

.task-row.task-completed .task-sub-line {
  opacity: 0.5;
  transition: opacity 0.3s ease;
}

.task-row.task-completed .meta-badge {
  opacity: 0.5;
  transition: opacity 0.3s ease;
}

.task-row.task-completed .text-project,
.task-row.task-completed .text-context {
  opacity: 0.5;
  transition: opacity 0.3s ease;
}

/* TransitionGroup Animations */
.task-row-move {
  transition:
    transform 0.35s ease,
    opacity 0.35s ease;
}

.task-row-enter-active,
.task-row-leave-active {
  transition: all 0.3s ease;
}

.task-row-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

.task-row-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.task-row-leave-active {
  position: absolute;
  width: 100%;
}

/* Checkbox */
.checkbox-wrapper {
  padding: 0.2rem 0.75rem 0 0.25rem;
  cursor: pointer;
}

.custom-checkbox {
  width: 18px;
  height: 18px;
  border: 2px solid #3b82f6;
  border-radius: 50%;
  transition: all 0.2s;
}

.custom-checkbox.checked {
  background-color: #3b82f6;
}

.custom-checkbox.checked::after {
  content: "";
  position: absolute;
  top: 4px;
  left: 3px;
  width: 8px;
  height: 4px;
  border: 2px solid white;
  border-top: none;
  border-right: none;
  transform: rotate(-45deg);
}

/* Content */
.task-body {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.task-main-line {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.task-text {
  color: var(--color-text);
  font-size: 1rem;
  line-height: 1.5;
}

.meta-badge {
  font-size: 0.75rem;
  padding: 2px 6px;
  border-radius: 10px;
  background-color: #e2e8f0;
  color: #475569;
}

.meta-due {
  background-color: #94a3b8;
  color: white;
}

.due-today {
  background-color: #ef4444 !important;
}

.due-soon {
  background-color: #f97316 !important;
}

.due-overdue {
  background-color: #78350f !important;
}

.task-sub-line {
  font-size: 0.85rem;
  margin-top: 2px;
  color: var(--color-text-secondary);
}

.text-project {
  color: #e0d04c;
  font-weight: bold;
  margin-right: 0.5rem;
}
.text-context {
  color: #3b82f6;
  margin-right: 0.5rem;
}

/* Actions */
.task-actions {
  opacity: 0;
  display: flex;
  align-items: center;
  padding-left: 0.5rem;
}

.action-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn svg {
  width: 16px;
  height: 16px;
  color: #94a3b8;
  transition: color 0.2s;
}

.action-btn:hover svg {
  color: #ef4444;
}
</style>
