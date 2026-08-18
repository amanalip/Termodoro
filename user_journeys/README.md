# Termodoro User Journeys & Visual Walkthroughs

Welcome to the **Termodoro User Journeys** documentation suite. This directory contains detailed, step-by-step interactive walkthroughs with embedded terminal interface screenshots, workflow diagrams, under-the-hood engineering callouts, and keybinding cheat sheets designed to help developers, students, and knowledge workers master every feature of Termodoro.

---

## Table of Contents

1. [Directory Purpose & Overview](#1-directory-purpose--overview)
2. [User Journey Catalog](#2-user-journey-catalog)
   - [01. Core Pomodoro Focus Session & Interval Cycling](#01-core-pomodoro-focus-session--interval-cycling)
   - [02. Task Creation, Estimation & Target Tracking](#02-task-creation-estimation--target-tracking)
   - [03. Analytics, Daily Streaks & Productivity Insights](#03-analytics-daily-streaks--productivity-insights)
   - [04. Customizing Durations, Acoustic Audio & 18 Color Themes](#04-customizing-durations-acoustic-audio--18-color-themes)
3. [Design Philosophy & Grounded Architecture](#3-design-philosophy--grounded-architecture)
4. [Master Keybinding Quick Reference](#4-master-keybinding-quick-reference)

---

## 1. Directory Purpose & Overview

The goal of this directory is to bridge technical implementation with practical, real-world productivity workflows. Each document in this folder:
- Demonstrates how to solve common focus and planning challenges using Termodoro.
- Provides visual UI references captured directly from the live application layout.
- Links user actions directly to the underlying Rust codebase modules and algorithms.
- Formally documents keyboard interactions, state transitions, and persistence invariants.

```
user_journeys/
├── README.md                           # Directory guide & workflow catalog (this document)
├── 01_focus_session_and_cycling.md     # Focus countdown, pauses, cycle dots & acoustic chimes
├── 02_task_management_and_estimates.md # Task creation modal, Pomodoro estimates & target binding
├── 03_analytics_and_streaks.md         # Daily focus metric cards, 7-day bar chart & streaks
└── 04_preferences_and_themes.md        # Live settings editor, audio/toasts & 18 color themes
```

---

## 2. User Journey Catalog

### 01. Core Pomodoro Focus Session & Interval Cycling
- **Guide**: [**`01_focus_session_and_cycling.md`**](01_focus_session_and_cycling.md)
- **Primary Tab**: `Tab 1: Timer View`
- **Core Workflows**:
  - Launching the app with $<10\text{ms}$ startup time.
  - Starting, pausing, resuming, and resetting countdown intervals via `Space`, `r`, and `s`.
  - Understanding the 5-row rasterized ASCII block clock (`src/ui/digits.rs`).
  - Progressing across the 24-cycle progress dot matrix (`● ● ○ ○`).
  - Experiencing mathematical audio feedback: 528 Hz Zen Singing Bowl, two-tone alert ($D_5 \rightarrow A_5$), and major triad chord ($C_5 \rightarrow E_5 \rightarrow G_5$).

---

### 02. Task Creation, Estimation & Target Tracking
- **Guide**: [**`02_task_management_and_estimates.md`**](02_task_management_and_estimates.md)
- **Primary Tab**: `Tab 2: Tasks View`
- **Core Workflows**:
  - Opening the dual-input task creation modal with `a`.
  - Estimating effort in discrete 25-minute Pomodoro blocks ($1 \le N \le 20$).
  - Binding the active target task to the countdown timer with `t`.
  - Automatic effort accounting upon interval completion (`🍅 1 / 3` $\rightarrow$ `🍅 2 / 3`).
  - Filtering between All (`1`), Active (`2`), and Completed (`3`) tasks.
  - Reassigning targets and deleting obsolete tasks with `d` / `x`.

---

### 03. Analytics, Daily Streaks & Productivity Insights
- **Guide**: [**`03_analytics_and_streaks.md`**](03_analytics_and_streaks.md)
- **Primary Tab**: `Tab 3: Stats View`
- **Core Workflows**:
  - Inspecting high-level focus metrics: Total Focus Time, Total Pomodoros, Current Streak, and Longest Streak.
  - Reading the 7-day activity histogram with proportional Unicode vertical bars (`█`).
  - Understanding consecutive daily habit streak retention across calendar dates, month boundaries, and leap years.
  - Auditing historical session activity logs with timestamped task attribution.

---

### 04. Customizing Durations, Acoustic Audio & 18 Color Themes
- **Guide**: [**`04_preferences_and_themes.md`**](04_preferences_and_themes.md)
- **Primary Tab**: `Tab 4: Settings View`
- **Core Workflows**:
  - Modifying focus durations ($1 \le \text{Work} \le 120\text{m}$, $1 \le \text{Short Break} \le 60\text{m}$, $1 \le \text{Long Break} \le 90\text{m}$).
  - Scaling the Long Break interval ceiling up to 24 cycles.
  - Enabling automated flow with Auto-Start Breaks and Auto-Start Work.
  - Toggling in-memory PCM audio chimes and native OS desktop toast notifications.
  - Cycling through all 18 handcrafted dark and light TrueColor RGB themes with real-time UI repainting.

---

## 3. Design Philosophy & Grounded Architecture

All user journeys in this directory adhere to the strict design invariants of Termodoro:
1. **Zero-Telemetry Privacy**: 100% offline; data stays strictly in local JSON storage (`~/.local/share/termodoro/data.json`).
2. **Crash-Resilient State**: Immediate atomic `.tmp` file replacement on every setting change or task mutation.
3. **Pure In-Memory Audio**: Mathematical sound synthesis without external audio files or third-party media players.
4. **Immediate Mode Rendering**: Zero-flicker terminal updates powered by Ratatui and Crossterm.

---

## 4. Master Keybinding Quick Reference

| Keybinding | Global Context | Tab-Specific Action |
| :---: | :--- | :--- |
| `1` - `4` | Switch Tab directly | Jump to Timer (`1`), Tasks (`2`), Stats (`3`), or Settings (`4`) |
| `Tab` / `Shift+Tab` | Cycle Tabs | Move to next or previous tab sequentially |
| `?` | Global | Open / close Keyboard Shortcuts Help modal dialog |
| `q` | Global | Save state atomically and exit application |
| `Space` | Tab 1 / Tab 2 / Tab 4 | Start/Pause timer (Tab 1), Toggle task done (Tab 2), Toggle setting (Tab 4) |
| `r` / `s` | Tab 1 (Timer) | Reset countdown timer (`r`) or Skip to next phase (`s`) |
| `a` | Tab 1 & Tab 2 | Open New Task modal overlay from anywhere |
| `t` | Tab 2 (Tasks) | Set highlighted task as active timer target |
| `d` / `x` | Tab 2 (Tasks) | Delete highlighted task permanently |
| `1` / `2` / `3` | Tab 2 (Tasks) | Filter list by All (`1`), Active (`2`), or Completed (`3`) |
| `j` / `k` (`↓` / `↑`) | Tab 2 & Tab 4 | Navigate task list rows (Tab 2) or settings options (Tab 4) |
| `h` / `l` (`←` / `→`) | Tab 4 (Settings) | Adjust numeric duration values or cycle color themes |
