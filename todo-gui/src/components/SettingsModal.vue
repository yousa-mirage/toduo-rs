<script setup lang="ts">
import logoUrl from "../assets/logo.png";

defineProps<{
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
        <!-- Storage Row -->
        <div class="setting-row path-row">
          <div class="input-wrapper">
            <div class="path-value" :title="currentPath">{{ currentPath }}</div>
          </div>
          <button
            class="change-btn btn-secondary"
            @click="emit('change-folder')"
          >
            Change Folder...
          </button>
        </div>

        <!-- Theme Row -->
        <div class="setting-row theme-row">
          <label class="setting-label">Theme</label>
          <div class="theme-selector">
            <button
              class="theme-btn"
              :class="{ active: theme === 'light' }"
              @click="emit('update:theme', 'light')"
            >
              Light
            </button>
            <button
              class="theme-btn"
              :class="{ active: theme === 'dark' }"
              @click="emit('update:theme', 'dark')"
            >
              Dark
            </button>
            <button
              class="theme-btn"
              :class="{ active: theme === 'system' }"
              @click="emit('update:theme', 'system')"
            >
              System
            </button>
          </div>
        </div>

        <!-- Window Row -->
        <div class="setting-row">
          <label class="checkbox-label">
            <input
              type="checkbox"
              :checked="closeToTray"
              @change="
                emit(
                  'update:closeToTray',
                  ($event.target as HTMLInputElement).checked,
                )
              "
            />
            <span>Minimize to tray on close</span>
          </label>
          <p class="setting-desc">
            When enabled, closing the window will hide it to the system tray
            instead of quitting.
          </p>
        </div>
      </div>

      <div class="modal-footer">
        <div class="footer-left">
          <img :src="logoUrl" class="footer-logo" />
          <span class="footer-brand">ToDuo-rs</span>
          <span class="footer-ver">v0.1.0</span>
        </div>
        <div class="footer-right">
          <span class="footer-author">Made with ❤️ by Yousa-Mirage</span>
          <a
            href="https://github.com/Yousa-Mirage/ToDuo-rs"
            target="_blank"
            class="about-link"
            >https://github.com/Yousa-Mirage/ToDuo-rs</a
          >
        </div>
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
  max-width: 550px;
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
  padding: var(--spacing-lg);
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
  gap: 2rem;
}

.setting-row {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* Path Row Layout */
.path-row {
  flex-direction: row;
  align-items: center;
  gap: 0.75rem;
}

.input-wrapper {
  flex: 1;
  min-width: 0;
}

.path-value {
  background-color: var(--color-bg-secondary);
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.9rem;
  border: 1px solid var(--color-border);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--color-text);
}

.change-btn {
  white-space: nowrap;
  height: 36px;
  padding: 0 16px;
  border: 1px solid var(--color-border);
  background-color: var(--color-bg);
  color: var(--color-text);
  border-radius: var(--radius-md);
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
  font-size: 0.85rem;
}

.change-btn:hover {
  background-color: var(--color-bg-secondary);
}

/* Theme Row Layout */
.theme-row {
  flex-direction: row;
  align-items: center;
  gap: 1rem;
}

.setting-label {
  font-weight: 700;
  color: var(--color-text);
  min-width: 60px;
}

.theme-selector {
  display: flex;
  background-color: var(--color-bg-secondary);
  border-radius: var(--radius-md);
  padding: 4px;
  gap: 4px;
  flex: 1;
}

.theme-btn {
  flex: 1;
  border: none;
  background: transparent;
  padding: 6px 12px;
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
  gap: 0.75rem;
  cursor: pointer;
  font-weight: 500;
  color: var(--color-text);
}

.setting-desc {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  margin-left: 28px;
  margin-top: -4px;
  line-height: 1.4;
}

.modal-footer {
  padding: var(--spacing-md) var(--spacing-lg);
  background-color: var(--color-bg-secondary);
  border-top: 1px solid var(--color-border);
  border-bottom-left-radius: var(--radius-lg);
  border-bottom-right-radius: var(--radius-lg);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.footer-logo {
  width: 20px;
  height: 20px;
}

.footer-brand {
  font-weight: 700;
  color: var(--color-text);
  font-size: 0.95rem;
}

.footer-ver {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  opacity: 0.8;
  /* Removed monospace font to match other text */
}

.footer-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
}

.footer-author {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.about-link {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
  text-decoration: none;
  opacity: 0.7;
}

.about-link:hover {
  text-decoration: underline;
  opacity: 1;
}
</style>
