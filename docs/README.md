# Termodoro Documentation and Web Showcase

This directory contains the static website, interactive demonstration, feature tour, and FAQ guide for [Termodoro](https://github.com/amanalip/Termodoro). It is served directly via GitHub Pages at https://amanalip.github.io/Termodoro/.

The site is built with plain HTML5, CSS3, and ES6 JavaScript without external build tools or framework dependencies.

---

## Directory Structure

```
docs/
├── index.html              # Main landing page (showcase, theme gallery, installation, keybindings)
├── features.html           # Detailed screen-by-screen feature tour and audio test buttons
├── faqs.html               # 32 searchable questions and answers across 7 categories
├── style.css               # Vanilla CSS design system with all 18 compiled theme tokens
├── app.js                  # Theme engine, localStorage persistence, Web Audio synth, search filters
├── favicon.svg             # Totoro leaf-hat vector logo and favicon
├── README.md               # Maintainer documentation for the docs directory (this file)
└── assets/
    ├── logo.svg            # Vector project logo
    └── screenshots/        # Full vector SVG and PNG captures for documentation
        ├── 01_timer_view.svg
        ├── 02_tasks_view.svg
        ├── 02_task_manager.svg
        ├── 03_stats_view.svg
        ├── 04_settings_view.svg
        ├── 05_task_modal.svg
        └── 06_help_modal.svg
```

---

## Page Breakdown

### 1. index.html (Main Page)
- Screenshot showcase cycling through the 6 core application views with keyboard controls (keys 1 to 6).
- 18-theme gallery with real-time preview and instant site-wide theme switching.
- Copyable installation commands for Cargo, Homebrew, Arch AUR, and Nix.
- Keybindings table with live text search.
- Interactive 5x4 block digit clock matching `src/ui/digits.rs`.

### 2. features.html (Feature Tour)
- Detailed breakdown of all 6 application screens:
  1. Timer View: Block clock, phase indicators, progress bar, active task card.
  2. Tasks View: Priority backlog, Pomodoro estimate tracking, status filters.
  3. Stats View: Daily focus accumulation, 7-day bar chart, streak counter.
  4. Settings View: Focus/break durations, interval cycles, sound toggles, theme switcher.
  5. Task Modal: Inline task creation and editing with form validation.
  6. Help Modal: Global keybinding cheatsheet overlay.
- Web Audio test buttons that synthesize the 3 procedural transition cues.

### 3. faqs.html (Knowledge Base)
- 34 categorized questions and answers covering:
  1. Installation and Environment Setup (7 questions)
  2. Terminal Multiplexers and Shell Integration (4 questions)
  3. Timer and Pomodoro Mechanics (4 questions)
  4. Task Management and Target Tracking (4 questions)
  5. In-Memory Audio Engine and Notifications (3 questions)
  6. Data Storage, Privacy, and Architecture (4 questions)
  7. Rust, Ratatui, Cargo, and System Performance (8 questions)
- Real-time search filter and category buttons.
- Standard HTML `<details>` and `<summary>` accordion markup.

---

## Theme Engine and Color Palettes

All 18 color schemes from `src/theme.rs` are declared directly in `style.css` using `html[data-theme="..."]` CSS variables:

| Theme Key | Name | Background | Primary Accent |
| :--- | :--- | :--- | :--- |
| `catppuccin_mocha` | Catppuccin Mocha (Default) | `#1e1e2e` | `#89b4fa` |
| `catppuccin_macchiato` | Catppuccin Macchiato | `#24273a` | `#8aadf4` |
| `catppuccin_frappe` | Catppuccin Frappe | `#303446` | `#8caaee` |
| `catppuccin_latte` | Catppuccin Latte (Light) | `#eff1f5` | `#1e66f5` |
| `nord` | Nord | `#2e3440` | `#88c0d0` |
| `gruvbox_dark` | Gruvbox Dark | `#282828` | `#fabd2f` |
| `tokyo_night` | Tokyo Night | `#1a1b26` | `#7aa2f7` |
| `dracula` | Dracula | `#282a36` | `#bd93f9` |
| `rose_pine` | Rose Pine | `#191724` | `#9ccfd8` |
| `one_dark` | One Dark | `#282c34` | `#61afef` |
| `kanagawa` | Kanagawa | `#1f1f28` | `#7e9cd8` |
| `everforest_dark` | Everforest Dark | `#2d353b` | `#7fbbb3` |
| `everforest_light` | Everforest Light | `#fdf6e3` | `#3a9486` |
| `solarized_dark` | Solarized Dark | `#002b36` | `#268bd2` |
| `solarized_light` | Solarized Light | `#fdf6e3` | `#268bd2` |
| `synthwave84` | Synthwave 84 | `#262335` | `#36f9f6` |
| `monokai_pro` | Monokai Pro | `#2d2a2e` | `#78dce8` |
| `oled_phosphor` | OLED Phosphor | `#000000` | `#00ff66` |

The selected theme is saved to `localStorage` under `termodoro_theme`. A small inline script in the `<head>` of each HTML file reads this value before the DOM renders, preventing any flash of the wrong theme when navigating between pages.

A sticky dropdown selector in the navigation bar on every page allows changing the theme at any time.

---

## Audio Synthesis Specifications

The Web Audio synthesizer in `app.js` mirrors the Rust audio engine in `src/audio.rs`:

1. Focus Session End (Zen Bowl Chime):
   - Fundamental frequency: 528 Hz
   - Harmonic overtones: 1056 Hz (second harmonic) and 1584 Hz (third harmonic)
   - Exponential amplitude decay over 3.0 seconds
2. Short Break End (Ascending Two-Tone):
   - Tone 1: 587.33 Hz (D5) for 180 ms
   - Tone 2: 880.00 Hz (A5) for 350 ms
3. Long Break End (Major Triad Arpeggio):
   - C5 (523.25 Hz), E5 (659.25 Hz), G5 (783.99 Hz) in sequence over 1.2 seconds

---

## Local Development and Preview

To view the website locally, run any static file server from the repository root:

```bash
# Python
python3 -m http.server 8000 --directory docs/

# Node.js
npx serve docs/

# Rust
cargo install basic-http-server
basic-http-server docs/
```

Open `http://localhost:8000` in your web browser.

---

## Automated Cross-Device Playwright E2E & Sanity Testing

The documentation portal includes an automated **Playwright End-to-End Testing Suite** and a **Sanity & Fact-Checking Engine** to continuously verify responsiveness, component interactivity, zero-overflow geometry, theme engine functionality, and exact correspondence with the Rust source code:

```bash
# Run the automated Playwright E2E test suite (41/41 passing tests)
make test-e2e

# Run the 81-assertion source sanity and fact-checking audit
make check-facts
```

### Viewport Matrix & Automated Verifications (41/41 Tests Passing)

| Viewport Profile | Target Resolution | Simulated Device Profiles |
| :--- | :---: | :--- |
| **Desktop Large** | `1920 × 1080` | Ultra-wide displays & large desktop monitors |
| **Desktop Standard** | `1280 × 800` | MacBook Air/Pro, laptops |
| **Tablet Portrait** | `768 × 1024` | Apple iPad, Android Tablets |
| **Mobile Flagship** | `390 × 844` | iPhone 14/15/16, Google Pixel 7/8 |
| **Mobile Medium** | `375 × 667` | iPhone SE (2nd/3rd gen), Galaxy A series |
| **Mobile Small** | `320 × 568` | iPhone 5/SE (1st gen), Compact screens |

**Automated Test Verifications:**
1. **Zero Horizontal Overflow**: Guarantees `scrollWidth <= clientWidth` across all 6 viewports on all 3 pages (`index.html`, `features.html`, `faqs.html`).
2. **Mobile Navigation Drawer**: Tests hamburger toggle button, slide-in drawer animation, backdrop overlay blur, link navigation, close button, and `Esc` key dismissal.
3. **Code Cards & Fixed Copy Buttons**: Verifies `.code-card-header` bar layout, prevents copy button from floating or overlapping code during horizontal scrolling, tests copy click feedback (`✓ Copied!`), and verifies toast notification triggers.
4. **18-Theme Palette Engine**: Validates theme switching via navbar dropdown `<select>` and gallery card clicks, testing dynamic CSS custom properties and pure pitch black `#000000` tokens for OLED Phosphor.
5. **Interactive Screenshot Showcase**: Tests tab switching across all 6 views and numerical keyboard navigation (`1`–`6`).
6. **Keybindings & FAQ Search**: Verifies real-time table row filtering and FAQ accordion query matching.
7. **Source Fact-Check Engine (`make check-facts`)**: Programmatically verifies all 18 theme enum variants, audio frequencies, configuration bounds, and installation cards against `src/`.

---

## Deployment

Changes pushed to the `main` branch trigger `.github/workflows/static.yml`. The workflow:
1. Runs verification checks to confirm all HTML files, stylesheets, scripts, and screenshot assets exist.
2. Packages the `docs/` folder.
3. Deploys the static bundle to GitHub Pages.

---

## Maintenance Notes

- Keep the codebase vanilla. Do not add JavaScript frameworks, bundlers, or third-party tracking scripts.
- If you add or modify a theme in `src/theme.rs`, update both `THEMES` in `docs/app.js` and the corresponding `html[data-theme="..."]` rules in `docs/style.css`.
- When making substantial CSS or JS changes, increment the version query parameter (for example, `style.css?v=...`) across `index.html`, `features.html`, and `faqs.html` to avoid stale browser cache issues.
- Always execute `make test-e2e` and `make check-facts` to verify zero horizontal overflow, mobile responsiveness, and source truth before committing changes to `docs/`.
