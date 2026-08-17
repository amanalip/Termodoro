# Termodoro: Terminal Pomodoro & Task Management System

Termodoro is a keyboard-driven, lightweight productivity suite built specifically for the terminal. It combines the Pomodoro time-management methodology with interactive task tracking, productivity statistics, daily streak tracking, and customizable color themes, written in Rust using Ratatui and Crossterm.

---

## Table of Contents

1. [Introduction to the Pomodoro Technique](#1-introduction-to-the-pomodoro-technique)
2. [Why Use a Terminal-Based Timer?](#2-why-use-a-terminal-based-timer)
3. [Key Features Overview](#3-key-features-overview)
4. [System Architecture](#4-system-architecture)
5. [Installation & Beginner Setup Guide](#5-installation--beginner-setup-guide)
   - [Prerequisites](#prerequisites)
   - [Installing Rust and Cargo](#installing-rust-and-cargo)
   - [Cloning and Building the Repository](#cloning-and-building-the-repository)
   - [Creating a Convenient Terminal Shortcut (Alias)](#creating-a-convenient-terminal-shortcut-alias)
6. [User Interface and Navigation Guide](#6-user-interface-and-navigation-guide)
   - [Global Navigation Controls](#global-navigation-controls)
   - [Tab 1: Pomodoro Countdown Timer](#tab-1-pomodoro-countdown-timer)
   - [Tab 2: Interactive Task Manager](#tab-2-interactive-task-manager)
   - [Tab 3: Productivity Analytics & Streak Tracking](#tab-3-productivity-analytics--streak-tracking)
   - [Tab 4: Preferences, Durations & Theme Selector](#tab-4-preferences-durations--theme-selector)
7. [Configuration Schema and Data Storage](#7-configuration-schema-and-data-storage)
8. [Troubleshooting & Frequently Asked Questions (FAQ)](#8-troubleshooting--frequently-asked-questions-faq)
9. [Development, Testing & Contribution](#9-development-testing--contribution)
10. [Glossary of Terms](#10-glossary-of-terms)
11. [References and Further Reading](#11-references-and-further-reading)
12. [License](#12-license)

---

## 1. Introduction to the Pomodoro Technique

The Pomodoro Technique is a time-management methodology created by Francesco Cirillo in the late 1980s. The core principle involves breaking work down into focused, uninterrupted intervals (traditionally 25 minutes long), separated by brief rest periods (traditionally 5 minutes long). After completing four consecutive focus intervals, a longer restorative break (traditionally 15 to 30 minutes) is taken.

### Key Benefits
- **Mitigates Mental Fatigue**: Regular structured breaks allow the brain to assimilate information and recover focus.
- **Reduces Procrastination**: Committing to a single 25-minute block lowers psychological resistance compared to tackling large open-ended tasks.
- **Promotes Monotasking**: Encourages single-minded attention on one objective at a time, discouraging context switching.
- **Improves Effort Estimation**: By tracking how many Pomodoro intervals a task actually requires versus its estimate, your future planning accuracy improves over time.

---

## 2. Why Use a Terminal-Based Timer?

Modern web-based or electron-based timer applications frequently introduce significant drawbacks for developers and command-line users:
- **Distraction Vectors**: Browser tabs and web applications place social media, notifications, and unrelated tabs one click away.
- **High Resource Overhead**: Browser-based applications often consume hundreds of megabytes of RAM and induce unnecessary CPU overhead.
- **Context Switching**: Switching away from your code editor or terminal to check a timer breaks focus.

Termodoro runs natively in your terminal. It uses negligible memory (less than 15 MB of RAM), starts instantly, works completely offline, and integrates seamlessly into terminal multiplexers such as tmux, Zellij, and terminal window splits.

---

## 3. Key Features Overview

- **Customizable Interval Engine**: Configure focus sessions, short breaks, and long breaks to match your personal working rhythm (such as 50/10 ultradian rhythms or classic 25/5 intervals).
- **Large Digital Clock**: High-visibility 5-row ASCII block font displaying the remaining time clearly from across the room.
- **Smooth Visual Gauge Bar**: Real-time progress bar rendering elapsed percentage for the active interval.
- **Full Task Lifecycle Management**: Create tasks, set estimated Pomodoro counts, mark items complete, and bind a target task to log effort automatically.
- **Analytics Dashboard & Streak Tracker**: Daily focus summaries, consecutive active calendar day streaks, 7-day visual bar charts, and a historical session log.
- **Six Built-in Color Themes**: Modern palettes including Catppuccin Mocha, Nord, Gruvbox Dark, Tokyo Night, Dracula, and Solarized Dark.
- **Native Desktop & Sound Notifications**: Cross-platform desktop popups using OS notification services paired with an audio bell alert.
- **Automatic State Persistence**: Automatically saves your tasks, preferences, and session history according to standard XDG data directory guidelines.

---

## 4. System Architecture

Termodoro is organized as a modular Rust application adhering to strict separation of concerns between state management, logic engines, and immediate-mode user interface rendering:

```
Termodoro/
├── Cargo.toml             # Package manifest, dependencies, and build profiles
├── .gitignore             # Excluded build artifacts and temporary files
├── README.md              # Main user manual and project overview
├── IMPLEMENTATION.md      # In-depth engineering specification and algorithms
├── WALKTHROUGH.md         # Operational workflows, code tour, and test benchmarks
└── src/
    ├── main.rs            # Terminal runtime initialization, event loop, panic hook
    ├── app.rs             # Central application state, keyboard event dispatcher
    ├── timer.rs           # Pomodoro finite state machine, tick calculation logic
    ├── tasks.rs           # Task model, UUID assignment, filter predicates
    ├── stats.rs           # Data aggregation, streak calculation algorithms
    ├── config.rs          # User preference schema and default parameters
    ├── theme.rs           # Theme choices and concrete RGB color palettes
    ├── storage.rs         # File I/O, XDG directory resolution, JSON persistence
    └── ui/
        ├── mod.rs         # Root view layout coordinator, tabs, header and footer
        ├── digits.rs      # 5x3 block font character rasterization for digital clock
        ├── timer_view.rs  # Main timer screen (digits, gauge, cycle dots, target card)
        ├── tasks_view.rs  # Interactive task table, status checkboxes, filter selector
        ├── stats_view.rs  # Metric cards, weekly bar chart, recent activity log
        ├── settings_view.rs # Live configuration editor and theme switcher
        ├── task_modal.rs  # Task creation modal dialog
        └── help_popup.rs  # Global keybinding modal overlay
```

---

## 5. Installation & Beginner Setup Guide

### Prerequisites
To build and run Termodoro, you will need:
1. **Rust and Cargo**: Version 1.74.0 or newer.
2. **A Modern Terminal Emulator**: Supporting TrueColor (24-bit RGB) and UTF-8 encoding (e.g., Alacritty, Kitty, WezTerm, iTerm2, Windows Terminal, GNOME Terminal).

### Installing Rust and Cargo

If you do not have Rust installed on your computer, you can install it using `rustup`, the official Rust toolchain installer:

- **Linux and macOS**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Follow the on-screen prompts (option 1 is recommended). Afterward, reload your shell:
  ```bash
  source "$HOME/.cargo/env"
  ```

- **Windows**:
  Download and run `rustup-init.exe` from the official website ([https://rustup.rs](https://rustup.rs)).

Verify your installation by running:
```bash
rustc --version
cargo --version
```

### Cloning and Building the Repository

1. Clone the Termodoro repository from GitHub to your local machine:
   ```bash
   git clone https://github.com/amanalip/Termodoro.git
   cd Termodoro
   ```

2. Build and run the project in release mode:
   ```bash
   cargo run --release
   ```

3. If you want to install the binary globally into your Cargo bin path (`~/.cargo/bin`):
   ```bash
   cargo install --path .
   ```
   Once installed, you can launch Termodoro from any directory simply by typing:
   ```bash
   termodoro
   ```

### Creating a Convenient Terminal Shortcut (Alias)

You can create an alias in your shell configuration file (`~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish`):

```bash
# Add this line to your ~/.bashrc or ~/.zshrc
alias pomo="termodoro"
```

Then reload your shell (`source ~/.bashrc` or `source ~/.zshrc`) and run `pomo` to start.

---

## 6. User Interface and Navigation Guide

### Global Navigation Controls

These shortcuts function across every screen in Termodoro:

| Keybinding | Action | Description |
| :--- | :--- | :--- |
| `Tab` | Next Tab | Move to the next tab to the right |
| `Shift+Tab` | Previous Tab | Move to the previous tab to the left |
| `1` | Switch to Timer | Jump directly to Tab 1 (Timer View) |
| `2` | Switch to Tasks | Jump directly to Tab 2 (Tasks View) |
| `3` | Switch to Stats | Jump directly to Tab 3 (Analytics View) |
| `4` | Switch to Settings | Jump directly to Tab 4 (Settings View) |
| `?` | Help Modal | Open or close the interactive keybinding reference dialog |
| `q` or `Esc` | Quit / Dismiss | Close any open modal overlay, or exit the application |

---

### Tab 1: Pomodoro Countdown Timer

The Timer view is the central hub for running focus sessions and breaks.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🍅 FOCUS SESSION   [● RUNNING]   Cycle 1/4 [◉ ○ ○ ○]                       │
│                                                                             │
│                  ████  ████    :   ████  ████                               │
│                     █  █       :   █  █  █  █                               │
│                  ████  ████        █  █  █  █                               │
│                     █     █    :   █  █  █  █                               │
│                  ████  ████    :   ████  ████                               │
│                                                                             │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ █████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 48%       │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│ ┌─ 🎯 Current Target ─────────────────────────────────────────────────────┐ │
│ │   Active Focus: Refactor storage module (🍅 2/4)                        │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│   Today's Sessions: 4 pomodoros (100 mins)   │   Streak: 🔥 3 days          │
│                                                                             │
│   [Space] Pause   [r] Reset   [s] Skip Phase   [a] Quick Add Task           │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Controls in Timer View
- `Space`: Toggle timer state between Running and Paused.
- `r`: Reset timer back to its initial full duration for the current phase.
- `s`: Skip the remaining time in the active phase and advance to the next interval.
- `a`: Open the task creation dialog to quickly add a new item without leaving the timer.

---

### Tab 2: Interactive Task Manager

The Tasks view provides a structured environment for managing tasks, estimating workload, and tracking completed Pomodoro sessions against specific goals.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Filter: [1] All (3)   [2] Active (2)   [3] Completed (1)                    │
├─────────────────────────────────────────────────────────────────────────────┤
│   Status   Task Title                                Est. Pomodoros Active  │
│ ▶  ○       Implement data export feature             🍅 1 / 3       🎯 ACTIVE│
│    ○       Write comprehensive documentation         🍅 0 / 2               │
│    ✔       Fix terminal raw mode cleanup             🍅 2 / 2               │
├─────────────────────────────────────────────────────────────────────────────┤
│ [a] Add Task  [Space] Toggle Done  [t] Set Target  [d] Delete  [↑/↓] Select │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Controls in Tasks View
- `a`: Open the **Add Task** dialog.
- `Space` or `Enter`: Toggle completion status on the selected task.
- `t`: Mark the selected task as the active focus target. As work sessions finish, Pomodoros are automatically credited to this item.
- `d` or `x`: Delete the selected task.
- `↑` / `k` and `↓` / `j`: Move selection up or down.
- `1`, `2`, `3`: Filter task list by `All`, `Active`, or `Completed`.

---

### Tab 3: Productivity Analytics & Streak Tracking

The Stats view visualizes your focus history and habit consistency:

```
┌────────────────────────┬────────────────────────┬───────────────────────────┐
│ 📅 Today's Focus       │ ⚡ Current Streak      │ 🏆 All-Time Focus         │
│ 4 🍅                   │ 🔥 3 Days              │ 28 Sessions               │
│ 100 minutes focused    │ Personal Best: 7 Days  │ 11.7 Total Focus Hours    │
├────────────────────────┴────────────────────────┴───────────────────────────┤
│ 📊 Daily Activity (Past 7 Days)                                             │
│                                                                             │
│    █        █        █        █        █        █        █                  │
│   Mon      Tue      Wed      Thu      Fri      Sat      Sun                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ 🕒 Recent Completed Sessions                                                │
│ Time              Phase              Duration    Task Assigned              │
│ 2026-08-17 15:30  🍅 FOCUS SESSION   25 mins     Implement data export      │
│ 2026-08-17 15:00  ☕ SHORT BREAK      5 mins      -                          │
│ 2026-08-17 14:30  🍅 FOCUS SESSION   25 mins     Implement data export      │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Key Metrics Explained
- **Today's Focus**: Total work sessions completed on the current local calendar day and cumulative focus minutes.
- **Current Streak**: Number of consecutive calendar days with at least one completed work session. If you completed work yesterday or today, your streak remains active.
- **Personal Best**: The highest consecutive daily streak recorded in your history.
- **7-Day Bar Chart**: Visual bar chart illustrating daily session volume over the past week.

---

### Tab 4: Preferences, Durations & Theme Selector

The Settings view allows you to customize durations, toggles, and visual appearance in real time.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│   Setting                 Current Value      Description                    │
│ ▶ Focus Duration          25 mins            Length of a standard work session│
│   Short Break             5 mins             Duration of short breaks       │
│   Long Break              15 mins            Duration of long breaks        │
│   Long Break Interval     4 sessions         Work sessions before long break│
│   Auto-start Breaks       Disabled           Automatically start break timer│
│   Auto-start Work         Disabled           Automatically start work timer │
│   Desktop Notifications   Enabled            Send native OS notification    │
│   Sound / Bell Alert      Enabled            Ring terminal audio bell       │
│   Color Theme             Catppuccin Mocha   Active TUI color scheme        │
├─────────────────────────────────────────────────────────────────────────────┤
│ [↑/↓] Select Setting   [← / →] or [+/-] Adjust Value / Theme   [Space] Toggle │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Controls in Settings View
- `↑` / `k` and `↓` / `j`: Select configuration option.
- `←` / `h` and `→` / `l` or `+` / `-`: Increase or decrease numerical values, or cycle through color themes.
- `Space` or `Enter`: Toggle boolean flags (Enabled / Disabled).

---

## 7. Configuration Schema and Data Storage

Termodoro uses standard XDG base directory specifications for local data persistence:

- **Linux / BSD**: `~/.local/share/termodoro/data.json`
- **macOS**: `~/Library/Application Support/com.termodoro.termodoro/data.json`
- **Windows**: `C:\Users\<User>\AppData\Roaming\termodoro\termodoro\data.json`

### Annotated `data.json` Example

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
    "tasks": [
      {
        "id": "e2c3664d-5282-4217-91a1-9a742ea3b2f1",
        "title": "Refactor storage module",
        "completed": false,
        "pomodoros_spent": 2,
        "pomodoros_estimated": 4,
        "created_at": "2026-08-17T18:30:00Z"
      }
    ],
    "active_task_id": "e2c3664d-5282-4217-91a1-9a742ea3b2f1"
  },
  "stats": {
    "sessions": [
      {
        "timestamp": "2026-08-17T19:00:00Z",
        "phase": "Work",
        "duration_mins": 25,
        "task_id": "e2c3664d-5282-4217-91a1-9a742ea3b2f1",
        "task_title": "Refactor storage module"
      }
    ]
  }
}
```

---

## 8. Troubleshooting & Frequently Asked Questions (FAQ)

### 1. The colors look washed out or inaccurate.
- **Cause**: Your terminal emulator might be running in 16-color or 256-color mode rather than 24-bit TrueColor mode.
- **Solution**: Set the environment variable `COLORTERM=truecolor` in your shell configuration (`export COLORTERM=truecolor`). Most modern terminals (Alacritty, Kitty, WezTerm, iTerm2) enable this by default.

### 2. The audio bell does not make any sound.
- **Cause**: Terminal bell notifications might be muted in your terminal emulator or desktop environment settings.
- **Solution**: Check your terminal preferences (e.g., Alacritty `bell` configuration or GNOME Terminal sound settings) to ensure audio alerts are enabled.

### 3. Desktop notifications are not appearing on Linux.
- **Cause**: A desktop notification daemon (such as `dunst`, `mako`, `swaync`, or `xfce4-notifyd`) may not be running.
- **Solution**: Install and start a notification daemon compatible with your desktop environment or window manager.

### 4. My terminal output became garbled after closing abnormally.
- **Cause**: If the process was terminated forcefully with `kill -9`, the terminal raw mode might not have been reset.
- **Solution**: Type `reset` in your terminal and press `Enter` to restore normal terminal state.

---

## 9. Development, Testing & Contribution

### Running Automated Unit Tests
```bash
cargo test
```

### Static Analysis and Code Formatting
```bash
# Check formatting
cargo fmt -- --check

# Run compiler linter
cargo clippy -- -D warnings
```

---

## 10. Glossary of Terms

- **ANSI Escape Codes**: In-band signaling sequences used to control formatting, color, and cursor position in terminal emulators.
- **Crossterm**: A cross-platform Rust library providing low-level terminal manipulation, event polling, and screen buffer controls.
- **Event Loop**: A programming construct that continuously waits for and dispatches events or messages in a program.
- **Finite State Machine (FSM)**: A mathematical model of computation where an entity transitions between a finite set of distinct states in response to external inputs.
- **Immediate Mode GUI / TUI**: A design pattern where graphical elements are constructed and rendered afresh on every frame rather than retaining long-lived stateful widget trees.
- **Pomodoro Technique**: A time-management method based on fixed-length focus intervals alternating with short and long breaks.
- **Ratatui**: A Rust library for building terminal user interfaces, evolved as a community fork of `tui-rs`.
- **Raw Mode**: A terminal state where input characters are passed directly to the application without line buffering or local echo.
- **Timeboxing**: A productivity strategy allocating a fixed, predetermined time period to a specific activity.
- **TrueColor (24-bit RGB)**: Digital color representation using 8 bits per channel (Red, Green, Blue) allowing over 16.7 million distinct colors.
- **XDG Base Directory Specification**: A standard defining base directory locations for user data, configuration, and cache files on Unix-like operating systems.

---

## 11. References and Further Reading

1. **Cirillo, Francesco (2006)**. *The Pomodoro Technique*. FC Garage GmbH. [https://francescocirillo.com/products/the-pomodoro-technique](https://francescocirillo.com/products/the-pomodoro-technique)
2. **Ratatui Documentation & Guide**. *Official Ratatui Book*. [https://ratatui.rs/](https://ratatui.rs/)
3. **Crossterm Documentation**. *Crossterm Crates.io Reference*. [https://docs.rs/crossterm/](https://docs.rs/crossterm/)
4. **Klabnik, Steve & Nichols, Carol (2023)**. *The Rust Programming Language*. No Starch Press. [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
5. **Freedesktop.org (2010)**. *XDG Base Directory Specification*. [https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
6. **Catppuccin Organization**. *Catppuccin Palette Specifications*. [https://github.com/catppuccin/catppuccin](https://github.com/catppuccin/catppuccin)
7. **Nord Theme Project**. *An arctic, north-bluish clean and elegant color palette*. [https://www.nordtheme.com/](https://www.nordtheme.com/)

---

## 12. License

This project is licensed under the terms of the GNU General Public License v3.0 ([GPL-3.0](LICENSE)).
