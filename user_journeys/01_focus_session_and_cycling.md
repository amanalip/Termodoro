# User Journey 01: Core Pomodoro Focus Session & Interval Cycling

Master your daily workflow with **Termodoro**'s primary operational loop: running distraction-free focus intervals, handling interruptions smoothly, progressing across multi-cycle intervals, and resting with acoustic notifications.

---

## Table of Contents

1. [Journey Narrative & Persona](#1-journey-narrative--persona)
2. [Step-by-Step Interactive Walkthrough](#2-step-by-step-interactive-walkthrough)
   - [Step 1: Launching into the Focus Workstation](#step-1-launching-into-the-focus-workstation)
   - [Step 2: Starting the Countdown Timer](#step-2-starting-the-countdown-timer)
   - [Step 3: Handling Pauses & Reset Recovery](#step-3-handling-pauses--reset-recovery)
   - [Step 4: Completing a Work Phase & Acoustic Chimes](#step-4-completing-a-work-phase--acoustic-chimes)
   - [Step 5: Taking a Restorative Short Break](#step-5-taking-a-restorative-short-break)
   - [Step 6: Progressing Through 4 Cycles to the Long Break](#step-6-progressing-through-4-cycles-to-the-long-break)
3. [Visual Layout & Interface Deep Dive](#3-visual-layout--interface-deep-dive)
4. [Pro Tips & Power Workflows](#4-pro-tips--power-workflows)
5. [Complete Keybinding Reference](#5-complete-keybinding-reference)

---

## 1. Journey Narrative & Persona

> **Meet Alex, a Backend Systems Engineer.**  
> Alex has a demanding sprint goal: implementing a complex database migration. Context switching, Slack pings, and open browser tabs constantly derail their concentration. Alex opens a split terminal pane next to their code editor, launches `termodoro`, and enters a distraction-free flow state.

---

## 2. Step-by-Step Interactive Walkthrough

### Step 1: Launching into the Focus Workstation
Open your terminal emulator and launch Termodoro:

```bash
termodoro
```

Termodoro starts instantaneously in less than **10 milliseconds**, presenting **Tab 1: Timer View**. 
- The large 5-row digital block clock is initialized at `25:00`.
- The cycle counter shows `Cycle 1/4`.
- The status indicator displays `[STOPPED]`.

---

### Step 2: Starting the Countdown Timer
- Press `Space` to initiate your focus block.
- The status switches to `[RUNNING]`.
- The clock smoothly decrements every second, while the gauge bar at the bottom fills proportionally.
- You can minimize the terminal or send it to the background; Termodoro continues tracking time with microsecond precision.

---

### Step 3: Handling Pauses & Reset Recovery
- **Need a quick pause?** Press `Space` to pause the countdown. The badge shifts to `[PAUSED]`.
- **Ready to resume?** Press `Space` again to pick up exactly where you left off.
- **Want a clean slate?** Press `r` to reset the active interval back to `25:00`.
- **Need to skip ahead?** Press `s` to immediately conclude the current phase and step to the next interval.

---

### Step 4: Completing a Work Phase & Acoustic Chimes
When the timer strikes `00:00`, Termodoro triggers its built-in feedback system:
1. 🎵 **Zen Tibetan Singing Bowl**: A pure in-memory 528 Hz harmonic chime plays softly with natural exponential decay.
2. 🔔 **Desktop Notification**: An OS native banner appears (*"Focus Session Completed! Take a well-deserved break."*).
3. 🍅 **Automated Accounting**: If an active task is selected, its spent Pomodoro counter increments automatically (`🍅 1 / 4`), and 25 focus minutes are logged to your permanent analytics history.
4. 🔄 **State Transition**: The phase automatically shifts to **Short Break** (5:00).

---

### Step 5: Taking a Restorative Short Break
- Press `Space` to begin your 5-minute break.
- Step away from your keyboard, hydrate, or stretch.
- When 5 minutes elapse, a dual-tone alert chime ($659.25\text{ Hz} \rightarrow 880\text{ Hz}$) signals that break time is over and readies you for Cycle 2.

---

### Step 6: Progressing Through 4 Cycles to the Long Break
As you complete successive sessions, the cycle progress indicators update dynamically:
- Cycle 1: `● ○ ○ ○` (1 focus block finished)
- Cycle 2: `● ● ○ ○` (2 focus blocks finished)
- Cycle 3: `● ● ● ○` (3 focus blocks finished)
- Cycle 4: `● ● ● ●` (4 focus blocks finished)

Upon finishing the 4th session, Termodoro automatically enters a **15-minute Long Break** with a rich major triad chord ($C_5 \text{ -- } E_5 \text{ -- } G_5$), providing deep mental restoration before resetting the cycle counter back to 1.

---

## 3. Visual Layout & Interface Deep Dive

Below is the live operational layout of Tab 1 during an active focus session:

![Termodoro Timer View](../assets/screenshots/01_timer_view.png)

### Anatomy of Tab 1:
1. **Header Navigation Bar**: Highlighting Tab `1: Timer` along with quick access tabs `2: Tasks`, `3: Stats`, `4: Settings`.
2. **Big Block Clock**: 5-row rasterized ASCII block numerals (`█`) rendered with TrueColor gradients.
3. **Phase Badge**: Color-coded indicator displaying current mode (`🎯 Work Phase`, `☕ Short Break`, `🌴 Long Break`).
4. **Cycle Progress Tracker**: Visual dot matrix (`● ● ○ ○`) tracking your progress toward the Long Break.
5. **Active Task Card**: Displays the title, progress (`🍅 2 / 4`), and estimation metrics of your currently bound target task.
6. **Progress Gauge**: Real-time progress bar rendering completion percentage.
7. **Status & Notification Footer**: Live keybinding helpers and real-time confirmation banners.

---

## 4. Pro Tips & Power Workflows

> [!TIP]
> **Floating Window Multiplexer Setup**: Keep Termodoro floating over your editor in `tmux` or `zellij`:
> ```bash
> # Zellij floating popup
> zellij run --floating --width 80% --height 80% -- termodoro
> ```

> [!NOTE]
> **Auto-Start Automation**: If you prefer seamless transitions without pressing `Space` after each break, navigate to **Settings (Tab 4)** and toggle **Auto-start Breaks** and **Auto-start Pomodoros** to `[Enabled]`.

---

## 5. Complete Keybinding Reference

| Keybinding | Action | Context / Behavior |
| :---: | :--- | :--- |
| `Space` | **Start / Pause** | Toggle active countdown state |
| `r` | **Reset** | Restore phase timer to default starting duration |
| `s` | **Skip** | Immediately complete current phase and advance |
| `a` | **Quick Add Task** | Open task creation modal without leaving Timer view |
| `1` - `4` | **Switch Tab** | Jump directly to Timer, Tasks, Stats, or Settings |
| `?` | **Help Overlay** | View interactive global keybinding cheat sheet |
| `q` / `Esc` | **Quit** | Save state atomically and exit |
