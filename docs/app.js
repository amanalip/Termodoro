/**
 * Termodoro - Interactive Terminal Simulator, Theme Engine & Web Audio Synthesizer
 */

// ============================================================================
// 1. Color Palettes (18 Themes matching src/theme.rs)
// ============================================================================
const THEMES = {
  catppuccin_mocha: {
    name: 'Catppuccin Mocha',
    bg: '#1e1e2e', surface: '#181825', card: '#252538', cardHover: '#2d2d44',
    fg: '#cdd6f4', muted: '#9399b2', dim: '#6c7086',
    primary: '#89b4fa', secondary: '#cba6f7', work: '#f38ba8',
    shortBreak: '#a6e3a1', longBreak: '#94e2d5', success: '#a6e3a1', warning: '#f9e2af',
    border: '#313244', borderActive: '#89b4fa', highlight: '#313244'
  },
  catppuccin_macchiato: {
    name: 'Catppuccin Macchiato',
    bg: '#24273a', surface: '#1e2030', card: '#2d314d', cardHover: '#363a5c',
    fg: '#cad3f5', muted: '#939ab7', dim: '#6e738d',
    primary: '#8aadf4', secondary: '#c6a0f6', work: '#ed8796',
    shortBreak: '#a6da95', longBreak: '#8bd5ca', success: '#a6da95', warning: '#eed49f',
    border: '#3c405b', borderActive: '#8aadf4', highlight: '#363a4f'
  },
  catppuccin_frappe: {
    name: 'Catppuccin Frappé',
    bg: '#303446', surface: '#292c3c', card: '#393e56', cardHover: '#424864',
    fg: '#c6d0f5', muted: '#949cbb', dim: '#737994',
    primary: '#8caaee', secondary: '#ca9ee6', work: '#e78284',
    shortBreak: '#a3d18c', longBreak: '#81c8be', success: '#a3d18c', warning: '#e5c890',
    border: '#474d6b', borderActive: '#8caaee', highlight: '#414559'
  },
  catppuccin_latte: {
    name: 'Catppuccin Latte (Light)',
    bg: '#eff1f5', surface: '#e6e9ef', card: '#dce0e8', cardHover: '#ccd0da',
    fg: '#4c4f69', muted: '#6c6f85', dim: '#8c8fa1',
    primary: '#1e66f5', secondary: '#8839ef', work: '#d20f39',
    shortBreak: '#40a02b', longBreak: '#179299', success: '#40a02b', warning: '#df8e1d',
    border: '#bcc0cc', borderActive: '#1e66f5', highlight: '#ccd0da'
  },
  nord: {
    name: 'Nord',
    bg: '#2e3440', surface: '#242933', card: '#3b4252', cardHover: '#434c5e',
    fg: '#eceff4', muted: '#d8dee9', dim: '#5e81ac',
    primary: '#88c0d0', secondary: '#81a1c1', work: '#bf616a',
    shortBreak: '#a3be8c', longBreak: '#8fbcbb', success: '#a3be8c', warning: '#ebcb8b',
    border: '#4c566a', borderActive: '#88c0d0', highlight: '#3b4252'
  },
  gruvbox_dark: {
    name: 'Gruvbox Dark',
    bg: '#282828', surface: '#1d2021', card: '#32302f', cardHover: '#3c3836',
    fg: '#ebdbb2', muted: '#d5c4a1', dim: '#928374',
    primary: '#fabd2f', secondary: '#fe8019', work: '#fb4934',
    shortBreak: '#b8bb26', longBreak: '#8ec07c', success: '#b8bb26', warning: '#fabd2f',
    border: '#504945', borderActive: '#fabd2f', highlight: '#3c3836'
  },
  tokyo_night: {
    name: 'Tokyo Night',
    bg: '#1a1b26', surface: '#16161e', card: '#24283b', cardHover: '#2f354f',
    fg: '#c0caf5', muted: '#9aa5ce', dim: '#565f89',
    primary: '#7aa2f7', secondary: '#bb9af7', work: '#f7768e',
    shortBreak: '#9ece6a', longBreak: '#73daca', success: '#9ece6a', warning: '#e0af68',
    border: '#414868', borderActive: '#7aa2f7', highlight: '#24283b'
  },
  dracula: {
    name: 'Dracula',
    bg: '#282a36', surface: '#21222c', card: '#343746', cardHover: '#44475a',
    fg: '#f8f8f2', muted: '#bfbfbf', dim: '#6272a4',
    primary: '#bd93f9', secondary: '#8be9fd', work: '#ff5555',
    shortBreak: '#50fa7b', longBreak: '#8be9fd', success: '#50fa7b', warning: '#f1fa8c',
    border: '#44475a', borderActive: '#bd93f9', highlight: '#44475a'
  },
  rose_pine: {
    name: 'Rose Pine',
    bg: '#191724', surface: '#1f1d2e', card: '#26233a', cardHover: '#312f49',
    fg: '#e0def4', muted: '#908caa', dim: '#6e6a86',
    primary: '#9ccfd8', secondary: '#c4a7e7', work: '#eb6f92',
    shortBreak: '#31748f', longBreak: '#9ccfd8', success: '#31748f', warning: '#f6c177',
    border: '#393552', borderActive: '#c4a7e7', highlight: '#2a283e'
  },
  one_dark: {
    name: 'One Dark',
    bg: '#282c34', surface: '#21252b', card: '#2c313a', cardHover: '#353b45',
    fg: '#abb2bf', muted: '#828997', dim: '#5c6370',
    primary: '#61afef', secondary: '#c678dd', work: '#e06c75',
    shortBreak: '#98c379', longBreak: '#56b6c2', success: '#98c379', warning: '#e5c07b',
    border: '#3e4452', borderActive: '#61afef', highlight: '#353b45'
  },
  kanagawa: {
    name: 'Kanagawa',
    bg: '#1f1f28', surface: '#16161d', card: '#2a2a37', cardHover: '#363646',
    fg: '#dcd7ba', muted: '#938aa9', dim: '#727169',
    primary: '#7e9cd8', secondary: '#957fb8', work: '#e46876',
    shortBreak: '#76946a', longBreak: '#6a9589', success: '#76946a', warning: '#ffa066',
    border: '#363646', borderActive: '#7e9cd8', highlight: '#2a2a37'
  },
  everforest_dark: {
    name: 'Everforest Dark',
    bg: '#2d353b', surface: '#232a2e', card: '#343f44', cardHover: '#3d484d',
    fg: '#d3c6aa', muted: '#9da9a0', dim: '#859289',
    primary: '#7fbbb3', secondary: '#d699b6', work: '#e67e80',
    shortBreak: '#a7c080', longBreak: '#83c092', success: '#a7c080', warning: '#dbbc7f',
    border: '#475258', borderActive: '#a7c080', highlight: '#343f44'
  },
  everforest_light: {
    name: 'Everforest Light',
    bg: '#fdf6e3', surface: '#f4ede8', card: '#eae4cb', cardHover: '#dfd8bd',
    fg: '#5c6a72', muted: '#708089', dim: '#93aa9f',
    primary: '#3a9486', secondary: '#df69ba', work: '#f85552',
    shortBreak: '#8da101', longBreak: '#35a77c', success: '#8da101', warning: '#dfa000',
    border: '#d3cbb7', borderActive: '#3a9486', highlight: '#eae4cb'
  },
  solarized_dark: {
    name: 'Solarized Dark',
    bg: '#002b36', surface: '#073642', card: '#094352', cardHover: '#0b4f61',
    fg: '#839496', muted: '#93a1a1', dim: '#586e75',
    primary: '#268bd2', secondary: '#2aa198', work: '#dc322f',
    shortBreak: '#859900', longBreak: '#2aa198', success: '#859900', warning: '#b58900',
    border: '#0d5568', borderActive: '#268bd2', highlight: '#073642'
  },
  solarized_light: {
    name: 'Solarized Light',
    bg: '#fdf6e3', surface: '#eee8d5', card: '#e4dcbe', cardHover: '#dad1b0',
    fg: '#657b83', muted: '#586e75', dim: '#93a1a1',
    primary: '#268bd2', secondary: '#6c71c4', work: '#dc322f',
    shortBreak: '#859900', longBreak: '#2aa198', success: '#859900', warning: '#b58900',
    border: '#d3cbb7', borderActive: '#268bd2', highlight: '#eee8d5'
  },
  synthwave84: {
    name: "Synthwave '84",
    bg: '#262335', surface: '#1e1c2a', card: '#34294f', cardHover: '#423565',
    fg: '#f0eff1', muted: '#b6b1d8', dim: '#848bbd',
    primary: '#36f9f6', secondary: '#ff7edb', work: '#fe4450',
    shortBreak: '#72f1b8', longBreak: '#36f9f6', success: '#72f1b8', warning: '#fede5d',
    border: '#494363', borderActive: '#ff7edb', highlight: '#34294f'
  },
  monokai_pro: {
    name: 'Monokai Pro',
    bg: '#2d2a2e', surface: '#221f22', card: '#3a383b', cardHover: '#49464b',
    fg: '#fcfcfa', muted: '#c1c0c0', dim: '#727072',
    primary: '#78dce8', secondary: '#ab9df2', work: '#ff6188',
    shortBreak: '#a9dc76', longBreak: '#78dce8', success: '#a9dc76', warning: '#ffd866',
    border: '#49474a', borderActive: '#ffd866', highlight: '#3a383b'
  },
  oled_phosphor: {
    name: 'OLED Phosphor',
    bg: '#000000', surface: '#050505', card: '#0d1a0d', cardHover: '#132613',
    fg: '#33ff66', muted: '#26cc52', dim: '#1a662a',
    primary: '#00ff66', secondary: '#00cc55', work: '#ff3333',
    shortBreak: '#33ff66', longBreak: '#00ffff', success: '#33ff66', warning: '#ffff33',
    border: '#1a331a', borderActive: '#00ff66', highlight: '#0a1f0a'
  }
};

let currentThemeKey = 'catppuccin_mocha';

function applyTheme(themeKey) {
  const t = THEMES[themeKey] || THEMES.catppuccin_mocha;
  currentThemeKey = themeKey;

  const root = document.documentElement;
  root.style.setProperty('--bg-base', t.bg);
  root.style.setProperty('--bg-surface', t.surface);
  root.style.setProperty('--bg-card', t.card);
  root.style.setProperty('--bg-card-hover', t.cardHover);
  root.style.setProperty('--text-main', t.fg);
  root.style.setProperty('--text-muted', t.muted);
  root.style.setProperty('--text-dim', t.dim);
  root.style.setProperty('--color-primary', t.primary);
  root.style.setProperty('--color-secondary', t.secondary);
  root.style.setProperty('--color-work', t.work);
  root.style.setProperty('--color-short-break', t.shortBreak);
  root.style.setProperty('--color-long-break', t.longBreak);
  root.style.setProperty('--color-success', t.success);
  root.style.setProperty('--color-warning', t.warning);
  root.style.setProperty('--border-color', t.border);
  root.style.setProperty('--border-active', t.borderActive);
  root.style.setProperty('--highlight-bg', t.highlight);

  // Update theme selector label if present
  const themeLabel = document.getElementById('current-theme-name');
  if (themeLabel) themeLabel.textContent = t.name;

  const themeSelect = document.getElementById('sim-theme-select');
  if (themeSelect) themeSelect.value = themeKey;

  // Update active state in theme gallery
  document.querySelectorAll('.theme-card').forEach(card => {
    card.classList.toggle('active', card.dataset.theme === themeKey);
  });
}

// ============================================================================
// 2. 5x4 Block Digits Rasterizer (matching src/ui/digits.rs)
// ============================================================================
const DIGIT_PATTERNS = {
  '0': ['████', '█  █', '█  █', '█  █', '████'],
  '1': ['  ██', '  ██', '  ██', '  ██', '  ██'],
  '2': ['████', '   █', '████', '█   ', '████'],
  '3': ['████', '   █', '████', '   █', '████'],
  '4': ['█  █', '█  █', '████', '   █', '   █'],
  '5': ['████', '█   ', '████', '   █', '████'],
  '6': ['████', '█   ', '████', '█  █', '████'],
  '7': ['████', '   █', '   █', '   █', '   █'],
  '8': ['████', '█  █', '████', '█  █', '████'],
  '9': ['████', '█  █', '████', '   █', '████'],
  ':': ['    ', ' ██ ', '    ', ' ██ ', '    '],
  ' ': ['    ', '    ', '    ', '    ', '    ']
};

function renderBigTime(mins, secs) {
  const str = String(mins).padStart(2, '0') + ':' + String(secs).padStart(2, '0');
  const lines = ['', '', '', '', ''];
  for (let i = 0; i < str.length; i++) {
    const pattern = DIGIT_PATTERNS[str[i]] || DIGIT_PATTERNS[' '];
    for (let row = 0; row < 5; row++) {
      if (i > 0) lines[row] += ' ';
      lines[row] += pattern[row];
    }
  }
  return lines.join('\n');
}

// ============================================================================
// 3. Web Audio Synthesizer (Replicating Termodoro PCM Audio Engine)
// ============================================================================
let audioCtx = null;
let soundEnabled = true;

function getAudioContext() {
  if (!audioCtx) {
    const AudioContextClass = window.AudioContext || window.webkitAudioContext;
    if (AudioContextClass) {
      audioCtx = new AudioContextClass();
    }
  }
  if (audioCtx && audioCtx.state === 'suspended') {
    audioCtx.resume();
  }
  return audioCtx;
}

// Zen Tibetan Bell (528 Hz transformation frequency + warm overtones)
function playZenBowlChime() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const masterGain = ctx.createGain();
  masterGain.gain.setValueAtTime(0.35, now);
  masterGain.gain.exponentialRampToValueAtTime(0.0001, now + 1.8);
  masterGain.connect(ctx.destination);

  // 528 Hz base, 1056 Hz, 1584 Hz harmonics
  const freqs = [528.0, 1056.0, 1584.0];
  const gains = [0.65, 0.25, 0.10];

  freqs.forEach((freq, idx) => {
    const osc = ctx.createOscillator();
    const g = ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(freq, now);
    g.gain.setValueAtTime(gains[idx], now);
    osc.connect(g);
    g.connect(masterGain);
    osc.start(now);
    osc.stop(now + 1.8);
  });
}

// Break Completion Double Chime (D5 587.33 Hz -> A5 880.0 Hz)
function playBreakChime() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;

  // Note 1: 587.33 Hz
  const osc1 = ctx.createOscillator();
  const gain1 = ctx.createGain();
  osc1.type = 'sine';
  osc1.frequency.setValueAtTime(587.33, now);
  gain1.gain.setValueAtTime(0.3, now);
  gain1.gain.exponentialRampToValueAtTime(0.0001, now + 0.25);
  osc1.connect(gain1);
  gain1.connect(ctx.destination);
  osc1.start(now);
  osc1.stop(now + 0.25);

  // Note 2: 880.0 Hz + overtone
  const osc2 = ctx.createOscillator();
  const gain2 = ctx.createGain();
  osc2.type = 'sine';
  osc2.frequency.setValueAtTime(880.0, now + 0.22);
  gain2.gain.setValueAtTime(0.0001, now);
  gain2.gain.setValueAtTime(0.3, now + 0.22);
  gain2.gain.exponentialRampToValueAtTime(0.0001, now + 1.4);
  osc2.connect(gain2);
  gain2.connect(ctx.destination);
  osc2.start(now + 0.22);
  osc2.stop(now + 1.4);
}

// Long Break Completion Triad (C5 523.25 Hz -> E5 659.25 Hz -> G5 783.99 Hz)
function playLongBreakChime() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const notes = [
    { freq: 523.25, time: 0.0, dur: 0.20 },
    { freq: 659.25, time: 0.20, dur: 0.20 },
    { freq: 783.99, time: 0.40, dur: 1.60 }
  ];

  notes.forEach(note => {
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(note.freq, now + note.time);
    gain.gain.setValueAtTime(0.0001, now);
    gain.gain.setValueAtTime(0.3, now + note.time);
    gain.gain.exponentialRampToValueAtTime(0.0001, now + note.time + note.dur);
    osc.connect(gain);
    gain.connect(ctx.destination);
    osc.start(now + note.time);
    osc.stop(now + note.time + note.dur);
  });
}

// ============================================================================
// 4. Interactive Screenshot Showcase Controller
// ============================================================================
function initShowcase() {
  const tabs = document.querySelectorAll('.showcase-tab');
  const img = document.getElementById('active-showcase-img');
  const caption = document.getElementById('active-showcase-caption');

  if (!tabs.length || !img) return;

  function setTab(tab) {
    tabs.forEach(t => t.classList.remove('active'));
    tab.classList.add('active');

    const imgSrc = tab.dataset.img;
    const capText = tab.dataset.caption;

    img.style.opacity = '0.3';
    setTimeout(() => {
      img.src = imgSrc;
      if (caption) caption.textContent = capText;
      img.style.opacity = '1';
    }, 80);
  }

  tabs.forEach(tab => {
    tab.addEventListener('click', () => setTab(tab));
  });

  // Numeric keyboard navigation [1] to [6] to switch showcase screenshots
  window.addEventListener('keydown', (e) => {
    if (['INPUT', 'TEXTAREA', 'SELECT'].includes(e.target.tagName)) return;
    const num = parseInt(e.key);
    if (num >= 1 && num <= tabs.length) {
      const targetTab = document.getElementById(`tab-ss-${num}`);
      if (targetTab) {
        setTab(targetTab);
        showToast(`Viewing Screen [${num}]`);
      }
    }
  });
}

function showToast(msg) {
  const toast = document.getElementById('toast-notice');
  if (!toast) return;
  toast.textContent = msg;
  toast.classList.add('show');
  clearTimeout(toast._timeout);
  toast._timeout = setTimeout(() => {
    toast.classList.remove('show');
  }, 2000);
}

// Setup common listeners across index.html and features.html
function setupCommonListeners() {
  // Theme card click in gallery
  document.querySelectorAll('.theme-card').forEach(card => {
    card.addEventListener('click', () => {
      const themeKey = card.dataset.theme;
      applyTheme(themeKey);
      showToast(`Switched theme to ${THEMES[themeKey].name}`);
    });
  });

  // Audio test buttons
  const testZenBtn = document.getElementById('test-zen-btn');
  if (testZenBtn) testZenBtn.addEventListener('click', () => { playZenBowlChime(); showToast('Playing 528 Hz Zen Bowl'); });
  
  const testBreakBtn = document.getElementById('test-break-btn');
  if (testBreakBtn) testBreakBtn.addEventListener('click', () => { playBreakChime(); showToast('Playing D5 → A5 Chime'); });

  const testLongBreakBtn = document.getElementById('test-long-break-btn');
  if (testLongBreakBtn) testLongBreakBtn.addEventListener('click', () => { playLongBreakChime(); showToast('Playing Long Break Triad'); });

  // Installation tab navigation
  document.querySelectorAll('.install-tab-btn').forEach(tab => {
    tab.addEventListener('click', () => {
      document.querySelectorAll('.install-tab-btn').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.install-code-panel').forEach(p => p.style.display = 'none');

      tab.classList.add('active');
      const panel = document.getElementById(`install-panel-${tab.dataset.install}`);
      if (panel) panel.style.display = 'block';
    });
  });

  // One-click copy buttons
  document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      const codeTargetId = btn.dataset.target;
      const codeEl = document.getElementById(codeTargetId);
      if (codeEl) {
        navigator.clipboard.writeText(codeEl.textContent.trim()).then(() => {
          const originalText = btn.textContent;
          btn.textContent = '✓ Copied!';
          showToast('Snippet copied to clipboard');
          setTimeout(() => { btn.textContent = originalText; }, 2000);
        });
      }
    });
  });

  // Keybindings Search filter
  const searchInput = document.getElementById('keybinding-search');
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      const query = e.target.value.toLowerCase().trim();
      const rows = document.querySelectorAll('.keybindings-table tbody tr');
      rows.forEach(row => {
        const text = row.textContent.toLowerCase();
        row.style.display = text.includes(query) ? '' : 'none';
      });
    });
  }

  // FAQ Search & Category Filter Controller
  const faqSearchInput = document.getElementById('faq-search-input');
  const faqPills = document.querySelectorAll('.faq-pill-btn');
  const faqGroups = document.querySelectorAll('.faq-group');
  const faqItems = document.querySelectorAll('.faq-item');
  const faqEmptyState = document.getElementById('faq-empty-state');

  function filterFAQs() {
    if (!faqSearchInput) return;
    const query = faqSearchInput.value.toLowerCase().trim();
    const activeCategory = document.querySelector('.faq-pill-btn.active')?.dataset.category || 'all';

    let totalVisible = 0;

    faqGroups.forEach(group => {
      const groupCategory = group.dataset.group;
      const categoryMatches = (activeCategory === 'all' || activeCategory === groupCategory);

      if (!categoryMatches) {
        group.style.display = 'none';
        return;
      }

      let visibleInGroup = 0;
      const itemsInGroup = group.querySelectorAll('.faq-item');

      itemsInGroup.forEach(item => {
        const questionText = item.querySelector('summary')?.textContent.toLowerCase() || '';
        const answerText = item.querySelector('.faq-content')?.textContent.toLowerCase() || '';
        const matchesQuery = !query || questionText.includes(query) || answerText.includes(query);

        if (matchesQuery) {
          item.style.display = '';
          if (query.length > 2) {
            item.setAttribute('open', '');
          }
          visibleInGroup++;
          totalVisible++;
        } else {
          item.style.display = 'none';
        }
      });

      group.style.display = visibleInGroup > 0 ? '' : 'none';
    });

    if (faqEmptyState) {
      if (totalVisible === 0) {
        faqEmptyState.classList.add('show');
      } else {
        faqEmptyState.classList.remove('show');
      }
    }
  }

  if (faqSearchInput) {
    faqSearchInput.addEventListener('input', filterFAQs);
  }

  faqPills.forEach(pill => {
    pill.addEventListener('click', () => {
      faqPills.forEach(p => p.classList.remove('active'));
      pill.classList.add('active');
      filterFAQs();
    });
  });
}

// Initialize on DOM load
document.addEventListener('DOMContentLoaded', () => {
  applyTheme('catppuccin_mocha');
  initShowcase();
  setupCommonListeners();
});

