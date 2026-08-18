# User Journey 04: Customizing Durations, Acoustic Audio & 18 Color Themes

Personalize your focus environment in **Termodoro**: adjust interval durations, scale long break intervals up to 24 cycles, configure mathematical audio synthesis, and explore 18 handcrafted color themes.

---

## Table of Contents

1. [Journey Narrative & Persona](#1-journey-narrative--persona)
2. [Step-by-Step Interactive Walkthrough](#2-step-by-step-interactive-walkthrough)
   - [Step 1: Opening the Live Settings Workstation](#step-1-opening-the-live-settings-workstation)
   - [Step 2: Customizing Focus & Break Interval Durations](#step-2-customizing-focus--break-interval-durations)
   - [Step 3: Scaling the Long Break Cycle Ceiling (Up to 24 Cycles)](#step-3-scaling-the-long-break-cycle-ceiling-up-to-24-cycles)
   - [Step 4: Audio Synthesis & Desktop Notification Toggles](#step-4-audio-synthesis--desktop-notification-toggles)
   - [Step 5: Automated Flow with Auto-Start Transitions](#step-5-automated-flow-with-auto-start-transitions)
   - [Step 6: Real-Time Cycling Across 18 Handcrafted Color Themes](#step-6-real-time-cycling-across-18-handcrafted-color-themes)
3. [Visual Layout & Interface Deep Dive](#3-visual-layout--interface-deep-dive)
4. [Complete 18-Theme Palette Catalog](#4-complete-18-theme-palette-catalog)
5. [Complete Keybinding Reference](#5-complete-keybinding-reference)

---

## 1. Journey Narrative & Persona

> **Meet Sarah, a UI/UX Designer & Rust Hacker.**  
> Sarah loves tailoring her terminal aesthetic to match her mood and lighting conditions. During bright daylight, she switches to `CatppuccinLatte` or `SolarizedLight`; at night, she prefers `Synthwave84` or pure `#000000` pitch-black `OledPhosphor`. She also prefers longer 50-minute focus blocks with 10-minute breaks for deep design sprints.

---

## 2. Step-by-Step Interactive Walkthrough

### Step 1: Opening the Live Settings Workstation
From any tab in Termodoro, press `4` to enter **Tab 4: Settings View**.

---

### Step 2: Customizing Focus & Break Interval Durations
1. Use `j` / `k` (or `↑` / `↓` arrows) to select duration rows:
   - **Work Duration**: Default 25 min (configurable from $1 \le N \le 120\text{ min}$).
   - **Short Break Duration**: Default 5 min (configurable from $1 \le N \le 60\text{ min}$).
   - **Long Break Duration**: Default 15 min (configurable from $1 \le N \le 60\text{ min}$).
2. Use `h` / `l` (or `Left` / `Right` arrows, `-` / `+`) to increment or decrement the duration.
3. The active countdown clock on Tab 1 immediately recalibrates to your new duration!

---

### Step 3: Scaling the Long Break Cycle Ceiling (Up to 24 Cycles)
- Select row 4: **Long Break Interval**.
- Adjust the number of focus cycles before a Long Break is triggered (from **1 to 24 cycles**).
- Tab 1 automatically updates its visual cycle dots (`● ● ○ ○ ...`) to match your exact cycle configuration.

---

### Step 4: Audio Synthesis & Desktop Notification Toggles
- Select **Sound Enabled** or **Desktop Notifications**.
- Press `Space` or `Enter` to toggle `[Enabled]` / `[Disabled]`.
- **Sound Alert**: Generates clean, click-free mathematical PCM WAV audio ($f = 44.1\text{ kHz}$) directly in memory—no sound files or codecs required.
- **Desktop Notifications**: Dispatches native OS notifications that notify you even when your terminal window is minimized.

---

### Step 5: Automated Flow with Auto-Start Transitions
- **Auto-start Breaks**: Automatically begins the break countdown the moment your focus session completes.
- **Auto-start Pomodoros**: Automatically begins the next focus countdown when your break expires.

---

### Step 6: Real-Time Cycling Across 18 Handcrafted Color Themes
1. Navigate to row 9: **Color Theme**.
2. Press `l` / `Right` or `h` / `Left` to cycle through all 18 palettes.
3. The entire terminal user interface instantly repaints with the new palette without restarting the app!
4. Your theme choice is saved automatically to `data.json`.

---

## 3. Visual Layout & Interface Deep Dive

### Live Settings View Editor
![Termodoro Settings View](../assets/screenshots/04_settings_view.png)

### Global Keybinding Reference Modal (`?`)
![Termodoro Help Modal](../assets/screenshots/06_help_modal.png)

---

## 4. Complete 18-Theme Palette Catalog

Every theme in Termodoro is handcrafted with balanced WCAG contrast ratios and 24-bit TrueColor RGB values:

| # | Theme Identifier | Aesthetic Style | Accent Highlights |
| :-: | :--- | :--- | :--- |
| **1** | `CatppuccinMocha` | Modern Dark Pastel *(Default)* | Mauve, Peach & Lavender |
| **2** | `CatppuccinMacchiato` | Midtone Dark Pastel | Soft Lavender & Rose |
| **3** | `CatppuccinFrappe` | Low-Glare Soft Dark | Soft Slate Blue & Mauve |
| **4** | `CatppuccinLatte` | Crisp Clean Light Theme | Pastel Sky Blue & Maroon |
| **5** | `Nord` | Arctic Bluish Dark | Frost Cyan & Snow White |
| **6** | `GruvboxDark` | Warm Retro Groove | Warm Amber & Sage Green |
| **7** | `TokyoNight` | Neon Japanese Dark | Neon Blue & Electric Magenta |
| **8** | `Dracula` | High-Contrast Gothic Dark | Vibrant Purple & Cyan |
| **9** | `SolarizedDark` | Low-Contrast Designer Dark | Cyan & Soft Green |
| **10** | `SolarizedLight` | Warm Paper Designer Light | Cyan & Amber Brown |
| **11** | `RosePine` | Atmospheric Rosé & Pine | Rose Water & Gold Accent |
| **12** | `OneDark` | Atom Pro Dark Syntax | Electric Cyan & Spring Green |
| **13** | `Kanagawa` | Japanese Ukiyo-e Wave Dark | Autumn Amber & Wave Teal |
| **14** | `EverforestDark` | Nature-Inspired Organic Dark | Forest Green & Rust Orange |
| **15** | `EverforestLight` | Organic Warm Paper Light | Soft Leaf Green & Earth Brown |
| **16** | `Synthwave84` | Cyberpunk 1984 Laser Neon | Hot Neon Pink & Electric Cyan |
| **17** | `MonokaiPro` | Spectrum Filtered Dark | Sunshine Yellow & Vibrant Red |
| **18** | `OledPhosphor` | Pure Pitch Black #000000 | Classic CRT Phosphor Green |

---

## 5. Complete Keybinding Reference

| Keybinding | Action | Context / Behavior |
| :---: | :--- | :--- |
| `j` / `↓` | **Next Setting** | Move cursor down to next setting option |
| `k` / `↑` | **Previous Setting** | Move cursor up to previous setting option |
| `h` / `←` / `-` | **Decrease / Prev Theme** | Decrement duration value or cycle theme backward |
| `l` / `→` / `+` | **Increase / Next Theme** | Increment duration value or cycle theme forward |
| `Space` / `Enter` | **Toggle State** | Toggle boolean ON/OFF options |
| `1` - `4` | **Switch Tab** | Jump directly to Timer, Tasks, Stats, or Settings |
| `?` | **Help Overlay** | View interactive global keybinding reference dialog |
| `q` / `Esc` | **Quit** | Save preferences and exit cleanly |
