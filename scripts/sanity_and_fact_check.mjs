/**
 * Comprehensive Sanity & Fact-Checking Audit Script for Termodoro Web Documentation
 * Cross-examines all website HTML, CSS, JS against actual Rust source code.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT_DIR = path.resolve(__dirname, '..');
const DOCS_DIR = path.resolve(ROOT_DIR, 'docs');
const SRC_DIR = path.resolve(ROOT_DIR, 'src');

let passCount = 0;
let failCount = 0;

function check(title, condition, extraInfo = '') {
  if (condition) {
    console.log(`  ✅ [PASS] ${title}`);
    passCount++;
  } else {
    console.error(`  ❌ [FAIL] ${title} ${extraInfo ? `-> ${extraInfo}` : ''}`);
    failCount++;
  }
}

console.log('🔍 Starting Termodoro Full Sanity & Fact-Checking Audit...\n');

// --------------------------------------------------------------------------
// 1. Rust Source Invariants Verification
// --------------------------------------------------------------------------
console.log('🦀 1. Auditing Rust Source Invariants vs Web Claims');

const themeRs = fs.readFileSync(path.join(SRC_DIR, 'theme.rs'), 'utf8');
const configRs = fs.readFileSync(path.join(SRC_DIR, 'config.rs'), 'utf8');
const audioRs = fs.readFileSync(path.join(SRC_DIR, 'audio.rs'), 'utf8');
const appRs = fs.readFileSync(path.join(SRC_DIR, 'app.rs'), 'utf8');
const timerRs = fs.readFileSync(path.join(SRC_DIR, 'timer.rs'), 'utf8');
const storageRs = fs.readFileSync(path.join(SRC_DIR, 'storage.rs'), 'utf8');

// A. 18 Themes in theme.rs
const themeVariants = [
  'CatppuccinMocha', 'CatppuccinMacchiato', 'CatppuccinFrappe', 'CatppuccinLatte',
  'Nord', 'GruvboxDark', 'TokyoNight', 'Dracula', 'RosePine', 'OneDark',
  'Kanagawa', 'EverforestDark', 'EverforestLight', 'SolarizedDark',
  'SolarizedLight', 'Synthwave84', 'MonokaiPro', 'OledPhosphor'
];

themeVariants.forEach(variant => {
  check(`Theme choice enum variant '${variant}' exists in src/theme.rs`, themeRs.includes(variant));
});

// B. Audio Frequencies
// Each frequency is checked with a word-boundary regex so that, for example,
// '528.0' cannot be satisfied by an unrelated literal like '1528.0' or
// '528.05'. These are the exact constants used by the chime synthesizers.
const AUDIO_FREQUENCIES = [
  { label: 'Focus chime fundamental 528.0 Hz', pattern: /\b528\.0\b/ },
  { label: 'Focus chime 2nd harmonic 1056.0 Hz', pattern: /\b1056\.0\b/ },
  { label: 'Focus chime 3rd harmonic 1584.0 Hz', pattern: /\b1584\.0\b/ },
  { label: 'Short break chime D5 587.33 Hz', pattern: /\b587\.33\b/ },
  { label: 'Short break chime A5 880.0 Hz', pattern: /\b880\.0\b/ },
  { label: 'Long break chime C5 523.25 Hz', pattern: /\b523\.25\b/ },
  { label: 'Long break chime E5 659.25 Hz', pattern: /\b659\.25\b/ },
  { label: 'Long break chime G5 783.99 Hz', pattern: /\b783\.99\b/ }
];

AUDIO_FREQUENCIES.forEach(({ label, pattern }) => {
  check(`${label} in src/audio.rs`, pattern.test(audioRs));
});

// C. Settings Defaults & Boundaries
// Defaults must be attached to their actual field names in the Default impl of
// Config (src/config.rs), not matched as bare substrings anywhere in the file.
const CONFIG_DEFAULTS = [
  { label: 'Default work duration 25 mins', field: 'work_duration_mins', value: 25 },
  { label: 'Default short break 5 mins', field: 'short_break_mins', value: 5 },
  { label: 'Default long break 15 mins', field: 'long_break_mins', value: 15 },
  { label: 'Default long break interval 4', field: 'long_break_interval', value: 4 }
];

CONFIG_DEFAULTS.forEach(({ label, field, value }) => {
  // Matches e.g. `work_duration_mins: 25` inside the Default impl
  const re = new RegExp(`${field}:\\s*${value}\\b`);
  check(`${label} in src/config.rs`, re.test(configRs));
});

// Bounds must mirror the exact clamps applied by the settings UI in src/app.rs
// (row order: work 1..=120, short break 1..=60, long break 1..=90, interval 1..=24).
const CONFIG_BOUNDS = [
  { label: 'Work duration clamp is 1..=120', field: 'work_duration_mins', min: 1, max: 120 },
  { label: 'Short break clamp is 1..=60', field: 'short_break_mins', min: 1, max: 60 },
  { label: 'Long break clamp is 1..=90', field: 'long_break_mins', min: 1, max: 90 },
  { label: 'Max long break interval is 24 (clamp 1..=24)', field: 'long_break_interval', min: 1, max: 24 }
];

CONFIG_BOUNDS.forEach(({ label, field, min, max }) => {
  // Matches e.g. `(self.config.work_duration_mins as i32 + delta).clamp(1, 120)`
  const re = new RegExp(`${field} as i32 \\+ delta\\)\\.clamp\\(${min}, ${max}\\)`);
  check(`${label} in src/app.rs`, re.test(appRs));
});

// --------------------------------------------------------------------------
// 2. HTML Files & Assets Integrity
// --------------------------------------------------------------------------
console.log('\n🌐 2. Auditing HTML Pages, Images & Link Targets');

const indexHtml = fs.readFileSync(path.join(DOCS_DIR, 'index.html'), 'utf8');
const featuresHtml = fs.readFileSync(path.join(DOCS_DIR, 'features.html'), 'utf8');
const faqsHtml = fs.readFileSync(path.join(DOCS_DIR, 'faqs.html'), 'utf8');
const styleCss = fs.readFileSync(path.join(DOCS_DIR, 'style.css'), 'utf8');
const appJs = fs.readFileSync(path.join(DOCS_DIR, 'app.js'), 'utf8');

const HTML_PAGES = [
  { name: 'index.html', content: indexHtml },
  { name: 'features.html', content: featuresHtml },
  { name: 'faqs.html', content: faqsHtml }
];

// Verify Meta Tags & Favicons
HTML_PAGES.forEach(page => {
  check(`${page.name} has charset UTF-8`, page.content.includes('<meta charset="UTF-8"'));
  check(`${page.name} has responsive viewport meta`, page.content.includes('name="viewport" content="width=device-width, initial-scale=1.0"'));
  check(`${page.name} has favicon.svg link`, page.content.includes('href="favicon.svg"'));
  check(`${page.name} has mobile menu toggle button`, page.content.includes('class="mobile-menu-toggle"'));
  check(`${page.name} has mobile navigation drawer`, page.content.includes('class="mobile-nav-drawer"'));
  check(`${page.name} has mobile nav backdrop`, page.content.includes('class="mobile-nav-backdrop"'));
});

// Verify Screenshots referenced on disk
const screenshotRefs = [
  'kde_01_timer_view.png',
  'kde_02_task_manager.png',
  'kde_03_stats_view.png',
  'kde_04_settings_view.png',
  'kde_05_task_modal.png',
  'kde_06_help_modal.png'
];

screenshotRefs.forEach(ss => {
  const filePath = path.join(DOCS_DIR, 'assets/screenshots', ss);
  const exists = fs.existsSync(filePath);
  const size = exists ? fs.statSync(filePath).size : 0;
  check(`Screenshot asset ${ss} exists and is valid (${(size / 1024).toFixed(1)} KB)`, exists && size > 1000);
});

// --------------------------------------------------------------------------
// 3. Theme Engine Parity (Rust <-> CSS <-> JS)
// --------------------------------------------------------------------------
console.log('\n🎨 3. Auditing 18-Theme Palette Synchronizations');

const THEME_KEYS = [
  'catppuccin_mocha', 'catppuccin_macchiato', 'catppuccin_frappe', 'catppuccin_latte',
  'nord', 'gruvbox_dark', 'tokyo_night', 'dracula', 'rose_pine', 'one_dark',
  'kanagawa', 'everforest_dark', 'everforest_light', 'solarized_dark',
  'solarized_light', 'synthwave84', 'monokai_pro', 'oled_phosphor'
];

THEME_KEYS.forEach(key => {
  const cssRuleExists = styleCss.includes(`html[data-theme="${key}"]`);
  const jsObjectExists = appJs.includes(`"${key}":`) || appJs.includes(`${key}:`);
  const htmlOptionExists = indexHtml.includes(`value="${key}"`);
  check(`Theme '${key}' defined in CSS tokens, JS palette engine, and HTML dropdown`, cssRuleExists && jsObjectExists && htmlOptionExists);
});

// --------------------------------------------------------------------------
// 4. Web Audio Synthesizer Parity in app.js
// --------------------------------------------------------------------------
console.log('\n🔊 4. Auditing Web Audio Engine in app.js');

// The web audio engine must mirror the exact frequencies synthesized by the
// Rust chime engine in src/audio.rs. Reuse the same anchored patterns from
// section 1 so a change on either side (Rust or JS) breaks this parity check.
check('app.js synthesizes 528 Hz Work Zen Bowl tone', /\b528\b/.test(appJs) && /\b528\.0\b/.test(audioRs));
check('app.js synthesizes 587.33 Hz & 880 Hz Short Break tones', /\b587\.33\b/.test(appJs) && /\b880\b/.test(appJs) && /\b587\.33\b/.test(audioRs) && /\b880\.0\b/.test(audioRs));
check('app.js synthesizes C5-E5-G5 (523.25, 659.25, 783.99) Long Break triad', /\b523\.25\b/.test(appJs) && /\b659\.25\b/.test(appJs) && /\b783\.99\b/.test(appJs) && /\b523\.25\b/.test(audioRs) && /\b659\.25\b/.test(audioRs) && /\b783\.99\b/.test(audioRs));

// --------------------------------------------------------------------------
// 5. Code Cards & Copy Buttons Architecture
// --------------------------------------------------------------------------
console.log('\n📋 5. Auditing Installation Code Cards in index.html');

const installPanels = ['linux', 'macos', 'windows', 'multiplexer', 'binary'];
installPanels.forEach(panel => {
  const panelExists = indexHtml.includes(`id="install-panel-${panel}"`);
  const cardHeaderExists = indexHtml.includes(`data-target="code-${panel}"`);
  check(`Install panel '${panel}' uses structured .code-card with anchored copy button`, panelExists && cardHeaderExists);
});

console.log('\n' + '='.repeat(60));
console.log(`📊 SANITY AUDIT SUMMARY: ${passCount}/${passCount + failCount} CHECKS PASSED (${failCount} FAILED)`);
console.log('='.repeat(60) + '\n');

if (failCount > 0) {
  process.exit(1);
}
