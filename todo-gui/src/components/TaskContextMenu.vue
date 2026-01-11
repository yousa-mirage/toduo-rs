<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";

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
  visible: boolean;
  x: number;
  y: number;
  task: Task | null;
}>();

const emit = defineEmits<{
  (e: "edit", task: Task): void;
  (e: "copy", task: Task): void;
  (e: "delete", task: Task): void;
  (e: "close"): void;
}>();

// Menu dimensions
const menuHeight = ref(140); // Estimated menu height in px
const menuWidth = ref(160); // Min menu width in px

// Calculate position - flip upward if not enough space below
const menuStyle = computed(() => {
  const padding = 4; // Safety padding
  const viewportHeight = window.innerHeight;
  const viewportWidth = window.innerWidth;

  // Check if there's enough space below
  const spaceBelow = viewportHeight - props.y - padding;
  const spaceAbove = props.y - padding;

  // Flip upward if space below is insufficient
  const flipUpward = spaceBelow < menuHeight.value && spaceAbove > spaceBelow;

  // Calculate final position
  let top: number;
  if (flipUpward) {
    // Position menu above the click point
    top = props.y - menuHeight.value;
  } else {
    // Position menu below the click point (default)
    top = props.y;
  }

  // Ensure menu stays within horizontal bounds
  let left = props.x;
  if (left + menuWidth.value > viewportWidth) {
    left = viewportWidth - menuWidth.value - padding;
  }
  if (left < padding) {
    left = padding;
  }

  return {
    top: `${top}px`,
    left: `${left}px`,
  };
});

function handleEdit() {
  if (props.task) {
    emit("edit", props.task);
  }
  emit("close");
}

function handleCopy() {
  if (props.task) {
    emit("copy", props.task);
  }
  emit("close");
}

function handleDelete() {
  if (props.task) {
    emit("delete", props.task);
  }
  emit("close");
}

function handleClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement;
  if (target.closest(".context-menu")) {
    return;
  }
  emit("close");
}

function handleMenuMounted(el: HTMLElement) {
  if (el) {
    const rect = el.getBoundingClientRect();
    menuHeight.value = rect.height;
    menuWidth.value = rect.width;
  }
}

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
});
</script>

<template>
  <Transition name="context-menu">
    <div
      v-if="visible && task"
      ref="menuRef"
      class="context-menu"
      :style="menuStyle"
      @vue:mounted="handleMenuMounted($el)"
    >
      <div class="context-menu-item" @click="handleEdit">
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
          <path
            d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
          ></path>
          <path
            d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
          ></path>
        </svg>
        <span>Edit</span>
      </div>
      <div class="context-menu-item" @click="handleCopy">
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
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
          <path
            d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"
          ></path>
        </svg>
        <span>Copy</span>
      </div>
      <div class="context-menu-separator"></div>
      <div class="context-menu-item delete" @click="handleDelete">
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
        <span>Delete</span>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.context-menu {
  position: fixed;
  z-index: 1000;
  min-width: 160px;
  background-color: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  padding: var(--spacing-xs) 0;
  overflow: hidden;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  transition: background-color 0.1s ease;
  color: var(--color-text);
  font-size: 0.875rem;
}

.context-menu-item:hover {
  background-color: var(--color-border);
}

.context-menu-item.delete {
  color: var(--color-danger);
}

.context-menu-item.delete:hover {
  background-color: rgba(239, 68, 68, 0.1);
}

.context-menu-item svg {
  flex-shrink: 0;
}

.context-menu-separator {
  height: 1px;
  background-color: var(--color-border);
  margin: var(--spacing-xs) 0;
}

/* Animations */
.context-menu-enter-active,
.context-menu-leave-active {
  transition:
    opacity 0.15s ease,
    transform 0.15s ease;
}

.context-menu-enter-from,
.context-menu-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
