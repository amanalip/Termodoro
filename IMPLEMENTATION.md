# Technical Implementation Specification: Termodoro

This document details the engineering architecture, data structures, state machines, and algorithms implemented in **Termodoro**.

---

## 1. System Overview

Termodoro is built in Rust using an event-driven terminal architecture. It relies on:
- **`ratatui` (v0.29)**: High-performance terminal user interface library providing immediate-mode rendering, widgets, and layout constraints.
- **`crossterm` (v0.28)**: Cross-platform terminal control backend managing raw mode, alternate screen buffers, cursor visibility, and keyboard event streams.
- **`serde` & `serde_json`**: Type-safe JSON serialization for state persistence.
- **`chrono`**: Timezone-aware date and timestamp operations for daily productivity tracking and streak analysis.
- **`notify-rust`**: Multi-platform native desktop notification dispatcher.
- **`directories`**: Standards-compliant resolution of system data and configuration directories.

---

## 2. Core Architecture & Module Breakdown

```
Termodoro/
├── Cargo.toml
├── src/
│   ├── main.rs            # Application bootstrap, event loop, panic recovery
│   ├── app.rs             # Application state, key dispatch, modal management
│   ├── timer.rs           # Pomodoro finite state machine, tick calculation
│   ├── tasks.rs           # Task model, filtering logic, effort counter
│   ├── stats.rs           # Aggregation algorithms, streak calculations
│   ├── config.rs          # User preferences schema
│   ├── theme.rs           # Color palette definitions and RGB style tokens
│   ├── storage.rs         # File I/O, XDG path resolution, atomic persistence
│   └── ui/
│       ├── mod.rs         # Root view layout coordinator, tabs, header/footer
│       ├── digits.rs      # 5x3 block font character rasterization
│       ├── timer_view.rs  # Main timer screen (digits, gauge, cycle dots)
│       ├── tasks_view.rs  # Task management list and filtering interface
│       ├── stats_view.rs  # Analytics dashboard and weekly bar chart
│       ├── settings_view.rs# Preferences editor and theme selector
│       ├── task_modal.rs  # Modal popup for task creation
│       └── help_popup.rs  # Modal overlay for keybindings
```

---

## 3. Module Specifications

### 3.1 `src/main.rs`: Lifecycle & Terminal Management

- **Terminal Initialization**:
  - `enable_raw_mode()`: Puts the terminal in raw mode, capturing keystrokes without requiring line feeds or echoing characters.
  - `EnterAlternateScreen`: Switches execution to an isolated screen buffer, ensuring the user's prior shell history is left untouched upon exiting.
  - `crossterm::cursor::Hide`: Suppresses the terminal cursor during active rendering.
- **Panic Hook Safety (`setup_panic_hook`)**:
  - Registers a custom panic hook via `panic::set_hook`.
  - In the event of an unhandled panic, the hook disables raw mode and restores the standard screen buffer before printing stack traces, preventing terminal corruption.
- **Event Loop (`run_app`)**:
  - Polling interval: 250 milliseconds (`Duration::from_millis(250)`).
  - Ticking: Executes `app.on_tick()` to advance active timers and expire transient status messages.
  - Event Handling: Intercepts `crossterm::event::Event::Key` press events and delegates to `app.on_key_event()`.

---

### 3.2 `src/timer.rs`: Finite State Machine (FSM)

The timer manages phase transitions across three states: `Work`, `ShortBreak`, and `LongBreak`.

```
                    ┌─────────────┐
                    │    Work     │
                    │  (Focus)    │
                    └──────┬──────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
  [cycle < interval]             [cycle == interval]
             │                           │
             ▼                           ▼
      ┌──────────────┐            ┌──────────────┐
      │ Short Break  │            │  Long Break  │
      └──────┬───────┘            └──────┬───────┘
             │                           │
             └─────────────┬─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │    Work     │
                    └─────────────┘
```

#### State Structures

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

- **Interval Computation**:
  - Target duration is calculated dynamically from `Config` parameters (`work_duration_mins * 60`, etc.).
  - Progress ratio: `(total_duration - time_remaining) / total_duration`.
- **Phase Completion Event (`TimerEvent::PhaseCompleted`)**:
  - Dispatched when `time_remaining_secs` reaches 0.
  - Automatically advances `completed_pomodoros` and updates `current_cycle`.
  - Checks `auto_start_work` or `auto_start_breaks` configuration flags to determine whether the subsequent phase begins immediately or idles in `TimerStatus::Stopped`.

---

### 3.3 `src/tasks.rs`: Task Collection & Work Tracking

Manages in-memory task lists with persistent UUID identifiers.

#### Data Model

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

- **Active Target Binding**:
  - An optional `active_task_id` binds a task to the timer.
  - When a `Work` Pomodoro completes, `app.tasks.increment_active_spent()` automatically increments the spent Pomodoro count on the active task.
- **Filtering Predicate (`filtered_indices`)**:
  - Evaluates tasks against `TaskFilter::All`, `TaskFilter::Active`, or `TaskFilter::Completed`, returning filtered indices for non-destructive display and navigation.

---

### 3.4 `src/stats.rs`: Analytics & Streak Algorithms

Records historical session events:

```rust
pub struct CompletedSession {
    pub timestamp: DateTime<Utc>,
    pub phase: PomodoroPhase,
    pub duration_mins: u32,
    pub task_id: Option<String>,
    pub task_title: Option<String>,
}
```

#### Consecutive Daily Streak Algorithm
1. Extracts all distinct local calendar dates (`NaiveDate`) where at least one `PomodoroPhase::Work` session occurred.
2. Evaluates the most recent active date: if neither today nor yesterday contain logged work, the current streak is `0`.
3. Iterates backwards through unique chronological dates, decrementing expected dates using `pred_opt()`. The streak increments for each consecutive date until a gap is encountered.
4. Longest streak is computed by evaluating the maximum contiguous sequence of calendar day successors across the entire history.

---

### 3.5 `src/theme.rs`: Color System

Encapsulates RGB color palettes across six curated themes:

| Theme | Background RGB | Accent / Primary RGB | Work Phase RGB |
| :--- | :--- | :--- | :--- |
| **Catppuccin Mocha** | `rgb(30, 30, 46)` | `rgb(137, 180, 250)` | `rgb(243, 139, 168)` |
| **Nord** | `rgb(46, 52, 64)` | `rgb(136, 192, 208)` | `rgb(191, 97, 106)` |
| **Gruvbox Dark** | `rgb(40, 40, 40)` | `rgb(250, 189, 47)` | `rgb(251, 73, 52)` |
| **Tokyo Night** | `rgb(26, 27, 38)` | `rgb(122, 162, 247)` | `rgb(247, 118, 142)` |
| **Dracula** | `rgb(40, 42, 54)` | `rgb(189, 147, 249)` | `rgb(255, 85, 85)` |
| **Solarized Dark** | `rgb(0, 43, 54)` | `rgb(38, 139, 210)` | `rgb(220, 50, 47)` |

---

### 3.6 `src/storage.rs`: Persistence Architecture

- **XDG Base Directory Resolution**:
  - Leverages `directories::ProjectDirs::from("com", "termodoro", "termodoro")`.
  - Automatically creates the data directory if it does not exist.
  - Fallback: Defaults to `./data.json` if system directory resolution fails.
- **Serialization Format**: Standard formatted JSON containing `config`, `tasks`, and `stats`.

---

### 3.7 `src/ui/digits.rs`: Block Digit Rasterization

Renders high-visibility countdown timers using 5-row by 4-column Unicode block characters (`█`):

```
████   ████    :   ████   ████
   █   █       :   █  █   █  █
████   ████        █  █   █  █
   █      █    :   █  █   █  █
████   ████    :   ████   ████
```

Each character (`'0'..'9'`, `':'`) maps to a static array `[&'static str; 5]`, concatenated row-by-row with spacing delimiters.

---

## 4. Verification & Testing Strategy

Automated test suites in `src/timer.rs`, `src/tasks.rs`, and `src/stats.rs` validate:
- **Timer Initialization**: Default durations, initial Stopped status, remaining seconds calculations.
- **Phase Transition Sequences**: Work ➔ Short Break ➔ ... ➔ Long Break cycles and cycle counter resetting.
- **Progress Ratio Accuracy**: Precision calculation of percentage completion.
- **Task Lifecycle**: Creation, active target binding, effort incrementing, completion status toggling, and multi-mode filtering.
- **Analytics Calculations**: Real-time aggregation of today's focus minutes, total sessions, and streak evaluations.

To execute the test suite:
```bash
cargo test
```
