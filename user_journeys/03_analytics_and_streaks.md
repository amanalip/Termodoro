# User Journey 03: Analytics, Daily Streaks & Productivity Insights

This guide walks you through the analytics, streak retention algorithms, and productivity retrospective workflows in **Termodoro**.

---

## Table of Contents

1. [Overview & Objective](#1-overview--objective)
2. [Step-by-Step Walkthrough](#2-step-by-step-walkthrough)
   - [Step 1: Navigating to Analytics View](#step-1-navigating-to-analytics-view)
   - [Step 2: Inspecting Core Productivity Metric Cards](#step-2-inspecting-core-productivity-metric-cards)
   - [Step 3: Understanding the 7-Day Activity Distribution Bar Chart](#step-3-understanding-the-7-day-activity-distribution-bar-chart)
   - [Step 4: Building & Preserving Consecutive Day Streaks](#step-4-building--preserving-consecutive-day-streaks)
   - [Step 5: Reviewing the Chronological Session Activity Log](#step-5-reviewing-the-chronological-session-activity-log)
3. [Visual Reference & Layout](#3-visual-reference--layout)
4. [Streak Calculation Rules & Guarantees](#4-streak-calculation-rules--guarantees)

---

## 1. Overview & Objective

Sustainable focus requires feedback and habit momentum. Termodoro automatically aggregates your focus sessions into comprehensive analytics cards, multi-day streak counters, and a 7-day activity bar chart—all calculated locally on your machine with zero cloud dependencies.

---

## 2. Step-by-Step Walkthrough

### Step 1: Navigating to Analytics View
From anywhere in Termodoro, press `3` to jump directly to **Tab 3: Stats View**.

### Step 2: Inspecting Core Productivity Metric Cards
The top row of Tab 3 presents four key quantitative metrics:

1. **⏱️ Total Focus Time**: Formatted aggregate focus hours and minutes (e.g., `42h 30m` or `125m`).
2. **🍅 Total Pomodoros**: Total number of completed 25-minute focus intervals.
3. **🔥 Current Streak**: Number of consecutive calendar days you have completed at least one focus session.
4. **🏆 Longest Streak**: Your all-time personal best continuous daily focus streak.

### Step 3: Understanding the 7-Day Activity Distribution Bar Chart
The middle panel renders a vertical ASCII bar chart visualizing your focus output over the last 7 calendar days:
- Each column corresponds to a day (`Mon 11`, `Tue 12`, `Wed 13`, etc.).
- The height of the bar corresponds to total minutes or pomodoros completed on that date.
- Helps you identify weekly productivity trends and cadence patterns.

### Step 4: Building & Preserving Consecutive Day Streaks
- Complete at least **one** focus session each calendar day to maintain your streak.
- Termodoro’s streak algorithm checks whether you recorded a session today or yesterday:
  - If your last session was yesterday, your streak is preserved and increments as soon as today's first session completes.
  - If more than 48 hours elapse without a session, the current streak cleanly resets to 1 upon your next session, while preserving your **Longest Streak** record permanently.

### Step 5: Reviewing the Chronological Session Activity Log
The bottom panel lists your most recent focus sessions in reverse chronological order:
- **Timestamp**: Time of completion (`14:25 UTC`).
- **Phase & Duration**: `Work (25m)`.
- **Target Task**: Title of the task bound during that session (e.g. `[🚀 Implement OAuth]`).

---

## 3. Visual Reference & Layout

![Termodoro Stats View](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/03_stats_view.png)

---

## 4. Streak Calculation Rules & Guarantees

Termodoro's streak engine (`src/stats.rs`) is formally tested against complex boundary conditions:
- **Timezone Resilience**: Calculations anchor to local calendar dates using `chrono::NaiveDate`.
- **Month & Year Boundaries**: Seamlessly transitions streaks across Dec 31 $\rightarrow$ Jan 1.
- **Leap Year Calculations**: Fully tested across 366-day leap year cycles (e.g., Feb 28 $\rightarrow$ Feb 29 $\rightarrow$ Mar 1).
- **Zero Break Pollution**: Short and Long Breaks are never counted as focus work, guaranteeing 100% genuine productivity records.
