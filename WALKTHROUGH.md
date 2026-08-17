# Termodoro: Application Walkthrough, Operational Guide & Code Tour

This document provides a detailed, beginner-friendly walkthrough of **Termodoro**. It covers user workflows, interface layouts, practical scenarios, codebase architecture tours, and testing procedures.

---

## Table of Contents

1. [Application Overview & Layout Hierarchy](#1-application-overview--layout-hierarchy)
2. [Step-by-Step User Workflows](#2-step-by-step-user-workflows)
   - [Scenario 1: Starting Your First Pomodoro Focus Session](#scenario-1-starting-your-first-pomodoro-focus-session)
   - [Scenario 2: Creating, Estimating, and Tracking Tasks](#scenario-2-creating-estimating-and-tracking-tasks)
   - [Scenario 3: Navigating and Interpreting Productivity Analytics](#scenario-3-navigating-and-interpreting-productivity-analytics)
   - [Scenario 4: Customizing Preferences and Color Themes](#scenario-4-customizing-preferences-and-color-themes)
3. [Codebase Architecture & Tour for Beginners](#3-codebase-architecture--tour-for-beginners)
4. [Verification, Testing & Quality Assurance](#4-verification-testing--quality-assurance)
5. [Operational Glossary](#5-operational-glossary)
6. [References and Citations](#6-references-and-citations)

---

## 1. Application Layout & Layout Hierarchy

Termodoro divides the terminal window into three main regions:

```
┌──────────────────┬──────────────────────────────────────────────────────────────────┐
│  🍅 Termodoro    │  [1] Timer  │  [2] Tasks  │  [3] Stats  │  [4] Settings            │
├──────────────────┴──────────────────────────────────────────────────────────────────┤
│                                                                                     │
│                                  Active Tab Content                                 │
│                                (Switched dynamically)                               │
│                                                                                     │
├─────────────────────────────────────────────────────────────────────────────────────┤
│  [Tab] Switch Tab   [?] Help Modal   [q] Quit  │ Theme: Catppuccin Mocha            │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

1. **Header Bar (Top, 3 rows)**: Shows the application branding logo and interactive tab selectors (`[1] Timer`, `[2] Tasks`, `[3] Stats`, `[4] Settings`).
2. **Main Content Area (Middle, Dynamic)**: Renders the active tab's dedicated view.
3. **Footer Status Bar (Bottom, 3 rows)**: Displays global shortcuts, the active color theme, and transient notification messages (such as session completions or task additions).

---

## 2. Step-by-Step User Workflows

### Scenario 1: Starting Your First Pomodoro Focus Session

1. **Launch the Application**: Open your terminal emulator and run:
   ```bash
   cargo run --release
   ```
2. **View the Timer**: By default, Termodoro opens on Tab 1 (**Timer**). The header indicates `🍅 FOCUS SESSION [■ READY] Cycle 1/4 [◉ ○ ○ ○]`.
3. **Begin Focus Interval**: Press `Space`. The status changes to `[● RUNNING]`, and the 5-row digital clock begins counting down from `25:00`. The progress gauge fills smoothly as time elapses.
4. **Pausing When Interrupted**: If you need to step away, press `Space` to pause the countdown (`[❚❚ PAUSED]`). Press `Space` again to resume.
5. **Phase Completion**: When the timer reaches `00:00`:
   - A native desktop notification pops up.
   - An audio bell alert sounds.
   - The cycle counter advances.
   - The phase transitions to `☕ SHORT BREAK [■ READY] (5 mins)`.
   - The completed session is automatically recorded in your statistics.

---

### Scenario 2: Creating, Estimating, and Tracking Tasks

1. **Open Tasks View**: Press `2` or `Tab` to navigate to the **Tasks** tab.
2. **Create a Task**: Press `a` to open the task creation dialog:
   - Type your task description (e.g., `Write unit tests`).
   - Press `Tab` to move to the estimated Pomodoros field.
   - Use `←` / `→` or `+` / `-` to set an estimated session count (e.g., `3 pomodoros`).
   - Press `Enter` to save.
3. **Set Active Focus Target**: Highlight the newly created task using `↑` / `k` or `↓` / `j` and press `t`. A `🎯 ACTIVE` badge appears next to the item.
4. **Work on the Task**: Return to Tab 1 (`1` or `Tab`). Notice the active task card now displays `Active Focus: Write unit tests (🍅 0/3)`.
5. **Completing Sessions**: Each time a 25-minute work session finishes, the task's completed Pomodoro count increments automatically (`🍅 1/3`, `🍅 2/3`, etc.).
6. **Marking Done**: Once finished, return to Tasks (`2`), highlight the task, and press `Space` or `Enter` to mark it complete (`✔`).

---

### Scenario 3: Navigating and Interpreting Productivity Analytics

1. **Open Analytics Dashboard**: Press `3` to view the **Stats** tab.
2. **Interpret Summary Cards**:
   - **Today's Focus**: Total Pomodoro work sessions completed today and total minutes spent in focus.
   - **Current Streak**: Number of consecutive calendar days with logged work sessions.
   - **All-Time Focus**: Cumulative count of all sessions and total focus hours across all time.
3. **Analyze Weekly Activity**: The 7-day bar chart shows your daily productivity volume, helping you identify peak productive days.
4. **Review Session History**: The bottom table lists your recent sessions chronologically with timestamps, phase types, durations, and associated task titles.

---

### Scenario 4: Customizing Preferences and Color Themes

1. **Open Settings View**: Press `4` to enter the **Settings** tab.
2. **Customize Durations**:
   - Highlight **Focus Duration** and press `→` or `+` to increase focus sessions (e.g., to 30 or 50 minutes).
   - Highlight **Short Break** or **Long Break** to tailor your rest intervals.
   - Highlight **Long Break Interval** to adjust how many work sessions occur before an extended break.
3. **Toggle Automation**:
   - Highlight **Auto-start Breaks** and press `Space` to enable automatic break starts.
   - Highlight **Desktop Notifications** or **Sound / Bell Alert** to customize alerts.
4. **Change Color Themes**:
   - Highlight **Color Theme** and press `←` or `→` to cycle between:
     - *Catppuccin Mocha*
     - *Nord*
     - *Gruvbox Dark*
     - *Tokyo Night*
     - *Dracula*
     - *Solarized Dark*
5. **Instant Persistence**: All changes are automatically saved to your storage file immediately.

---

## 3. Codebase Architecture & Tour for Beginners

If you are new to Rust or TUI development, here is how the code flows:

1. **`src/main.rs`**: The program entry point. Initializes terminal raw mode, sets up the crash recovery panic hook, creates the `App` instance, and runs the 250ms tick event loop.
2. **`src/app.rs`**: The central state container. Holds the timer, task manager, stats history, and active settings. Dispatches keystrokes to the active tab.
3. **`src/timer.rs`**: The Pomodoro state machine. Tracks seconds remaining, handles state changes between Work and Breaks, and computes progress ratios.
4. **`src/tasks.rs`**: Manages the task vector, active target ID, completion toggles, and filtered views.
5. **`src/stats.rs`**: Records session history and runs the consecutive calendar day streak algorithm.
6. **`src/theme.rs`**: Defines all color schemes using 24-bit RGB values.
7. **`src/storage.rs`**: Handles saving and loading JSON state to the user's standard XDG data directory.
8. **`src/ui/`**: Contains pure rendering functions that turn state into visual widgets on screen on every frame.

---

## 4. Verification, Testing & Quality Assurance

### Running the Test Suite
Termodoro includes a suite of automated unit tests:

```bash
cargo test
```

### Test Coverage Summary

| Test Function | Location | Purpose |
| :--- | :--- | :--- |
| `test_timer_initialization` | `src/timer.rs` | Verifies default duration values and initial Stopped state |
| `test_phase_advancement` | `src/timer.rs` | Verifies cycle progression from Work through Short and Long breaks |
| `test_progress_ratio` | `src/timer.rs` | Validates percentage calculation precision |
| `test_pause_and_reset` | `src/timer.rs` | Validates toggle, pause, and reset state transitions |
| `test_task_lifecycle` | `src/tasks.rs` | Tests adding tasks, effort increments, and completion status |
| `test_task_filtering` | `src/tasks.rs` | Tests filtering tasks by All, Active, and Completed |
| `test_stats_recording` | `src/stats.rs` | Tests session recording, daily aggregation, and streak calculation |

---

## 5. Operational Glossary

- **Active Focus Target**: The specific task currently bound to the timer. Effort is automatically attributed to this task when a work session ends.
- **Cycle**: A sequence of Pomodoro work intervals and short breaks leading up to a long break (typically 4 work sessions).
- **Daily Streak**: The number of consecutive calendar days where at least one Pomodoro focus session was successfully completed.
- **Focus Interval**: A continuous, uninterrupted period dedicated to working on a single task (default: 25 minutes).
- **Long Break**: An extended recovery rest period (default: 15 minutes) taken after completing a full cycle of focus sessions.
- **Short Break**: A brief rest period (default: 5 minutes) taken between individual focus sessions.
- **Timeboxing**: Allocating a fixed, pre-planned duration to a specific activity.
- **Unit Test**: An automated software test verifying that a specific unit of code behaves correctly.

---

## 6. References and Citations

1. **Cirillo, Francesco (2006)**. *The Pomodoro Technique*. FC Garage GmbH. [https://francescocirillo.com/products/the-pomodoro-technique](https://francescocirillo.com/products/the-pomodoro-technique)
2. **Ratatui Documentation & Guide**. *Official Ratatui Book*. [https://ratatui.rs/](https://ratatui.rs/)
3. **Crossterm Documentation**. *Crossterm Crates.io Reference*. [https://docs.rs/crossterm/](https://docs.rs/crossterm/)
4. **The Rust Reference**. *Behavior of Panic Hooks and Raw Mode*. [https://doc.rust-lang.org/reference/](https://doc.rust-lang.org/reference/)
5. **Freedesktop.org**. *XDG Base Directory Specification*. [https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
