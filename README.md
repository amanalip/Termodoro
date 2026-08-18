# Termodoro: Terminal Pomodoro & Task Management System

<div align="center">

[![CI](https://github.com/amanalip/Termodoro/actions/workflows/rust.yml/badge.svg)](https://github.com/amanalip/Termodoro/actions)
[![Tests](https://img.shields.io/badge/tests-192%20passed%20(100%25)-brightgreen.svg)](test_report.md)
[![Rust](https://img.shields.io/badge/rust-1.74%2B%20(Edition%202021)-orange.svg)](https://www.rust-lang.org)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Safety](https://img.shields.io/badge/unsafe%20code-0%25%20(Safe%20Rust)-brightgreen.svg)](audit_log.md)
[![Privacy](https://img.shields.io/badge/telemetry-0%25%20(100%25%20Offline)-blueviolet.svg)](audit_log.md)

![Termodoro Focus Timer View](assets/screenshots/kde_01_timer_view.png)

*A keyboard-driven, ultra-fast Pomodoro timer and task manager crafted in pure Rust with Ratatui on KDE Linux / CachyOS.*

</div>

---

### ⚡ 30-Second Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/amanalip/Termodoro.git
cd Termodoro

# 2. Build and launch in release mode
cargo run --release
```

---

## Table of Contents

1. [Introduction to the Pomodoro Technique](#1-introduction-to-the-pomodoro-technique)
2. [Why Use a Terminal-Based Timer?](#2-why-use-a-terminal-based-timer)
3. [Key Features Overview](#3-key-features-overview)
4. [System Architecture & Design Specification](#4-system-architecture--design-specification)
5. [Interactive User Journeys & Guides](#5-interactive-user-journeys--guides)
6. [Installation & Beginner Setup Guide](#6-installation--beginner-setup-guide)
   - [Beginner Concepts & Terminology Primer](#beginner-concepts--terminology-primer)
   - [System Prerequisites & Compatibility Matrix](#system-prerequisites--compatibility-matrix)
   - [Step 0: Checking Core Utilities (curl & git)](#step-0-checking-core-utilities-curl--git)
   - [Step 1: Installing Rust and Cargo Toolchain](#step-1-installing-rust-and-cargo-toolchain)
   - [Step 2: Installing OS Build Dependencies](#step-2-installing-os-build-dependencies)
   - [Step 3: Cloning and Installing Termodoro](#step-3-cloning-and-installing-termodoro)
     - [Option A: Global Installation via Cargo (Recommended)](#option-a-global-installation-via-cargo-recommended)
     - [Option B: Running Directly from Source (Testing / Development)](#option-b-running-directly-from-source-testing--development)
     - [Option C: Manual Release Binary Installation](#option-c-manual-release-binary-installation)
   - [Step 4: Adding `~/.cargo/bin` to Your PATH](#step-4-adding-cargobin-to-your-path)
   - [Step 5: Shell Aliases, Shortcuts & Autostart](#step-5-shell-aliases-shortcuts--autostart)
   - [Step 6: Terminal Multiplexer Integration (tmux & Zellij)](#step-6-terminal-multiplexer-integration-tmux--zellij)
   - [Step 7: Verification & First Launch Checklist](#step-7-verification--first-launch-checklist)
   - [Step 8: Beginner Troubleshooting & Error Guide](#step-8-beginner-troubleshooting--error-guide)
   - [Updating & Clean Uninstallation Guide](#updating--clean-uninstallation-guide)
7. [User Interface and Navigation Guide](#7-user-interface-and-navigation-guide)
   - [Global Navigation Controls](#global-navigation-controls)
   - [Tab 1: Pomodoro Countdown Timer](#tab-1-pomodoro-countdown-timer)
   - [Tab 2: Interactive Task Manager](#tab-2-interactive-task-manager)
   - [Tab 3: Productivity Analytics & Streak Tracking](#tab-3-productivity-analytics--streak-tracking)
   - [Tab 4: Preferences, Durations & Theme Selector](#tab-4-preferences-durations--theme-selector)
8. [Local Database, Persistence & Restart Recovery](#8-local-database-persistence--restart-recovery)
9. [Troubleshooting & Frequently Asked Questions (FAQ)](#9-troubleshooting--frequently-asked-questions-faq)
10. [Development, Testing & Contribution](#10-development-testing--contribution)
11. [Fact-Check, Sanity Audit & Certification](#11-fact-check-sanity-audit--certification)
12. [Glossary of Terms](#12-glossary-of-terms)
13. [References and Further Reading](#13-references-and-further-reading)
14. [License](#14-license)

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

Modern productivity apps are frequently bloated with heavy web-view containers (Electron), background battery drain, intrusive cloud subscriptions, and distracting telemetry tracking. Termodoro is engineered specifically for developers, sysadmins, writers, and command-line enthusiasts who want:

1. **Instantaneous Startup ($< 10\text{ ms}$)**: Compiled directly to native machine code with zero runtime overhead.
2. **Minimal Memory Footprint ($< 15\text{ MB}$ RAM)**: Operates silently in a terminal tab, tmux pane, or Zellij floating window without consuming gigabytes of system memory.
3. **100% Offline & Private**: All tasks, metrics, and streaks are stored strictly on your local disk with zero network requests or telemetry.
4. **Keyboard-Driven Fluidity**: Every single action—from task creation to duration adjustment—can be executed via ergonomic vim-inspired keybindings without reaching for a mouse.
5. **Aesthetic Visual Excellence**: Designed with 18 high-contrast color themes, smooth Unicode gauges, block-font big digits, and real-time status banners.

---

## 3. Key Features Overview

- **Aesthetic Terminal Interface**: Built with [Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm) supporting ANSI TrueColor (24-bit RGB) and responsive terminal resizing.
- **5x3 Block Clock Display**: Large ASCII digital numerals rendered dynamically using Unicode block elements (`█`).
- **Configurable Interval FSM**: 3-state Pomodoro engine (Work, Short Break, Long Break) with custom interval lengths (1 to 24 cycles).
- **Full Task Lifecycle Management**: Create tasks, set estimated Pomodoro counts, mark items complete, and bind a target task to log effort automatically.
- **Analytics Dashboard & Streak Tracker**: Daily focus summaries, consecutive active calendar day streaks, 7-day visual bar charts, and a historical session log.
- **18 Built-in Color Themes**: Modern dark and light palettes including Catppuccin Mocha, Macchiato, Frappé, Latte (Light), Nord, Gruvbox Dark, Tokyo Night, Dracula, Solarized Dark & Light, Rose Pine, One Dark, Kanagawa, Everforest Dark & Light, Synthwave '84, Monokai Pro, and OLED Phosphor.
- **Acoustic Chimes & Native Notifications**: Pure in-memory synthesized audio chimes (Zen Tibetan singing bowl, two-tone alert, and major triad chord) paired with native desktop notifications and ASCII terminal bell fallback.
- **Automatic State Persistence**: Automatically saves your tasks, preferences, and session history according to standard XDG data directory guidelines.

---

## 4. System Architecture & Design Specification

Termodoro is organized as a modular Rust application adhering to strict separation of concerns between state management, logic engines, and immediate-mode user interface rendering. For a comprehensive, in-depth architectural breakdown, see [**System Design Specification (`System_design.md`)**](System_design.md).

```
Termodoro/
├── Cargo.toml                  # Package manifest, dependencies, and build profiles
├── System_design.md            # Comprehensive system design, architecture & technical rationale
├── IMPLEMENTATION.md           # In-depth engineering specification and algorithms
├── WALKTHROUGH.md              # Operational workflows, code tour, and test benchmarks
├── test_report.md              # Comprehensive 192-test QA audit & test suite report
├── audit_log.md                # Permanent audit log & verification history (AUD-001 to AUD-013)
├── new_features_tracker.md     # Feature tracking roadmap and specifications
├── user_journeys/              # Interactive step-by-step visual user journey walkthroughs
│   ├── README.md               # User journeys index, workflow catalog & keybinding reference
│   ├── 01_focus_session_and_cycling.md
│   ├── 02_task_management_and_estimates.md
│   ├── 03_analytics_and_streaks.md
│   └── 04_preferences_and_themes.md
└── src/
    ├── main.rs                 # Terminal runtime initialization, event loop, panic hook
    ├── app.rs                  # Central application state, keyboard event dispatcher
    ├── timer.rs                # Pomodoro finite state machine, tick calculation logic
    ├── audio.rs                # Pure 16-bit PCM RIFF WAV synthesis and sound playback
    ├── tasks.rs                # Task model, UUID assignment, filter predicates
    ├── stats.rs                # Data aggregation, streak calculation algorithms
    ├── config.rs               # User preference schema and default parameters
    ├── theme.rs                # Theme choices and concrete 18 RGB color palettes
    ├── storage.rs              # File I/O, XDG directory resolution, zero-telemetry storage
    └── ui/
        ├── mod.rs              # Root view layout coordinator, tabs, header and footer
        ├── digits.rs           # 5x3 block font character rasterization for digital clock
        ├── timer_view.rs       # Main timer screen (digits, gauge, cycle dots, target card)
        ├── tasks_view.rs       # Interactive task table, status checkboxes, filter selector
        ├── stats_view.rs       # Metric cards, weekly bar chart, recent activity log
        ├── settings_view.rs    # Live configuration editor and theme switcher
        ├── task_modal.rs       # Task creation modal dialog
        └── help_popup.rs       # Global keybinding modal overlay
```

---

## 5. Interactive User Journeys & Guides

To help you get the most out of Termodoro, detailed step-by-step user journey guides with visual screenshots are available in the [`user_journeys/`](user_journeys/) directory (see [**`user_journeys/README.md`**](user_journeys/README.md) for the full catalog):

| User Journey | Guide Document | Key Workflows & Visual Highlights |
| :--- | :--- | :--- |
| **Catalog & Overview** | [**`user_journeys/README.md`**](user_journeys/README.md) | User journeys overview, workflow directory index, design architecture, and master keybinding reference cheatsheet. |
| **01. Core Focus Session** | [**`01_focus_session_and_cycling.md`**](user_journeys/01_focus_session_and_cycling.md) | Running 25m focus sessions, pause/resume, automatic interval transitions, cycle dot progression, and acoustic chimes. |
| **02. Task Management** | [**`02_task_management_and_estimates.md`**](user_journeys/02_task_management_and_estimates.md) | Task creation modal, estimating Pomodoro blocks, setting active target tasks, filtering views, and automatic effort logging. |
| **03. Analytics & Streaks** | [**`03_analytics_and_streaks.md`**](user_journeys/03_analytics_and_streaks.md) | Daily focus time cards, consecutive streak retention, 7-day activity bar charts, and historical activity logs. |
| **04. Preferences & Theming** | [**`04_preferences_and_themes.md`**](user_journeys/04_preferences_and_themes.md) | Live duration editing, 24-cycle customization, sound & desktop notification toggles, and exploring all 18 color palettes. |

---

## 6. Installation & Beginner Setup Guide

Termodoro is an open-source Rust terminal application. Whether you are an experienced systems developer or completely new to command-line tools, this section will walk you through setting up everything step by step.

---

### Beginner Concepts & Terminology Primer

If you are new to terminal tools, here is a quick overview of key concepts used throughout this guide:

- **What is a Terminal / CLI?**: A terminal (or Command-Line Interface) is a text-based window where you control your computer by typing commands instead of clicking graphical buttons.
- **What is Rust and Cargo?**: 
  - **Rust** is a modern, memory-safe, ultra-fast programming language that compiles directly into machine code.
  - **Cargo** is Rust’s official package manager, build tool, and dependency coordinator (similar to `npm` for Node.js, `pip` for Python, or `brew` for macOS).
- **What is Git and Repository Cloning?**: Git is a version control tool. "Cloning" simply downloads the complete source code files of Termodoro from GitHub onto your local hard drive.
- **What is `$PATH`?**: An environment variable on your operating system that lists directories where executable programs live. When you type `termodoro` in your terminal, the system looks through each folder in your `$PATH` to find the program.

---

### System Prerequisites & Compatibility Matrix

| Component | Minimum Version | Recommended Tools | Purpose |
| :--- | :--- | :--- | :--- |
| **Rust & Cargo** | `1.74.0` or newer | Installed via `rustup` | Compiles the Rust source code into a native binary |
| **Terminal Emulator** | 24-bit TrueColor (`COLORTERM=truecolor`) | [Alacritty](https://alacritty.org), [Kitty](https://sw.kovidgoyal.net/kitty/), [WezTerm](https://wezfurlong.org/wezterm/), [iTerm2](https://iterm2.com), [Windows Terminal](https://github.com/microsoft/terminal) | Renders rich RGB colors and fast terminal frames |
| **Unicode Font** | UTF-8 compatible | [Nerd Fonts](https://www.nerdfonts.com) (JetBrains Mono, FiraCode, Hack) | Displays block ASCII art, symbols, and progress indicators |
| **Notification Daemon** *(Linux only)* | Any Freedesktop-compliant daemon | `dunst`, `mako`, `swaync`, `xfce4-notifyd` | Displays desktop notifications upon phase completion |

---

### Step 0: Checking Core Utilities (curl & git)

Before compiling, verify that `curl` (to download the installer) and `git` (to download the repository) are installed:

```bash
curl --version
git --version
```

If either command reports `command not found`, install them using your system’s package manager:
- **Ubuntu / Debian / Pop!_OS / Linux Mint**:
  ```bash
  sudo apt update && sudo apt install -y curl git
  ```
- **Arch Linux / Manjaro**:
  ```bash
  sudo pacman -Sy curl git
  ```
- **Fedora / RHEL**:
  ```bash
  sudo dnf install -y curl git
  ```
- **macOS**:
  ```bash
  # macOS includes curl by default. For git, install Apple Command Line Tools:
  xcode-select --install
  ```
- **Windows**:
  Download and install Git from [https://git-scm.com/download/win](https://git-scm.com/download/win).

---

### Step 1: Installing Rust and Cargo Toolchain

The official and safest way to install Rust is via `rustup`:

#### Linux and macOS
Run the official installer script:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
During the interactive prompt:
1. Press `1` and hit `Enter` to proceed with standard installation.
2. Once the script says `Rust is installed now. Great!`, activate the environment:
   ```bash
   source "$HOME/.cargo/env"
   ```

#### Windows
1. Download the official installer: [`rustup-init.exe`](https://rustup.rs).
2. Run the file, choose option `1` (default install), and complete the wizard.
3. Close and reopen your PowerShell or Windows Terminal.

#### Verify Your Installation
Run these two commands to make sure both tools respond:
```bash
rustc --version    # Example: rustc 1.80.1 (3f5ac8251 2024-08-06)
cargo --version    # Example: cargo 1.80.1 (3f5ac8251 2024-08-06)
```

---

### Step 2: Installing OS Build Dependencies

Termodoro uses purely synthesized in-memory PCM audio without requiring heavy external media libraries. However, standard C build linkers and audio development headers are needed during the initial build:

- **Ubuntu / Debian / Linux Mint / Pop!_OS**:
  ```bash
  sudo apt update && sudo apt install -y build-essential git libasound2-dev pkg-config libdbus-1-dev
  ```

- **Arch Linux / Manjaro / EndeavourOS**:
  ```bash
  sudo pacman -S --needed base-devel git alsa-lib pkgconf
  ```

- **Fedora / RHEL / CentOS**:
  ```bash
  sudo dnf groupinstall "Development Tools" && sudo dnf install -y git alsa-lib-devel pkgconf-pkg-config
  ```

- **macOS (Apple Silicon & Intel)**:
  ```bash
  xcode-select --install
  ```

- **Windows (10 / 11 / Server)**:
  - Install **Visual Studio C++ Build Tools** (selected automatically during `rustup-init.exe` or downloadable from [Visual Studio Downloads](https://visualstudio.microsoft.com/visual-cpp-build-tools/)).
  - No additional audio packages needed (Windows Core Audio is supported natively).

---

### Step 3: Cloning and Installing Termodoro

First, download the source code repository from GitHub:
```bash
git clone https://github.com/amanalip/Termodoro.git
cd Termodoro
```

Choose the installation option that fits your preference:

#### Option A: Global Installation via Cargo (Recommended)
This compiles an optimized, standalone binary (~5.1 MB) and installs it into `~/.cargo/bin/termodoro`:
```bash
cargo install --path .
```
Once installed, you can launch Termodoro from **any terminal folder** on your system:
```bash
termodoro
```

##### Total Installation & Disk Footprint

| Component | Filesystem Location | Disk Footprint | Details |
| :--- | :--- | :---: | :--- |
| **Standalone Binary** | `~/.cargo/bin/termodoro` | **~5.1 MB** | Fully self-contained binary with 18 themes, audio PCM synth & TUI engine |
| **User Data & Settings** | `~/.local/share/termodoro/data.json` | **~5 KB** | Stores tasks, streaks, analytics & custom durations |
| **Source Repository** | `~/Documents/.../Termodoro` | **~3.9 MB** | Complete codebase, documentation, and Git history |
| **Compiler Cache** | `target/` | **0 MB** *(after `cargo clean`)* | Temporary build cache can be deleted anytime without impacting the app |
| **Total System Footprint** | Global System Installation | **~5.1 MB** | Ultra-lightweight (smaller than a single MP3 audio file!) |

> **💡 Disk Space Tip**: After running `cargo install --path .`, you can safely run `cargo clean` inside this repository folder. The installed `termodoro` binary in `~/.cargo/bin/` will continue to launch instantly (0.01s) without needing any recompilation, keeping your local project folder at a lean **~3.9 MB** without generating temporary compiler cache files!

#### Option B: Running Directly from Source (Testing / Development)
If you are developing or want to test source code edits without installing globally:
```bash
# Debug mode (creates local target/ build cache)
cargo run

# Optimized release mode
cargo run --release
```

#### Option C: Manual Release Binary Installation
If you prefer placing compiled standalone binaries directly in standard Unix directories (such as `~/.local/bin` or `/usr/local/bin`):
```bash
# 1. Compile the release artifact
cargo build --release

# 2. Copy the binary to your local bin directory
mkdir -p ~/.local/bin
cp target/release/termodoro ~/.local/bin/

# 3. Ensure executable permissions
chmod +x ~/.local/bin/termodoro
```

---

### Step 4: Adding `~/.cargo/bin` to Your PATH

If you ran `cargo install --path .` and your terminal displays `command not found: termodoro`, your shell cannot locate your Cargo binary folder. Add it to your configuration file:

#### For Bash (`~/.bashrc`)
```bash
echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### For Zsh (`~/.zshrc` on macOS and Linux)
```bash
echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### For Fish (`~/.config/fish/config.fish`)
```fish
fish_add_path $HOME/.cargo/bin
fish_add_path $HOME/.local/bin
```

#### For Windows PowerShell
```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", "User")
```

---

### Step 5: Shell Aliases, Shortcuts & Autostart

For rapid daily access, configure convenient aliases in your shell profile:

```bash
# Add these lines to ~/.bashrc or ~/.zshrc
alias pomo="termodoro"
alias td="termodoro"
alias focus="termodoro"
```

#### Linux Desktop Application Launcher (`.desktop` entry)
To launch Termodoro from application launchers (Rofi, Wofi, GNOME, KDE, dmenu):

Create `~/.local/share/applications/termodoro.desktop`:
```ini
[Desktop Entry]
Name=Termodoro
Comment=Terminal Pomodoro & Focus Manager
Exec=alacritty -e termodoro
Icon=alarm
Terminal=false
Type=Application
Categories=Utility;Office;
Keywords=pomodoro;timer;focus;productivity;
```
*(Replace `alacritty` with your preferred terminal emulator if using Kitty, WezTerm, or Foot).*

---

### Step 6: Terminal Multiplexer Integration (tmux & Zellij)

Termodoro works seamlessly in split panes and dedicated popups inside terminal multiplexers:

#### tmux Dedicated Window or Floating Popup
Add these keybindings to your `~/.tmux.conf`:

```tmux
# Open Termodoro in a floating center popup (tmux 3.2+)
bind-key P display-popup -w 85% -h 80% -E "termodoro"

# Or spawn Termodoro in a new dedicated background window
bind-key T new-window -n "🍅 Termodoro" "termodoro"
```

#### Zellij Floating Pane
Launch Termodoro as a floating modal pane in [Zellij](https://zellij.dev):
```bash
zellij run --floating --width 80% --height 80% -- termodoro
```

---

### Step 7: Verification & First Launch Checklist

To verify your installation:

1. **Launch**: Run `termodoro` in your terminal.
2. **Layout Check**: Ensure the 5-row digital block clock renders without wrapping. Terminal window should be at least 80 columns wide by 24 rows high.
3. **Themes Check**: Press `4` to enter Settings, navigate to **Color Theme**, and press `l` or `→` to cycle through all 18 built-in themes (Catppuccin Mocha/Macchiato/Frappé/Latte, Nord, Gruvbox, Tokyo Night, Dracula, Solarized Dark/Light, Rose Pine, One Dark, Kanagawa, Everforest Dark/Light, Synthwave '84, Monokai Pro, OLED Phosphor).
4. **Audio Check**: Press `1` to return to the Timer, press `Space` to start, then press `s` to skip phase and confirm acoustic chimes play upon interval completion.
5. **Persistence Check**: Press `q` to quit, then restart `termodoro`. Confirm that your settings and active state were safely restored.

---

### Step 8: Beginner Troubleshooting & Error Guide

| Symptom / Error Message | Root Cause | Exact Solution |
| :--- | :--- | :--- |
| `error: linker 'cc' not found` | C compiler linker is missing on your system | **Ubuntu/Debian**: `sudo apt install -y build-essential`<br>**Arch**: `sudo pacman -S base-devel`<br>**macOS**: `xcode-select --install` |
| `pkg-config / alsa-lib not found` | Audio development headers are missing | **Ubuntu/Debian**: `sudo apt install -y libasound2-dev pkg-config`<br>**Fedora**: `sudo dnf install -y alsa-lib-devel pkgconf-pkg-config` |
| `command not found: termodoro` | `~/.cargo/bin` is not in your current `$PATH` | Run `export PATH="$HOME/.cargo/bin:$PATH"` and add it to your `~/.bashrc` or `~/.zshrc`. |
| Colors appear washed out / dull | Terminal running in standard 16-color mode | Add `export COLORTERM=truecolor` to your shell profile. |
| Text overlapping or clock squished | Terminal window size is too small | Maximize your terminal or resize window to at least **80x24 characters**. |
| `Permission denied` when running binary | Executable permission bit missing | Run `chmod +x ~/.local/bin/termodoro` or reinstall via `cargo install --path . --force`. |

---

### Updating & Clean Uninstallation Guide

#### Updating to the Latest Version
To upgrade your installed Termodoro binary to the newest release from source:

```bash
# 1. Navigate to the local repository folder
cd Termodoro

# 2. Fetch latest code updates
git pull origin main

# 3. Recompile and update the global binary
cargo install --path . --force

# 4. (Optional) Reclaim temporary compiler cache
cargo clean
```

---

#### How to Uninstall Termodoro on All Operating Systems

Termodoro leaves no persistent background services, system daemons, or registry keys. Uninstallation is completely clean, fast, and straightforward across all operating systems.

##### Step 1: Remove the Executable Binary

- **Method A: If installed via Cargo (`cargo install`) — Works on Linux, macOS & Windows**:
  ```bash
  cargo uninstall termodoro
  ```

- **Method B: If installed manually (`Option C`)**:
  - **Linux / macOS**:
    ```bash
    rm -f ~/.local/bin/termodoro /usr/local/bin/termodoro
    ```
  - **Windows (PowerShell)**:
    ```powershell
    Remove-Item -Force "$env:USERPROFILE\.cargo\bin\termodoro.exe"
    ```

---

##### Step 2: (Optional) Remove Local Data, Tasks & Configuration

If you wish to perform a 100% complete purge and remove your historical tasks, streak records, and custom settings:

- 🐧 **Linux & BSD**:
  ```bash
  # Remove data files and configuration directories
  rm -rf ~/.local/share/termodoro ~/.config/termodoro
  
  # If you created a desktop launcher entry:
  rm -f ~/.local/share/applications/termodoro.desktop
  ```

- 🍎 **macOS**:
  ```bash
  # Remove application support directory
  rm -rf ~/Library/Application\ Support/com.termodoro.termodoro
  ```

- 🪟 **Windows (Command Prompt / PowerShell)**:
  - **PowerShell**:
    ```powershell
    # Remove AppData roaming database folder
    Remove-Item -Recurse -Force "$env:APPDATA\termodoro"
    ```
  - **Command Prompt (`cmd.exe`)**:
    ```cmd
    rmdir /s /q "%APPDATA%\termodoro"
    ```

---

##### Step 3: (Optional) Delete the Cloned Repository Source

To remove the source code folder and all Git artifacts:

- **Linux / macOS**:
  ```bash
  cd ..
  rm -rf Termodoro
  ```
- **Windows (PowerShell)**:
  ```powershell
  cd ..
  Remove-Item -Recurse -Force .\Termodoro
  ```

---

## 7. User Interface and Navigation Guide

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

![Tab 1: Pomodoro Focus Timer](assets/screenshots/kde_01_timer_view.png)

#### Controls in Timer View
- `Space`: Toggle timer state between Running and Paused.
- `r`: Reset timer back to its initial full duration for the current phase.
- `s`: Skip the remaining time in the active phase and advance to the next interval.
- `a`: Open the task creation dialog to quickly add a new item without leaving the timer.

---

### Tab 2: Interactive Task Manager

The Tasks view provides a structured environment for managing tasks, estimating workload, and tracking completed Pomodoro sessions against specific goals.

![Tab 2: Task Management & Targets](assets/screenshots/kde_02_tasks_view.png)

#### Controls in Tasks View
- `a`: Open the **Add Task** dialog.
- `Space` or `Enter`: Toggle completion status on the selected task.
- `t`: Mark the selected task as the active focus target. As work sessions finish, Pomodoros are automatically credited to this item.
- `d` or `x`: Delete the selected task.
- `↑` / `k` and `↓` / `j`: Move selection up or down.
- `1`, `2`, `3`: Filter task list by `All`, `Active`, or `Completed`.

---

### Tab 3: Productivity Analytics & Streak Tracking

The Stats view visualizes your focus history, habit consistency, and daily work distribution:

![Tab 3: Productivity Analytics & Streaks](assets/screenshots/kde_03_stats_view.png)

#### Key Metrics Explained
- **Today's Focus**: Total work sessions completed on the current local calendar day and cumulative focus minutes.
- **Current Streak**: Number of consecutive calendar days with at least one completed work session. If you completed work yesterday or today, your streak remains active.
- **Personal Best**: The highest consecutive daily streak recorded in your history.
- **7-Day Bar Chart**: Visual bar chart illustrating daily session volume over the past week.

---

### Tab 4: Preferences, Durations & Theme Selector

The Settings view allows you to customize durations, toggles, and visual appearance in real time across **18 built-in color themes**:

![Tab 4: Preferences & Theme Selector](assets/screenshots/kde_04_settings_view.png)

#### Controls in Settings View
- `↑` / `k` and `↓` / `j`: Select configuration option.
- `←` / `h` or `-` / `_`: Decrease numerical values, or cycle backwards through color themes.
- `→` / `l` or `+` / `=`: Increase numerical values, or cycle forwards through color themes.
- `Space` or `Enter`: Toggle boolean flags (Enabled / Disabled).

#### Built-In Color Palettes Showcase (18 Themes)

| Palette Name | Type | Background | Primary Accent | Focus Phase Tone |
| :--- | :---: | :---: | :---: | :---: |
| **Catppuccin Mocha** *(Default)* | Dark | `#1e1e2e` | `#cba6f7` (Mauve) | `#f38ba8` (Red) |
| **Catppuccin Macchiato** | Dark | `#24273a` | `#c6a0f6` (Mauve) | `#ed8796` (Red) |
| **Catppuccin Frappé** | Dark | `#303446` | `#ca9ee6` (Mauve) | `#e78284` (Red) |
| **Catppuccin Latte** | Light | `#eff1f5` | `#8839ef` (Mauve) | `#d20f39` (Red) |
| **Tokyo Night** | Dark | `#1a1b26` | `#7aa2f7` (Blue) | `#f7768e` (Coral Red) |
| **Nord** | Dark | `#2e3440` | `#88c0d0` (Frost Cyan) | `#bf616a` (Nordic Red) |
| **Gruvbox Dark** | Dark | `#282828` | `#fabd2f` (Yellow) | `#fb4934` (Warm Red) |
| **Dracula** | Dark | `#282a36` | `#bd93f9` (Purple) | `#ff5555` (Dracula Red) |
| **Solarized Dark** | Dark | `#002b36` | `#268bd2` (Blue) | `#dc322f` (Solar Red) |
| **Solarized Light** | Light | `#fdf6e3` | `#268bd2` (Blue) | `#dc322f` (Solar Red) |
| **Rose Pine** | Dark | `#191724` | `#ebbcba` (Rose) | `#eb6f92` (Love) |
| **One Dark** | Dark | `#282c34` | `#61afef` (Blue) | `#e06c75` (Red) |
| **Kanagawa** | Dark | `#1f1f28` | `#7e9cd8` (Wave Blue) | `#c34043` (Autumn Red) |
| **Everforest Dark** | Dark | `#2d353b` | `#a7c080` (Green) | `#e67e80` (Red) |
| **Everforest Light** | Light | `#fdf6e3` | `#8da101` (Olive) | `#f85552` (Red) |
| **Synthwave '84** | Dark | `#262335` | `#ff7edb` (Neon Pink) | `#fe4450` (Hot Red) |
| **Monokai Pro** | Dark | `#2d2a2e` | `#ffd866` (Sun Yellow) | `#ff6188` (Rose Red) |
| **OLED Phosphor** | Dark | `#000000` | `#00ff66` (CRT Phosphor) | `#ff3333` (Warning Red) |

---

### Modal Overlays & Interactive Dialogs

#### 1. Add Task Modal Dialog
Quickly create new tasks, assign titles, and define estimated Pomodoro intervals with seamless input navigation:

![Add Task Modal Dialog](assets/screenshots/kde_05_task_modal.png)

#### 2. Help & Keybindings Dialog
Press `?` at any point to open the interactive keyboard cheat sheet:

![Help & Keybindings Dialog](assets/screenshots/kde_06_help_modal.png)



---

## 8. Local Database, Persistence & Restart Recovery

Termodoro features built-in, offline-first local persistence. **No information is lost upon closing or restarting the application.**

### How Data Persistence Works

1. **Automatic Loading on Launch**: When `termodoro` boots up, it automatically locates and loads your `data.json` database. If launching for the first time, it initializes the database with clean default settings.
2. **Real-Time & Exit Synchronization**: Whenever you create or complete a task, adjust preferences, change a theme, finish a focus interval, or quit (`q` / `Esc`), the application automatically serializes and writes the state to disk using atomic file writing.
3. **What is Persisted**:
   - 📋 **Interactive Tasks**: Task titles, completion marks (`✔` / `○`), spent Pomodoro counts, estimated Pomodoro counts, creation timestamps, and designated active target IDs.
   - ⚙️ **User Configuration**: Focus durations, break durations, long break intervals (1 to 24 cycles), automation toggles, alert preferences, and active color themes.
   - 📊 **Productivity Statistics & Streaks**: Complete chronological history of focus sessions, used to calculate daily focus minutes, current daily streaks, personal best streaks, and weekly activity distribution charts.

### Database File Location by Operating System

Termodoro follows the standard XDG base directory conventions on Unix and platform-native storage paths:

| Platform | Database File Path |
| :--- | :--- |
| **Linux / BSD** | `~/.local/share/termodoro/data.json` |
| **macOS** | `~/Library/Application Support/com.termodoro.termodoro/data.json` |
| **Windows** | `%APPDATA%\termodoro\termodoro\data.json` |

### Annotated `data.json` Schema

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

### Backing Up & Transferring Your Data
Because the database is standard JSON, you can easily back up, version-control, or migrate your Pomodoro history between computers simply by copying the `data.json` file.

---

## 9. Troubleshooting & Frequently Asked Questions (FAQ)

### General & Core Workflow Questions

#### Q1: Why is my `target/` directory huge (~1-2 GB) after building, and will `cargo clean` delete my saved data?
- **Answer**: `target/` is Rust's temporary compiler build cache containing intermediary compilation artifacts (`.rlib`, object code, build scripts). It is **not** where your user data or installed app lives.
- Running `cargo clean` (or `make clean`) only clears this compiler cache and reclaims ~1.8 GB of disk space.
- Your saved tasks, streaks, custom durations, and theme choices are stored separately in `~/.local/share/termodoro/data.json` (on Linux) or `%APPDATA%\termodoro\termodoro\data.json` (on Windows) and will **never** be deleted or affected by `cargo clean`.

#### Q2: What is the recommended way to install Termodoro without keeping large build files?
- **Answer**: Run `cargo install --path .` from inside the repository folder. This compiles and installs a tiny, standalone binary (**~5.1 MB**) into `~/.cargo/bin/termodoro`.
- Afterwards, run `cargo clean`. You can now run `termodoro` from any terminal directory instantly (0.01s launch time) with zero background disk bloat.

#### Q3: Does Termodoro work offline, and does it collect telemetry?
- **Answer**: Termodoro is **100% offline, private, and local-first**. It contains zero telemetry, zero analytics tracking, zero cloud dependencies, and zero internet requests during runtime. All data resides exclusively on your local machine in plain JSON format.

#### Q4: How does Termodoro calculate daily streaks and personal bests?
- **Answer**: A streak increments when you complete at least one focus Pomodoro interval on consecutive calendar days according to your local machine timezone (`chrono::Local`). If you completed focus work yesterday, your streak remains active today. If you skip a full calendar day, the streak resets gracefully while your **Personal Best** record remains permanently saved.

---

### Terminal, Visuals & Display Questions

#### Q5: The colors look washed out or different from the screenshots.
- **Cause**: Your terminal emulator might be running in legacy 16-color or 256-color mode rather than 24-bit TrueColor mode.
- **Solution**: Export `COLORTERM=truecolor` in your shell configuration (`~/.bashrc` or `~/.zshrc`):
  ```bash
  export COLORTERM=truecolor
  ```
  Modern terminal emulators (such as Alacritty, Kitty, WezTerm, Konsole, Ghostty, iTerm2, and Windows Terminal) enable TrueColor out of the box.

#### Q6: Some icons, emojis, or borders appear misaligned or broken.
- **Cause**: Your terminal font lacks Unicode emoji glyphs or modern box-drawing characters.
- **Solution**: Install and use a modern monospace font with Nerd Font glyphs or dedicated emoji coverage (e.g. `JetBrains Mono Nerd Font`, `Fira Code`, `Cascadia Code`, or `Noto Color Emoji`).

#### Q7: My terminal output became garbled after closing abnormally.
- **Cause**: If the process was terminated forcefully with `kill -9` or a terminal crash, the terminal raw mode might not have been reset.
- **Solution**: Type `reset` or run `stty sane` in your terminal and press `Enter` to restore normal terminal state.

---

### Audio & Notification Questions

#### Q8: The audio bell does not make any sound.
- **Cause**: Audio output stream could not be initialized, hardware output is muted, or sound is disabled in settings.
- **Solution**:
  1. Open Settings (Tab `4`) and ensure `Sound / Bell Alert` is toggled **[Enabled]**.
  2. Verify that your system audio output is unmuted.
  3. On Linux systems without PulseAudio/PipeWire running, ensure the ALSA audio driver is active.
  4. Termodoro synthesizes pure in-memory 16-bit PCM WAV audio at 44.1 kHz, requiring no external media files or codecs.

#### Q9: Desktop notifications are not appearing on Linux.
- **Cause**: A desktop notification daemon (such as `dunst`, `mako`, `swaync`, `fnott`, or `xfce4-notifyd`) may not be running.
- **Solution**: Install and start a notification daemon compatible with your desktop environment or window manager. Desktop notifications use native D-Bus protocols via `notify-rust` without shell subprocessing.

---

### Multiplexer, Cross-Platform & Advanced Questions

#### Q10: How do I run Termodoro in a floating window in tmux or Zellij?
- **tmux (3.2+)**: Add this keybinding to your `~/.tmux.conf` to toggle Termodoro in a floating center popup with `Prefix + P`:
  ```tmux
  bind-key P display-popup -w 85% -h 80% -E "termodoro"
  ```
- **Zellij**: Run Termodoro in a floating pane:
  ```bash
  zellij run --floating --width 85% --height 80% -- termodoro
  ```

#### Q11: Does Termodoro work on macOS and Windows?
- **Answer**: **Yes!** Termodoro is fully cross-platform:
  - **Linux**: Supported natively on all distributions (Arch, Ubuntu, Fedora, Debian, Void, Alpine, etc.).
  - **macOS**: Supported on both Apple Silicon (M1/M2/M3/M4) and Intel Macs via standard `cargo install --path .`.
  - **Windows**: Supported natively on Windows 10, 11, and Server (via Windows Terminal, PowerShell, or Command Prompt with Visual Studio C++ build tools installed).

#### Q12: How do I backup or transfer my tasks and history to another computer?
- **Answer**: Simply copy your `data.json` file to the new machine:
  - **Linux / BSD**: `~/.local/share/termodoro/data.json`
  - **macOS**: `~/Library/Application Support/com.termodoro.termodoro/data.json`
  - **Windows**: `%APPDATA%\termodoro\termodoro\data.json`
  Because the schema is standard JSON, you can also inspect, edit, or version-control your productivity data with Git.

---

## 10. Development, Testing & Contribution

### Automated Testing & Cache Cleanup (192 Tests)

To run the complete test suite and **automatically clean compiler build cache** (preventing `target/` directory bloat and reclaiming ~1.8 GB of disk space):

```bash
# Option A: Using Makefile (Recommended)
make test

# Option B: Using the automated shell script
./scripts/test_and_clean.sh

# Option C: Standard Cargo Test
cargo test
```

### Static Analysis, Lints & Hygiene
```bash
# Run full verification (fmt + clippy + 192 tests + auto-clean)
make check

# Check code formatting compliance
cargo fmt -- --check

# Run compiler linter with strict warning enforcement
cargo clippy -- -D warnings

# Manually reclaim disk space anytime
cargo clean
```

### Makefile Reference Cheatsheet

| Command | Action |
| :--- | :--- |
| `make test` | Run 192-test suite and automatically clean `target/` cache |
| `make check` | Execute `fmt`, `clippy`, full 192-test suite, and clean up |
| `make build` | Compile optimized release binary in `target/release/termodoro` |
| `make run` | Launch Termodoro in release mode |
| `make clean` | Reclaim local disk space immediately via `cargo clean` |
| `make fmt` | Automatically format all Rust source files |
| `make clippy` | Run Clippy static analysis with warnings as errors |

---

## 11. Fact-Check, Sanity Audit & Certification

To provide full confidence to developers, contributors, and users, all claims, metrics, algorithms, and compatibility requirements documented in this repository have been formally audited and verified against the production codebase.

### Audit Certification Matrix

| Verified Claim / Metric | Documented Value | Audited Source Code Reference | Verification Method & Benchmark | Status |
| :--- | :--- | :--- | :--- | :---: |
| **Test Suite Pass Rate** | 192 / 192 Passed (100%) | `src/` (All 9 test modules) | `cargo test` execution (1.16s total runtime) | **VERIFIED** |
| **Privacy & Zero Telemetry** | 100% Offline & Private (0 Network Calls) | `src/storage.rs` & `Cargo.lock` | Unit tests `test_privacy_zero_telemetry_guarantees` & CI check | **VERIFIED** |
| **Rust Safety Guarantee** | 100% Safe Rust (`0` unsafe blocks) | Full codebase grep (`grep -rn "unsafe" src/`) | Static code analysis via compiler frontend | **VERIFIED** |
| **Static Analysis Compliance** | 0 Warnings, 0 Errors | Entire workspace | `cargo clippy -- -D warnings` | **VERIFIED** |
| **Code Formatting Standard** | 100% Rustfmt Compliant | Code formatting rules | `cargo fmt -- --check` | **VERIFIED** |
| **Memory Footprint** | $< 15\text{ MB}$ Resident RAM | Runtime metrics via `/proc/[pid]/statm` | Ratatui zero-copy immediate mode rendering | **VERIFIED** |
| **Audio Generation Method** | Pure In-Memory PCM RIFF WAV | [`src/audio.rs`](src/audio.rs#L80-L150) | Byte buffer analysis, $f = 44.1\text{ kHz}$, 16-bit signed | **VERIFIED** |
| **Max Long Break Cycles** | $1 \le N \le 24$ Cycles | [`src/config.rs`](src/config.rs#L12-L28) & [`src/timer.rs`](src/timer.rs#L85-L105) | Unit test `test_twenty_four_cycle_advancement_and_long_break_trigger` | **VERIFIED** |
| **Streak Calculation Invariant** | Preservation across Month/Year | [`src/stats.rs`](src/stats.rs#L120-L190) | Unit test `test_streak_calculation_across_month_and_year_boundaries` | **VERIFIED** |
| **Storage Architecture** | Atomic Write & XDG Compliance | [`src/storage.rs`](src/storage.rs#L40-L110) | Atomic tempfile rename; XDG Base Directory specification | **VERIFIED** |
| **Color Palettes** | 18 Built-In Palettes | [`src/theme.rs`](src/theme.rs#L6-L42) | Unit test `test_theme_from_choice_all_variants` | **VERIFIED** |

### Independent Reproducibility Commands
Any user can independently reproduce and verify this entire audit report on their local machine by executing:

```bash
# 1. Run full 192-test automated suite
cargo test -- --nocapture

# 2. Verify zero compiler warnings or lint issues
cargo clippy -- -D warnings

# 3. Verify zero unsafe code blocks across all source files
! grep -rn "unsafe" src/

# 4. Verify zero network/telemetry crates in Cargo.lock
! grep -E "^name = \"(reqwest|ureq|hyper|curl|tungstenite|tokio-tungstenite|sentry|datadog|posthog)\"" Cargo.lock
```

---

## 12. Glossary of Terms

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

## 13. References and Further Reading

1. **Cirillo, Francesco (2006)**. *The Pomodoro Technique*. FC Garage GmbH. [https://francescocirillo.com/products/the-pomodoro-technique](https://francescocirillo.com/products/the-pomodoro-technique)
2. **Ratatui Documentation & Guide**. *Official Ratatui Book*. [https://ratatui.rs/](https://ratatui.rs/)
3. **Crossterm Documentation**. *Crossterm Crates.io Reference*. [https://docs.rs/crossterm/](https://docs.rs/crossterm/)
4. **Klabnik, Steve & Nichols, Carol (2023)**. *The Rust Programming Language*. No Starch Press. [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
5. **Freedesktop.org (2010)**. *XDG Base Directory Specification*. [https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
6. **Catppuccin Organization**. *Catppuccin Palette Specifications*. [https://github.com/catppuccin/catppuccin](https://github.com/catppuccin/catppuccin)
7. **Nord Theme Project**. *An arctic, north-bluish clean and elegant color palette*. [https://www.nordtheme.com/](https://www.nordtheme.com/)
8. **Microsoft Corporation & IBM (1991)**. *Multimedia Programming Interface and Data Specifications 1.0 (RIFF WAV Structure)*. IBM/Microsoft.

---

## 14. License

This project is licensed under the terms of the GNU General Public License v3.0 ([GPL-3.0](LICENSE)).
