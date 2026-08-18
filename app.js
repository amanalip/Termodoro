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
// 2. 5x3 Block Digits Rasterizer (matching Ratatui implementation)
// ============================================================================
const DIGIT_PATTERNS = {
  '0': ['███', '█ █', '█ █', '█ █', '███'],
  '1': [' ██', '  █', '  █', '  █', '  █'],
  '2': ['███', '  █', '███', '█  ', '███'],
  '3': ['███', '  █', '███', '  █', '███'],
  '4': ['█ █', '█ █', '███', '  █', '  █'],
  '5': ['███', '█  ', '███', '  █', '███'],
  '6': ['███', '█  ', '███', '█ █', '███'],
  '7': ['███', '  █', '  █', '  █', '  █'],
  '8': ['███', '█ █', '███', '█ █', '███'],
  '9': ['███', '█ █', '███', '  █', '███'],
  ':': [' ', '█', ' ', '█', ' ']
};

function renderBlockDigit(char) {
  const pattern = DIGIT_PATTERNS[char] || DIGIT_PATTERNS['0'];
  return pattern.join('\n');
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

// ASCII Terminal Bell (800 Hz beep)
function playBellBeep() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const osc = ctx.createOscillator();
  const gain = ctx.createGain();
  osc.type = 'sine';
  osc.frequency.setValueAtTime(800, now);
  gain.gain.setValueAtTime(0.25, now);
  gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.15);
  osc.connect(gain);
  gain.connect(ctx.destination);
  osc.start(now);
  osc.stop(now + 0.15);
}

// ============================================================================
// 4. Interactive Terminal State Machine & App Controller
// ============================================================================
const state = {
  currentTab: 1,
  phase: 'work', // 'work', 'shortBreak', 'longBreak'
  workDuration: 25 * 60,
  shortBreakDuration: 5 * 60,
  longBreakDuration: 15 * 60,
  cyclesBeforeLongBreak: 4,
  timeRemaining: 25 * 60,
  isRunning: false,
  completedCycles: 0,
  targetTask: 'Implement GitHub Pages Website',
  tasks: [
    { id: 1, title: 'Implement GitHub Pages Website', poms: 2, est: 4, tag: 'HIGH', done: false },
    { id: 2, title: 'Write Ratatui block digits renderer', poms: 3, est: 3, tag: 'MED', done: true },
    { id: 3, title: 'Synthesize PCM RIFF WAV sound chimes', poms: 1, est: 2, tag: 'LOW', done: false }
  ],
  activeFilter: 'all',
  helpModalOpen: false,
  taskModalOpen: false
};

let timerInterval = null;

function initSimulator() {
  renderTabs();
  renderTimerScreen();
  renderTasksScreen();
  renderStatsScreen();
  setupEventListeners();
}

function updateClockDisplay() {
  const mins = Math.floor(state.timeRemaining / 60);
  const secs = state.timeRemaining % 60;

  const minStr = String(mins).padStart(2, '0');
  const secStr = String(secs).padStart(2, '0');

  const d1 = document.getElementById('digit-1');
  const d2 = document.getElementById('digit-2');
  const colon = document.getElementById('digit-colon');
  const d3 = document.getElementById('digit-3');
  const d4 = document.getElementById('digit-4');

  if (d1) d1.textContent = renderBlockDigit(minStr[0]);
  if (d2) d2.textContent = renderBlockDigit(minStr[1]);
  if (colon) colon.textContent = renderBlockDigit(':');
  if (d3) d3.textContent = renderBlockDigit(secStr[0]);
  if (d4) d4.textContent = renderBlockDigit(secStr[1]);

  // Update gauge fill
  const total = state.phase === 'work' ? state.workDuration :
                state.phase === 'shortBreak' ? state.shortBreakDuration : state.longBreakDuration;
  const progressPercent = Math.max(0, Math.min(100, ((total - state.timeRemaining) / total) * 100));
  
  const gaugeFill = document.getElementById('sim-gauge-fill');
  if (gaugeFill) {
    gaugeFill.style.width = `${progressPercent}%`;
    const t = THEMES[currentThemeKey] || THEMES.catppuccin_mocha;
    gaugeFill.style.backgroundColor = state.phase === 'work' ? t.work :
                                      state.phase === 'shortBreak' ? t.shortBreak : t.longBreak;
  }

  // Update dots
  const dotsWrapper = document.getElementById('sim-cycle-dots');
  if (dotsWrapper) {
    let dotsHtml = '';
    const currentCycleInBlock = state.completedCycles % state.cyclesBeforeLongBreak;
    for (let i = 0; i < state.cyclesBeforeLongBreak; i++) {
      const isFilled = i < currentCycleInBlock;
      dotsHtml += `<span class="cycle-dot ${isFilled ? 'filled' : ''}">${isFilled ? '●' : '○'}</span> `;
    }
    dotsWrapper.innerHTML = dotsHtml;
  }

  // Update phase indicator
  const phaseLabel = document.getElementById('sim-phase-title');
  const phasePulse = document.getElementById('sim-phase-pulse');
  const t = THEMES[currentThemeKey] || THEMES.catppuccin_mocha;

  if (phaseLabel && phasePulse) {
    if (state.phase === 'work') {
      phaseLabel.textContent = state.isRunning ? 'FOCUS WORK' : 'WORK PAUSED';
      phaseLabel.style.color = t.work;
      phasePulse.style.backgroundColor = t.work;
      phasePulse.style.boxShadow = `0 0 10px ${t.work}`;
    } else if (state.phase === 'shortBreak') {
      phaseLabel.textContent = 'SHORT BREAK';
      phaseLabel.style.color = t.shortBreak;
      phasePulse.style.backgroundColor = t.shortBreak;
      phasePulse.style.boxShadow = `0 0 10px ${t.shortBreak}`;
    } else {
      phaseLabel.textContent = 'LONG BREAK';
      phaseLabel.style.color = t.longBreak;
      phasePulse.style.backgroundColor = t.longBreak;
      phasePulse.style.boxShadow = `0 0 10px ${t.longBreak}`;
    }
  }

  // Update target task title
  const taskTitleEl = document.getElementById('sim-target-task-title');
  if (taskTitleEl) taskTitleEl.textContent = state.targetTask;
}

function toggleTimer() {
  state.isRunning = !state.isRunning;
  if (state.isRunning) {
    getAudioContext(); // Warm audio context on user gesture
    if (!timerInterval) {
      timerInterval = setInterval(tickTimer, 1000);
    }
    showToast('Timer started');
  } else {
    clearInterval(timerInterval);
    timerInterval = null;
    showToast('Timer paused');
  }
  updateClockDisplay();
}

function tickTimer() {
  if (state.timeRemaining > 0) {
    state.timeRemaining--;
    updateClockDisplay();
  } else {
    // Phase completed!
    if (state.phase === 'work') {
      state.completedCycles++;
      playZenBowlChime();
      showToast('Focus session complete! 🎉');
      if (state.completedCycles % state.cyclesBeforeLongBreak === 0) {
        state.phase = 'longBreak';
        state.timeRemaining = state.longBreakDuration;
      } else {
        state.phase = 'shortBreak';
        state.timeRemaining = state.shortBreakDuration;
      }
    } else {
      playBreakChime();
      showToast('Break finished! Back to focus ⚡');
      state.phase = 'work';
      state.timeRemaining = state.workDuration;
    }
    updateClockDisplay();
  }
}

function resetTimer() {
  state.isRunning = false;
  clearInterval(timerInterval);
  timerInterval = null;
  state.timeRemaining = state.phase === 'work' ? state.workDuration :
                        state.phase === 'shortBreak' ? state.shortBreakDuration : state.longBreakDuration;
  updateClockDisplay();
  showToast('Timer reset');
}

function skipPhase() {
  if (state.phase === 'work') {
    state.completedCycles++;
    if (state.completedCycles % state.cyclesBeforeLongBreak === 0) {
      state.phase = 'longBreak';
      state.timeRemaining = state.longBreakDuration;
    } else {
      state.phase = 'shortBreak';
      state.timeRemaining = state.shortBreakDuration;
    }
  } else {
    state.phase = 'work';
    state.timeRemaining = state.workDuration;
  }
  updateClockDisplay();
  showToast(`Skipped to ${state.phase.toUpperCase()}`);
}

function switchTab(tabIndex) {
  state.currentTab = tabIndex;
  document.querySelectorAll('.terminal-tab').forEach(tab => {
    tab.classList.toggle('active', parseInt(tab.dataset.tab) === tabIndex);
  });
  document.querySelectorAll('.screen-view').forEach(screen => {
    screen.classList.toggle('active', parseInt(screen.dataset.screen) === tabIndex);
  });
}

function renderTabs() {
  document.querySelectorAll('.terminal-tab').forEach(tab => {
    tab.addEventListener('click', () => {
      switchTab(parseInt(tab.dataset.tab));
    });
  });
}

function renderTimerScreen() {
  updateClockDisplay();
}

function renderTasksScreen() {
  const tbody = document.getElementById('sim-tasks-tbody');
  if (!tbody) return;

  const filtered = state.tasks.filter(t => {
    if (state.activeFilter === 'pending') return !t.done;
    if (state.activeFilter === 'completed') return t.done;
    return true;
  });

  tbody.innerHTML = filtered.map(t => `
    <tr class="task-row ${t.done ? 'completed' : ''}" data-id="${t.id}">
      <td><span class="task-checkbox">${t.done ? '[x]' : '[ ]'}</span> ${t.title}</td>
      <td><span class="task-tag tag-${t.tag.toLowerCase()}">${t.tag}</span></td>
      <td>${t.poms}/${t.est} 🍅</td>
    </tr>
  `).join('');

  tbody.querySelectorAll('.task-row').forEach(row => {
    row.addEventListener('click', () => {
      const id = parseInt(row.dataset.id);
      const task = state.tasks.find(x => x.id === id);
      if (task) {
        task.done = !task.done;
        renderTasksScreen();
        showToast(`Task ${task.done ? 'completed' : 'reopened'}`);
      }
    });
  });
}

function renderStatsScreen() {
  // Weekly bar chart values
  const pillars = document.querySelectorAll('.bar-pillar');
  const heights = [45, 60, 80, 50, 95, 110, 70];
  pillars.forEach((p, idx) => {
    p.style.height = `${heights[idx] || 50}px`;
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
  }, 2200);
}

// Modal handling
function toggleHelpModal(show) {
  state.helpModalOpen = show !== undefined ? show : !state.helpModalOpen;
  const el = document.getElementById('terminal-help-modal');
  if (el) el.style.display = state.helpModalOpen ? 'flex' : 'none';
}

function toggleTaskModal(show) {
  state.taskModalOpen = show !== undefined ? show : !state.taskModalOpen;
  const el = document.getElementById('terminal-task-modal');
  if (el) {
    el.style.display = state.taskModalOpen ? 'flex' : 'none';
    if (state.taskModalOpen) {
      const input = document.getElementById('modal-task-title-input');
      if (input) {
        input.value = '';
        input.focus();
      }
    }
  }
}

// Setup event listeners
function setupEventListeners() {
  // Keyboard navigation
  window.addEventListener('keydown', (e) => {
    // Ignore keystrokes if typing inside text inputs
    if (['INPUT', 'TEXTAREA', 'SELECT'].includes(e.target.tagName)) {
      if (e.key === 'Escape') {
        toggleTaskModal(false);
        toggleHelpModal(false);
      }
      return;
    }

    if (e.key === ' ' || e.code === 'Space') {
      e.preventDefault();
      toggleTimer();
    } else if (e.key === 'r') {
      resetTimer();
    } else if (e.key === 's') {
      skipPhase();
    } else if (e.key === '1') {
      switchTab(1);
    } else if (e.key === '2') {
      switchTab(2);
    } else if (e.key === '3') {
      switchTab(3);
    } else if (e.key === '4') {
      switchTab(4);
    } else if (e.key === '?' || e.key === 'h') {
      toggleHelpModal();
    } else if (e.key === 'a') {
      toggleTaskModal(true);
    } else if (e.key === 'Escape') {
      toggleHelpModal(false);
      toggleTaskModal(false);
    }
  });

  // Sound toggle button
  const soundBtn = document.getElementById('sound-toggle-btn');
  if (soundBtn) {
    soundBtn.addEventListener('click', () => {
      soundEnabled = !soundEnabled;
      soundBtn.innerHTML = soundEnabled ? '🔊 Sound: ON' : '🔇 Sound: OFF';
      if (soundEnabled) playBellBeep();
      showToast(`Sound ${soundEnabled ? 'Enabled' : 'Muted'}`);
    });
  }

  // Simulator footer key pills
  document.querySelectorAll('.sim-key-pill').forEach(pill => {
    pill.addEventListener('click', () => {
      const action = pill.dataset.action;
      if (action === 'toggle') toggleTimer();
      else if (action === 'reset') resetTimer();
      else if (action === 'skip') skipPhase();
      else if (action === 'help') toggleHelpModal(true);
      else if (action === 'add') toggleTaskModal(true);
    });
  });

  // Task Filters
  document.querySelectorAll('.task-filter-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.task-filter-btn').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      state.activeFilter = btn.dataset.filter;
      renderTasksScreen();
    });
  });

  // Theme card click in gallery
  document.querySelectorAll('.theme-card').forEach(card => {
    card.addEventListener('click', () => {
      const themeKey = card.dataset.theme;
      applyTheme(themeKey);
      showToast(`Switched theme to ${THEMES[themeKey].name}`);
    });
  });

  // Theme select in Settings tab
  const simThemeSelect = document.getElementById('sim-theme-select');
  if (simThemeSelect) {
    simThemeSelect.addEventListener('change', (e) => {
      applyTheme(e.target.value);
    });
  }

  // Audio test buttons
  const testZenBtn = document.getElementById('test-zen-btn');
  if (testZenBtn) testZenBtn.addEventListener('click', () => playZenBowlChime());
  
  const testBreakBtn = document.getElementById('test-break-btn');
  if (testBreakBtn) testBreakBtn.addEventListener('click', () => playBreakChime());

  const testBellBtn = document.getElementById('test-bell-btn');
  if (testBellBtn) testBellBtn.addEventListener('click', () => playBellBeep());

  // Task creation form
  const createTaskForm = document.getElementById('modal-task-form');
  if (createTaskForm) {
    createTaskForm.addEventListener('submit', (e) => {
      e.preventDefault();
      const input = document.getElementById('modal-task-title-input');
      const tagSelect = document.getElementById('modal-task-tag-select');
      const estInput = document.getElementById('modal-task-est-input');

      if (input && input.value.trim()) {
        const newTask = {
          id: Date.now(),
          title: input.value.trim(),
          poms: 0,
          est: parseInt(estInput.value) || 2,
          tag: tagSelect.value || 'MED',
          done: false
        };
        state.tasks.unshift(newTask);
        renderTasksScreen();
        toggleTaskModal(false);
        showToast(`Added task: "${newTask.title}"`);
      }
    });
  }

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
}

// Initialize on DOM load
document.addEventListener('DOMContentLoaded', () => {
  applyTheme('catppuccin_mocha');
  initSimulator();
});
