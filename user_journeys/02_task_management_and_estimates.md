# User Journey 02: Task Creation, Estimation & Target Tracking

Organize your projects, estimate effort in discrete Pomodoro blocks, bind active target tasks to your timer, and track completed workload with **Termodoro**'s interactive Task Manager.

---

## Table of Contents

1. [Journey Overview & Persona Context](#1-journey-overview--persona-context)
2. [Step-by-Step Interactive Walkthrough](#2-step-by-step-interactive-walkthrough)
   - [Step 1: Navigating to the Tasks Workstation](#step-1-navigating-to-the-tasks-workstation)
   - [Step 2: Creating a New Task via Modal Dialog](#step-2-creating-a-new-task-via-modal-dialog)
   - [Step 3: Setting an Active Target Task](#step-3-setting-an-active-target-task)
   - [Step 4: Real-Time Focus Binding & Automatic Effort Logging](#step-4-real-time-focus-binding--automatic-effort-logging)
   - [Step 5: Filtering Views & Toggling Completion](#step-5-filtering-views--toggling-completion)
   - [Step 6: Deleting Obsolete Tasks](#step-6-deleting-obsolete-tasks)
3. [Visual Layout & Interface Details](#3-visual-layout--interface-details)
4. [Under the Hood: Engineering & Logic Architecture](#4-under-the-hood-engineering--logic-architecture)
5. [Pro Tips & Power Workflows](#5-pro-tips--power-workflows)
6. [Complete Keybinding Reference](#6-complete-keybinding-reference)

---

## 1. Journey Overview & Persona Context

A timer alone provides cadence, but pairing cadence with discrete task objectives produces maximum productivity. When tackling a large project (e.g. implementing API endpoints, writing documentation, or fixing bugs), decomposing work into 25-minute Pomodoro chunks makes progress tangible and prevents procrastination.

This user journey demonstrates how a practitioner uses Termodoro to manage their daily task backlog, assign workload estimates, and automatically log focus intervals against specific goals:

```
[Press 'a' (New Task)] ──> [Set Title & Estimate (1-20)] ──> [Press 't' to Target] ──> [Run Timer on Tab 1] ──> [Auto-Increment 🍅]
```

---

## 2. Step-by-Step Interactive Walkthrough

### Step 1: Navigating to the Tasks Workstation
From anywhere in Termodoro, press `2` (or press `Tab`) to navigate to **Tab 2: Tasks View**.

---

### Step 2: Creating a New Task via Modal Dialog
1. Press `a` to bring up the **New Task Modal**.
2. Type your task title (e.g., `⚡ Refactor Storage Engine with Zero-Telemetry Invariants`).
3. Press `Tab` or `↓` to move focus to the **Estimated Pomodoros** input field.
4. Adjust your estimated effort:
   - Press `+` / `=` / `Right` to increment (up to 20 Pomodoros).
   - Press `-` / `_` / `Left` to decrement (down to 1 Pomodoro).
   - Or type a digit key (`1` - `9`) to set the estimate directly.
5. Press `Enter` to commit the task. (Press `Esc` anytime to cancel).

---

### Step 3: Setting an Active Target Task
1. Use `j` / `k` (or `↑` / `↓` arrow keys) to navigate through your task list.
2. Press `t` on the selected task.
3. A glowing `🎯 [ACTIVE]` badge instantly appears on the row, and a confirmation banner alerts: *"Target set to: ⚡ Refactor Storage Engine..."*.
4. Switch to **Tab 1 (`1`)**: Notice the task is now pinned in the active target card directly beneath the big digital countdown clock!

---

### Step 4: Real-Time Focus Binding & Automatic Effort Logging
- Start your focus timer on Tab 1 with `Space`.
- When the 25-minute work session completes, Termodoro automatically increments the active task's spent counter: `🍅 1 / 3` $\rightarrow$ `🍅 2 / 3` via `TaskManager::increment_active_spent()`.
- Once `pomodoros_spent >= pomodoros_estimated`, the counter dynamically shifts from primary theme color to amber/green, signaling that your estimated threshold has been reached.

---

### Step 5: Filtering Views & Toggling Completion
- When you finish an objective, highlight the task on Tab 2 and press `Space` (or `Enter`) to toggle its checkbox (`[ ]` $\rightarrow$ `[x]`).
- Press `1`, `2`, or `3` while on the Tasks tab to switch filter views:
  - **`1` (All Tasks)**: Shows everything in your backlog.
  - **`2` (Active Only)**: Hides completed items for a clean, distraction-free view.
  - **`3` (Completed Only)**: Displays finished items for end-of-day retrospectives and standup notes.

---

### Step 6: Deleting Obsolete Tasks
- Finished or deprecated task? Press `d` (or `x`) on Tab 2 to remove it.
- If you delete the currently active target task, `TaskManager::remove_selected()` safely clears the active target reference without causing panics or corrupting state.

---

## 3. Visual Layout & Interface Details

### Interactive Tasks Table View
![Termodoro Tasks View](../assets/screenshots/02_tasks_view.png)

### Task Creation Modal Dialog
![Termodoro Task Modal](../assets/screenshots/05_task_modal.png)

### Tasks View Components:
1. **Filter Selector Bar**: Visual badges showing `[● All (1)]`, `[  Active (2)]`, `[  Completed (3)]`.
2. **Interactive Table Rows**:
   - **Status Checkbox**: `[ ]` (Pending) or `[x]` (Completed).
   - **Active Badge**: `🎯 [ACTIVE]` indicating timer binding.
   - **Task Title**: Left-aligned with automatic Unicode cell-width truncation on narrow screens.
   - **Pomodoro Counters**: Visual emoji counter (`🍅 2 / 3`).
3. **Modal Input Overlay**: Dual-field dialog with high-visibility input cursors and validation bounds ($1 \le \text{Estimate} \le 20$).

---

## 4. Under the Hood: Engineering & Logic Architecture

- **UUID Identification**: In [`src/tasks.rs`](../src/tasks.rs), every task is assigned a globally unique V4 UUID (`uuid::Uuid::new_v4()`) and an ISO-8601 UTC creation timestamp.
- **Dynamic Filter Clamping**: When toggling between filters (`All`, `Active`, `Completed`), `selected_index` is automatically clamped to the filtered slice length to prevent index-out-of-bounds panics.
- **Atomic Persistence**: Every task mutation triggers an atomic JSON serialization to `~/.local/share/termodoro/data.json` via a `.tmp` file swap pattern.

---

## 5. Pro Tips & Power Workflows

> [!TIP]
> **Quick Add from Timer View**: You don't need to switch tabs to add a task! Press `a` while on Tab 1 (Timer View) to pop up the task creation modal instantly.

> [!IMPORTANT]
> **Zero Telemetry Guarantee**: Task titles and descriptions never leave your computer. There are zero cloud databases, remote telemetry endpoints, or web scrapers.

---

## 6. Complete Keybinding Reference

| Keybinding | Action | Codebase Handler & Behavior |
| :---: | :--- | :--- |
| `j` / `↓` | **Select Next** | [`src/app.rs:535`](../src/app.rs#L535) (`tasks.next()`) |
| `k` / `↑` | **Select Previous** | [`src/app.rs:540`](../src/app.rs#L540) (`tasks.previous()`) |
| `a` | **Add Task** | [`src/app.rs:502`](../src/app.rs#L502) (`open_task_modal()`) |
| `Space` / `Enter` | **Toggle Done** | [`src/app.rs:507`](../src/app.rs#L507) (`tasks.toggle_selected()`) |
| `t` | **Set Active Target** | [`src/app.rs:514`](../src/app.rs#L514) (`tasks.set_selected_active()`) |
| `d` / `x` | **Delete Task** | [`src/app.rs:526`](../src/app.rs#L526) (`tasks.remove_selected()`) |
| `1` | **Filter: All** | [`src/app.rs:545`](../src/app.rs#L545) (`tasks.filter = TaskFilter::All`) |
| `2` | **Filter: Active** | [`src/app.rs:552`](../src/app.rs#L552) (`tasks.filter = TaskFilter::Active`) |
| `3` | **Filter: Completed** | [`src/app.rs:559`](../src/app.rs#L559) (`tasks.filter = TaskFilter::Completed`) |
| `Tab` / `BackTab` | **Switch Tabs** | [`src/app.rs:268-286`](../src/app.rs#L268-L286) (`next_tab()` / `previous_tab()`) |
| `?` | **Help Overlay** | [`src/app.rs:261`](../src/app.rs#L261) (`show_help = true`) |
| `q` | **Quit** | [`src/app.rs:254`](../src/app.rs#L254) (`should_quit = true`) |
