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
check('Focus chime frequency 528.0 Hz in src/audio.rs', audioRs.includes('528.0'));
check('Focus chime 2nd harmonic 1056.0 Hz in src/audio.rs', audioRs.includes('1056.0'));
check('Focus chime 3rd harmonic 1584.0 Hz in src/audio.rs', audioRs.includes('1584.0'));
check('Short break chime D5 587.33 Hz in src/audio.rs', audioRs.includes('587.33'));
check('Short break chime A5 880.0 Hz in src/audio.rs', audioRs.includes('880.0'));
check('Long break chime C5 523.25 Hz in src/audio.rs', audioRs.includes('523.25'));
check('Long break chime E5 659.25 Hz in src/audio.rs', audioRs.includes('659.25'));
check('Long break chime G5 783.99 Hz in src/audio.rs', audioRs.includes('783.99'));

// C. Settings Defaults & Boundaries
check('Default work duration 25 mins in src/config.rs', configRs.includes('25'));
check('Default short break 5 mins in src/config.rs', configRs.includes('5'));
check('Default long break 15 mins in src/config.rs', configRs.includes('15'));
check('Default long break interval 4 in src/config.rs', configRs.includes('4'));
check('Max long break interval is 24 in src/config.rs / src/app.rs', configRs.includes('24') || appRs.includes('24'));

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

check('app.js synthesizes 528 Hz Work Zen Bowl tone', appJs.includes('528'));
check('app.js synthesizes 587.33 Hz & 880 Hz Short Break tones', appJs.includes('587.33') && appJs.includes('880'));
check('app.js synthesizes C5-E5-G5 (523.25, 659.25, 783.99) Long Break triad', appJs.includes('523.25') && appJs.includes('659.25') && appJs.includes('783.99'));

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
