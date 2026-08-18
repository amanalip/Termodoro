# 🍅 Termodoro — Documentation & Web App Hub

[![Deploy static content to Pages](https://github.com/amanalip/Termodoro/actions/workflows/static.yml/badge.svg)](https://github.com/amanalip/Termodoro/actions/workflows/static.yml)
[![Live Site](https://img.shields.io/badge/Live%20Website-GitHub%20Pages-blue?logo=github&style=flat-square)](https://amanalip.github.io/Termodoro/)
[![Built with Pure Vanilla Web](https://img.shields.io/badge/Stack-HTML5%20%7C%20Vanilla%20CSS%20%7C%20ES6%20JS-F16529?style=flat-square)](https://amanalip.github.io/Termodoro/)

This directory (`docs/`) hosts the static website, interactive demonstration showcase, deep-dive feature guides, and comprehensive FAQs hub for **[Termodoro](https://github.com/amanalip/Termodoro)**, published automatically via **GitHub Pages**.

---

## 📁 Directory Architecture

```
docs/
├── index.html              # Main interactive portal (Showcase, Theme Gallery, Quickstart, Keybindings)
├── features.html           # In-depth 6-screen feature tour with Web Audio test buttons
├── faqs.html               # 32-question categorized knowledge base with instant search
├── style.css               # Vanilla CSS design system with all 18 compiled theme tokens
├── app.js                  # Theme engine, localStorage sync, Web Audio synth & search controllers
├── favicon.svg             # Vector Totoro Leaf-Hat logo & site favicon
├── README.md               # Directory documentation & maintainer guide (this file)
└── assets/
    ├── logo.svg            # High-resolution vector project logo
    └── screenshots/        # Full vector SVG & PNG screen captures
        ├── 01_timer_view.svg
        ├── 02_tasks_view.svg
        ├── 03_stats_view.svg
        ├── 04_settings_view.svg
        ├── 05_task_modal.svg
        └── 06_help_modal.svg
```

---

## 🌐 Pages Overview

### 1. [`index.html`](index.html) — Main Portal
- **Interactive Screenshot Showcase**: Tabbed viewer cycling through all 6 screens with keyboard navigation (<kbd>1</kbd>–<kbd>6</kbd>).
- **18-Theme Interactive Gallery**: Live preview cards allowing instant site-wide theme switching.
- **One-Click Installation Tabs**: Copy-to-clipboard recipes for Cargo, Homebrew, Arch AUR, and Nix.
- **Searchable Keybindings Cheatsheet**: Real-time keyword filter across Global, Timer, Task, and Modal shortcuts.
- **Interactive Block Digits**: 5x4 ASCII/Unicode glyph rasterizer matching `src/ui/digits.rs`.

### 2. [`features.html`](features.html) — Deep-Dive Feature Tour
- Dedicated high-resolution cards explaining all 6 views:
  1. **Dual-Ring Timer View**: 5x4 block clock, phase badges, and active target card.
  2. **Task Backlog & Priority Matrix**: Estimations, completion blocks, and status filters.
  3. **Productivity Analytics**: Daily focus metrics, 7-day histograms, and streak tracking.
  4. **Customizable Settings**: Granular phase durations, auto-transitions, and themes.
  5. **Modal Dialogs**: Accessible task editor with validation and priority toggles.
  6. **Help & Shortcuts**: Universal keyboard reference overlay.
- **Web Audio Soundboard**: Test buttons replicating the 3 procedural WAV transition cues.

### 3. [`faqs.html`](faqs.html) — 32-Question Knowledge Base
- **Instant Search**: Real-time keyword filtering that dynamically matches question titles and body text.
- **7 Filter Categories**:
  1. `📦 Installation & Environment Setup` (5 FAQs)
  2. `🪟 Terminal Multiplexers & Shell Integration` (4 FAQs)
  3. `⏱️ Timer & Pomodoro Mechanics` (4 FAQs)
  4. `🎯 Task Management & Target Tracking` (4 FAQs)
  5. `🔔 In-Memory Audio Engine & Notifications` (3 FAQs)
  6. `🔒 Data Storage, Privacy & Architecture` (4 FAQs)
  7. `🦀 Rust, Ratatui, Cargo & System Performance` (8 FAQs)
- **Accessible Accordions**: `<details>` / `<summary>` implementation with smooth CSS chevron animations.

---

## 🎨 Theme Engine & 18 Color Palettes

All 18 color schemes from [`src/theme.rs`](../src/theme.rs) are compiled directly into [`style.css`](style.css) via `html[data-theme="..."]` custom properties and synchronized through `localStorage` (`termodoro_theme`):

| Theme Key | Palette Name | Background | Primary Accent |
| :--- | :--- | :--- | :--- |
| `catppuccin_mocha` | **Catppuccin Mocha** (Default) | `#1e1e2e` | `#89b4fa` (Blue) |
| `catppuccin_macchiato` | **Catppuccin Macchiato** | `#24273a` | `#8aadf4` (Blue) |
| `catppuccin_frappe` | **Catppuccin Frappé** | `#303446` | `#8caaee` (Blue) |
| `catppuccin_latte` | **Catppuccin Latte** (Light) | `#eff1f5` | `#1e66f5` (Blue) |
| `nord` | **Nord** | `#2e3440` | `#88c0d0` (Frost Cyan) |
| `gruvbox_dark` | **Gruvbox Dark** | `#282828` | `#fabd2f` (Yellow) |
| `tokyo_night` | **Tokyo Night** | `#1a1b26` | `#7aa2f7` (Blue) |
| `dracula` | **Dracula** | `#282a36` | `#bd93f9` (Purple) |
| `rose_pine` | **Rose Pine** | `#191724` | `#9ccfd8` (Foam) |
| `one_dark` | **One Dark** | `#282c34` | `#61afef` (Blue) |
| `kanagawa` | **Kanagawa** | `#1f1f28` | `#7e9cd8` (Wave Blue) |
| `everforest_dark` | **Everforest Dark** | `#2d353b` | `#7fbbb3` (Aqua) |
| `everforest_light` | **Everforest Light** | `#fdf6e3` | `#3a9486` (Aqua) |
| `solarized_dark` | **Solarized Dark** | `#002b36` | `#268bd2` (Blue) |
| `solarized_light` | **Solarized Light** | `#fdf6e3` | `#268bd2` (Blue) |
| `synthwave84` | **Synthwave '84** | `#262335` | `#36f9f6` (Cyan Neon) |
| `monokai_pro` | **Monokai Pro** | `#2d2a2e` | `#78dce8` (Cyan) |
| `oled_phosphor` | **OLED Phosphor** | `#000000` | `#00ff66` (Terminal Green) |

---

## 🔔 Web Audio Procedural Synthesizer

The client-side JavaScript engine in [`app.js`](app.js) faithfully replicates Termodoro's native Rust in-memory PCM WAV generator ([`src/audio.rs`](../src/audio.rs)):

1. **Focus Session End (528 Hz Zen Bowl)**:
   $$\text{Fundamental: } 528\text{ Hz}, \quad \text{Overtones: } 1056\text{ Hz } (2f_0), \; 1584\text{ Hz } (3f_0)$$
   Synthesizes an acoustic Tibetan singing bowl with an exponential decay curve ($A(t) = A_0 e^{-\lambda t}$).
2. **Short Break End (D5 $\rightarrow$ A5 Chime)**:
   $$\text{Tone 1: } 587.33\text{ Hz (D5 for 180ms)}, \quad \text{Tone 2: } 880.00\text{ Hz (A5 for 350ms)}$$
   Ascending musical interval signaling readiness to refocus.
3. **Long Break End (Major Triad Arpeggio)**:
   $$\text{C5: } 523.25\text{ Hz } \longrightarrow \text{E5: } 659.25\text{ Hz } \longrightarrow \text{G5: } 783.99\text{ Hz}$$
   Celebratory chord arpeggio concluding the 4-interval cycle.

---

## 🛠️ Local Development & Testing

To test and preview the website locally without any build tools:

### Option A: Python HTTP Server (Recommended)
```bash
# From the repository root
python3 -m http.server 8000 --directory docs/
```
Open **`http://localhost:8000`** in your browser.

### Option B: Node.js `serve`
```bash
npx -y serve docs/
```

### Option C: Rust `basic-http-server`
```bash
cargo install basic-http-server
basic-http-server docs/
```

---

## 🚀 CI/CD & Deployment Pipeline

Every commit pushed to the `main` branch triggers the [`.github/workflows/static.yml`](../.github/workflows/static.yml) workflow:

1. **Validation Step**:
   - Asserts the existence and integrity of `index.html`, `features.html`, `faqs.html`, `style.css`, `app.js`, and all 6 screenshot assets.
2. **Artifact Packaging**:
   - Uploads `docs/` as the GitHub Pages artifact bundle.
3. **Deployment**:
   - Deploys the static site to the official GitHub Pages endpoint at **`https://amanalip.github.io/Termodoro/`**.

---

## 🤝 Contributing to Documentation

When updating documentation or styling:
1. **Zero External Dependencies**: Keep the website pure Vanilla HTML5, CSS3, and ES6 JavaScript. Avoid pulling large frontend frameworks or heavy external CDNs.
2. **Synchronize Themes**: If new themes are added to `src/theme.rs`, ensure corresponding entries are added to `THEMES` in `docs/app.js` and `docs/style.css`.
3. **Cache Busting**: When updating core scripts or stylesheets, bump the version query parameter (e.g. `style.css?v=...`) across all HTML files.
4. **Accessible Semantics**: Ensure all new interactive controls include appropriate ARIA roles, labels, and keyboard focus states.
