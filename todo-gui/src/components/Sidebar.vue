<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  currentFilter: string;
  tasks: any[];
}>();

const emit = defineEmits<{
  (e: "update:filter", filter: string): void;
  (e: "add-task"): void;
  (e: "open-path"): void;
}>();

// State
const isCollapsed = ref(false);
const sidebarWidth = ref(250);
const isResizing = ref(false);

// Constants
const MIN_WIDTH = 200;
const MAX_WIDTH = 450;
const COLLAPSED_WIDTH = 70;

// Helper to count tasks
const counts = computed(() => {
  const all = props.tasks.length;
  // Simple date checks
  const now = new Date();
  const todayStr = now.toISOString().split("T")[0];

  // Next 7 days
  const next7 = new Date();
  next7.setDate(now.getDate() + 7);
  const next7Str = now.toISOString().split("T")[0];

  const todayCount = props.tasks.filter(
    (t) => t.due_date === todayStr && !t.completed,
  ).length;
  const next7Count = props.tasks.filter((t) => {
    if (!t.due_date || t.completed) return false;
    return t.due_date >= todayStr && t.due_date <= next7Str;
  }).length;

  return { all, today: todayCount, next7: next7Count };
});

// Priority configuration
const priorityConfig = [
  { key: "A", label: "High Priority", colorClass: "dot-a" },
  { key: "B", label: "Medium Priority", colorClass: "dot-b" },
  { key: "C", label: "Low Priority", colorClass: "dot-c" },
  { key: "D", label: "Priority D", colorClass: "dot-d" },
  { key: "E", label: "Priority E", colorClass: "dot-e" },
];

// Compute which priorities exist in tasks
const existingPriorities = computed(() => {
  const priorities = new Set<string>();
  props.tasks.forEach((task) => {
    if (task.priority) {
      priorities.add(task.priority);
    }
  });
  return priorities;
});

// Check if there are tasks without priority
const hasNoPriorityTasks = computed(() => {
  return props.tasks.some((task) => !task.priority);
});

function isFilterActive(filter: string) {
  return props.currentFilter === filter;
}

// Resizing Logic
function startResize() {
  isResizing.value = true;
  document.addEventListener("mousemove", handleResize);
  document.addEventListener("mouseup", stopResize);
  document.body.style.cursor = "col-resize";
  document.body.style.userSelect = "none"; // Prevent selection while dragging
}

function handleResize(e: MouseEvent) {
  if (isCollapsed.value) return;
  // Limit width
  let newWidth = e.clientX;
  if (newWidth < MIN_WIDTH) newWidth = MIN_WIDTH;
  if (newWidth > MAX_WIDTH) newWidth = MAX_WIDTH;
  sidebarWidth.value = newWidth;
}

function stopResize() {
  isResizing.value = false;
  document.removeEventListener("mousemove", handleResize);
  document.removeEventListener("mouseup", stopResize);
  document.body.style.cursor = "";
  document.body.style.userSelect = "";
}

function toggleSidebar() {
  isCollapsed.value = !isCollapsed.value;
}

// Icons (Simple SVGs)
const icons = {
  plus: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>`,
  inbox: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path><polyline points="22,6 12,13 2,6"></polyline></svg>`,
  today: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line></svg>`,
  calendar: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line><text x="7" y="17" font-size="6" font-family="sans-serif">7</text></svg>`,
  folder: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>`,
  chevronLeft: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"></polyline></svg>`,
  chevronRight: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"></polyline></svg>`,
};
</script>

<template>
  <aside
    class="sidebar"
    :class="{ 'sidebar-collapsed': isCollapsed, 'is-resizing': isResizing }"
    :style="{
      width: isCollapsed ? COLLAPSED_WIDTH + 'px' : sidebarWidth + 'px',
    }"
  >
    <!-- Add Task Button Area -->
    <div class="sidebar-top">
      <button class="btn-add" @click="emit('add-task')" title="Add Task">
        <span class="icon-plus" v-html="icons.plus"></span>
        <span v-if="!isCollapsed" class="label">Add Task</span>
      </button>
    </div>

    <!-- Navigation -->
    <nav class="sidebar-nav">
      <!-- Main Filters -->
      <div class="nav-group">
        <button
          class="nav-item"
          :class="{ active: isFilterActive('all') }"
          @click="emit('update:filter', 'all')"
          title="Inbox"
        >
          <span class="nav-icon" v-html="icons.inbox"></span>
          <span v-if="!isCollapsed" class="nav-label">Inbox</span>
          <span v-if="!isCollapsed" class="nav-count">{{ counts.all }}</span>
        </button>
        <button
          class="nav-item"
          :class="{ active: isFilterActive('today') }"
          @click="emit('update:filter', 'today')"
          title="Today"
        >
          <span class="nav-icon" v-html="icons.today"></span>
          <span v-if="!isCollapsed" class="nav-label">Today</span>
          <span v-if="!isCollapsed && counts.today" class="nav-count">{{
            counts.today
          }}</span>
        </button>
        <button
          class="nav-item"
          :class="{ active: isFilterActive('next7') }"
          @click="emit('update:filter', 'next7')"
          title="Next 7 Days"
        >
          <span class="nav-icon" v-html="icons.calendar"></span>
          <span v-if="!isCollapsed" class="nav-label">Next 7 Days</span>
          <span v-if="!isCollapsed && counts.next7" class="nav-count">{{
            counts.next7
          }}</span>
        </button>
      </div>

      <!-- Priorities -->
      <div class="nav-group" v-if="existingPriorities.size > 0 || hasNoPriorityTasks">
        <div class="nav-header" v-if="!isCollapsed">Priorities</div>

        <button
          v-for="priority in priorityConfig"
          :key="priority.key"
          v-show="existingPriorities.has(priority.key)"
          class="nav-item"
          :class="{ active: isFilterActive('priority:' + priority.key) }"
          @click="emit('update:filter', 'priority:' + priority.key)"
          :title="priority.label"
        >
          <span class="dot-priority" :class="priority.colorClass">{{ priority.key }}</span>
          <span v-if="!isCollapsed" class="nav-label">{{ priority.label }}</span>
        </button>

        <!-- No priority (gray dash) -->
        <button
          v-if="hasNoPriorityTasks"
          class="nav-item"
          :class="{ active: isFilterActive('priority:none') }"
          @click="emit('update:filter', 'priority:none')"
          title="No Priority"
        >
          <span class="dot-priority dot-none">-</span>
          <span v-if="!isCollapsed" class="nav-label">No Priority</span>
        </button>
      </div>
    </nav>

    <!-- Footer -->
    <div class="sidebar-footer">
      <button
        class="nav-item open-btn"
        @click="emit('open-path')"
        title="Open Folder"
      >
        <span class="nav-icon" v-html="icons.folder"></span>
        <span v-if="!isCollapsed" class="nav-label">Open Path</span>
      </button>
      <button class="btn-toggle" @click="toggleSidebar" title="Toggle Sidebar">
        <span
          class="nav-icon"
          v-html="isCollapsed ? icons.chevronRight : icons.chevronLeft"
        ></span>
        <span v-if="!isCollapsed" class="nav-label">Toggle Sidebar</span>
      </button>
    </div>

    <!-- Resize Handle -->
    <div class="resize-handle" @mousedown="startResize"></div>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  background-color: #f8f9fa; /* Light gray inspired by design */
  border-right: 1px solid var(--color-border);
  position: relative;
  /* Transition handled by checking 'is-resizing' class to avoid lag during drag if needed, 
     but 'width' transition is nice for collapse */
  transition: width 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
  flex-shrink: 0;
  height: 100vh;
  z-index: 10;
}

.is-resizing {
  transition: none; /* Disable transition during resize for responsiveness */
}

/* --- Top Section (Add Button) --- */
.sidebar-top {
  padding: 20px 16px;
}

.btn-add {
  background-color: var(--color-primary);
  color: white;
  border: none;
  border-radius: 8px; /* Slightly rounded */
  height: 44px;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: 1rem;
  box-shadow: 0 2px 5px rgba(59, 130, 246, 0.3);
  transition: all 0.2s;
  overflow: hidden;
}

.btn-add:hover {
  background-color: var(--color-primary-hover);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(59, 130, 246, 0.4);
}

.sidebar-collapsed .btn-add {
  width: 44px;
  height: 44px;
  border-radius: 50%; /* Circle when collapsed */
  padding: 0;
}

.sidebar-collapsed .sidebar-top {
  padding: 20px 0; /* Center align horizontally */
  display: flex;
  justify-content: center;
}

/* --- Navigation --- */
.sidebar-nav {
  flex: 1;
  overflow-y: auto;
  padding: 0 10px;
}

.nav-group {
  margin-bottom: 24px;
}

.nav-header {
  font-size: 0.75rem;
  text-transform: uppercase;
  color: var(--color-text-secondary);
  font-weight: 700;
  margin-bottom: 8px;
  padding-left: 12px;
  letter-spacing: 0.05em;
}

.nav-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 10px 12px;
  margin-bottom: 2px;
  border: none;
  background: transparent;
  border-radius: 8px;
  cursor: pointer;
  color: var(--color-text);
  font-size: 0.95rem;
  transition: background-color 0.1s;
  text-align: left;
  height: 40px;
}

.nav-item:hover {
  background-color: rgba(0, 0, 0, 0.05);
}

.nav-item.active {
  background-color: #e0f2fe; /* Light blue */
  color: var(--color-primary);
  font-weight: 500;
}

.sidebar-collapsed .nav-item {
  justify-content: center;
  padding: 0;
  width: 40px;
  margin: 0 auto 4px auto;
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  /* transition: margin 0.2s; */
}

.nav-label {
  flex: 1;
  margin-left: 10px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-count {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  background-color: rgba(0, 0, 0, 0.05);
  padding: 2px 8px;
  border-radius: 10px;
}

/* Priority Dots */
.dot-priority {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  color: white;
  font-size: 0.75rem;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.dot-a {
  background-color: var(--color-danger);
}
.dot-b {
  background-color: var(--color-priority-b);
}
.dot-c {
  background-color: var(--color-priority-c);
}
.dot-d {
  background-color: var(--color-priority-d);
}
.dot-e {
  background-color: var(--color-priority-e);
}
.dot-none {
  background-color: var(--color-text-secondary);
}

/* Footer */
.sidebar-footer {
  padding: 16px 10px;
  border-top: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.btn-toggle {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 8px 12px;
  border: none;
  background: transparent;
  cursor: pointer;
  color: var(--color-text-secondary);
  border-radius: 8px;
  height: 40px;
}
.btn-toggle:hover {
  background-color: rgba(0, 0, 0, 0.05);
  color: var(--color-text);
}

.sidebar-collapsed .btn-toggle {
  justify-content: center;
  padding: 0;
  width: 40px;
  margin: 0 auto;
}

/* Resize Handle */
.resize-handle {
  position: absolute;
  top: 0;
  right: 0;
  width: 4px; /* Grabbable area */
  height: 100%;
  cursor: col-resize;
  transition: background-color 0.2s;
  z-index: 20;
}

.resize-handle:hover,
.is-resizing .resize-handle {
  background-color: var(--color-primary);
}
</style>
