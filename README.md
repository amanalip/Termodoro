# 🍅 Termodoro

> A sleek, high-performance Pomodoro timer, task manager, and productivity tracker built for the terminal in **Rust** with **Ratatui** and **Crossterm**.

---

## ✨ Features

- ⏳ **Customizable Pomodoro Cycles**: Standard Work (25m), Short Break (5m), and Long Break (15m) with automatic/manual transitions and customizable intervals.
- ⏱️ **Large ASCII Block Countdown Display**: Dynamic 5-row bold digits and smooth percentage gauge bar.
- 📋 **Integrated Task Manager**:
  - Add, filter (`All`, `Active`, `Completed`), complete, and delete tasks.
  - Set active focus target to automatically log spent Pomodoros against specific tasks.
  - Track estimated vs actual Pomodoro sessions per task.
- 📊 **Productivity Analytics & Daily Streaks**:
  - Today's completed sessions and total focus minutes.
  - Daily consecutive streak counter with historical personal best.
  - 7-day visual bar chart and recent activity log.
- 🎨 **Rich Built-in Color Themes**:
  - Catppuccin Mocha
  - Nord
  - Gruvbox Dark
  - Tokyo Night
  - Dracula
  - Solarized Dark
- 🔔 **Desktop & Audio Alerts**:
  - Native OS desktop notifications (`notify-rust`).
  - Terminal audio bell (`\x07`) on session completion.
- 💾 **Automatic State Persistence**:
  - Automatically saves settings, tasks, and historical session logs in XDG standard directories (`~/.local/share/termodoro/data.json`).
- 📖 **Line-by-Line Commented Codebase**: Every single line of source code is meticulously commented for maximum readability and clarity.

---

## ⌨️ Keybindings

### Navigation (Global)
| Key | Action |
| --- | --- |
| `Tab` / `Shift+Tab` | Switch between tabs (`Timer` ⇄ `Tasks` ⇄ `Stats` ⇄ `Settings`) |
| `1`, `2`, `3`, `4` | Jump directly to tab (1: Timer, 2: Tasks, 3: Stats, 4: Settings) |
| `?` | Open / Close interactive Help modal |
| `q` / `Esc` | Quit application / Close modal |

### Tab 1: Pomodoro Timer
| Key | Action |
| --- | --- |
| `Space` | Start / Pause countdown timer |
| `r` | Reset timer to full duration for current phase |
| `s` | Skip current phase and advance to next phase |
| `a` | Quick add new task |

### Tab 2: Tasks
| Key | Action |
| --- | --- |
| `a` | Add new task (opens modal) |
| `Space` / `Enter` | Toggle task completion |
| `t` | Set selected task as the active focus target |
| `d` / `x` | Delete selected task |
| `↑` / `k`, `↓` / `j` | Navigate tasks list |
| `1`, `2`, `3` | Filter tasks (`1: All`, `2: Active`, `3: Completed`) |

### Tab 4: Preferences & Settings
| Key | Action |
| --- | --- |
| `↑` / `k`, `↓` / `j` | Select setting row |
| `←` / `h`, `→` / `l` | Adjust duration values / cycle color themes |
| `+`, `-` | Increment / Decrement values |
| `Space` / `Enter` | Toggle feature flags on / off |

---

## 🚀 Installation & Running

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (edition 2021+, Rust 1.74+)

### Build & Run
```bash
# Clone the repository
git clone https://github.com/amanalip/Termodoro.git
cd Termodoro

# Run development build
cargo run

# Or build optimized release binary
cargo build --release
./target/release/termodoro
```

### Running Tests
```bash
cargo test
```

---

## 📜 License
GPL-3.0 License. See [LICENSE](LICENSE) for details.
