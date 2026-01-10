<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  currentFilter: string;
  projects: string[];
  contexts: string[];
  tasks: any[];
}>();

const emit = defineEmits<{
  (e: "update:filter", filter: string): void;
}>();

// Helper to count tasks for filters
const counts = computed(() => {
  const all = props.tasks.length;
  // Simple today logic - assumes format YYYY-MM-DD
  const today = new Date().toISOString().split('T')[0];
  const todayCount = props.tasks.filter(t => t.due_date === today).length;
  
  return { all, today: todayCount };
});

function isFilterActive(filter: string) {
  return props.currentFilter === filter;
}
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-section">
      <h3 class="sidebar-title">Filters</h3>
      <ul class="nav-list">
        <li
          class="nav-item"
          :class="{ active: isFilterActive('all') }"
          @click="emit('update:filter', 'all')"
        >
          <span class="nav-icon">Inbox</span>
          <span class="nav-label">All</span>
          <span class="nav-count">{{ counts.all }}</span>
        </li>
        <li
          class="nav-item"
          :class="{ active: isFilterActive('today') }"
          @click="emit('update:filter', 'today')"
        >
          <span class="nav-icon">📅</span>
          <span class="nav-label">Today</span>
          <span class="nav-count" v-if="counts.today">{{ counts.today }}</span>
        </li>
      </ul>
    </div>

    <div class="sidebar-section" v-if="projects.length > 0">
      <h3 class="sidebar-title">Projects</h3>
      <ul class="nav-list">
        <li
          v-for="project in projects"
          :key="project"
          class="nav-item"
          :class="{ active: isFilterActive('project:' + project) }"
          @click="emit('update:filter', 'project:' + project)"
        >
          <span class="nav-icon">+</span>
          <span class="nav-label">{{ project }}</span>
        </li>
      </ul>
    </div>

    <div class="sidebar-section" v-if="contexts.length > 0">
      <h3 class="sidebar-title">Contexts</h3>
      <ul class="nav-list">
        <li
          v-for="context in contexts"
          :key="context"
          class="nav-item"
          :class="{ active: isFilterActive('context:' + context) }"
          @click="emit('update:filter', 'context:' + context)"
        >
          <span class="nav-icon">@</span>
          <span class="nav-label">{{ context }}</span>
        </li>
      </ul>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 240px;
  background-color: var(--color-bg-secondary);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  padding: var(--spacing-md) 0;
  overflow-y: auto;
}

.sidebar-section {
  margin-bottom: var(--spacing-lg);
}

.sidebar-title {
  padding: 0 var(--spacing-md);
  margin-bottom: var(--spacing-xs);
  font-size: 0.75rem;
  text-transform: uppercase;
  color: var(--color-text-secondary);
  font-weight: 600;
  letter-spacing: 0.05em;
}

.nav-list {
  list-style: none;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  color: var(--color-text);
  font-size: 0.9375rem;
  transition: all 0.15s ease;
  border-left: 3px solid transparent;
}

.nav-item:hover {
  background-color: var(--color-bg);
}

.nav-item.active {
  background-color: var(--color-priority-c-bg);
  color: var(--color-primary);
  border-left-color: var(--color-primary);
}

.nav-icon {
  width: 1.5rem;
  display: flex;
  justify-content: center;
  margin-right: var(--spacing-xs);
  font-size: 1rem;
}

.nav-label {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-count {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  background-color: var(--color-bg);
  padding: 0.125rem 0.375rem;
  border-radius: 999px;
}
</style>
