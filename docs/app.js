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
  currentTab: 1, // 1: Timer, 2: Tasks, 3: Stats, 4: Settings
  phase: 'work', // 'work', 'shortBreak', 'longBreak'
  workDurationMins: 25,
  shortBreakMins: 5,
  longBreakMins: 15,
  longBreakInterval: 4,
  autoStartBreaks: false,
  autoStartWork: false,
  desktopNotifications: true,
  soundEnabled: true,
  timeRemaining: 25 * 60,
  isRunning: false,
  currentCycle: 1,
  completedCycles: 0,
  targetTask: 'Implement GitHub Pages Website',
  tasks: [
    { id: 1, title: 'Implement GitHub Pages Website', completed_poms: 2, est_poms: 4, completed: false, is_active: true },
    { id: 2, title: 'Write Ratatui block digits renderer', completed_poms: 3, est_poms: 3, completed: true, is_active: false },
    { id: 3, title: 'Synthesize PCM RIFF WAV sound chimes', completed_poms: 1, est_poms: 2, completed: false, is_active: false }
  ],
  selectedTaskIndex: 0,
  activeFilter: 'all', // 'all', 'active', 'completed'
  settingsIndex: 0, // 0 to 8
  helpModalOpen: false,
  taskModalOpen: false
};

let timerInterval = null;

function initSimulator() {
  renderTabs();
  renderTimerScreen();
  renderTasksScreen();
  renderStatsScreen();
  renderSettingsScreen();
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

  const total = state.phase === 'work' ? state.workDurationMins * 60 :
                state.phase === 'shortBreak' ? state.shortBreakMins * 60 : state.longBreakMins * 60;
  const progressPercent = Math.max(0, Math.min(100, ((total - state.timeRemaining) / total) * 100));
  
  // Update gauge fill
  const gaugeFill = document.getElementById('sim-gauge-fill');
  const gaugePercentText = document.getElementById('sim-gauge-percent');
  const t = THEMES[currentThemeKey] || THEMES.catppuccin_mocha;

  if (gaugeFill) {
    gaugeFill.style.width = `${progressPercent}%`;
    gaugeFill.style.backgroundColor = state.phase === 'work' ? t.work :
                                      state.phase === 'shortBreak' ? t.shortBreak : t.longBreak;
  }
  if (gaugePercentText) {
    gaugePercentText.textContent = `${Math.round(progressPercent)}%`;
    gaugePercentText.style.color = t.primary;
  }

  // Update dots and cycle tracker
  const dotsWrapper = document.getElementById('sim-cycle-dots');
  const cycleTracker = document.getElementById('sim-cycle-tracker');
  if (dotsWrapper) {
    let dotsHtml = '';
    for (let i = 1; i <= state.longBreakInterval; i++) {
      if (i < state.currentCycle) {
        dotsHtml += '● ';
      } else if (i === state.currentCycle && state.phase === 'work') {
        dotsHtml += '◉ ';
      } else {
        dotsHtml += '○ ';
      }
    }
    dotsWrapper.innerHTML = dotsHtml.trim();
  }
  if (cycleTracker) {
    cycleTracker.innerHTML = `Cycle ${state.currentCycle}/${state.longBreakInterval} [<span id="sim-cycle-dots">${dotsWrapper ? dotsWrapper.innerHTML : ''}</span>]`;
  }

  // Update phase indicator
  const phaseTitle = document.getElementById('sim-phase-title');
  const statusBadge = document.getElementById('sim-status-badge');
  const spaceLabel = document.getElementById('sim-space-label');

  if (phaseTitle) {
    if (state.phase === 'work') {
      phaseTitle.textContent = '🍅 Focus Work';
      phaseTitle.style.color = t.work;
    } else if (state.phase === 'shortBreak') {
      phaseTitle.textContent = '☕ Short Break';
      phaseTitle.style.color = t.shortBreak;
    } else {
      phaseTitle.textContent = '🌴 Long Break';
      phaseTitle.style.color = t.longBreak;
    }
  }

  if (statusBadge) {
    if (state.isRunning) {
      statusBadge.textContent = '[● RUNNING]';
      statusBadge.style.color = t.success;
    } else {
      statusBadge.textContent = '[❚❚ PAUSED]';
      statusBadge.style.color = t.warning;
    }
  }

  if (spaceLabel) {
    spaceLabel.textContent = state.isRunning ? 'Pause' : 'Start';
  }

  // Update target task title and count
  const targetTask = state.tasks.find(x => x.is_active);
  const taskTitleEl = document.getElementById('sim-target-task-title');
  const taskPomsEl = document.getElementById('sim-target-task-poms');

  if (taskTitleEl && taskPomsEl) {
    if (targetTask) {
      taskTitleEl.textContent = targetTask.title;
      taskPomsEl.textContent = ` (🍅 ${targetTask.completed_poms}/${targetTask.est_poms})`;
    } else {
      taskTitleEl.textContent = 'No active task selected. Press [2] for Tasks, or [a] to add one.';
      taskPomsEl.textContent = '';
    }
  }

  // Digits color matching phase
  const digits = document.querySelectorAll('.tui-digit');
  digits.forEach(d => {
    d.style.color = state.phase === 'work' ? t.work :
                    state.phase === 'shortBreak' ? t.shortBreak : t.longBreak;
  });
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
      
      const activeTask = state.tasks.find(t => t.is_active);
      if (activeTask) {
        activeTask.completed_poms++;
        renderTasksScreen();
      }

      if (state.currentCycle >= state.longBreakInterval) {
        state.phase = 'longBreak';
        state.timeRemaining = state.longBreakMins * 60;
        state.currentCycle = 1;
      } else {
        state.phase = 'shortBreak';
        state.timeRemaining = state.shortBreakMins * 60;
        state.currentCycle++;
      }
    } else {
      playBreakChime();
      showToast('Break finished! Back to focus ⚡');
      state.phase = 'work';
      state.timeRemaining = state.workDurationMins * 60;
    }
    updateClockDisplay();
  }
}

function resetTimer() {
  state.isRunning = false;
  clearInterval(timerInterval);
  timerInterval = null;
  state.timeRemaining = state.phase === 'work' ? state.workDurationMins * 60 :
                        state.phase === 'shortBreak' ? state.shortBreakMins * 60 : state.longBreakMins * 60;
  updateClockDisplay();
  showToast('Timer reset.');
}

function skipPhase() {
  if (state.phase === 'work') {
    state.completedCycles++;
    if (state.currentCycle >= state.longBreakInterval) {
      state.phase = 'longBreak';
      state.timeRemaining = state.longBreakMins * 60;
      state.currentCycle = 1;
    } else {
      state.phase = 'shortBreak';
      state.timeRemaining = state.shortBreakMins * 60;
      state.currentCycle++;
    }
  } else {
    state.phase = 'work';
    state.timeRemaining = state.workDurationMins * 60;
  }
  updateClockDisplay();
  showToast(`Skipped to ${state.phase === 'work' ? 'Focus Work' : state.phase === 'shortBreak' ? 'Short Break' : 'Long Break'}`);
}

function switchTab(tabIndex) {
  state.currentTab = tabIndex;
  document.querySelectorAll('.terminal-tab').forEach(tab => {
    tab.classList.toggle('active', parseInt(tab.dataset.tab) === tabIndex);
  });
  document.querySelectorAll('.screen-view').forEach(screen => {
    screen.classList.toggle('active', parseInt(screen.dataset.screen) === tabIndex);
  });
  if (tabIndex === 4) {
    renderSettingsScreen();
  } else if (tabIndex === 2) {
    renderTasksScreen();
  }
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

function getFilteredTasks() {
  return state.tasks.filter(t => {
    if (state.activeFilter === 'active') return !t.completed;
    if (state.activeFilter === 'completed') return t.completed;
    return true;
  });
}

function renderTasksScreen() {
  const tbody = document.getElementById('sim-tasks-tbody');
  if (!tbody) return;

  const filtered = getFilteredTasks();
  const totalCount = state.tasks.length;
  const doneCount = state.tasks.filter(t => t.completed).length;
  const activeCount = totalCount - doneCount;

  // Update filter tabs
  const fAll = document.getElementById('tui-filter-all');
  const fAct = document.getElementById('tui-filter-active');
  const fComp = document.getElementById('tui-filter-completed');

  if (fAll) {
    fAll.textContent = `[1] All (${totalCount})`;
    fAll.classList.toggle('active', state.activeFilter === 'all');
  }
  if (fAct) {
    fAct.textContent = `[2] Active (${activeCount})`;
    fAct.classList.toggle('active', state.activeFilter === 'active');
  }
  if (fComp) {
    fComp.textContent = `[3] Completed (${doneCount})`;
    fComp.classList.toggle('active', state.activeFilter === 'completed');
  }

  if (state.selectedTaskIndex >= filtered.length) {
    state.selectedTaskIndex = Math.max(0, filtered.length - 1);
  }

  tbody.innerHTML = filtered.map((t, idx) => {
    const isSelected = idx === state.selectedTaskIndex;
    const pointer = isSelected ? '<span class="tui-primary">▶</span>' : ' ';
    const checkGlyph = t.completed ? '<span class="tui-success">✔</span>' : '<span class="tui-dim">○</span>';
    const targetBadge = t.is_active ? '<span class="tui-work">🎯 ACTIVE</span>' : '<span class="tui-dim">-</span>';
    
    let blocks = '';
    for (let b = 0; b < Math.min(t.est_poms, 10); b++) {
      blocks += b < t.completed_poms ? '■' : '□';
    }

    return `
      <tr class="tui-clickable-row ${isSelected ? 'tui-selected-row' : ''} ${t.completed ? 'completed' : ''}" data-index="${idx}">
        <td style="text-align: center; width: 4%;">${pointer}</td>
        <td style="text-align: center; width: 8%;">${checkGlyph}</td>
        <td style="width: 48%; ${t.completed ? 'text-decoration: line-through; opacity: 0.7;' : ''}">${t.title}</td>
        <td style="width: 22%;"><code class="tui-warning">${blocks}</code> ${t.completed_poms}/${t.est_poms} 🍅</td>
        <td style="width: 18%;">${targetBadge}</td>
      </tr>
    `;
  }).join('');

  tbody.querySelectorAll('tr').forEach(row => {
    row.addEventListener('click', () => {
      const idx = parseInt(row.dataset.index);
      state.selectedTaskIndex = idx;
      renderTasksScreen();
    });
  });
}

function renderStatsScreen() {
  const pillars = document.querySelectorAll('.tui-bar-fill');
  const heights = [35, 55, 75, 45, 90, 100, 65];
  pillars.forEach((p, idx) => {
    p.style.height = `${heights[idx] || 50}%`;
  });
}

// 100% Authentic Ratatui Settings Renderer matching src/ui/settings_view.rs
function getSettingItems() {
  return [
    {
      id: 0,
      name: "Focus Duration",
      value: `${state.workDurationMins} mins`,
      desc: "Length of a standard work pomodoro (1 - 120 mins)",
      type: "number"
    },
    {
      id: 1,
      name: "Short Break",
      value: `${state.shortBreakMins} mins`,
      desc: "Duration of short breaks between sessions (1 - 60 mins)",
      type: "number"
    },
    {
      id: 2,
      name: "Long Break",
      value: `${state.longBreakMins} mins`,
      desc: "Duration of long break after completing a full cycle (1 - 90 mins)",
      type: "number"
    },
    {
      id: 3,
      name: "Long Break Interval",
      value: `${state.longBreakInterval} sessions`,
      desc: "Number of focus sessions before a long break (1 - 24)",
      type: "number"
    },
    {
      id: 4,
      name: "Auto-start Breaks",
      value: state.autoStartBreaks ? "Enabled" : "Disabled",
      desc: "Automatically start countdown when entering a break",
      type: "bool"
    },
    {
      id: 5,
      name: "Auto-start Work",
      value: state.autoStartWork ? "Enabled" : "Disabled",
      desc: "Automatically start countdown after break finishes",
      type: "bool"
    },
    {
      id: 6,
      name: "Desktop Notifications",
      value: state.desktopNotifications ? "Enabled" : "Disabled",
      desc: "Send native OS desktop notification on phase completion",
      type: "bool"
    },
    {
      id: 7,
      name: "Sound / Bell Alert",
      value: state.soundEnabled ? "Enabled" : "Disabled",
      desc: "Ring audio / terminal bell when a session finishes",
      type: "bool"
    },
    {
      id: 8,
      name: "Color Theme",
      value: THEMES[currentThemeKey].name,
      desc: "Select your favorite TUI visual color scheme",
      type: "theme"
    }
  ];
}

function renderSettingsScreen() {
  const tbody = document.getElementById('sim-settings-tbody');
  if (!tbody) return;

  const items = getSettingItems();

  tbody.innerHTML = items.map((item, idx) => {
    const isSelected = idx === state.settingsIndex;
    const pointer = isSelected ? '<span class="tui-primary">▶</span>' : ' ';
    const valClass = isSelected ? 'tui-val-col tui-primary' : 'tui-val-col';

    return `
      <tr class="tui-clickable-row ${isSelected ? 'tui-selected-row' : ''}" data-index="${idx}">
        <td style="width: 3%; text-align: center;">${pointer}</td>
        <td style="width: 27%; font-weight: ${isSelected ? '700' : '500'};">${item.name}</td>
        <td class="${valClass}" style="width: 20%;">${item.value}</td>
        <td class="tui-muted" style="width: 50%;">${item.desc}</td>
      </tr>
    `;
  }).join('');

  tbody.querySelectorAll('tr').forEach(row => {
    row.addEventListener('click', () => {
      state.settingsIndex = parseInt(row.dataset.index);
      renderSettingsScreen();
    });
  });
}

function adjustSetting(delta) {
  switch (state.settingsIndex) {
    case 0: // Focus Duration (1 - 120 mins)
      state.workDurationMins = Math.max(1, Math.min(120, state.workDurationMins + delta));
      if (!state.isRunning && state.phase === 'work') {
        state.timeRemaining = state.workDurationMins * 60;
        updateClockDisplay();
      }
      showToast(`Focus Duration: ${state.workDurationMins} mins`);
      break;
    case 1: // Short Break (1 - 60 mins)
      state.shortBreakMins = Math.max(1, Math.min(60, state.shortBreakMins + delta));
      if (!state.isRunning && state.phase === 'shortBreak') {
        state.timeRemaining = state.shortBreakMins * 60;
        updateClockDisplay();
      }
      showToast(`Short Break: ${state.shortBreakMins} mins`);
      break;
    case 2: // Long Break (1 - 90 mins)
      state.longBreakMins = Math.max(1, Math.min(90, state.longBreakMins + delta));
      if (!state.isRunning && state.phase === 'longBreak') {
        state.timeRemaining = state.longBreakMins * 60;
        updateClockDisplay();
      }
      showToast(`Long Break: ${state.longBreakMins} mins`);
      break;
    case 3: // Long Break Interval (1 - 24 sessions)
      state.longBreakInterval = Math.max(1, Math.min(24, state.longBreakInterval + delta));
      updateClockDisplay();
      showToast(`Long Break Interval: ${state.longBreakInterval} sessions`);
      break;
    case 4: // Auto-start Breaks
      state.autoStartBreaks = !state.autoStartBreaks;
      showToast(`Auto-start Breaks: ${state.autoStartBreaks ? 'Enabled' : 'Disabled'}`);
      break;
    case 5: // Auto-start Work
      state.autoStartWork = !state.autoStartWork;
      showToast(`Auto-start Work: ${state.autoStartWork ? 'Enabled' : 'Disabled'}`);
      break;
    case 6: // Desktop Notifications
      state.desktopNotifications = !state.desktopNotifications;
      showToast(`Desktop Notifications: ${state.desktopNotifications ? 'Enabled' : 'Disabled'}`);
      break;
    case 7: // Sound / Bell Alert
      state.soundEnabled = !state.soundEnabled;
      soundEnabled = state.soundEnabled;
      if (soundEnabled) playBellBeep();
      showToast(`Sound / Bell Alert: ${state.soundEnabled ? 'Enabled' : 'Disabled'}`);
      break;
    case 8: // Color Theme (Cycle 18 themes)
      const themeKeys = Object.keys(THEMES);
      const curIdx = themeKeys.indexOf(currentThemeKey);
      const nextIdx = (curIdx + delta + themeKeys.length) % themeKeys.length;
      const nextKey = themeKeys[nextIdx];
      applyTheme(nextKey);
      showToast(`Theme: ${THEMES[nextKey].name}`);
      break;
  }
  renderSettingsScreen();
}

function toggleSetting() {
  if (state.settingsIndex >= 4 && state.settingsIndex <= 7) {
    adjustSetting(1);
  } else if (state.settingsIndex === 8) {
    adjustSetting(1);
  }
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
  // Global & Contextual Keyboard dispatcher matching src/app.rs
  window.addEventListener('keydown', (e) => {
    // Ignore keystrokes inside text inputs unless Enter/Escape
    if (['INPUT', 'TEXTAREA', 'SELECT'].includes(e.target.tagName)) {
      if (e.key === 'Escape') {
        toggleTaskModal(false);
        toggleHelpModal(false);
      }
      return;
    }

    // Modal open handling
    if (state.helpModalOpen || state.taskModalOpen) {
      if (e.key === 'Escape' || e.key === 'q' || e.key === '?') {
        toggleHelpModal(false);
        toggleTaskModal(false);
      }
      return;
    }

    // Global Keybindings
    if (e.key === 'q') {
      showToast('Termodoro: Running in browser simulator.');
      return;
    }
    if (e.key === '?' || e.key === 'F1') {
      toggleHelpModal(true);
      return;
    }
    if (e.key === 'Tab') {
      e.preventDefault();
      if (e.shiftKey) {
        const prevTab = state.currentTab === 1 ? 4 : state.currentTab - 1;
        switchTab(prevTab);
      } else {
        const nextTab = state.currentTab === 4 ? 1 : state.currentTab + 1;
        switchTab(nextTab);
      }
      return;
    }

    // Tab-specific keybindings
    if (state.currentTab === 1) {
      // TAB 1: TIMER
      if (e.key === ' ' || e.code === 'Space') {
        e.preventDefault();
        toggleTimer();
      } else if (e.key === 'r') {
        resetTimer();
      } else if (e.key === 's') {
        skipPhase();
      } else if (e.key === 'a') {
        toggleTaskModal(true);
      } else if (['1', '2', '3', '4'].includes(e.key)) {
        switchTab(parseInt(e.key));
      }
    } else if (state.currentTab === 2) {
      // TAB 2: TASKS
      const filtered = getFilteredTasks();
      if (e.key === '1') {
        state.activeFilter = 'all';
        state.selectedTaskIndex = 0;
        renderTasksScreen();
      } else if (e.key === '2') {
        state.activeFilter = 'active';
        state.selectedTaskIndex = 0;
        renderTasksScreen();
      } else if (e.key === '3') {
        state.activeFilter = 'completed';
        state.selectedTaskIndex = 0;
        renderTasksScreen();
      } else if (e.key === '4') {
        switchTab(4);
      } else if (e.key === 'j' || e.key === 'ArrowDown') {
        if (state.selectedTaskIndex < filtered.length - 1) {
          state.selectedTaskIndex++;
          renderTasksScreen();
        }
      } else if (e.key === 'k' || e.key === 'ArrowUp') {
        if (state.selectedTaskIndex > 0) {
          state.selectedTaskIndex--;
          renderTasksScreen();
        }
      } else if (e.key === ' ' || e.key === 'Enter') {
        const task = filtered[state.selectedTaskIndex];
        if (task) {
          task.completed = !task.completed;
          renderTasksScreen();
          showToast(`Task ${task.completed ? 'completed' : 'reopened'}`);
        }
      } else if (e.key === 't') {
        const task = filtered[state.selectedTaskIndex];
        if (task) {
          state.tasks.forEach(t => t.is_active = (t.id === task.id));
          state.targetTask = task.title;
          updateClockDisplay();
          renderTasksScreen();
          showToast(`Target set to: ${task.title}`);
        }
      } else if (e.key === 'd' || e.key === 'x') {
        const task = filtered[state.selectedTaskIndex];
        if (task) {
          state.tasks = state.tasks.filter(t => t.id !== task.id);
          if (task.is_active) {
            state.targetTask = '';
            updateClockDisplay();
          }
          renderTasksScreen();
          showToast('Task deleted.');
        }
      } else if (e.key === 'a') {
        toggleTaskModal(true);
      }
    } else if (state.currentTab === 3) {
      // TAB 3: STATS
      if (['1', '2', '3', '4'].includes(e.key)) {
        switchTab(parseInt(e.key));
      }
    } else if (state.currentTab === 4) {
      // TAB 4: SETTINGS (matching src/app.rs handle_settings_key)
      if (['1', '2', '3', '4'].includes(e.key)) {
        switchTab(parseInt(e.key));
      } else if (e.key === 'j' || e.key === 'ArrowDown') {
        state.settingsIndex = (state.settingsIndex + 1) % 9;
        renderSettingsScreen();
      } else if (e.key === 'k' || e.key === 'ArrowUp') {
        state.settingsIndex = (state.settingsIndex - 1 + 9) % 9;
        renderSettingsScreen();
      } else if (e.key === 'l' || e.key === 'ArrowRight' || e.key === '+' || e.key === '=') {
        adjustSetting(1);
      } else if (e.key === 'h' || e.key === 'ArrowLeft' || e.key === '-' || e.key === '_') {
        adjustSetting(-1);
      } else if (e.key === ' ' || e.key === 'Enter') {
        e.preventDefault();
        toggleSetting();
      }
    }
  });

  // Sound toggle button
  const soundBtn = document.getElementById('sound-toggle-btn');
  if (soundBtn) {
    soundBtn.addEventListener('click', () => {
      soundEnabled = !soundEnabled;
      state.soundEnabled = soundEnabled;
      soundBtn.innerHTML = soundEnabled ? '🔊 Sound: ON' : '🔇 Sound: OFF';
      if (soundEnabled) playBellBeep();
      showToast(`Sound ${soundEnabled ? 'Enabled' : 'Muted'}`);
      renderSettingsScreen();
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
  const fAll = document.getElementById('tui-filter-all');
  if (fAll) fAll.addEventListener('click', () => { state.activeFilter = 'all'; state.selectedTaskIndex = 0; renderTasksScreen(); });
  const fAct = document.getElementById('tui-filter-active');
  if (fAct) fAct.addEventListener('click', () => { state.activeFilter = 'active'; state.selectedTaskIndex = 0; renderTasksScreen(); });
  const fComp = document.getElementById('tui-filter-completed');
  if (fComp) fComp.addEventListener('click', () => { state.activeFilter = 'completed'; state.selectedTaskIndex = 0; renderTasksScreen(); });

  // Theme card click in gallery
  document.querySelectorAll('.theme-card').forEach(card => {
    card.addEventListener('click', () => {
      const themeKey = card.dataset.theme;
      applyTheme(themeKey);
      showToast(`Switched theme to ${THEMES[themeKey].name}`);
      renderSettingsScreen();
    });
  });

  // Audio test buttons
  const testZenBtn = document.getElementById('test-zen-btn');
  if (testZenBtn) testZenBtn.addEventListener('click', () => { playZenBowlChime(); showToast('Playing 528 Hz Zen Bowl'); });
  
  const testBreakBtn = document.getElementById('test-break-btn');
  if (testBreakBtn) testBreakBtn.addEventListener('click', () => { playBreakChime(); showToast('Playing D5 → A5 Chime'); });

  const testBellBtn = document.getElementById('test-bell-btn');
  if (testBellBtn) testBellBtn.addEventListener('click', () => { playBellBeep(); showToast('Playing 800 Hz Alert Bell'); });

  // Task creation form
  const createTaskForm = document.getElementById('modal-task-form');
  if (createTaskForm) {
    createTaskForm.addEventListener('submit', (e) => {
      e.preventDefault();
      const input = document.getElementById('modal-task-title-input');
      const estInput = document.getElementById('modal-task-est-input');

      if (input && input.value.trim()) {
        const est = parseInt(estInput.value) || 1;
        const newTask = {
          id: Date.now(),
          title: input.value.trim(),
          completed_poms: 0,
          est_poms: Math.max(1, Math.min(20, est)),
          completed: false,
          is_active: false
        };
        state.tasks.unshift(newTask);
        state.selectedTaskIndex = 0;
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
