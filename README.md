# Termodoro

A keyboard-driven, customizable Pomodoro timer and task management system built specifically for the terminal.

Termodoro combines the Pomodoro time-management methodology with task tracking, daily productivity statistics, streak monitoring, and customizable themes, all within a fast, lightweight terminal user interface (TUI) powered by Rust, Ratatui, and Crossterm.

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [System Architecture](#system-architecture)
- [Installation and Setup](#installation-and-setup)
  - [Prerequisites](#prerequisites)
  - [Building from Source](#building-from-source)
  - [Running the Application](#running-the-application)
- [Usage and Navigation](#usage-and-navigation)
  - [Global Navigation](#global-navigation)
  - [Timer View (Tab 1)](#timer-view-tab-1)
  - [Task Manager (Tab 2)](#task-manager-tab-2)
  - [Analytics & History (Tab 3)](#analytics--history-tab-3)
  - [Preferences & Themes (Tab 4)](#preferences--themes-tab-4)
- [Configuration & Data Storage](#configuration--data-storage)
- [Development and Testing](#development-and-testing)
- [Additional Documentation](#additional-documentation)
- [License](#license)

---

## Overview

Traditional desktop Pomodoro timers often introduce unnecessary distraction through web interfaces or heavy browser wrappers. Termodoro delivers a focused environment directly inside your terminal emulator. It tracks work sessions, manages task estimation and completion, analyzes long-term habits, and issues native desktop and audio notifications while maintaining minimal system resource usage.

---

## Key Features

### 1. Robust Pomodoro Engine
- **Standard Interval Support**: Configurable focus sessions (default 25 min), short breaks (default 5 min), and long breaks (default 15 min).
- **Cycle Tracking**: Automated tracking of Pomodoro cycles with custom intervals before triggering a long break.
- **Large Digital Clock**: 5-row ASCII block font displaying remaining time clearly across various terminal dimensions.
- **Visual Progress Gauge**: Real-time progress bar reflecting the percentage of completion for the active phase.
- **Flexible Automation**: Optional auto-start settings for breaks and work sessions.

### 2. Task & Todo Management
- **Task Association**: Link active Pomodoro sessions directly to tasks to log actual vs. estimated effort.
- **State Filtering**: View tasks by `All`, `Active`, or `Completed` status.
- **Keyboard-driven CRUD**: Fast inline task creation, completion toggling, deletion, and active target switching.

### 3. Productivity Analytics & Streak Tracking
- **Daily Focus Totals**: Real-time aggregation of today's completed Pomodoro count and total minutes spent in focus.
- **Streak Calculation**: Continuous daily streak calculation based on local calendar dates, including tracking of personal best records.
- **Weekly Distribution Chart**: Visual 7-day bar chart showing daily session counts.
- **Historical Session Log**: Reverse-chronological table of recent sessions with phase, duration, and assigned task details.

### 4. Color Themes & Aesthetics
- Six carefully curated terminal palettes:
  - **Catppuccin Mocha** (Default)
  - **Nord**
  - **Gruvbox Dark**
  - **Tokyo Night**
  - **Dracula**
  - **Solarized Dark**
- Full RGB color rendering tailored for standard terminal contrast and readability.

### 5. Cross-Platform Desktop & Audio Notifications
- Asynchronous native desktop notifications via Freedesktop/D-Bus (Linux), macOS Notification Center, or Windows Toast.
- Terminal bell (`\x07`) audio alert fallback.

### 6. Persistence & XDG Compliance
- Automatic state management persisting configurations, tasks, and historical session logs to standard XDG data directories (`~/.local/share/termodoro/data.json`).

---

## System Architecture

Termodoro follows a clean, modular structure with strict separation of concerns:

```
Termodoro/
├── Cargo.toml
├── .gitignore
├── README.md
├── IMPLEMENTATION.md
├── WALKTHROUGH.md
└── src/
    ├── main.rs            # Terminal setup, raw mode lifecycle, panic safety, event loop
    ├── app.rs             # Application state, key dispatch, modal handling, notifications
    ├── timer.rs           # Pomodoro finite state machine, interval calculations, tick engine
    ├── tasks.rs           # Task data model, filtering logic, effort counter, UUID management
    ├── stats.rs           # Session logs, daily aggregation, streak calculation algorithm
    ├── config.rs          # User preferences schema and default values
    ├── theme.rs           # Color palette definitions and RGB style tokens
    ├── storage.rs         # File I/O, XDG path resolution, atomic JSON persistence
    └── ui/
        ├── mod.rs         # Root view layout coordinator and tab routing
        ├── digits.rs      # 5x3 block font character rasterization for digital clock
        ├── timer_view.rs  # Main timer screen (digits, gauge, cycle dots, target card)
        ├── tasks_view.rs  # Interactive task table, status checkboxes, filter selector
        ├── stats_view.rs  # Metric cards, weekly bar chart, recent activity log
        ├── settings_view.rs# Live configuration editor and theme selector
        ├── task_modal.rs  # Task creation modal dialog
        └── help_popup.rs  # Global keybinding modal overlay
```

---

## Installation and Setup

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (Cargo and `rustc` version 1.74.0 or newer).
- A terminal emulator with TrueColor (24-bit RGB) and UTF-8 support (e.g., Alacritty, Kitty, WezTerm, iTerm2, Windows Terminal).

### Building from Source

```bash
# Clone the repository
git clone https://github.com/amanalip/Termodoro.git
cd Termodoro

# Build the release binary
cargo build --release
```

The compiled binary will be placed at `./target/release/termodoro`.

### Running the Application

```bash
# Run directly via Cargo
cargo run --release

# Or execute the compiled binary directly
./target/release/termodoro
```

---

## Usage and Navigation

### Global Navigation

These shortcuts are accessible from any screen within the application:

| Keybinding | Action |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Cycle forward / backward through application tabs |
| `1` | Switch to **Timer** view |
| `2` | Switch to **Tasks** view |
| `3` | Switch to **Stats** view |
| `4` | Switch to **Settings** view |
| `?` | Open / close the interactive **Help & Keybindings** modal |
| `q` / `Esc` | Quit application (or dismiss open modal dialog) |

---

### Timer View (Tab 1)

The Timer view displays the active phase (Work, Short Break, Long Break), current cycle progress, big countdown timer, percentage progress bar, and active task card.

| Keybinding | Action |
| :--- | :--- |
| `Space` | Start or pause the countdown timer |
| `r` | Reset the timer to the start of the current phase |
| `s` | Skip the current phase and advance to the next interval |
| `a` | Open the **Add Task** modal |

---

### Task Manager (Tab 2)

The Tasks view allows you to organize work items, specify estimated Pomodoro counts, mark items complete, and assign the active target for the timer.

| Keybinding | Action |
| :--- | :--- |
| `a` | Create a new task |
| `Space` / `Enter` | Toggle completed status for selected task |
| `t` | Set the selected task as the active focus target |
| `d` / `x` | Delete selected task |
| `↑` / `k` | Move selection up |
| `↓` / `j` | Move selection down |
| `1` | Filter: Show **All** tasks |
| `2` | Filter: Show **Active** tasks only |
| `3` | Filter: Show **Completed** tasks only |

---

### Analytics & History (Tab 3)

The Stats view provides an overview of your productivity habits:
- **Today's Focus**: Total work sessions completed today and cumulative focus minutes.
- **Current & Longest Streak**: Number of consecutive active days and all-time record.
- **All-Time Stats**: Total historical sessions and cumulative focus hours.
- **Daily Activity Bar Chart**: Session counts distributed across the past 7 days.
- **Recent Completed Sessions**: Detailed log of recent work intervals and break periods.

---

### Preferences & Themes (Tab 4)

Modify runtime parameters and visual styles. Changes take effect immediately and are saved to disk.

| Keybinding | Action |
| :--- | :--- |
| `↑` / `k` | Navigate to previous setting |
| `↓` / `j` | Navigate to next setting |
| `←` / `h`, `→` / `l` | Adjust duration values / cycle through themes |
| `+`, `-` | Increment / decrement selected numerical value |
| `Space` / `Enter` | Toggle boolean feature flags |

#### Available Settings
1. **Focus Duration**: Length of work sessions (1 - 120 minutes).
2. **Short Break**: Duration of standard breaks (1 - 60 minutes).
3. **Long Break**: Duration of extended recovery breaks (1 - 90 minutes).
4. **Long Break Interval**: Number of focus sessions before a long break (1 - 12 sessions).
5. **Auto-start Breaks**: Automatically begin break countdown when a work session ends.
6. **Auto-start Work**: Automatically begin work countdown when a break ends.
7. **Desktop Notifications**: Enable/disable OS-level notification popups.
8. **Sound / Bell Alert**: Enable/disable audio bell (`\x07`) cues.
9. **Color Theme**: Select active color palette.

---

## Configuration & Data Storage

Termodoro uses standard XDG base directory specifications for local data persistence:

- **Linux / BSD**: `~/.local/share/termodoro/data.json`
- **macOS**: `~/Library/Application Support/com.termodoro.termodoro/data.json`
- **Windows**: `C:\Users\<User>\AppData\Roaming\termodoro\termodoro\data.json`

The storage file contains user preferences, task collections, and historical timestamps in standard JSON format:

```json
{
  "config": {
    "work_duration_mins": 25,
    "short_break_mins": 5,
    "long_break_mins": 15,
    "long_break_interval": 4,
    "auto_start_breaks": false,
    "auto_start_work": false,
    "sound_enabled": true,
    "desktop_notifications": true,
    "theme": "CatppuccinMocha"
  },
  "tasks": {
    "tasks": [],
    "active_task_id": null
  },
  "stats": {
    "sessions": []
  }
}
```

---

## Development and Testing

### Running Unit Tests

Termodoro includes automated unit test suites covering the Pomodoro state machine, task filtering, statistics aggregation, and streak algorithms:

```bash
cargo test
```

### Code Formatting and Lints

```bash
# Check code formatting
cargo fmt -- --check

# Run clippy static analysis
cargo clippy -- -D warnings
```

---

## Additional Documentation

For technical implementation specifics and detailed architectural documentation, refer to:
- [`IMPLEMENTATION.md`](IMPLEMENTATION.md): In-depth software design, state machines, algorithms, and module breakdown.
- [`WALKTHROUGH.md`](WALKTHROUGH.md): Step-by-step feature walkthrough, testing results, and usage guide.

---

## License

This project is licensed under the terms of the GNU General Public License v3.0 ([GPL-3.0](LICENSE)).
