# User Journey 02: Task Creation, Estimation & Target Tracking

This guide walks you through the task management workflows in **Termodoro**: creating actionable tasks, estimating effort in Pomodoros, filtering items, and binding target tasks to active focus sessions.

---

## Table of Contents

1. [Overview & Objective](#1-overview--objective)
2. [Step-by-Step Walkthrough](#2-step-by-step-walkthrough)
   - [Step 1: Navigating to Tasks View](#step-1-navigating-to-tasks-view)
   - [Step 2: Creating a New Task with Modal Dialog](#step-2-creating-a-new-task-with-modal-dialog)
   - [Step 3: Setting an Active Target Task](#step-3-setting-an-active-target-task)
   - [Step 4: Automatic Effort Logging on Timer Completion](#step-4-automatic-effort-logging-on-timer-completion)
   - [Step 5: Filtering & Completing Tasks](#step-5-filtering--completing-tasks)
   - [Step 6: Deleting or Adjusting Estimates](#step-6-deleting-or-adjusting-estimates)
3. [Visual References & Layouts](#3-visual-references--layouts)
4. [Tasks View Keybindings](#4-tasks-view-keybindings)

---

## 1. Overview & Objective

Working with a timer is most effective when paired with a clear, discrete objective. Termodoro includes an interactive Task Manager that lets you plan your day's work into estimated Pomodoro blocks and automatically track your progress without leaving your terminal.

---

## 2. Step-by-Step Walkthrough

### Step 1: Navigating to Tasks View
From any screen in Termodoro, press `2` (or press `Tab` to cycle tabs) to enter **Tab 2: Tasks View**.

### Step 2: Creating a New Task with Modal Dialog
1. Press `a` (or `n`) to open the **New Task Modal**.
2. Type your task title (e.g., `"🚀 Implement OAuth Authentication"`).
3. Press `Tab` or `Down` to navigate to the **Estimated Pomodoros** input field.
4. Type the number of 25-minute Pomodoros you expect this task to take (e.g., `4`).
5. Press `Enter` to save the task. (Press `Esc` to cancel).

### Step 3: Setting an Active Target Task
1. Use `j` / `k` (or `Up` / `Down` arrow keys) to highlight a task in your table.
2. Press `Enter` to mark it as the **Active Target Task**.
3. A bright `🎯 [ACTIVE]` badge appears on the row, and a confirmation toast banner notifies: *"Target set to: 🚀 Implement OAuth Authentication"*.
4. Switch back to **Tab 1** (`1`), and notice the task is now prominently highlighted in the active target card under the timer clock!

### Step 4: Automatic Effort Logging on Timer Completion
- When you run and complete a 25-minute work interval on Tab 1, Termodoro automatically increments the active task's spent counter: `🍅 1 / 4` $\rightarrow$ `🍅 2 / 4`.
- When `pomodoros_spent >= pomodoros_estimated`, the counter turns amber/green indicating progress milestones.

### Step 5: Filtering & Completing Tasks
- Finished your work? Press `Space` on any selected task row to toggle its completion status (`[ ]` $\rightarrow$ `[x]`).
- Press `f` to cycle through filter modes:
  - **All**: Displays all tasks.
  - **Active**: Hides completed items to keep your view distraction-free.
  - **Completed**: Reviews finished tasks for end-of-day retrospectives.

### Step 6: Deleting or Adjusting Estimates
- Need to adjust an estimate on the fly? Press `+` / `=` to increment the estimate or `-` / `_` to decrement.
- To remove an obsolete task, select it and press `d` (or `Delete` / `x`).

---

## 3. Visual References & Layouts

### Interactive Tasks Table View
![Termodoro Tasks View](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/02_tasks_view.png)

### Task Creation Modal Dialog
![Termodoro Task Modal](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/05_task_modal.png)

---

## 4. Tasks View Keybindings

| Key | Action | Description |
| :---: | :--- | :--- |
| `j` / `↓` | **Select Next** | Move selection down |
| `k` / `↑` | **Select Previous** | Move selection up |
| `a` / `n` | **Add Task** | Open task creation modal dialog |
| `Space` | **Toggle Complete** | Toggle `[ ]` / `[x]` completion checkbox |
| `Enter` | **Set Active Target** | Bind highlighted task to countdown timer |
| `d` / `x` | **Delete Task** | Remove highlighted task |
| `f` | **Cycle Filter** | Switch between All, Active, and Completed |
| `+` / `=` | **Increment Estimate** | Increase estimated pomodoro count |
| `-` / `_` | **Decrement Estimate** | Decrease estimated pomodoro count |
