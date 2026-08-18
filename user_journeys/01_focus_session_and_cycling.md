# User Journey 01: Core Pomodoro Focus Session & Interval Cycling

This guide walks you through the primary user flow in **Termodoro**: running a focused work session, managing interval transitions, and taking structured short and long breaks.

---

## Table of Contents

1. [Overview & Objective](#1-overview--objective)
2. [Step-by-Step Walkthrough](#2-step-by-step-walkthrough)
   - [Step 1: Launching Termodoro](#step-1-launching-termodoro)
   - [Step 2: Starting Your Focus Countdown](#step-2-starting-your-focus-countdown)
   - [Step 3: Pausing & Resuming](#step-3-pausing--resuming)
   - [Step 4: Completing a Work Phase & Acoustic Chimes](#step-4-completing-a-work-phase--acoustic-chimes)
   - [Step 5: Taking a Short Break](#step-5-taking-a-short-break)
   - [Step 6: Progressing through 4 Cycles to Long Break](#step-6-progressing-through-4-cycles-to-long-break)
3. [Visual Reference & Layout](#3-visual-reference--layout)
4. [Quick Keybindings Cheat Sheet](#4-quick-keybindings-cheat-sheet)

---

## 1. Overview & Objective

The core premise of the Pomodoro Technique is breaking complex work into 25-minute uninterrupted focus intervals separated by structured rest periods. Termodoro makes this workflow completely seamless in the terminal.

---

## 2. Step-by-Step Walkthrough

### Step 1: Launching Termodoro
Open your favorite terminal emulator and type:

```bash
termodoro
```

Termodoro launches instantly into **Tab 1: Timer View**. By default, it initializes at **Cycle 1 of 4** in the **Work** phase with a 25-minute countdown.

### Step 2: Starting Your Focus Countdown
- Press `Space` to start the timer.
- The status indicator switches from `[STOPPED]` to `[RUNNING]`.
- The 5-row digital block clock counts down second by second.
- The progress bar at the bottom fills as you advance.

### Step 3: Pausing & Resuming
- Need to answer an urgent interruption? Press `Space` to pause.
- The status banner displays `[PAUSED]`.
- Press `Space` again to resume focus right where you left off.
- To reset the phase to 25:00, press `r`.

### Step 4: Completing a Work Phase & Acoustic Chimes
- When 25 minutes elapse, Termodoro automatically:
  1. Plays the soothing **528 Hz Zen Singing Bowl** chime.
  2. Dispatches a native desktop notification (*"Focus Session Completed!"*).
  3. Increments your active task's spent pomodoro count (if assigned).
  4. Automatically records 25 focus minutes to your analytics dashboard.
  5. Transitions to the **Short Break** phase.

### Step 5: Taking a Short Break
- Press `Space` to start your 5-minute break.
- Step away from the screen, stretch, or grab water.
- At the end of 5 minutes, a two-tone chime alerts you that break time is over.

### Step 6: Progressing through 4 Cycles to Long Break
- The cycle progress dots below the clock show your position:
  - `● ○ ○ ○` (Cycle 1 complete)
  - `● ● ○ ○` (Cycle 2 complete)
  - `● ● ● ○` (Cycle 3 complete)
  - `● ● ● ●` (Cycle 4 complete)
- Upon completing the 4th Work session, Termodoro automatically triggers a **15-minute Long Break** with a celebratory major triad chime.

---

## 3. Visual Reference & Layout

![Termodoro Timer View](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/01_timer_view.png)

### Key Interface Elements on Tab 1:
- **Header**: Shows current tab navigation (`1: Timer`, `2: Tasks`, `3: Stats`, `4: Settings`).
- **Big Digital Clock**: Monospace 5x3 rasterized ASCII digits displaying `MM:SS`.
- **Phase Banner**: Visual badges showing `🎯 Work Phase (Cycle 1/4)`.
- **Active Task Card**: Highlights the currently targeted task name and progress.
- **Progress Gauge**: Proportional completion bar.

---

## 4. Quick Keybindings Cheat Sheet

| Key | Action | Description |
| :---: | :--- | :--- |
| `Space` | **Start / Pause** | Toggle countdown state |
| `r` | **Reset Phase** | Reset timer to full duration |
| `s` | **Skip Phase** | Immediately jump to the next interval |
| `?` | **Help Dialog** | View all keyboard shortcuts |
| `q` | **Quit** | Save state and exit cleanly |
