# Termodoro: Application Walkthrough & User Guide

This document provides a comprehensive operational guide for **Termodoro**, detailing user workflows, interface layouts, and validation benchmarks.

---

## 1. Application Layout & Navigation

Termodoro organizes its interface into four functional views selectable via numeric keys (`1` - `4`) or through sequential cycling (`Tab` / `Shift+Tab`).

```
┌──────────────────┬──────────────────────────────────────────────────────────────────┐
│  🍅 Termodoro    │  [1] Timer  │  [2] Tasks  │  [3] Stats  │  [4] Settings            │
├──────────────────┴──────────────────────────────────────────────────────────────────┤
│                                                                                     │
│                                  Active Tab Content                                 │
│                                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────┤
│  [Tab] Switch Tab   [?] Help Modal   [q] Quit  │ Theme: Catppuccin Mocha            │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core Workflows

### 2.1 Starting a Focus Session (Timer View)

1. **Launch**: Launch Termodoro with `cargo run --release` or `./target/release/termodoro`.
2. **Start Countdown**: Press `Space` to initiate the countdown timer.
3. **Pause / Resume**: Press `Space` at any time to pause or resume.
4. **Reset**: Press `r` to stop the timer and reset the countdown back to full duration.
5. **Phase Skipping**: Press `s` to immediately skip the current phase and advance to the next interval (e.g., from Work to Short Break).
6. **Cycle Completion**: Upon reaching `00:00`, a desktop notification is dispatched, an audio bell sounds (if enabled), and session history is recorded.

---

### 2.2 Managing Tasks & Linking Focus Targets (Tasks View)

1. **Navigate**: Press `2` or `Tab` to enter the Tasks view.
2. **Add Task**: Press `a` to open the task creation dialog:
   - Type the task description.
   - Press `Tab` to switch to estimated Pomodoro count and adjust with `←` / `→` or `+` / `-`.
   - Press `Enter` to save the task.
3. **Set Active Focus Target**: Highlight a task with `↑` / `k` or `↓` / `j` and press `t`. This assigns the task to the timer. As Pomodoro work sessions complete, effort is automatically logged against this task.
4. **Toggle Completion**: Press `Space` or `Enter` to mark the highlighted task complete.
5. **Filtering**: Press `1` for all tasks, `2` for active tasks, or `3` for completed tasks.
6. **Delete**: Highlight a task and press `d` or `x` to delete it.

---

### 2.3 Reviewing Productivity Metrics (Stats View)

1. **Navigate**: Press `3` to access the Analytics & Stats dashboard.
2. **Review Metrics**:
   - **Today's Focus**: Completed work sessions and total minutes logged today.
   - **Current Streak**: Active daily streak in consecutive calendar days.
   - **All-Time Focus**: Total cumulative sessions and focus hours across all logged history.
3. **Activity Bar Chart**: View the 7-day session distribution to assess productivity consistency.
4. **Session Log**: Inspect the table of recent sessions to review start times, phase types, durations, and associated task titles.

---

### 2.4 Customizing Durations & Visual Themes (Settings View)

1. **Navigate**: Press `4` to enter the Settings view.
2. **Adjust Values**:
   - Use `↑` / `↓` to select a configuration row.
   - Use `←` / `→` or `+` / `-` to adjust numeric durations (e.g., increase Work session from 25 to 30 minutes).
   - Press `Space` or `Enter` to toggle boolean options (Auto-start Breaks, Desktop Notifications, Sound Alerts).
   - Use `←` / `→` on the **Color Theme** row to cycle between palettes:
     - *Catppuccin Mocha*
     - *Nord*
     - *Gruvbox Dark*
     - *Tokyo Night*
     - *Dracula*
     - *Solarized Dark*
3. **Persistence**: All configuration adjustments are immediately written to local storage.

---

## 3. Keyboard Shortcuts Reference Table

| Scope | Shortcut | Action |
| :--- | :--- | :--- |
| **Global** | `Tab` / `Shift+Tab` | Cycle forward / backward through tabs |
| **Global** | `1` / `2` / `3` / `4` | Switch directly to Timer / Tasks / Stats / Settings |
| **Global** | `?` | Toggle Help & Keybindings modal |
| **Global** | `q` / `Esc` | Quit application / Dismiss open modal |
| **Timer** | `Space` | Toggle start / pause timer |
| **Timer** | `r` | Reset timer countdown |
| **Timer** | `s` | Skip to next phase |
| **Timer** | `a` | Quick add new task |
| **Tasks** | `a` | Add new task |
| **Tasks** | `Space` / `Enter` | Toggle completed status |
| **Tasks** | `t` | Set selected task as active timer target |
| **Tasks** | `d` / `x` | Delete selected task |
| **Tasks** | `↑` / `k`, `↓` / `j` | Navigate tasks list |
| **Tasks** | `1` / `2` / `3` | Filter tasks (All / Active / Completed) |
| **Settings** | `↑` / `k`, `↓` / `j` | Select setting row |
| **Settings** | `←` / `h`, `→` / `l` | Adjust duration values / cycle color themes |
| **Settings** | `+` / `-` | Increment / decrement values |
| **Settings** | `Space` / `Enter` | Toggle boolean feature flags |

---

## 4. Verification & Testing Summary

### Automated Test Suite
The test suite validates data consistency and state transitions across the application. Execute via:

```bash
cargo test
```

#### Test Results Matrix

| Test Suite | Test Identifier | Verification Focus | Status |
| :--- | :--- | :--- | :--- |
| `tasks::tests` | `test_task_lifecycle` | Task creation, active target binding, effort counters, completion toggling | Passed |
| `tasks::tests` | `test_task_filtering` | Filter predicates (`All`, `Active`, `Completed`) | Passed |
| `timer::tests` | `test_timer_initialization` | Default configuration durations and stopped state | Passed |
| `timer::tests` | `test_phase_advancement` | Full cycle progression (Work ➔ ShortBreak ➔ ... ➔ LongBreak) | Passed |
| `timer::tests` | `test_progress_ratio` | Percentage completion precision | Passed |
| `timer::tests` | `test_pause_and_reset` | Start, pause, and reset transitions | Passed |
| `stats::tests` | `test_stats_recording` | Daily minute aggregation, session counts, and streak evaluations | Passed |

---

## 5. Build Optimization

For production use, compile the optimized release binary with Link-Time Optimization (LTO) enabled:

```bash
cargo build --release
```

Binary artifact is generated at `./target/release/termodoro`.
