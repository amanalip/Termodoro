# Termodoro: Detailed Technical Implementation Specification

This document provides a comprehensive, beginner-friendly technical specification of the engineering architecture, data structures, state machines, and algorithms implemented in **Termodoro**.

---

## Table of Contents

1. [Architectural Overview & Terminal Mechanics](#1-architectural-overview--terminal-mechanics)
2. [Lifecycle & Event Loop Architecture](#2-lifecycle--event-loop-architecture)
3. [Pomodoro Finite State Machine (FSM)](#3-pomodoro-finite-state-machine-fsm)
4. [Task Manager & Focus Target Association](#4-task-manager--focus-target-association)
5. [Analytics Engine & Streak Calculation Algorithm](#5-analytics-engine--streak-calculation-algorithm)
6. [Design System, Theme Tokens & Palettes](#6-design-system-theme-tokens--palettes)
7. [Storage Persistence & XDG Directory Resolution](#7-storage-persistence--xdg-directory-resolution)
8. [Block Digit Rasterization Engine](#8-block-digit-rasterization-engine)
9. [Error Handling & Terminal Recovery Hooks](#9-error-handling--terminal-recovery-hooks)
10. [Automated Testing Strategy & Benchmarks](#10-automated-testing-strategy--benchmarks)
11. [Glossary of Technical Terms](#11-glossary-of-technical-terms)
12. [References & Citations](#12-references--citations)

---

## 1. Architectural Overview & Terminal Mechanics

Building a terminal user interface (TUI) in Rust requires controlling how the operating system handles keyboard input and display output:

### Cooked Mode vs. Raw Mode
- **Cooked Mode (Standard)**: In standard terminal mode, the OS buffers keyboard input line-by-line until the user presses `Enter`. Keystrokes are automatically echoed back to the screen.
- **Raw Mode (Termodoro)**: Termodoro enables raw mode using `crossterm::terminal::enable_raw_mode()`. In raw mode, every keypress (such as pressing `Space`, `Tab`, or `j`) is passed immediately to the application without line buffering or local echo.

### Screen Buffers & Alternate Screen
- When Termodoro starts, it executes `EnterAlternateScreen`. This instructs the terminal emulator to switch to a separate memory screen buffer.
- When the user exits the application, Termodoro executes `LeaveAlternateScreen`. This restores the original terminal buffer, leaving your prior shell session completely intact.

---

## 2. Lifecycle & Event Loop Architecture

The application execution loop is implemented in `src/main.rs` and `src/app.rs`:

```
               ┌──────────────────────────┐
               │    Application Start     │
               └────────────┬─────────────┘
                            │ (Enable Raw Mode, Enter Alternate Screen)
                            ▼
               ┌──────────────────────────┐
               │    Load Stored State     │
               │   (Config, Tasks, Stats) │
               └────────────┬─────────────┘
                            │
               ┌────────────▼─────────────┐
        ┌─────►│     Render Frame UI      │
        │      │    (Terminal.draw())     │
        │      └────────────┬─────────────┘
        │                   │
        │      ┌────────────▼─────────────┐
        │      │  Poll Event (250ms max)  │
        │      └──────┬────────────┬──────┘
        │             │            │
        │  [Key Press]│            │[Timeout / Tick]
        │             ▼            ▼
        │      ┌────────────┐┌────────────┐
        │      │ Handle Key ││  on_tick() │
        │      │  Events    ││  (Timer)   │
        │      └──────┬─────┘└─────┬──────┘
        │             │            │
        │             └─────┬──────┘
        │                   │
        │                   ▼
        │      ┌──────────────────────────┐
        └──────┤     Check should_quit    │
               └────────────┬─────────────┘
                            │ [should_quit == true]
                            ▼
               ┌──────────────────────────┐
               │    Save State to Disk    │
               │  (Atomic Storage Write)  │
               └────────────┬─────────────┘
                            │ (Disable Raw Mode, Leave Alternate Screen)
                            ▼
               ┌──────────────────────────┐
               │     Graceful Exit        │
               └──────────────────────────┘
```

### Event Dispatch Mechanism
- The event loop polls for input with a 250ms timeout.
- If a key press occurs within 250ms, `crossterm::event::read()` captures the `KeyEvent` and passes it to `app.on_key_event(key)`.
- If no key is pressed within 250ms, `poll()` times out, allowing `app.on_tick()` to execute periodically and update countdown clocks.

---

## 3. Pomodoro Finite State Machine (FSM)

The timer state is governed by a finite state machine (`src/timer.rs`) with three core interval phases:

```
                      ┌────────────────┐
                      │      Work      │
                      │ (Focus Phase)  │
                      └───────┬────────┘
                              │
               ┌──────────────┴──────────────┐
               │                             │
    [cycle < interval]               [cycle == interval]
               │                             │
               ▼                             ▼
        ┌──────────────┐              ┌──────────────┐
        │ Short Break  │              │  Long Break  │
        │ (Rest Phase) │              │ (Long Rest)  │
        └──────┬───────┘              └──────┬───────┘
               │                             │
               └──────────────┬──────────────┘
                              │
                              ▼
                      ┌────────────────┐
                      │      Work      │
                      │ (Focus Phase)  │
                      └────────────────┘
```

### State Model

```rust
pub enum PomodoroPhase {
    Work,
    ShortBreak,
    LongBreak,
}

pub enum TimerStatus {
    Stopped,
    Running,
    Paused,
}
```

### Transition Logic (`advance_phase`)
1. If the completed phase was `Work`, `completed_pomodoros` is incremented.
2. If `current_cycle >= config.long_break_interval`, the cycle counter resets to 1 and transitions to `LongBreak`. Otherwise, `current_cycle` increments and transitions to `ShortBreak`.
3. If the completed phase was `ShortBreak` or `LongBreak`, the next phase transitions to `Work`.
4. If `auto_start_breaks` or `auto_start_work` is enabled, the new phase starts automatically; otherwise, status resets to `Stopped`.

---

## 4. Task Manager & Focus Target Association

The task management subsystem (`src/tasks.rs`) provides structured workload tracking.

### Task Data Model

```rust
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub pomodoros_spent: u32,
    pub pomodoros_estimated: u32,
    pub created_at: DateTime<Utc>,
}
```

### Active Target Binding
- Users can highlight any task and press `t` to set it as the active target (`active_task_id = Some(task.id)`).
- When a `Work` Pomodoro completes, the application automatically invokes `tasks.increment_active_spent()`, adding 1 to the spent Pomodoro count on the designated task.
- When an active task is marked completed or deleted, the system automatically finds the next available uncompleted task and sets it as the new target.

---

## 5. Analytics Engine & Streak Calculation Algorithm

The analytics engine (`src/stats.rs`) stores an append-only log of completed sessions:

```rust
pub struct CompletedSession {
    pub timestamp: DateTime<Utc>,
    pub phase: PomodoroPhase,
    pub duration_mins: u32,
    pub task_id: Option<String>,
    pub task_title: Option<String>,
}
```

### Consecutive Daily Streak Algorithm

To calculate streaks without timezone anomalies:
1. All sessions with `phase == PomodoroPhase::Work` are converted to the local calendar date (`NaiveDate`).
2. Dates are sorted chronologically and deduplicated (`distinct_work_dates`).
3. If the list is empty, the streak is 0.
4. The algorithm checks the latest work date: if it is neither `today` nor `yesterday`, the streak is considered broken and returns 0.
5. The algorithm steps backwards through the deduplicated date array, verifying that each preceding element matches `current_expected.pred_opt()`. The counter increments until a gap is detected:

```rust
let mut streak = 0;
let mut current_expected = last_date;

for &date in dates.iter().rev() {
    if date == current_expected {
        streak += 1;
        if let Some(prev) = current_expected.pred_opt() {
            current_expected = prev;
        } else {
            break;
        }
    } else if date < current_expected {
        break;
    }
}
```

---

## 6. Design System, Theme Tokens & Palettes

The theme system (`src/theme.rs`) defines a set of semantic color tokens mapped to concrete 24-bit RGB values:

```rust
pub struct Theme {
    pub choice: ThemeChoice,
    pub bg: Color,
    pub fg: Color,
    pub primary: Color,
    pub secondary: Color,
    pub work: Color,
    pub short_break: Color,
    pub long_break: Color,
    pub success: Color,
    pub warning: Color,
    pub border: Color,
    pub border_active: Color,
    pub muted: Color,
    pub highlight: Color,
}
```

### Theme Palettes Summary

| Theme Choice | Background | Primary (Accent) | Work Phase | Break Phase |
| :--- | :--- | :--- | :--- | :--- |
| **Catppuccin Mocha** | `#1e1e2e` | `#89b4fa` (Blue) | `#f38ba8` (Red) | `#a6e3a1` (Green) |
| **Nord** | `#2e3440` | `#88c0d0` (Frost) | `#bf616a` (Red) | `#a3be8c` (Green) |
| **Gruvbox Dark** | `#282828` | `#fabd2f` (Yellow)| `#fb4934` (Red) | `#b8bb26` (Green) |
| **Tokyo Night** | `#1a1b26` | `#7aa2f7` (Blue) | `#f7768e` (Red) | `#9ece6a` (Green) |
| **Dracula** | `#282a36` | `#bd93f9` (Purple)| `#ff5555` (Red) | `#50fa7b` (Green) |
| **Solarized Dark** | `#002b36` | `#268bd2` (Blue) | `#dc322f` (Red) | `#859900` (Green) |

---

## 7. Storage Persistence & XDG Directory Resolution

Persistence is managed by `src/storage.rs`:

```rust
pub struct AppData {
    pub config: Config,
    pub tasks: TaskManager,
    pub stats: StatsHistory,
}
```

- **XDG Base Directory Resolution**:
  - Uses `directories::ProjectDirs::from("com", "termodoro", "termodoro")`.
  - On Linux: `~/.local/share/termodoro/data.json`
  - On macOS: `~/Library/Application Support/com.termodoro.termodoro/data.json`
  - On Windows: `%APPDATA%\termodoro\termodoro\data.json`
- **Atomic File Writing**:
  - Automatically creates parent directory paths if missing.
  - Serializes `AppData` to pretty-printed JSON before writing to disk.

---

## 8. Block Digit Rasterization Engine

To display a large countdown clock across varying terminal dimensions, `src/ui/digits.rs` implements a 5-row by 4-column Unicode block rasterizer:

```
Char '3':
Row 0: ████
Row 1:    █
Row 2: ████
Row 3:    █
Row 4: ████
```

- Format: Strings such as `"25:00"` are parsed character-by-character.
- Rasterization: Each character maps to a static array of 5 row slices. Rows are horizontally concatenated with a 1-column spacing delimiter.

---

## 9. Error Handling & Terminal Recovery Hooks

A major challenge in terminal application development is ensuring the terminal is not left in an unusable state if a panic occurs.

### Panic Hook Implementation (`src/main.rs`)

```rust
fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
        original_hook(panic_info);
    }));
}
```

If an unhandled error triggers a panic, this hook executes first:
1. Disables raw mode (`disable_raw_mode()`).
2. Leaves the alternate screen buffer (`LeaveAlternateScreen`).
3. Re-enables cursor visibility (`cursor::Show`).
4. Invokes the original standard panic hook to print the stack trace cleanly to the terminal.

---

## 10. Automated Testing Strategy & Benchmarks

Termodoro includes comprehensive unit test suites:

- **`test_timer_initialization`**: Verifies initial phase, durations, and stopped status.
- **`test_phase_advancement`**: Verifies full cycle progression (Work -> ShortBreak -> ... -> LongBreak) and cycle resetting.
- **`test_progress_ratio`**: Validates percentage calculation precision.
- **`test_pause_and_reset`**: Validates pause, toggle, and reset transitions.
- **`test_task_lifecycle`**: Validates task creation, effort incrementing, and completion.
- **`test_task_filtering`**: Validates `All`, `Active`, and `Completed` predicate filters.
- **`test_stats_recording`**: Validates daily minutes calculation, session counts, and streak arithmetic.

Run the test suite with:
```bash
cargo test
```

---

## 11. Glossary of Technical Terms

- **Alternate Screen Buffer**: A secondary screen buffer in terminal emulators used by full-screen applications to prevent overwriting shell scrollback history.
- **Atomic Write**: A file operation that completes fully or not at all, preventing partially written or corrupted files.
- **Crossterm Backend**: An abstraction layer in Ratatui that uses the Crossterm library to send terminal escape sequences and read input events.
- **D-Bus**: An inter-process communication mechanism commonly used on Linux desktop environments for sending notifications.
- **Immediate-Mode GUI**: A user interface architecture where the UI tree is re-evaluated and redrawn on every single frame rather than storing stateful UI component objects.
- **Panic Hook**: A user-defined callback in Rust executed when a thread panics, allowing cleanup before process termination.
- **RAII (Resource Acquisition Is Initialization)**: A programming idiom where resource lifetime is tied to variable scope, guaranteeing cleanup on destruction.
- **Raw Mode**: A low-level terminal mode disabling canonical line processing and character echoing.
- **Timeboxing**: Allocating a fixed, predetermined timeframe to a specific task.
- **UUID (Universally Unique Identifier)**: A 128-bit label used for unique identification of entities in software systems without central coordination.
- **XDG Specification**: Standards published by Freedesktop.org defining directory paths for user configuration, data, and cache files.

---

## 12. References & Citations

1. **Cirillo, Francesco (2006)**. *The Pomodoro Technique*. FC Garage GmbH. [https://francescocirillo.com/products/the-pomodoro-technique](https://francescocirillo.com/products/the-pomodoro-technique)
2. **Ratatui Project Developers (2024)**. *Ratatui: A Rust library for cooking up terminal user interfaces*. [https://ratatui.rs/](https://ratatui.rs/)
3. **Crossterm Project Developers (2024)**. *Crossterm: Cross-platform Terminal Manipulation Library*. [https://docs.rs/crossterm/](https://docs.rs/crossterm/)
4. **Rust Programming Language Documentation**. *Error Handling and Panic Management in Rust*. [https://doc.rust-lang.org/book/ch09-00-error-handling.html](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
5. **Freedesktop.org Standard Specifications**. *XDG Base Directory Specification (Version 0.8)*. [https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
6. **IEEE / The Open Group (2018)**. *POSIX.1-2017: General Terminal Interface (termios)*. Standard for Information Technology. [https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/termios.h.html](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/termios.h.html)
