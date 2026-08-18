# User Journey 04: Customizing Durations, Acoustic Audio & 18 Color Themes

This guide walks you through customizing your experience in **Termodoro**: configuring interval durations, adjusting audio/notification preferences, auto-start workflows, and cycling through all 18 built-in dark and light themes.

---

## Table of Contents

1. [Overview & Objective](#1-overview--objective)
2. [Step-by-Step Walkthrough](#2-step-by-step-walkthrough)
   - [Step 1: Navigating to Preferences View](#step-1-navigating-to-preferences-view)
   - [Step 2: Adjusting Focus & Break Durations](#step-2-adjusting-focus--break-durations)
   - [Step 3: Configuring the Long Break Cycle Interval (Up to 24)](#step-3-configuring-the-long-break-cycle-interval-up-to-24)
   - [Step 4: Toggling Audio Chimes & Desktop Notifications](#step-4-toggling-audio-chimes--desktop-notifications)
   - [Step 5: Enabling Automated Phase Transitions](#step-5-enabling-automated-phase-transitions)
   - [Step 6: Choosing from 18 Handcrafted Color Themes](#step-6-choosing-from-18-handcrafted-color-themes)
3. [Visual References & Layouts](#3-visual-references--layouts)
4. [Complete 18-Theme Catalog](#4-complete-18-theme-catalog)
5. [Settings View Navigation Keys](#5-settings-view-navigation-keys)

---

## 1. Overview & Objective

Every developer and knowledge worker has different focus rhythms and aesthetic preferences. Termodoro includes a live Settings editor that allows you to tune every parameter of the timer and switch between 18 distinct visual themes in real time without editing configuration files or restarting the app.

---

## 2. Step-by-Step Walkthrough

### Step 1: Navigating to Preferences View
From any tab in Termodoro, press `4` to enter **Tab 4: Settings View**.

### Step 2: Adjusting Focus & Break Durations
1. Use `j` / `k` (or `Up` / `Down` arrows) to select:
   - **Work Duration**: Default 25 minutes ($1 \le N \le 120$).
   - **Short Break Duration**: Default 5 minutes ($1 \le N \le 60$).
   - **Long Break Duration**: Default 15 minutes ($1 \le N \le 60$).
2. Press `h` / `l` (or `Left` / `Right` arrows, `-` / `+`) to increment or decrement values.
3. The countdown timer on Tab 1 updates immediately!

### Step 3: Configuring the Long Break Cycle Interval (Up to 24)
- Select row 4: **Long Break Interval**.
- Adjust the number of cycles required before a long break triggers (default is 4, scalable from **1 up to 24 cycles**).
- Tab 1 automatically adjusts its cycle progress dots (`● ○ ○ ○ ...`) to match your custom interval.

### Step 4: Toggling Audio Chimes & Desktop Notifications
- Select **Sound Enabled** or **Desktop Notifications**.
- Press `Space`, `Enter`, `h`, or `l` to toggle `[ON]` / `[OFF]`.
- When Sound is enabled, Termodoro plays mathematical synthesized chimes at phase boundaries.
- When Desktop Notifications are enabled, native OS toast notifications alert you even when your terminal window is minimized or behind other apps.

### Step 5: Enabling Automated Phase Transitions
- **Auto-start Breaks**: When enabled, the break countdown starts immediately when a focus session ends.
- **Auto-start Pomodoros**: When enabled, focus countdown starts automatically when break time expires.

### Step 6: Choosing from 18 Handcrafted Color Themes
1. Navigate to row 9: **Color Theme**.
2. Press `l` / `Right` or `h` / `Left` to cycle through all 18 palettes.
3. The UI immediately repaints with the newly selected theme colors!
4. Settings, tasks, and analytics are saved atomically to disk on every change.

---

## 3. Visual References & Layouts

### Live Settings View Editor
![Termodoro Settings View](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/04_settings_view.png)

### Global Help & Keybinding Modal (`?`)
![Termodoro Help Modal](file:///home/amanap/Documents/GitHub/Termodoro/assets/screenshots/06_help_modal.png)

---

## 4. Complete 18-Theme Catalog

Termodoro includes 18 built-in palettes calibrated for high contrast and visual comfort:

| # | Theme Identifier | Style / Aesthetic | Primary Accent |
| :-: | :--- | :--- | :--- |
| **1** | `CatppuccinMocha` | Default Modern Dark Pastel | Mauve / Peach / Lavender |
| **2** | `CatppuccinMacchiato` | Midtone Dark Pastel | Soft Lavender & Rose |
| **3** | `CatppuccinFrappe` | Low-Glare Soft Dark | Soft Blue & Mauve |
| **4** | `CatppuccinLatte` | Crisp Clean Light | Pastel Blue & Maroon |
| **5** | `Nord` | Arctic Bluish Dark | Frost Cyan & Snow White |
| **6** | `GruvboxDark` | Warm Retro Groove | Warm Amber & Sage Green |
| **7** | `TokyoNight` | Japanese Neon Dark | Neon Blue & Magenta |
| **8** | `Dracula` | High-Contrast Gothic Dark | Vibrant Purple & Cyan |
| **9** | `SolarizedDark` | Designer Low-Contrast Dark | Cyan & Soft Green |
| **10** | `SolarizedLight` | Warm Paper Designer Light | Cyan & Amber Brown |
| **11** | `RosePine` | Atmospheric Rosé & Pine | Rose & Gold Accent |
| **12** | `OneDark` | Atom Pro Dark Syntax | Electric Cyan & Green |
| **13** | `Kanagawa` | Japanese Ukiyo-e Wave Dark | Autumn Amber & Wave Teal |
| **14** | `EverforestDark` | Nature-Inspired Organic Dark | Forest Green & Rust Orange |
| **15** | `EverforestLight` | Organic Warm Paper Light | Soft Leaf Green & Earth Brown |
| **16** | `Synthwave84` | Cyberpunk 1984 Neon Glow | Neon Hot Pink & Cyan |
| **17** | `MonokaiPro` | Spectrum Filtered Dark | Sunshine Yellow & Vibrant Red |
| **18** | `OledPhosphor` | Pure Pitch Black #000000 | Classic CRT Terminal Phosphor Green |

---

## 5. Settings View Navigation Keys

| Key | Action | Description |
| :---: | :--- | :--- |
| `j` / `↓` | **Next Setting** | Move cursor down to next setting row |
| `k` / `↑` | **Previous Setting** | Move cursor up to previous setting row |
| `h` / `←` / `-` | **Decrease / Prev Theme** | Decrement duration or cycle theme backward |
| `l` / `→` / `+` | **Increase / Next Theme** | Increment duration or cycle theme forward |
| `Space` / `Enter` | **Toggle State** | Toggle boolean ON/OFF settings |
| `1` - `3` | **Switch Tab** | Jump to Timer, Tasks, or Stats |
| `?` | **Help Dialog** | View all global keybindings |
