# Rust Todo.txt 全栈项目实施计划文档

**核心理念**: 标准协议驱动 (Protocol Driven) | 双端原生体验 (Native Performance)
**基础依赖**: `todo-txt` crate (Standard Parsing)

---

## 1. 项目愿景 (Vision)

本项目旨在构建一个**严格遵循 todo.txt 协议**的高性能任务管理系统。通过利用 Rust 生态中现有的 `todo-txt` crate，我们将开发重心从基础解析转移到构建卓越的用户体验上。

### 核心特质

1. **协议原生 (Protocol Native)**: 数据存储仅依赖纯文本的 `.txt` 文件。任何对任务的修改（优先级、完成状态、创建日期）都必须完美映射回 todo.txt 的标准语法字符串，不产生任何非标准元数据。
2. **结构化录入 (Structured Input)**: 尽管底层是文本，但用户交互层（GUI/TUI）必须提供结构化的表单（Form）。用户通过下拉框选择优先级、日期选择器选择时间，程序负责将其“编译”为 todo.txt 文本，杜绝格式错误。
3. **现代化 GUI**: 使用 **Vue 3** 构建，抛弃传统的桌面软件外观，打造 Web 级精致的 UI。
4. **极客 TUI**: 使用 **Nord 主题**，为键盘党提供极致的终端效率工具。

---

## 2. 技术栈架构 (Tech Stack)

采用 Cargo Workspace (Monorepo) 结构。

| 模块 | 类型 | 关键依赖 | 职责描述 |
| :---- | :--- | :------- | :--------- |
| **todo-core** | Lib | **`todo-txt`**, `fs2` (文件锁), `chrono`, `serde` | 封装 `todo-txt` crate，管理文件原子读写，维护任务行号(ID)映射。 |
| **todo-tui** | Bin | `ratatui`, `crossterm` | 终端界面，Nord 配色，Vim 风格键位。 |
| **todo-gui** | Bin | **Backend**: `tauri` **Frontend**: `Vue3`, `headlessui` | 现代化桌面界面，通过 Tauri Invoke 调用 Core。 |

---

## 3. 详细架构设计

### 3.1 核心库 (`todo-core`) - 协议守护者

直接利用社区验证过的 `todo-txt` crate 进行序列化与反序列化，确保最大的兼容性。

**核心结构体封装**:

```rust
// src/model.rs
use todo_txt::Task as RawTask;

// 我们需要包装原始 Task，因为 todo.txt 文件本身不保存 ID（行号），
// 但前端操作需要 ID 来定位修改哪一行。
#[derive(Debug, Serialize, Clone)]
pub struct AppTask {
    pub id: usize,          // 对应文件中的物理行号（1-based）
    pub raw_content: String,// 原始文本（用于无损显示或校验）
    pub parsed: RawTask,    // 来自 todo-txt crate 的结构化数据
}
```

**关键业务逻辑**:

1. **加载 (Load)**: 读取文件 -> 按行分割 -> `todo_txt::Task::from_str` -> 注入 `id` -> 返回 `Vec<AppTask>`。
2. **结构化组装 (Builder)**: 接收前端传来的结构化字段（Description, Priority, Projects...），使用 `todo_txt::TaskBuilder` 构建标准 Task 对象。
3. **持久化 (Persist)**: 将内存中的 Task 列表转为 String -> 使用 `fs2` 获取文件排他锁 -> 原子写入磁盘 -> 释放锁。

---

### 3.2 图形界面 (`todo-gui`) - Vue 原生样式

**设计理念**: 使用 Vue 3 原生的 Scoped CSS 构建响应式、组件化的 UI。

**样式规范**:

```css
/* Layout */
.app-container {
  height: 100vh;
  width: 100%;
  display: flex;
  flex-direction: column;
  background-color: #f8fafc;
  color: #0f172a;
}

/* Task Card */
.task-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  margin-bottom: 0.5rem;
  background-color: #ffffff;
  border-radius: 0.5rem;
  box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  border: 1px solid #e2e8f0;
  transition: all 0.2s ease;
}

.task-card:hover {
  box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
}

/* Priority Badge */
.priority-badge {
  padding: 0.125rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  font-weight: 700;
}

.priority-badge--a {
  background-color: #fee2e2;
  color: #b91c1c;
}

.priority-badge--b {
  background-color: #fef3c7;
  color: #b45309;
}

/* Tags */
.tag {
  color: #3b82f6;
  cursor: pointer;
  font-size: 0.875rem;
  margin-right: 0.5rem;
}

.tag:hover {
  text-decoration: underline;
}
```

**核心组件 (Components)**:

1. **`SmartTaskList.vue`**:
    - **功能**: 负责"已完成沉底"的视觉排序。
    - **逻辑**: 获取原始列表后，使用 computed 属性生成渲染列表。
    - `const displayTasks = computed(() => [...tasks].sort((a,b) => a.completed - b.completed || ...))`
2. **`StructuredInputModal.vue`**:
    - **功能**: 替代文本输入框。
    - **UI**: 使用 Vue 原生 `<dialog>` 元素 + Scoped CSS 样式。
    - **字段**:
      - `Input (text)`: 任务描述
      - `Select (dropdown)`: 优先级 (A-Z)
      - `Autocomplete`: 项目 (+Project) 和 上下文 (@Context) - 支持自动补全现有标签。

---

### 3.3 终端界面 (`todo-tui`) - Nord Theme

**设计理念**: 极简、冷峻、高效。

**Nord 配色映射**:

- **背景**: `CurrentLine (#2E3440)` 作为 App 背景。
- **面板边框**: `Nord3 (#4C566A)`。
- **选中项**: `Nord2 (#434C5E)` 高亮背景。
- **优先级 A**: `Aurora Red (#BF616A)`。
- **完成标记**: `Aurora Green (#A3BE8C)`。

**交互流程**:

1. 用户按下 `a` (Add)。
2. 弹出 TUI Modal (覆盖在列表之上的 Block)。
3. 用户通过 `Tab` 在 Description, Priority, Due Date 输入框之间切换。
4. 按下 `Enter` 提交 -> `todo-core` 将其转换为标准字符串 -> 写入文件。

---

## 4. 开发步骤 (Implementation Steps)

### Phase 1: 核心重构 (Core with `todo-txt` crate)

1. 初始化 Workspace。
2. 引入 `todo-txt` crate。
3. 实现 `TaskService`：
    - `load_tasks()`: 读取文件，映射行号。
    - `add_task(struct: InputData)`: 使用 builder 模式生成 Task 并追加写入。
    - `complete_task(id: usize)`: 找到对应行，设置 `completed: true` 和 `completion_date`。

### Phase 2: TUI 原型 (验证逻辑)

1. 构建 Ratatui 基本布局。
2. 集成 `todo-core` 数据源。
3. 实现 **Nord Theme** 的渲染代码。
4. 实现简单的列表浏览和“按 Space 标记完成”功能。

### Phase 3: GUI 实现 (Vue Native & Tauri)

1. 配置 Tauri + Vite + Vue3。
2. **配置 Vue 样式系统**: 使用 Vue 单文件组件的 `<style scoped>` 特性，建立 CSS 变量系统管理主题颜色。
3. 开发前端组件：
    - 先实现无样式的逻辑对接。
    - 然后应用 Scoped CSS 类名进行样式编写。
4. 对接 Tauri Commands：确保前端表单数据能正确传递给 Rust 并保存为 todo.txt 格式。

### Phase 4: 高级视图特性

1. **截止日期逻辑**: 在 Core 中解析 `due:YYYY-MM-DD`。
2. **视图层排序**:
    - 在 GUI (Computed Ref) 和 TUI (State Sort) 中分别实现“未完成优先”、“优先级高优先”的排序算法。
    - 确保这只是**显示顺序**，保存文件时依然可以使用追加模式或保持原行号（除非执行显式重排）。
