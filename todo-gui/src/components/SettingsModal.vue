<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  currentPath: string;
  closeToTray: boolean;
  theme: "light" | "dark" | "system";
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "update:closeToTray", value: boolean): void;
  (e: "update:theme", value: "light" | "dark" | "system"): void;
  (e: "change-folder"): void;
}>();

function handleBackdropClick(e: MouseEvent) {
  if (e.target === e.currentTarget) {
    emit("close");
  }
}
</script>

<template>
  <div class="modal-backdrop" @click="handleBackdropClick">
    <div class="modal-container">
      <div class="modal-header">
        <h2>Settings</h2>
        <button class="close-btn" @click="emit('close')">&times;</button>
      </div>
      
      <div class="modal-body">
        <!-- File Path Section -->
        <div class="setting-group">
            <div class="group-title">Storage</div>
            <div class="path-display">
                <span class="path-label">Current File:</span>
                <div class="path-value">{{ currentPath }}</div>
            </div>
            <button class="btn btn-secondary btn-sm" @click="emit('change-folder')">
                Change Folder...
            </button>
        </div>

        <!-- Appearance Section -->
        <div class="setting-group">
            <div class="group-title">Appearance</div>
            <div class="start-item">
                <label>Theme</label>
                <div class="theme-selector">
                    <button 
                        class="theme-btn" 
                        :class="{ active: theme === 'light' }"
                        @click="emit('update:theme', 'light')"
                    >Light</button>
                    <button 
                        class="theme-btn" 
                        :class="{ active: theme === 'dark' }"
                        @click="emit('update:theme', 'dark')"
                    >Dark</button>
                    <button 
                        class="theme-btn" 
                        :class="{ active: theme === 'system' }"
                        @click="emit('update:theme', 'system')"
                    >System</button>
                </div>
            </div>
        </div>

        <!-- Window Behavior Section -->
        <div class="setting-group">
            <div class="group-title">Window</div>
            <label class="checkbox-label">
                <input 
                    type="checkbox" 
                    :checked="closeToTray" 
                    @change="emit('update:closeToTray', ($event.target as HTMLInputElement).checked)"
                />
                <span>Minimize to tray on close</span>
            </label>
            <p class="setting-desc">When enabled, closing the window will hide it to the system tray instead of quitting.</p>
        </div>

      </div>
      
      <div class="modal-footer">
        <!-- Footer info -->
        <span class="version-info">Todo.txt GUI v0.1.0</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  /* Blur effect */
  backdrop-filter: blur(2px);
}

.modal-container {
  background-color: var(--color-bg);
  width: 90%;
  max-width: 500px;
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  display: flex;
  flex-direction: column;
  animation: modal-slide-in 0.2s ease-out;
  border: 1px solid var(--color-border);
}

@keyframes modal-slide-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  padding: var(--spacing-lg) var(--spacing-lg) var(--spacing-md);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--color-text);
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--color-text-secondary);
  line-height: 1;
}

.close-btn:hover {
  color: var(--color-text);
}

.modal-body {
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
}

.setting-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
}

.group-title {
    font-size: 0.85rem;
    text-transform: uppercase;
    color: var(--color-text-secondary);
    font-weight: 700;
    margin-bottom: 4px;
    letter-spacing: 0.05em;
}

.path-display {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 8px;
}

.path-label {
    font-size: 0.9rem;
    font-weight: 500;
}

.path-value {
    background-color: var(--color-bg-secondary);
    padding: 8px;
    border-radius: var(--radius-sm);
    font-size: 0.85rem;
    word-break: break-all;
    border: 1px solid var(--color-border);
}

.theme-selector {
    display: flex;
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    padding: 4px;
    gap: 4px;
    margin-top: 4px;
}

.theme-btn {
    flex: 1;
    border: none;
    background: transparent;
    padding: 6px;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: 0.9rem;
    color: var(--color-text-secondary);
    transition: all 0.2s;
}

.theme-btn.active {
    background-color: var(--color-bg);
    color: var(--color-primary);
    box-shadow: var(--shadow-sm);
    font-weight: 600;
}

.checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-weight: 500;
}

.setting-desc {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
    margin-left: 24px; /* Align with text logic roughly */
    line-height: 1.4;
}

.modal-footer {
    padding: var(--spacing-md) var(--spacing-lg);
    background-color: var(--color-bg-secondary);
    border-top: 1px solid var(--color-border);
    border-bottom-left-radius: var(--radius-lg);
    border-bottom-right-radius: var(--radius-lg);
    text-align: center;
}

.version-info {
    font-size: 0.8rem;
    color: var(--color-text-secondary);
}

.btn-secondary {
    align-self: flex-start;
}
.btn-sm {
    padding: 4px 12px;
    font-size: 0.85rem;
}
</style>
