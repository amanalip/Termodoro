# User Journey 03: Analytics, Daily Streaks & Productivity Insights

Track your focus metrics, analyze 7-day productivity distribution charts, preserve multi-day habit streaks across calendar boundaries, and inspect your session activity logs in **Termodoro**.

---

## Table of Contents

1. [Journey Narrative & Persona](#1-journey-narrative--persona)
2. [Step-by-Step Interactive Walkthrough](#2-step-by-step-interactive-walkthrough)
   - [Step 1: Navigating to the Analytics Workstation](#step-1-navigating-to-the-analytics-workstation)
   - [Step 2: Inspecting Quantitative Metric Cards](#step-2-inspecting-quantitative-metric-cards)
   - [Step 3: Analyzing the 7-Day Activity Distribution Histogram](#step-3-analyzing-the-7-day-activity-distribution-histogram)
   - [Step 4: Building & Preserving Consecutive Day Streaks](#step-4-building--preserving-consecutive-day-streaks)
   - [Step 5: Reviewing the Chronological Session Activity Log](#step-5-reviewing-the-chronological-session-activity-log)
3. [Visual Layout & Interface Deep Dive](#3-visual-layout--interface-deep-dive)
4. [Streak Retention & Algorithm Guarantees](#4-streak-retention--algorithm-guarantees)
5. [Complete Keybinding Reference](#5-complete-keybinding-reference)

---

## 1. Journey Narrative & Persona

> **Meet Daniel, an Open Source Maintainer & Writer.**  
> Daniel wants to build a consistent daily coding habit without burnout. Rather than relying on cloud services that monetize personal productivity data, Daniel reviews Termodoro's local analytics at the end of each working day to celebrate his focus milestones and maintain a continuous habit streak.

---

## 2. Step-by-Step Interactive Walkthrough

### Step 1: Navigating to the Analytics Workstation
From any tab in Termodoro, press `3` to jump directly into **Tab 3: Stats View**.

---

### Step 2: Inspecting Quantitative Metric Cards
The top metrics banner gives you an instantaneous summary of your output:

1. ⏱️ **Total Focus Time**: Cumulative focused work time formatted as hours and minutes (e.g., `42h 30m` or `125m`).
2. 🍅 **Total Pomodoros**: Exact count of fully completed 25-minute focus intervals.
3. 🔥 **Current Streak**: Number of consecutive calendar days where at least one focus session was completed.
4. 🏆 **Longest Streak**: Your all-time personal best continuous daily streak record.

---

### Step 3: Analyzing the 7-Day Activity Distribution Histogram
The middle panel renders a vertical ASCII bar chart visualizing your focus output over the last 7 calendar days:
- **X-Axis**: Local calendar dates with day of the week labels (`Mon 11`, `Tue 12`, `Wed 13`, `Thu 14`, etc.).
- **Y-Axis**: Proportional vertical bars representing total focus minutes or completed Pomodoros on that day.
- Allows you to easily spot mid-week productivity peaks and balance your workload across days.

---

### Step 4: Building & Preserving Consecutive Day Streaks
Termodoro's streak tracker motivates continuous daily discipline:
- Complete at least **one** focus session on any calendar day to maintain your streak.
- If you worked yesterday, your streak is preserved and increments automatically as soon as today's first session concludes.
- If you skip a full 24-hour day, the streak resets gracefully to 1 on your next session, while your **Longest Streak** milestone is permanently locked in.

---

### Step 5: Reviewing the Chronological Session Activity Log
The bottom panel displays an activity history table detailing your recent sessions in reverse chronological order:
- **Timestamp**: Local completion time (`14:25 UTC` / `09:30 Local`).
- **Phase & Length**: Mode and duration logged (`Work (25m)`).
- **Target Task**: Title of the specific task associated with the session (e.g. `[⚡ Refactor Storage Engine]`).

---

## 3. Visual Layout & Interface Deep Dive

Below is the live operational layout of Tab 3:

![Termodoro Stats View](../assets/screenshots/03_stats_view.png)

### Key Analytics Layout Elements:
1. **Summary Cards Header**: 4 bordered visual blocks showing Focus Time, Pomodoros, Current Streak, and Personal Best.
2. **Weekly Bar Chart**: High-contrast Unicode bar columns (`█`) calibrated dynamically to your weekly maximum output.
3. **Session History Table**: Timestamped audit trail showing session attribution and durations.

---

## 4. Streak Retention & Algorithm Guarantees

Termodoro’s streak calculation engine (`src/stats.rs`) is mathematically certified across edge cases:
- **Timezone Continuity**: Uses `chrono::Local` calendar dates to prevent timezone drift.
- **Month & Year Boundaries**: Seamlessly bridges streaks across month transitions (e.g., Feb 28 $\rightarrow$ Mar 1) and New Year’s Eve (Dec 31 $\rightarrow$ Jan 1).
- **366-Day Leap Year Resilience**: Formally verified in automated tests for full leap-year date continuity.
- **Zero Break Contamination**: Break intervals (Short and Long Breaks) are strictly excluded from focus metrics, ensuring 100% genuine habit data.

---

## 5. Complete Keybinding Reference

| Keybinding | Action | Context / Behavior |
| :---: | :--- | :--- |
| `1` | **Switch to Timer** | Jump back to countdown timer view |
| `2` | **Switch to Tasks** | Jump to task management view |
| `3` | **Switch to Stats** | Refresh and view analytics |
| `4` | **Switch to Settings** | Open configuration and theme selector |
| `?` | **Help Overlay** | View global keybinding reference dialog |
| `q` / `Esc` | **Quit** | Save state and exit application |
