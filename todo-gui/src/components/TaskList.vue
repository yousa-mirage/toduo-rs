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
  projects: string[];
  contexts: string[];
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

// Group tasks by priority
const groupedTasks = computed(() => {
  const groups: Record<string, Task[]> = {};
  const noPriority: Task[] = [];

  props.tasks.forEach((task) => {
    if (task.completed) return; // We handle completed separately if needed, or included in groups?
    // Let's include completed in their priority groups for now, but usually completed tasks are separate.
    // Actually, Sleek often shows completed tasks at the bottom or crossed out.
    // The previous sorting logic in App.vue put completed tasks at the bottom.
    // Let's respect the incoming order but group by priority key.
    
    // Wait, if we want Sleek style A, B, C headers, we need to group.
    
    if (task.priority) {
      if (!groups[task.priority]) {
        groups[task.priority] = [];
      }
      groups[task.priority].push(task);
    } else {
      noPriority.push(task);
    }
  });

  // Convert to array of groups, sorted by key A-Z
  const result = Object.keys(groups)
    .sort()
    .map((key) => ({
      key,
      tasks: groups[key],
    }));

  if (noPriority.length > 0) {
    result.push({ key: "", tasks: noPriority });
  }

  return result;
});
</script>

<template>
  <div class="task-list-container">
    <div v-for="group in groupedTasks" :key="group.key" class="task-group">
      <!-- Group Header -->
      <div v-if="group.key" class="group-header">
        <span class="group-badge" :class="getPriorityClass(group.key)">
          {{ group.key }}
        </span>
        <div class="group-line"></div>
      </div>

      <!-- Group Content -->
      <div class="group-items">
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
             <div class="custom-checkbox" :class="{ checked: task.completed }"></div>
          </div>

          <!-- Task Body -->
          <div class="task-body">
            <div class="task-main-line">
              <span class="task-text" :class="{ completed: task.completed }">
                {{ task.subject }}
              </span>
              
              <!-- Inline Metadata Badges -->
              <span v-if="task.due_date" class="meta-badge meta-due">
                due: {{ task.due_date }}
              </span>
            </div>

            <div class="task-sub-line" v-if="task.projects.length || task.contexts.length">
                <span v-for="p in task.projects" :key="p" class="text-project">+{{p}}</span>
                <span v-for="c in task.contexts" :key="c" class="text-context">@{{c}}</span>
            </div>
          </div>

          <!-- Actions (Hover only) -->
          <div class="task-actions">
             <button class="action-btn delete-btn" @click.stop="emit('delete', task)" title="Delete">✕</button>
             <div class="priority-changer">
                 <!-- Simple quick priority change could go here -->
             </div>
          </div>
        </div>
      </div>
    </div>
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
.priority-a { background-color: #ef4444; }
.priority-b { background-color: #f97316; }
.priority-c { background-color: #eab308; }
.priority-other { background-color: #94a3b8; }

/* Item Row */
.task-row {
  display: flex;
  align-items: flex-start;
  padding: 0.5rem 0;
  border-radius: 4px;
  transition: background-color 0.1s;
}

.task-row:hover {
  background-color: rgba(0,0,0,0.02);
}

.task-row:hover .task-actions {
  opacity: 1;
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
  position: relative;
}

.custom-checkbox.checked::after {
  content: '';
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

.task-text.completed {
  text-decoration: line-through;
  color: var(--color-text-secondary);
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
    font-size: 1rem;
    color: #94a3b8;
    padding: 4px;
}

.action-btn:hover {
    color: #ef4444;
}

</style>
