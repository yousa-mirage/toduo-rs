<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from "vue";

const props = defineProps<{
  modelValue: string | null;
  placeholder?: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string | null): void;
}>();

const showPicker = ref(false);
const pickerRef = ref<HTMLElement | null>(null);

// Date logic
const today = new Date();
const currentCursor = ref(new Date()); // The month we are viewing

// Sync cursor with modelValue if present
watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      const d = new Date(val);
      if (!isNaN(d.getTime())) {
        currentCursor.value = new Date(d.getFullYear(), d.getMonth(), 1);
      }
    }
  },
  { immediate: true }
);

// Helpers
const monthNames = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December"
];

const currentMonthLabel = computed(() => {
  return `${monthNames[currentCursor.value.getMonth()]} ${currentCursor.value.getFullYear()}`;
});

const daysInMonth = computed(() => {
  const year = currentCursor.value.getFullYear();
  const month = currentCursor.value.getMonth();
  return new Date(year, month + 1, 0).getDate();
});

const firstDayOffset = computed(() => {
  const year = currentCursor.value.getFullYear();
  const month = currentCursor.value.getMonth();
  return new Date(year, month, 1).getDay(); // 0 = Sunday
});

const paddingDays = computed(() => {
  const used = firstDayOffset.value + daysInMonth.value;
  return 42 - used;
});

// Navigation
function prevMonth() {
  currentCursor.value = new Date(
    currentCursor.value.getFullYear(),
    currentCursor.value.getMonth() - 1,
    1
  );
}

function nextMonth() {
  currentCursor.value = new Date(
    currentCursor.value.getFullYear(),
    currentCursor.value.getMonth() + 1,
    1
  );
}

// Selection
function selectDay(day: number) {
  const year = currentCursor.value.getFullYear();
  const month = currentCursor.value.getMonth() + 1; // 1-based
  
  const yStr = year.toString();
  const mStr = month.toString().padStart(2, '0');
  const dStr = day.toString().padStart(2, '0');
  
  const dateStr = `${yStr}-${mStr}-${dStr}`;
  emit("update:modelValue", dateStr);
  showPicker.value = false;
}

function isSelected(day: number) {
  if (!props.modelValue) return false;
  const d = new Date(props.modelValue);
  return (
    d.getFullYear() === currentCursor.value.getFullYear() &&
    d.getMonth() === currentCursor.value.getMonth() &&
    d.getDate() === day
  );
}

function isToday(day: number) {
  return (
    today.getFullYear() === currentCursor.value.getFullYear() &&
    today.getMonth() === currentCursor.value.getMonth() &&
    today.getDate() === day
  );
}

// Click Outside
function handleClickOutside(e: MouseEvent) {
  if (pickerRef.value && !pickerRef.value.contains(e.target as Node)) {
    showPicker.value = false;
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
  <div class="date-picker-container" ref="pickerRef">
    <!-- Input Trigger -->
    <div class="input-wrapper" @click="showPicker = !showPicker">
      <input
        type="text"
        :value="modelValue"
        readonly
        class="date-input"
        :placeholder="placeholder || 'yyyy-mm-dd'"
      />
      <span class="calendar-icon">📅</span>
    </div>

    <!-- Popup -->
    <div v-if="showPicker" class="picker-popup">
      <div class="picker-header">
        <button type="button" class="nav-btn" @click.stop="prevMonth">&lt;</button>
        <span class="month-label">{{ currentMonthLabel }}</span>
        <button type="button" class="nav-btn" @click.stop="nextMonth">&gt;</button>
      </div>
      
      <div class="picker-grid">
        <!-- Weekdays -->
        <div class="weekday" v-for="d in ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa']" :key="d">
          {{ d }}
        </div>
        
        <!-- Empty slots -->
        <div 
          v-for="n in firstDayOffset" 
          :key="'empty-' + n" 
          class="day empty"
        ></div>
        
        <!-- Days -->
        <div 
          v-for="day in daysInMonth" 
          :key="day" 
          class="day"
          :class="{ 
            selected: isSelected(day),
            today: isToday(day)
          }"
          @click.stop="selectDay(day)"
        >
          {{ day }}
        </div>
        
        <!-- Trailing Empty slots -->
        <div 
          v-for="n in paddingDays" 
          :key="'end-empty-' + n" 
          class="day empty"
        ></div>
      </div>
      
      <div class="picker-footer">
          <button type="button" class="clear-btn" @click.stop="emit('update:modelValue', null)">Clear</button>
          <button type="button" class="today-btn" @click.stop="() => {
             const now = new Date();
             const y = now.getFullYear();
             const m = (now.getMonth()+1).toString().padStart(2, '0');
             const d = now.getDate().toString().padStart(2, '0');
             emit('update:modelValue', `${y}-${m}-${d}`);
             showPicker = false;
          }">Today</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.date-picker-container {
  position: relative;
  width: 100%;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.date-input {
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  padding-right: 40px; /* Space for icon */
  font-size: 1rem;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  background-color: var(--color-bg);
  color: var(--color-text);
  font-family: inherit;
  height: 42px;
  line-height: 1.5;
  cursor: pointer;
  transition: border-color 0.2s;
}

.date-input::placeholder {
  color: var(--color-text-secondary);
  opacity: 0.7;
}

.date-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.calendar-icon {
  position: absolute;
  right: 12px;
  pointer-events: none;
  opacity: 0.6;
}

/* Popup */
.picker-popup {
  position: absolute;
  bottom: 100%; /* Anchor above */
  right: 0;
  margin-bottom: 8px;
  width: 280px;
  background-color: white; /* Force light mode bg for calendar usually safer, or var */
  background-color: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  z-index: 1000;
  padding: 12px;
  animation: slideUp 0.15s ease-out;
}

@keyframes slideUp {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}

.picker-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.month-label {
    font-weight: 600;
    color: var(--color-text);
}

.nav-btn {
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: 4px;
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--color-text);
}
.nav-btn:hover {
    background-color: var(--color-border);
}

.picker-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 4px;
    margin-bottom: 8px;
}

.weekday {
    text-align: center;
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    padding-bottom: 4px;
}

.day {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 32px; /* Fixed height for all days */
    font-size: 0.9rem;
    border-radius: 4px;
    cursor: pointer;
    color: var(--color-text);
}

.day:hover {
    background-color: var(--color-bg);
}

.day.empty {
    cursor: default;
}
.day.empty:hover {
    background-color: transparent;
}

.day.today {
    color: var(--color-primary);
    font-weight: 700;
}

.day.selected {
    background-color: var(--color-primary);
    color: white;
}

.picker-footer {
    display: flex;
    justify-content: space-between;
    border-top: 1px solid var(--color-border);
    padding-top: 8px;
}

.clear-btn, .today-btn {
    background: none;
    border: none;
    font-size: 0.8rem;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
}

.clear-btn { color: var(--color-text-secondary); }
.clear-btn:hover { background-color: var(--color-danger); color: white; }

.today-btn { color: var(--color-primary); }
.today-btn:hover { background-color: var(--color-primary-hover); color: white; }

</style>
