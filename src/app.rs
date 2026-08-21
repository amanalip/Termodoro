// Import crossterm key event codes and modifiers
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// Import standard I/O stdout and Write trait for emitting terminal bell sound
use std::io::{stdout, Write};

// Import Config struct from config module
use crate::config::Config;
// Import StatsHistory from stats module
use crate::stats::StatsHistory;
// Import Storage from storage module
use crate::storage::Storage;
// Import TaskFilter and TaskManager from tasks module
use crate::tasks::{TaskFilter, TaskManager};
// Import ThemeChoice from theme module
use crate::theme::ThemeChoice;
// Import PomodoroPhase, PomodoroTimer, and TimerEvent from timer module
use crate::timer::{PomodoroPhase, PomodoroTimer, TimerEvent};

// Enum representing the four primary navigation tabs of the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    // Main Pomodoro countdown timer tab
    Timer,
    // Task management and todo list tab
    Tasks,
    // Daily statistics, streaks, and charts tab
    Stats,
    // User preferences and color theme settings tab
    Settings,
}

// Core application state holding data, active tab, timer, tasks, stats, and modals
pub struct App {
    // User configuration settings
    pub config: Config,
    // Active Pomodoro timer engine
    pub timer: PomodoroTimer,
    // Task list and active target manager
    pub tasks: TaskManager,
    // Productivity analytics and history
    pub stats: StatsHistory,
    // Storage persistence engine
    pub storage: Storage,
    // Currently focused navigation tab
    pub active_tab: ActiveTab,
    // Flag indicating whether the application should exit
    pub should_quit: bool,
    // Flag indicating whether the Help keybinding modal is open
    pub show_help: bool,
    // Flag indicating whether the Add Task modal is open
    pub show_task_modal: bool,
    // Text buffer for new task title in Add Task modal
    pub task_input_title: String,
    // Estimated pomodoros for new task in Add Task modal
    pub task_input_estimated: u32,
    // Focus index inside Add Task modal (0: Title input, 1: Estimated Pomodoros)
    pub task_modal_focus: usize,
    // Currently highlighted row index in Settings tab
    pub settings_index: usize,
    // Optional temporary notification message shown in footer
    pub status_message: Option<String>,
    // Tick counter for expiring status message
    pub status_message_ticks: usize,
    // Sub-second tick counter for pacing 1-second timer decrements (4 ticks @ 250ms = 1s)
    pub tick_count: u32,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    // Constructs and initializes the application state by loading saved data from disk
    pub fn new() -> Self {
        // Create storage handle
        let storage = Storage::new();
        Self::new_with_storage(storage)
    }

    // Constructs application state using custom storage instance
    #[allow(dead_code)]
    pub fn new_with_storage(storage: Storage) -> Self {
        // Load persisted state from storage file
        let app_data = storage.load();
        // Initialize Pomodoro timer with loaded configuration
        let timer = PomodoroTimer::new(&app_data.config);

        // Build App instance
        Self {
            // Loaded config
            config: app_data.config,
            // Initialized timer
            timer,
            // Loaded tasks
            tasks: app_data.tasks,
            // Loaded statistics
            stats: app_data.stats,
            // Storage manager
            storage,
            // Default to Timer tab on launch
            active_tab: ActiveTab::Timer,
            // Application is running
            should_quit: false,
            // Help modal is closed
            show_help: false,
            // Task modal is closed
            show_task_modal: false,
            // Empty initial task title
            task_input_title: String::new(),
            // Default estimated pomodoros for new task is 1
            task_input_estimated: 1,
            // Focus on title input initially
            task_modal_focus: 0,
            // First settings item highlighted
            settings_index: 0,
            // No status message on launch
            status_message: None,
            // 0 ticks elapsed for message
            status_message_ticks: 0,
            // 0 sub-second ticks elapsed
            tick_count: 0,
        }
    }

    // Persists current application state (config, tasks, stats) to disk
    //
    // Persistence failures are never silent: they are logged to stderr and
    // surfaced as a footer banner so a full disk or read-only mount cannot
    // quietly discard an entire session of tasks and statistics.
    pub fn save_state(&mut self) {
        // Invoke storage save method and translate any failure into user feedback
        if let Err(err) = self.storage.save(&self.config, &self.tasks, &self.stats) {
            // Always leave a diagnostic trail for terminal logs
            eprintln!("termodoro: failed to save state: {}", err);
            // Show a warning banner (reuses the standard expiry countdown)
            self.set_status_message(format!("⚠ Save failed: {}", err));
        }
    }

    // Sets a temporary notification banner message shown in the footer
    pub fn set_status_message(&mut self, message: String) {
        // Set message string
        self.status_message = Some(message);
        // Reset message expiration counter (lasts approx 10-15 seconds)
        self.status_message_ticks = 40;
    }

    // Dispatches desktop notifications and audio bells when a timer phase completes
    pub fn notify_phase_completed(&self, finished_phase: PomodoroPhase, next_phase: PomodoroPhase) {
        // Play acoustic chime and ring terminal bell if sound is enabled in configuration
        if self.config.sound_enabled {
            // Play acoustic chime for the finished phase
            crate::audio::play_phase_sound(finished_phase);
            // Write ASCII bell character (\x07) to standard output as additional fallback
            let mut out = stdout();
            let _ = out.write_all(b"\x07");
            let _ = out.flush();
        }

        // Send OS native desktop notification if enabled
        if self.config.desktop_notifications {
            // Format notification summary title
            let summary = format!("Termodoro - {} Finished!", finished_phase.title());
            // Format notification body message
            let body = format!("Time for {}! Let's go.", next_phase.title());

            // Build and send desktop notification in background thread
            std::thread::spawn(move || {
                // Use notify_rust Notification builder
                let _ = notify_rust::Notification::new()
                    // Notification summary
                    .summary(&summary)
                    // Notification body
                    .body(&body)
                    // App icon or name
                    .appname("Termodoro")
                    // Dispatch notification
                    .show();
            });
        }
    }

    // Advances the Pomodoro countdown timer by 1 second and handles phase completion lifecycles
    pub fn tick_second(&mut self) {
        // Snapshot the RUNNING phase and its scheduled duration BEFORE tick():
        // advance_phase overwrites total_duration_secs with the NEXT phase's
        // length, and the Settings tab allows editing durations mid-flight, so
        // reading self.config at completion time would log fabricated minutes
        // instead of what the session actually ran.
        let scheduled_total_secs = self.timer.total_duration_secs;

        // Tick countdown timer by 1 full second
        if let Some(event) = self.timer.tick(&self.config) {
            // Handle timer completion event
            match event {
                // Phase completed
                TimerEvent::PhaseCompleted {
                    finished_phase,
                    next_phase,
                } => {
                    // Actual minutes spent: on a natural completion the phase
                    // ran from its full scheduled duration down to zero, so
                    // the pre-tick total IS the elapsed time.
                    let dur_mins = scheduled_total_secs / 60;

                    // Extract active task information if session was Work
                    let (task_id, task_title) = if finished_phase == PomodoroPhase::Work {
                        // Increment active task pomodoro counter
                        self.tasks.increment_active_spent();
                        // Retrieve task metadata
                        if let Some(task) = self.tasks.active_task() {
                            (Some(task.id.clone()), Some(task.title.clone()))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    };

                    // Record completed session in historical analytics
                    self.stats
                        .record(finished_phase, dur_mins, task_id, task_title);
                    // Trigger audio bell and desktop notification
                    self.notify_phase_completed(finished_phase, next_phase);
                    // Update status banner
                    self.set_status_message(format!(
                        "{} completed! Next: {}",
                        finished_phase.title(),
                        next_phase.title()
                    ));
                    // Automatically persist state to disk
                    self.save_state();
                }
            }
        }
    }

    // Periodic tick method invoked on every event loop tick interval (~250ms)
    pub fn on_tick(&mut self) {
        // Increment sub-second tick counter (4 ticks @ 250ms = 1 full second)
        self.tick_count = self.tick_count.wrapping_add(1);
        if self.tick_count.is_multiple_of(4) {
            // Advance timer by 1 second
            self.tick_second();
        }

        // Decrement status message counter if active
        if self.status_message_ticks > 0 {
            // Decrement ticks
            self.status_message_ticks -= 1;
            // Clear message once expired
            if self.status_message_ticks == 0 {
                // Reset status message
                self.status_message = None;
            }
        }
    }

    // Main key event routing dispatcher
    pub fn on_key_event(&mut self, key: KeyEvent) {
        // If Help modal is open, forward key events to help handler
        if self.show_help {
            // Handle help key
            self.handle_help_key(key);
            // Return early
            return;
        }

        // If Add Task modal is open, forward key events to task modal handler
        if self.show_task_modal {
            // Handle task modal key
            self.handle_task_modal_key(key);
            // Return early
            return;
        }

        // Global keybindings available across all tabs
        match key.code {
            // 'q' key quits the application
            KeyCode::Char('q') => {
                // Set should_quit flag to true
                self.should_quit = true;
                // Return
                return;
            }
            // '?' key toggles Help modal
            KeyCode::Char('?') => {
                // Open help dialog
                self.show_help = true;
                // Return
                return;
            }
            // Tab key switches to next tab
            KeyCode::Tab => {
                // Check if Shift modifier was pressed (backward tab)
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    // Switch to previous tab
                    self.previous_tab();
                } else {
                    // Switch to next tab
                    self.next_tab();
                }
                // Return
                return;
            }
            // BackTab key (Shift+Tab on some terminals) switches to previous tab
            KeyCode::BackTab => {
                // Switch to previous tab
                self.previous_tab();
                // Return
                return;
            }
            // Number keys 1-4 jump directly to specific tab
            KeyCode::Char('1') if self.active_tab != ActiveTab::Tasks => {
                // Jump to Timer tab
                self.active_tab = ActiveTab::Timer;
                // Return
                return;
            }
            KeyCode::Char('2') if self.active_tab != ActiveTab::Tasks => {
                // Jump to Tasks tab
                self.active_tab = ActiveTab::Tasks;
                // Return
                return;
            }
            KeyCode::Char('3') if self.active_tab != ActiveTab::Tasks => {
                // Jump to Stats tab
                self.active_tab = ActiveTab::Stats;
                // Return
                return;
            }
            KeyCode::Char('4') => {
                // Jump to Settings tab
                self.active_tab = ActiveTab::Settings;
                // Return
                return;
            }
            // Other keys dispatched to active tab handler
            _ => {}
        }

        // Dispatch key event to the active tab's dedicated input handler
        match self.active_tab {
            // Dispatch to Timer tab handler
            ActiveTab::Timer => self.handle_timer_key(key),
            // Dispatch to Tasks tab handler
            ActiveTab::Tasks => self.handle_tasks_key(key),
            // Dispatch to Stats tab handler
            ActiveTab::Stats => self.handle_stats_key(key),
            // Dispatch to Settings tab handler
            ActiveTab::Settings => self.handle_settings_key(key),
        }
    }

    // Switches active tab to the next sequential tab
    pub fn next_tab(&mut self) {
        // Match current tab
        self.active_tab = match self.active_tab {
            ActiveTab::Timer => ActiveTab::Tasks,
            ActiveTab::Tasks => ActiveTab::Stats,
            ActiveTab::Stats => ActiveTab::Settings,
            ActiveTab::Settings => ActiveTab::Timer,
        };
    }

    // Switches active tab to the previous sequential tab
    pub fn previous_tab(&mut self) {
        // Match current tab
        self.active_tab = match self.active_tab {
            ActiveTab::Timer => ActiveTab::Settings,
            ActiveTab::Tasks => ActiveTab::Timer,
            ActiveTab::Stats => ActiveTab::Tasks,
            ActiveTab::Settings => ActiveTab::Stats,
        };
    }

    // Handles key events inside the Help popup modal
    fn handle_help_key(&mut self, key: KeyEvent) {
        // Match key code
        match key.code {
            // Esc, 'q', or '?' closes the Help dialog
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::Enter => {
                // Close help modal
                self.show_help = false;
            }
            // Ignore other keys
            _ => {}
        }
    }

    // Opens the Add Task creation modal dialog
    pub fn open_task_modal(&mut self) {
        // Clear title input
        self.task_input_title.clear();
        // Default estimated pomodoros
        self.task_input_estimated = 1;
        // Focus on title input
        self.task_modal_focus = 0;
        // Show modal flag
        self.show_task_modal = true;
    }

    // Handles key events inside the Add Task modal dialog
    fn handle_task_modal_key(&mut self, key: KeyEvent) {
        // Match key code
        match key.code {
            // Esc closes the modal without saving
            KeyCode::Esc => {
                // Close modal
                self.show_task_modal = false;
            }
            // Tab or Down switches focus between Title and Estimated Pomodoros
            KeyCode::Tab | KeyCode::Down => {
                // Cycle focus index between 0 and 1
                self.task_modal_focus = (self.task_modal_focus + 1) % 2;
            }
            // BackTab or Up switches focus upward
            KeyCode::BackTab | KeyCode::Up => {
                // Invert focus index
                self.task_modal_focus = if self.task_modal_focus == 0 { 1 } else { 0 };
            }
            // Enter key saves the task if title is valid
            KeyCode::Enter => {
                // Check if title is not blank
                if !self.task_input_title.trim().is_empty() {
                    // Store the same trimmed title the list will display so
                    // the confirmation toast echoes what the user actually
                    // sees, not raw pre-trim buffer contents
                    let trimmed_title = self.task_input_title.trim().to_string();
                    // Add task to task manager
                    self.tasks
                        .add(trimmed_title.clone(), self.task_input_estimated);
                    // Close task modal
                    self.show_task_modal = false;
                    // Show confirmation notification
                    self.set_status_message(format!("Task added: {}", trimmed_title));
                    // Persist state to disk
                    self.save_state();
                }
            }
            // Key handling when focused on Title input (field 0)
            _ if self.task_modal_focus == 0 => match key.code {
                // Append character to title buffer
                KeyCode::Char(c) => {
                    // Push character
                    self.task_input_title.push(c);
                }
                // Backspace removes last character
                KeyCode::Backspace => {
                    // Pop character
                    self.task_input_title.pop();
                }
                // Ignore other keys
                _ => {}
            },
            // Key handling when focused on Estimated Pomodoros (field 1)
            _ if self.task_modal_focus == 1 => match key.code {
                // Right arrow, '+', '=', or 'l' increments estimated pomodoros
                KeyCode::Right | KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char('l') => {
                    // Maximum limit 20 pomodoros
                    if self.task_input_estimated < 20 {
                        // Increment
                        self.task_input_estimated += 1;
                    }
                }
                // Left arrow, '-', '_', or 'h' decrements estimated pomodoros
                KeyCode::Left | KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Char('h') => {
                    // Minimum limit 1 pomodoro
                    if self.task_input_estimated > 1 {
                        // Decrement
                        self.task_input_estimated -= 1;
                    }
                }
                // Digit keys directly set estimated value
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    // Parse digit
                    if let Some(digit) = c.to_digit(10) {
                        // Update if greater than 0
                        if digit > 0 {
                            // Set estimated
                            self.task_input_estimated = digit;
                        }
                    }
                }
                // Ignore other keys
                _ => {}
            },
            // Fallback
            _ => {}
        }
    }

    // Handles key events when viewing the Pomodoro Timer tab
    fn handle_timer_key(&mut self, key: KeyEvent) {
        // Match key code
        match key.code {
            // Space toggles start / pause
            KeyCode::Char(' ') => {
                // Toggle timer
                self.timer.toggle();
            }
            // 'r' resets the current timer countdown
            KeyCode::Char('r') => {
                // Reset timer
                self.timer.reset(&self.config);
                // Set notification message
                self.set_status_message("Timer reset.".to_string());
            }
            // 's' skips to the next Pomodoro phase
            KeyCode::Char('s') => {
                // Skip to next phase
                let next = self.timer.skip_to_next(&self.config);
                // Set notification message
                self.set_status_message(format!("Skipped to {}", next.title()));
            }
            // 'a' opens Add Task creation dialog
            KeyCode::Char('a') => {
                // Open task modal
                self.open_task_modal();
            }
            // Ignore other keys
            _ => {}
        }
    }

    // Handles key events when viewing the Tasks tab
    fn handle_tasks_key(&mut self, key: KeyEvent) {
        // Match key code
        match key.code {
            // 'a' opens Add Task creation dialog
            KeyCode::Char('a') => {
                // Open task modal
                self.open_task_modal();
            }
            // Space or Enter toggles completed status of selected task
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Toggle task completion
                self.tasks.toggle_selected();
                // Persist state
                self.save_state();
            }
            // 't' assigns selected task as active timer target
            KeyCode::Char('t') => {
                // set_selected_active refuses completed tasks so finished
                // work can never become the focus target
                if self.tasks.set_selected_active() {
                    // Set status notification
                    if let Some(task) = self.tasks.active_task() {
                        // Status banner
                        self.set_status_message(format!("Target set to: {}", task.title));
                    }
                } else if self.tasks.tasks.is_empty() {
                    // Nothing in the list at all
                    self.set_status_message("No task to target.".to_string());
                } else {
                    // Selection exists but is completed (or filtered out)
                    self.set_status_message("Cannot target a completed task.".to_string());
                }
                // Persist state
                self.save_state();
            }
            // 'd' or 'x' deletes selected task
            KeyCode::Char('d') | KeyCode::Char('x') => {
                // Remove task; remove_selected reports whether anything was
                // actually deleted so we never announce a deletion that did
                // not happen (for example on an empty or filtered-out list)
                if self.tasks.remove_selected() {
                    // Set notification
                    self.set_status_message("Task deleted.".to_string());
                } else {
                    // Nothing was removed; tell the user instead of lying
                    self.set_status_message("No task selected to delete.".to_string());
                }
                // Persist state
                self.save_state();
            }
            // Down arrow or 'j' moves cursor down
            KeyCode::Down | KeyCode::Char('j') => {
                // Move down
                self.tasks.next();
            }
            // Up arrow or 'k' moves cursor up
            KeyCode::Up | KeyCode::Char('k') => {
                // Move up
                self.tasks.previous();
            }
            // '1' sets filter to All tasks
            KeyCode::Char('1') => {
                // Set filter to All
                self.tasks.filter = TaskFilter::All;
                // Reset selected index
                self.tasks.selected_index = 0;
            }
            // '2' sets filter to Active tasks
            KeyCode::Char('2') => {
                // Set filter to Active
                self.tasks.filter = TaskFilter::Active;
                // Reset selected index
                self.tasks.selected_index = 0;
            }
            // '3' sets filter to Completed tasks
            KeyCode::Char('3') => {
                // Set filter to Completed
                self.tasks.filter = TaskFilter::Completed;
                // Reset selected index
                self.tasks.selected_index = 0;
            }
            // Ignore other keys
            _ => {}
        }
    }

    // Handles key events when viewing the Stats tab
    fn handle_stats_key(&mut self, _key: KeyEvent) {
        // No interactive controls on stats dashboard beyond global navigation
    }

    // Handles key events when viewing the Settings tab
    fn handle_settings_key(&mut self, key: KeyEvent) {
        // Match key code
        match key.code {
            // Down arrow or 'j' moves to next setting row
            KeyCode::Down | KeyCode::Char('j') => {
                // Maximum 8 setting rows (0..=8)
                if self.settings_index < 8 {
                    // Increment row
                    self.settings_index += 1;
                } else {
                    // Wrap to first row
                    self.settings_index = 0;
                }
            }
            // Up arrow or 'k' moves to previous setting row
            KeyCode::Up | KeyCode::Char('k') => {
                // Decrement row
                if self.settings_index > 0 {
                    // Decrement
                    self.settings_index -= 1;
                } else {
                    // Wrap to last row
                    self.settings_index = 8;
                }
            }
            // Right arrow, '+', '=', or 'l' increments setting value
            KeyCode::Right | KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char('l') => {
                // Adjust setting positive
                self.adjust_setting(1);
            }
            // Left arrow, '-', '_', or 'h' decrements setting value
            KeyCode::Left | KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Char('h') => {
                // Adjust setting negative
                self.adjust_setting(-1);
            }
            // Space or Enter toggles boolean setting flags
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Toggle setting
                self.toggle_setting();
            }
            // Ignore other keys
            _ => {}
        }
    }

    // Modifies numerical settings or cycles themes by delta (+1 or -1)
    fn adjust_setting(&mut self, delta: i32) {
        // Match on active settings row index
        match self.settings_index {
            // Setting 0: Work duration minutes
            0 => {
                // New value clamped between 1 and 120 minutes
                let new_val = (self.config.work_duration_mins as i32 + delta).clamp(1, 120) as u32;
                // Update config
                self.config.work_duration_mins = new_val;
                // Reset timer if currently stopped
                if self.timer.status == crate::timer::TimerStatus::Stopped
                    && self.timer.phase == PomodoroPhase::Work
                {
                    self.timer.reset(&self.config);
                }
            }
            // Setting 1: Short break minutes
            1 => {
                // New value clamped between 1 and 60 minutes
                let new_val = (self.config.short_break_mins as i32 + delta).clamp(1, 60) as u32;
                // Update config
                self.config.short_break_mins = new_val;
                // Reset timer if in ShortBreak and stopped
                if self.timer.status == crate::timer::TimerStatus::Stopped
                    && self.timer.phase == PomodoroPhase::ShortBreak
                {
                    self.timer.reset(&self.config);
                }
            }
            // Setting 2: Long break minutes
            2 => {
                // New value clamped between 1 and 90 minutes
                let new_val = (self.config.long_break_mins as i32 + delta).clamp(1, 90) as u32;
                // Update config
                self.config.long_break_mins = new_val;
                // Reset timer if in LongBreak and stopped
                if self.timer.status == crate::timer::TimerStatus::Stopped
                    && self.timer.phase == PomodoroPhase::LongBreak
                {
                    self.timer.reset(&self.config);
                }
            }
            // Setting 3: Long break interval (sessions count)
            3 => {
                // New value clamped between 1 and 24 sessions
                let new_val = (self.config.long_break_interval as i32 + delta).clamp(1, 24) as u32;
                // Update config
                self.config.long_break_interval = new_val;
            }
            // Setting 4: Auto-start breaks toggle
            4 => {
                // Toggle flag
                self.config.auto_start_breaks = !self.config.auto_start_breaks;
            }
            // Setting 5: Auto-start work toggle
            5 => {
                // Toggle flag
                self.config.auto_start_work = !self.config.auto_start_work;
            }
            // Setting 6: Desktop notifications toggle
            6 => {
                // Toggle flag
                self.config.desktop_notifications = !self.config.desktop_notifications;
            }
            // Setting 7: Sound enabled toggle
            7 => {
                // Toggle flag
                self.config.sound_enabled = !self.config.sound_enabled;
            }
            // Setting 8: Color theme selection
            8 => {
                // Get all theme choices
                let all_themes = ThemeChoice::all();
                // Find current theme index
                let cur_idx = all_themes
                    .iter()
                    .position(|&t| t == self.config.theme)
                    .unwrap_or(0);
                // Calculate next theme index with wrapping
                let next_idx = if delta > 0 {
                    (cur_idx + 1) % all_themes.len()
                } else {
                    (cur_idx + all_themes.len() - 1) % all_themes.len()
                };
                // Update theme choice
                self.config.theme = all_themes[next_idx];
            }
            // Fallback
            _ => {}
        }
        // Save updated preferences to disk
        self.save_state();
    }

    // Toggles boolean feature flags or cycles theme on Enter / Space press
    fn toggle_setting(&mut self) {
        // Re-use adjust_setting with +1
        self.adjust_setting(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    fn make_key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    fn make_key_with_mod(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    fn create_test_app() -> (App, std::path::PathBuf) {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_app_test_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let app = App::new_with_storage(storage);
        (app, temp_dir)
    }

    #[test]
    fn test_tab_navigation_methods() {
        let (mut app, temp_dir) = create_test_app();
        assert_eq!(app.active_tab, ActiveTab::Timer);

        app.next_tab();
        assert_eq!(app.active_tab, ActiveTab::Tasks);

        app.next_tab();
        assert_eq!(app.active_tab, ActiveTab::Stats);

        app.next_tab();
        assert_eq!(app.active_tab, ActiveTab::Settings);

        app.next_tab();
        assert_eq!(app.active_tab, ActiveTab::Timer);

        app.previous_tab();
        assert_eq!(app.active_tab, ActiveTab::Settings);

        app.previous_tab();
        assert_eq!(app.active_tab, ActiveTab::Stats);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_global_keys_tab_navigation() {
        let (mut app, temp_dir) = create_test_app();

        // Jump to Tasks with '2'
        app.on_key_event(make_key(KeyCode::Char('2')));
        assert_eq!(app.active_tab, ActiveTab::Tasks);

        // From Tasks tab, jumping with '4' (Settings)
        app.on_key_event(make_key(KeyCode::Char('4')));
        assert_eq!(app.active_tab, ActiveTab::Settings);

        // Jump to Stats with '3'
        app.on_key_event(make_key(KeyCode::Char('3')));
        assert_eq!(app.active_tab, ActiveTab::Stats);

        // Jump to Timer with '1'
        app.on_key_event(make_key(KeyCode::Char('1')));
        assert_eq!(app.active_tab, ActiveTab::Timer);

        // Tab switches to next tab
        app.on_key_event(make_key(KeyCode::Tab));
        assert_eq!(app.active_tab, ActiveTab::Tasks);

        // Shift+Tab switches to previous tab
        app.on_key_event(make_key_with_mod(KeyCode::Tab, KeyModifiers::SHIFT));
        assert_eq!(app.active_tab, ActiveTab::Timer);

        // BackTab switches to previous tab
        app.on_key_event(make_key(KeyCode::BackTab));
        assert_eq!(app.active_tab, ActiveTab::Settings);

        // 'q' quits
        app.on_key_event(make_key(KeyCode::Char('q')));
        assert!(app.should_quit);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_tasks_tab_filter_keys_do_not_switch_tabs() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Tasks;

        // Press '2' on Tasks tab -> switches filter to Active
        app.on_key_event(make_key(KeyCode::Char('2')));
        assert_eq!(app.active_tab, ActiveTab::Tasks);
        assert_eq!(app.tasks.filter, TaskFilter::Active);

        // Press '3' on Tasks tab -> switches filter to Completed
        app.on_key_event(make_key(KeyCode::Char('3')));
        assert_eq!(app.active_tab, ActiveTab::Tasks);
        assert_eq!(app.tasks.filter, TaskFilter::Completed);

        // Press '1' on Tasks tab -> switches filter to All
        app.on_key_event(make_key(KeyCode::Char('1')));
        assert_eq!(app.active_tab, ActiveTab::Tasks);
        assert_eq!(app.tasks.filter, TaskFilter::All);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_task_modal_key_interactions() {
        let (mut app, temp_dir) = create_test_app();

        // Open modal with 'a'
        app.on_key_event(make_key(KeyCode::Char('a')));
        assert!(app.show_task_modal);
        assert_eq!(app.task_modal_focus, 0);

        // Type "Test"
        for c in "Test".chars() {
            app.on_key_event(make_key(KeyCode::Char(c)));
        }
        assert_eq!(app.task_input_title, "Test");

        // Backspace
        app.on_key_event(make_key(KeyCode::Backspace));
        assert_eq!(app.task_input_title, "Tes");
        app.on_key_event(make_key(KeyCode::Char('t')));
        assert_eq!(app.task_input_title, "Test");

        // Switch focus to estimated pomodoros
        app.on_key_event(make_key(KeyCode::Down));
        assert_eq!(app.task_modal_focus, 1);

        // Increment with '+'
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.task_input_estimated, 2);

        // Direct digit key '4'
        app.on_key_event(make_key(KeyCode::Char('4')));
        assert_eq!(app.task_input_estimated, 4);

        // Decrement with '-'
        app.on_key_event(make_key(KeyCode::Char('-')));
        assert_eq!(app.task_input_estimated, 3);

        // Submit with Enter
        app.on_key_event(make_key(KeyCode::Enter));
        assert!(!app.show_task_modal);
        assert_eq!(app.tasks.tasks.len(), 1);
        assert_eq!(app.tasks.tasks[0].title, "Test");
        assert_eq!(app.tasks.tasks[0].pomodoros_estimated, 3);
        assert!(app.status_message.is_some());

        // Cancel modal with Esc
        app.on_key_event(make_key(KeyCode::Char('a')));
        assert!(app.show_task_modal);
        app.on_key_event(make_key(KeyCode::Esc));
        assert!(!app.show_task_modal);
        assert_eq!(app.tasks.tasks.len(), 1);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_help_modal_workflow() {
        let (mut app, temp_dir) = create_test_app();
        app.on_key_event(make_key(KeyCode::Char('?')));
        assert!(app.show_help);

        app.on_key_event(make_key(KeyCode::Esc));
        assert!(!app.show_help);

        app.on_key_event(make_key(KeyCode::Char('?')));
        assert!(app.show_help);
        app.on_key_event(make_key(KeyCode::Char('q')));
        assert!(!app.show_help);
        // Quitting from inside help modal should not quit the app
        assert!(!app.should_quit);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_settings_adjustments_and_clamping() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;

        // Row 0: Work duration (default 25)
        app.settings_index = 0;
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.config.work_duration_mins, 26);
        // Underflow clamp test
        for _ in 0..50 {
            app.on_key_event(make_key(KeyCode::Char('-')));
        }
        assert_eq!(app.config.work_duration_mins, 1);

        // Row 1: Short break (default 5, clamped 1..=60)
        app.settings_index = 1;
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.config.short_break_mins, 6);

        // Row 2: Long break (default 15, clamped 1..=90)
        app.settings_index = 2;
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.config.long_break_mins, 16);

        // Row 3: Long break interval (default 4, clamped 1..=12)
        app.settings_index = 3;
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.config.long_break_interval, 5);

        // Row 4: Auto start breaks (toggle)
        app.settings_index = 4;
        let initial_auto_breaks = app.config.auto_start_breaks;
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert_eq!(app.config.auto_start_breaks, !initial_auto_breaks);

        // Row 5: Auto start work (toggle)
        app.settings_index = 5;
        let initial_auto_work = app.config.auto_start_work;
        app.on_key_event(make_key(KeyCode::Enter));
        assert_eq!(app.config.auto_start_work, !initial_auto_work);

        // Row 6: Desktop notifications (toggle)
        app.settings_index = 6;
        let initial_notifs = app.config.desktop_notifications;
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert_eq!(app.config.desktop_notifications, !initial_notifs);

        // Row 7: Sound enabled (toggle)
        app.settings_index = 7;
        let initial_sound = app.config.sound_enabled;
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert_eq!(app.config.sound_enabled, !initial_sound);

        // Row 8: Theme cycling
        app.settings_index = 8;
        app.config.theme = ThemeChoice::CatppuccinMocha;
        app.on_key_event(make_key(KeyCode::Char('+')));
        assert_eq!(app.config.theme, ThemeChoice::CatppuccinMacchiato);
        app.on_key_event(make_key(KeyCode::Char('-')));
        assert_eq!(app.config.theme, ThemeChoice::CatppuccinMocha);

        // Settings navigation wrapping
        app.settings_index = 8;
        app.on_key_event(make_key(KeyCode::Down));
        assert_eq!(app.settings_index, 0);
        app.on_key_event(make_key(KeyCode::Up));
        assert_eq!(app.settings_index, 8);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_timer_keys_and_on_tick_flow() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Timer;

        // Toggle timer with Space
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert_eq!(app.timer.status, crate::timer::TimerStatus::Running);

        // Skip timer with 's'
        app.on_key_event(make_key(KeyCode::Char('s')));
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);

        // Reset timer with 'r'
        app.on_key_event(make_key(KeyCode::Char('r')));
        assert_eq!(app.timer.status, crate::timer::TimerStatus::Stopped);
        assert_eq!(app.timer.time_remaining_secs, 5 * 60);

        // Add task and test tick_second completion
        app.tasks.add("Focus Task".to_string(), 2);
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;

        // Tick 1 second to complete phase
        app.tick_second();
        assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);
        assert_eq!(app.stats.sessions.len(), 1);
        assert_eq!(app.stats.sessions[0].phase, PomodoroPhase::Work);
        assert!(app.status_message.is_some());

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_full_pomodoro_cycle_e2e() {
        let (mut app, temp_dir) = create_test_app();
        app.config.auto_start_breaks = true;
        app.config.auto_start_work = true;

        // User adds Task A (estimate 3) and Task B (estimate 1)
        app.tasks.add("Task A".to_string(), 3);
        app.tasks.add("Task B".to_string(), 1);

        // Cycle 1: Work (Task A active)
        assert_eq!(app.timer.current_cycle, 1);
        assert_eq!(app.timer.phase, PomodoroPhase::Work);
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes Work 1 -> ShortBreak 1 (cycle 2)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 2);
        assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);

        // Complete ShortBreak 1
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes ShortBreak 1 -> Work 2
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Switch target to Task B
        app.tasks.selected_index = 1;
        app.tasks.set_selected_active();
        assert_eq!(app.tasks.active_task().unwrap().title, "Task B");

        // Cycle 2: Work (Task B active)
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes Work 2 -> ShortBreak 2 (cycle 3)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 3);
        assert_eq!(app.tasks.tasks[1].pomodoros_spent, 1);

        // Complete ShortBreak 2
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes ShortBreak 2 -> Work 3
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Cycle 3: Work
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes Work 3 -> ShortBreak 3 (cycle 4)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 4);

        // Complete ShortBreak 3
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes ShortBreak 3 -> Work 4
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Cycle 4: Work (at cycle 4)
        app.timer.time_remaining_secs = 1;
        app.tick_second(); // Completes Work 4 -> LongBreak! Cycle resets to 1
        assert_eq!(app.timer.phase, PomodoroPhase::LongBreak);
        assert_eq!(app.timer.current_cycle, 1);
        assert_eq!(app.timer.completed_pomodoros, 4);

        // Verify aggregated stats
        assert_eq!(app.stats.total_work_sessions(), 4);
        assert_eq!(app.stats.today_work_sessions(), 4);
        assert_eq!(app.stats.today_focus_minutes(), 100); // 4 * 25m

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_restart_and_state_recovery_e2e() {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_recovery_test_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");

        // Session 1: Launch app, modify settings, add tasks, record stats, save state
        {
            let storage1 = Storage::with_path(file_path.clone());
            let mut app1 = App::new_with_storage(storage1);

            // Customize settings
            app1.config.work_duration_mins = 45;
            app1.config.short_break_mins = 10;
            app1.config.theme = ThemeChoice::Dracula;

            // Add tasks
            app1.tasks.add("Persistent Task 1".to_string(), 4);
            app1.tasks.add("Persistent Task 2".to_string(), 2);
            app1.tasks.toggle_selected(); // Mark Task 2 completed

            // Record some stats
            app1.stats.record(
                PomodoroPhase::Work,
                45,
                Some("Persistent Task 1".to_string()),
                Some("Persistent Task 1".to_string()),
            );

            app1.save_state();
        }

        // Session 2: Launch fresh app from same storage path and verify state
        {
            let storage2 = Storage::with_path(file_path.clone());
            let app2 = App::new_with_storage(storage2);

            // Verify configuration recovery
            assert_eq!(app2.config.work_duration_mins, 45);
            assert_eq!(app2.config.short_break_mins, 10);
            assert_eq!(app2.config.theme, ThemeChoice::Dracula);

            // Verify tasks recovery
            assert_eq!(app2.tasks.tasks.len(), 2);
            assert_eq!(app2.tasks.tasks[0].title, "Persistent Task 1");
            assert_eq!(app2.tasks.tasks[0].pomodoros_estimated, 4);
            assert!(!app2.tasks.tasks[0].completed);
            assert_eq!(app2.tasks.tasks[1].title, "Persistent Task 2");
            assert!(app2.tasks.tasks[1].completed);

            // Verify stats recovery
            assert_eq!(app2.stats.total_work_sessions(), 1);
            assert_eq!(app2.stats.total_focus_minutes(), 45);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_tasks_tab_key_interactions() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Tasks;

        // On empty list, keys should not crash
        app.on_key_event(make_key(KeyCode::Char('j')));
        app.on_key_event(make_key(KeyCode::Char('k')));
        app.on_key_event(make_key(KeyCode::Char('d')));
        app.on_key_event(make_key(KeyCode::Char('x')));
        app.on_key_event(make_key(KeyCode::Char('t')));
        app.on_key_event(make_key(KeyCode::Char(' ')));
        app.on_key_event(make_key(KeyCode::Enter));
        assert_eq!(app.tasks.tasks.len(), 0);

        // Add 3 tasks
        app.tasks.add("Task 1".to_string(), 2);
        app.tasks.add("Task 2".to_string(), 1);
        app.tasks.add("Task 3".to_string(), 3);

        // Navigate with j and k
        app.tasks.selected_index = 0;
        app.on_key_event(make_key(KeyCode::Char('j')));
        assert_eq!(app.tasks.selected_index, 1);

        app.on_key_event(make_key(KeyCode::Char('k')));
        assert_eq!(app.tasks.selected_index, 0);

        // Toggle task completion with Space
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert!(app.tasks.tasks[0].completed);

        // Toggle back with Enter
        app.on_key_event(make_key(KeyCode::Enter));
        assert!(!app.tasks.tasks[0].completed);

        // Set active target with 't'
        app.tasks.selected_index = 2;
        app.on_key_event(make_key(KeyCode::Char('t')));
        assert_eq!(app.tasks.active_task().unwrap().title, "Task 3");
        assert!(app
            .status_message
            .as_ref()
            .unwrap()
            .contains("Target set to: Task 3"));

        // Delete with 'd'
        app.tasks.selected_index = 1;
        app.on_key_event(make_key(KeyCode::Char('d')));
        assert_eq!(app.tasks.tasks.len(), 2);
        assert_eq!(app.tasks.tasks[0].title, "Task 1");
        assert_eq!(app.tasks.tasks[1].title, "Task 3");

        // Delete with 'x'
        app.tasks.selected_index = 0;
        app.on_key_event(make_key(KeyCode::Char('x')));
        assert_eq!(app.tasks.tasks.len(), 1);
        assert_eq!(app.tasks.tasks[0].title, "Task 3");

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_settings_tab_vim_keys_and_live_timer_updates() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;

        // Navigate settings rows with 'j' and 'k'
        app.settings_index = 0;
        app.on_key_event(make_key(KeyCode::Char('j')));
        assert_eq!(app.settings_index, 1);
        app.on_key_event(make_key(KeyCode::Char('k')));
        assert_eq!(app.settings_index, 0);

        // Adjust setting with 'l' (+) and 'h' (-)
        app.on_key_event(make_key(KeyCode::Char('l')));
        assert_eq!(app.config.work_duration_mins, 26);
        assert_eq!(app.timer.time_remaining_secs, 26 * 60);

        app.on_key_event(make_key(KeyCode::Char('h')));
        assert_eq!(app.config.work_duration_mins, 25);
        assert_eq!(app.timer.time_remaining_secs, 25 * 60);

        // Adjust setting with '=' (+) and '_' (-)
        app.on_key_event(make_key(KeyCode::Char('=')));
        assert_eq!(app.config.work_duration_mins, 26);
        assert_eq!(app.timer.time_remaining_secs, 26 * 60);

        app.on_key_event(make_key(KeyCode::Char('_')));
        assert_eq!(app.config.work_duration_mins, 25);
        assert_eq!(app.timer.time_remaining_secs, 25 * 60);

        // When timer is running, changing work duration does not reset countdown
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 500;
        app.on_key_event(make_key(KeyCode::Char('l')));
        assert_eq!(app.config.work_duration_mins, 26);
        assert_eq!(app.timer.time_remaining_secs, 500); // unaffected while running
        app.timer.status = crate::timer::TimerStatus::Stopped;

        // Live update for ShortBreak
        app.settings_index = 1;
        app.timer.phase = PomodoroPhase::ShortBreak;
        app.timer.status = crate::timer::TimerStatus::Stopped;
        app.timer.time_remaining_secs = 5 * 60;
        app.on_key_event(make_key(KeyCode::Char('l'))); // +1 min to 6 mins
        assert_eq!(app.config.short_break_mins, 6);
        assert_eq!(app.timer.time_remaining_secs, 6 * 60);

        // Live update for LongBreak
        app.settings_index = 2;
        app.timer.phase = PomodoroPhase::LongBreak;
        app.timer.status = crate::timer::TimerStatus::Stopped;
        app.timer.time_remaining_secs = 15 * 60;
        app.on_key_event(make_key(KeyCode::Char('l'))); // +1 min to 16 mins
        assert_eq!(app.config.long_break_mins, 16);
        assert_eq!(app.timer.time_remaining_secs, 16 * 60);

        // Clamping for long break interval (row 3)
        app.settings_index = 3;
        for _ in 0..30 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.long_break_interval, 24);
        for _ in 0..30 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.long_break_interval, 1);

        // Theme wrap-around backwards
        app.settings_index = 8;
        app.config.theme = ThemeChoice::CatppuccinMocha;
        app.on_key_event(make_key(KeyCode::Char('h')));
        assert_eq!(app.config.theme, ThemeChoice::OledPhosphor);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_status_message_expiration_on_ticks() {
        let (mut app, temp_dir) = create_test_app();
        app.set_status_message("Temporary Notification".to_string());
        assert_eq!(
            app.status_message.as_deref(),
            Some("Temporary Notification")
        );
        assert_eq!(app.status_message_ticks, 40);

        // Tick 39 times
        for _ in 0..39 {
            app.on_tick();
            assert!(app.status_message.is_some());
        }
        assert_eq!(app.status_message_ticks, 1);

        // Final 40th tick expires the status message
        app.on_tick();
        assert_eq!(app.status_message_ticks, 0);
        assert_eq!(app.status_message, None);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_task_modal_validation_and_bounds() {
        let (mut app, temp_dir) = create_test_app();
        app.open_task_modal();
        assert!(app.show_task_modal);

        // Submitting with whitespace only should be rejected
        app.task_input_title = "     ".to_string();
        app.on_key_event(make_key(KeyCode::Enter));
        assert!(app.show_task_modal); // Still open
        assert_eq!(app.tasks.tasks.len(), 0);

        // Focus navigation with Up and BackTab
        app.task_modal_focus = 1;
        app.on_key_event(make_key(KeyCode::Up));
        assert_eq!(app.task_modal_focus, 0);

        app.task_modal_focus = 1;
        app.on_key_event(make_key(KeyCode::BackTab));
        assert_eq!(app.task_modal_focus, 0);

        // Bounds on estimated pomodoros
        app.task_modal_focus = 1;
        // Upper bound 20
        for _ in 0..25 {
            app.on_key_event(make_key(KeyCode::Right));
        }
        assert_eq!(app.task_input_estimated, 20);

        // Lower bound 1
        for _ in 0..25 {
            app.on_key_event(make_key(KeyCode::Left));
        }
        assert_eq!(app.task_input_estimated, 1);

        // Test '_' to decrement and '=' to increment
        app.task_input_estimated = 3;
        app.on_key_event(make_key(KeyCode::Char('_')));
        assert_eq!(app.task_input_estimated, 2);

        app.on_key_event(make_key(KeyCode::Char('=')));
        assert_eq!(app.task_input_estimated, 3);

        // Digit '0' should not change estimated to 0
        app.on_key_event(make_key(KeyCode::Char('0')));
        assert_eq!(app.task_input_estimated, 3);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_notify_phase_completed_sound_and_notification_flags() {
        let (mut app, temp_dir) = create_test_app();
        // Scoped mute: holds the shared audio flag lock and restores the
        // previous value on drop, so parallel tests cannot race on it
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);

        // Sound enabled = true, notifications = true
        app.config.sound_enabled = true;
        app.config.desktop_notifications = true;
        app.notify_phase_completed(PomodoroPhase::Work, PomodoroPhase::ShortBreak);

        // Sound enabled = false, notifications = false
        app.config.sound_enabled = false;
        app.config.desktop_notifications = false;
        app.notify_phase_completed(PomodoroPhase::ShortBreak, PomodoroPhase::Work);

        // Long break notification
        app.config.sound_enabled = true;
        app.notify_phase_completed(PomodoroPhase::LongBreak, PomodoroPhase::Work);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_twenty_four_cycle_app_e2e_workflow() {
        let (mut app, temp_dir) = create_test_app();
        // Scoped mute: holds the shared audio flag lock and restores the
        // previous value on drop, so parallel tests cannot race on it
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);
        app.config.long_break_interval = 24;
        app.config.auto_start_breaks = true;
        app.config.auto_start_work = true;

        for cycle in 1..=24 {
            assert_eq!(app.timer.current_cycle, cycle);
            assert_eq!(app.timer.phase, PomodoroPhase::Work);
            // Tick work to 0
            app.timer.time_remaining_secs = 1;
            app.timer.status = crate::timer::TimerStatus::Running;
            app.tick_second();

            if cycle < 24 {
                assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
                assert_eq!(app.timer.current_cycle, cycle + 1);
                // Tick short break to 0
                app.timer.time_remaining_secs = 1;
                app.timer.status = crate::timer::TimerStatus::Running;
                app.tick_second();
                assert_eq!(app.timer.phase, PomodoroPhase::Work);
            } else {
                // 24th cycle completes into LongBreak and resets cycle to 1
                assert_eq!(app.timer.phase, PomodoroPhase::LongBreak);
                assert_eq!(app.timer.current_cycle, 1);
                assert_eq!(app.timer.completed_pomodoros, 24);
                assert_eq!(app.stats.total_work_sessions(), 24);
            }
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_all_eighteen_themes_cycle_and_persistence_e2e() {
        let (mut app, temp_dir) = create_test_app();
        let all_themes = ThemeChoice::all();
        assert_eq!(all_themes.len(), 18);

        // Select theme setting row (row 8)
        app.active_tab = ActiveTab::Settings;
        app.settings_index = 8;
        app.config.theme = ThemeChoice::CatppuccinMocha;

        // Step forward through every single theme
        for (i, expected_theme) in all_themes.iter().enumerate() {
            assert_eq!(
                app.config.theme, *expected_theme,
                "Mismatch at theme index {}",
                i
            );
            app.save_state();

            // Save state and verify re-load in fresh app instance
            let file_path = temp_dir.join("data.json");
            let storage = Storage::with_path(file_path);
            let app_data = storage.load();
            assert_eq!(app_data.config.theme, *expected_theme);

            // Advance to next theme
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        // After 18 steps, wrapped back to start
        assert_eq!(app.config.theme, ThemeChoice::CatppuccinMocha);

        // Step backwards through every theme in reverse
        for expected_theme in all_themes.iter().rev() {
            app.on_key_event(make_key(KeyCode::Char('h')));
            assert_eq!(app.config.theme, *expected_theme);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_all_eighteen_themes_full_ui_render_all_tabs_e2e() {
        let (mut app, temp_dir) = create_test_app();
        app.tasks.add("Implement Section A Themes".to_string(), 3);
        app.stats
            .record(crate::timer::PomodoroPhase::Work, 25, None, None);

        let backend = ratatui::backend::TestBackend::new(100, 30);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();

        for theme_choice in ThemeChoice::all() {
            app.config.theme = *theme_choice;
            for tab in [
                ActiveTab::Timer,
                ActiveTab::Tasks,
                ActiveTab::Stats,
                ActiveTab::Settings,
            ] {
                app.active_tab = tab;
                terminal.draw(|f| crate::ui::render(f, &app)).unwrap();
            }

            // Also test modal overlays with this theme
            app.open_task_modal();
            terminal.draw(|f| crate::ui::render(f, &app)).unwrap();
            app.show_task_modal = false;

            app.show_help = true;
            terminal.draw(|f| crate::ui::render(f, &app)).unwrap();
            app.show_help = false;
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_all_settings_rows_min_max_clamping_exhaustive() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;

        // Row 0: work_duration_mins (1..=120)
        app.settings_index = 0;
        for _ in 0..150 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.work_duration_mins, 120);
        for _ in 0..150 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.work_duration_mins, 1);

        // Row 1: short_break_mins (1..=60)
        app.settings_index = 1;
        for _ in 0..100 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.short_break_mins, 60);
        for _ in 0..100 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.short_break_mins, 1);

        // Row 2: long_break_mins (1..=90)
        app.settings_index = 2;
        for _ in 0..120 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.long_break_mins, 90);
        for _ in 0..120 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.long_break_mins, 1);

        // Row 3: long_break_interval (1..=24)
        app.settings_index = 3;
        for _ in 0..30 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.long_break_interval, 24);
        for _ in 0..30 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.long_break_interval, 1);

        // Rows 4, 5, 6, 7: toggles via space and enter
        for row in 4..=7 {
            app.settings_index = row;
            let before = match row {
                4 => app.config.auto_start_breaks,
                5 => app.config.auto_start_work,
                6 => app.config.desktop_notifications,
                7 => app.config.sound_enabled,
                _ => false,
            };
            app.on_key_event(make_key(KeyCode::Char(' ')));
            let after = match row {
                4 => app.config.auto_start_breaks,
                5 => app.config.auto_start_work,
                6 => app.config.desktop_notifications,
                7 => app.config.sound_enabled,
                _ => false,
            };
            assert_eq!(after, !before);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_modal_exclusive_key_handling() {
        let (mut app, temp_dir) = create_test_app();
        app.open_task_modal();
        assert!(app.show_task_modal);

        // Pressing 'q' inside modal must NOT quit the application, but type into title
        app.on_key_event(make_key(KeyCode::Char('q')));
        assert!(!app.should_quit);
        assert_eq!(app.task_input_title, "q");

        // Pressing '1' or '2' inside modal must type '1' or '2' and NOT switch tabs
        app.on_key_event(make_key(KeyCode::Char('1')));
        assert_eq!(app.active_tab, ActiveTab::Timer);
        assert_eq!(app.task_input_title, "q1");

        // Esc closes modal cleanly
        app.on_key_event(make_key(KeyCode::Esc));
        assert!(!app.show_task_modal);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_empty_tasks_key_interactions_graceful() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Tasks;
        assert_eq!(app.tasks.tasks.len(), 0);

        // All keys on empty list must not panic
        app.on_key_event(make_key(KeyCode::Char('j')));
        app.on_key_event(make_key(KeyCode::Char('k')));
        app.on_key_event(make_key(KeyCode::Char('x')));
        app.on_key_event(make_key(KeyCode::Char('d')));
        app.on_key_event(make_key(KeyCode::Char(' ')));
        app.on_key_event(make_key(KeyCode::Char('t')));
        app.on_key_event(make_key(KeyCode::Enter));

        assert_eq!(app.tasks.tasks.len(), 0);
        assert_eq!(app.tasks.active_task_id, None);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_unhandled_keys_and_modifier_combinations() {
        let (mut app, temp_dir) = create_test_app();

        // Random / unhandled keys across all tabs
        for tab in [
            ActiveTab::Timer,
            ActiveTab::Tasks,
            ActiveTab::Stats,
            ActiveTab::Settings,
        ] {
            app.active_tab = tab;
            app.on_key_event(make_key(KeyCode::F(1)));
            app.on_key_event(make_key(KeyCode::F(12)));
            app.on_key_event(make_key(KeyCode::PageUp));
            app.on_key_event(make_key(KeyCode::PageDown));
            app.on_key_event(make_key(KeyCode::Home));
            app.on_key_event(make_key(KeyCode::End));
            app.on_key_event(make_key(KeyCode::Insert));
            app.on_key_event(make_key(KeyCode::Delete));
            app.on_key_event(make_key_with_mod(KeyCode::Char('z'), KeyModifiers::CONTROL));
            app.on_key_event(make_key_with_mod(KeyCode::Char('x'), KeyModifiers::ALT));
        }

        assert!(!app.should_quit);
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_modal_backspace_empty_and_focus_toggle_chain() {
        let (mut app, temp_dir) = create_test_app();
        app.open_task_modal();
        assert!(app.show_task_modal);

        // Backspacing on an already empty string should not underflow or crash
        for _ in 0..10 {
            app.on_key_event(make_key(KeyCode::Backspace));
        }
        assert_eq!(app.task_input_title, "");

        // Type Unicode emojis and international text
        let unicode_text = "🍅 Focus on Rust 🚀 日本語";
        for c in unicode_text.chars() {
            app.on_key_event(make_key(KeyCode::Char(c)));
        }
        assert_eq!(app.task_input_title, unicode_text);

        // Rapid focus switching between title and estimate
        for _ in 0..5 {
            app.on_key_event(make_key(KeyCode::Tab));
            assert_eq!(app.task_modal_focus, 1);
            app.on_key_event(make_key(KeyCode::BackTab));
            assert_eq!(app.task_modal_focus, 0);
        }

        // Switch to estimate, adjust with keys
        app.on_key_event(make_key(KeyCode::Down));
        assert_eq!(app.task_modal_focus, 1);
        app.on_key_event(make_key(KeyCode::Char('8')));
        assert_eq!(app.task_input_estimated, 8);

        // Submit task
        app.on_key_event(make_key(KeyCode::Enter));
        assert!(!app.show_task_modal);
        assert_eq!(app.tasks.tasks.len(), 1);
        assert_eq!(app.tasks.tasks[0].title, unicode_text);
        assert_eq!(app.tasks.tasks[0].pomodoros_estimated, 8);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_task_reassignment_on_deletion_and_completion_chain() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Tasks;

        // Add 3 tasks: A, B, C
        app.tasks.add("Task A".to_string(), 1);
        app.tasks.add("Task B".to_string(), 2);
        app.tasks.add("Task C".to_string(), 3);
        assert_eq!(app.tasks.active_task().unwrap().title, "Task A");

        // Complete Task A -> active task should reassign to Task B
        app.tasks.selected_index = 0;
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert!(app.tasks.tasks[0].completed);
        assert_eq!(app.tasks.active_task().unwrap().title, "Task B");

        // Complete Task B -> active task should reassign to Task C
        app.tasks.selected_index = 1;
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert!(app.tasks.tasks[1].completed);
        assert_eq!(app.tasks.active_task().unwrap().title, "Task C");

        // Delete Task C -> all tasks are completed, active task becomes None
        app.tasks.selected_index = 2;
        app.on_key_event(make_key(KeyCode::Char('d')));
        assert_eq!(app.tasks.active_task_id, None);

        // Now run a Pomodoro tick with None active task -> stats should still record without error
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;
        app.tick_second();
        assert_eq!(app.stats.sessions.len(), 1);
        assert_eq!(app.stats.sessions[0].task_id, None);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_phase_transitions_with_auto_start_disabled_combinations() {
        let (mut app, temp_dir) = create_test_app();
        // Scoped mute: holds the shared audio flag lock and restores the
        // previous value on drop, so parallel tests cannot race on it
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);
        app.config.auto_start_breaks = false;
        app.config.auto_start_work = false;

        // Work finishes -> ShortBreak should be STOPPED (awaiting user start)
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;
        app.tick_second();

        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.status, crate::timer::TimerStatus::Stopped);

        // User starts break with Space
        app.on_key_event(make_key(KeyCode::Char(' ')));
        assert_eq!(app.timer.status, crate::timer::TimerStatus::Running);

        // Break finishes -> Work should be STOPPED (awaiting user start)
        app.timer.time_remaining_secs = 1;
        app.tick_second();

        assert_eq!(app.timer.phase, PomodoroPhase::Work);
        assert_eq!(app.timer.status, crate::timer::TimerStatus::Stopped);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_long_break_interval_boundary_one_and_twenty_four() {
        let (mut app, temp_dir) = create_test_app();
        // Scoped mute: holds the shared audio flag lock and restores the
        // previous value on drop, so parallel tests cannot race on it
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);

        // Test long_break_interval = 1 (every focus session leads directly to long break)
        app.config.long_break_interval = 1;
        app.timer.phase = PomodoroPhase::Work;
        app.timer.current_cycle = 1;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;
        app.tick_second();

        assert_eq!(app.timer.phase, PomodoroPhase::LongBreak);
        assert_eq!(app.timer.current_cycle, 1);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_fuzz_randomized_key_events_chaos_resilience() {
        let (mut app, temp_dir) = create_test_app();
        // Scoped mute: holds the shared audio flag lock and restores the
        // previous value on drop, so parallel tests cannot race on it
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);

        // Pre-populate tasks and stats
        app.tasks.add("Task 1".to_string(), 3);
        app.tasks.add("Task 2".to_string(), 1);

        // Seeded pseudorandom key stream (1000 keystrokes)
        let key_pool = [
            KeyCode::Char(' '),
            KeyCode::Char('a'),
            KeyCode::Char('s'),
            KeyCode::Char('r'),
            KeyCode::Char('t'),
            KeyCode::Char('d'),
            KeyCode::Char('x'),
            KeyCode::Char('j'),
            KeyCode::Char('k'),
            KeyCode::Char('h'),
            KeyCode::Char('l'),
            KeyCode::Char('+'),
            KeyCode::Char('-'),
            KeyCode::Char('1'),
            KeyCode::Char('2'),
            KeyCode::Char('3'),
            KeyCode::Char('4'),
            KeyCode::Char('?'),
            KeyCode::Tab,
            KeyCode::BackTab,
            KeyCode::Esc,
            KeyCode::Enter,
            KeyCode::Backspace,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
        ];

        let mut rng_state: u64 = 0xDEADBEEFCAFEBABE;
        for _ in 0..1000 {
            // Simple xorshift64 PRNG
            rng_state ^= rng_state << 13;
            rng_state ^= rng_state >> 7;
            rng_state ^= rng_state << 17;

            let key_idx = (rng_state as usize) % key_pool.len();
            let code = key_pool[key_idx];

            // If 'q' or quitting, reset should_quit to continue stress testing
            if app.should_quit {
                app.should_quit = false;
            }

            app.on_key_event(make_key(code));
            app.on_tick();

            // Verify core invariants never break
            assert!(app.settings_index <= 8);
            assert!(app.task_input_estimated >= 1 && app.task_input_estimated <= 20);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_rapid_filter_switching_and_index_clamping_chaos() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Tasks;

        // Add 10 tasks with alternating completion
        for i in 1..=10 {
            app.tasks.add(format!("Chaos Task {}", i), (i % 3) + 1);
            if i % 2 == 0 {
                app.tasks.toggle_selected();
            }
        }

        // Rapid filter switches and selection adjustments
        for _ in 0..20 {
            // Switch to Active filter
            app.on_key_event(make_key(KeyCode::Char('2')));
            assert_eq!(app.tasks.filter, TaskFilter::Active);
            app.on_key_event(make_key(KeyCode::Char('j')));
            app.on_key_event(make_key(KeyCode::Char('j')));

            // Switch to Completed filter
            app.on_key_event(make_key(KeyCode::Char('3')));
            assert_eq!(app.tasks.filter, TaskFilter::Completed);
            app.on_key_event(make_key(KeyCode::Char('k')));

            // Switch to All filter
            app.on_key_event(make_key(KeyCode::Char('1')));
            assert_eq!(app.tasks.filter, TaskFilter::All);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_status_message_overwrite_and_expiry() {
        let (mut app, temp_dir) = create_test_app();
        app.set_status_message("First Notification".to_string());
        assert_eq!(app.status_message.as_deref(), Some("First Notification"));
        assert_eq!(app.status_message_ticks, 40);

        // Tick 20 times
        for _ in 0..20 {
            app.on_tick();
        }
        assert_eq!(app.status_message_ticks, 20);

        // Overwrite with second notification
        app.set_status_message("Second Notification".to_string());
        assert_eq!(app.status_message.as_deref(), Some("Second Notification"));
        assert_eq!(app.status_message_ticks, 40); // Reset to 40

        // Tick 39 times
        for _ in 0..39 {
            app.on_tick();
        }
        assert_eq!(app.status_message.as_deref(), Some("Second Notification"));
        assert_eq!(app.status_message_ticks, 1);

        // 40th tick clears notification
        app.on_tick();
        assert_eq!(app.status_message, None);
        assert_eq!(app.status_message_ticks, 0);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_settings_navigation_bounds_with_all_key_variants() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;
        assert_eq!(app.settings_index, 0);

        // Step down using Down arrow
        for i in 1..=8 {
            app.on_key_event(make_key(KeyCode::Down));
            assert_eq!(app.settings_index, i);
        }
        // Down wraps to 0
        app.on_key_event(make_key(KeyCode::Down));
        assert_eq!(app.settings_index, 0);

        // Step up using Up arrow (wraps to 8)
        app.on_key_event(make_key(KeyCode::Up));
        assert_eq!(app.settings_index, 8);

        // Step up with 'k'
        for i in (0..=7).rev() {
            app.on_key_event(make_key(KeyCode::Char('k')));
            assert_eq!(app.settings_index, i);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_task_modal_rapid_editing_backspace_and_navigation() {
        let (mut app, temp_dir) = create_test_app();
        app.open_task_modal();

        // Type characters "Hello"
        for c in "Hello".chars() {
            app.on_key_event(make_key(KeyCode::Char(c)));
        }
        assert_eq!(app.task_input_title, "Hello");

        // Backspace twice -> "Hel"
        app.on_key_event(make_key(KeyCode::Backspace));
        app.on_key_event(make_key(KeyCode::Backspace));
        assert_eq!(app.task_input_title, "Hel");

        // Navigate down to estimated pomodoros
        app.on_key_event(make_key(KeyCode::Tab));
        assert_eq!(app.task_modal_focus, 1);

        // Tab back up
        app.on_key_event(make_key(KeyCode::BackTab));
        assert_eq!(app.task_modal_focus, 0);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_quit_command_handling() {
        let (mut app, temp_dir) = create_test_app();
        assert!(!app.should_quit);

        // 'q' sets should_quit
        app.on_key_event(make_key(KeyCode::Char('q')));
        assert!(app.should_quit);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_direct_tab_numeric_navigation_integration() {
        let (mut app, temp_dir) = create_test_app();
        assert_eq!(app.active_tab, ActiveTab::Timer);

        app.on_key_event(make_key(KeyCode::Char('3')));
        assert_eq!(app.active_tab, ActiveTab::Stats);

        app.on_key_event(make_key(KeyCode::Char('4')));
        assert_eq!(app.active_tab, ActiveTab::Settings);

        app.on_key_event(make_key(KeyCode::Char('1')));
        assert_eq!(app.active_tab, ActiveTab::Timer);

        app.on_key_event(make_key(KeyCode::Char('2')));
        assert_eq!(app.active_tab, ActiveTab::Tasks);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_help_modal_all_dismiss_keys() {
        let (mut app, temp_dir) = create_test_app();

        for dismiss_key in [
            KeyCode::Esc,
            KeyCode::Char('q'),
            KeyCode::Char('?'),
            KeyCode::Enter,
        ] {
            app.show_help = true;
            app.on_key_event(make_key(dismiss_key));
            assert!(!app.show_help);
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_settings_toggle_all_boolean_rows() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;

        // Rows 4, 5, 6, 7 are booleans: auto_start_breaks, auto_start_work, desktop_notifications, sound_enabled
        let initial_flags = (
            app.config.auto_start_breaks,
            app.config.auto_start_work,
            app.config.desktop_notifications,
            app.config.sound_enabled,
        );

        for row in 4..=7 {
            app.settings_index = row;
            app.on_key_event(make_key(KeyCode::Char(' ')));
        }

        assert_eq!(app.config.auto_start_breaks, !initial_flags.0);
        assert_eq!(app.config.auto_start_work, !initial_flags.1);
        assert_eq!(app.config.desktop_notifications, !initial_flags.2);
        assert_eq!(app.config.sound_enabled, !initial_flags.3);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_app_task_target_binding_and_unbinding_e2e() {
        let (mut app, temp_dir) = create_test_app();
        app.tasks.add("Sprint Task".to_string(), 3);
        app.active_tab = ActiveTab::Tasks;
        app.tasks.selected_index = 0;

        // 't' sets active target
        app.on_key_event(make_key(KeyCode::Char('t')));
        assert!(app
            .status_message
            .as_ref()
            .unwrap()
            .contains("Target set to"));
        assert_eq!(app.tasks.active_task().unwrap().title, "Sprint Task");

        // 'd' deletes active task
        app.on_key_event(make_key(KeyCode::Char('d')));
        assert!(app.tasks.active_task().is_none());
        assert!(app
            .status_message
            .as_ref()
            .unwrap()
            .contains("Task deleted"));

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_subsecond_tick_accumulation_and_second_decrement() {
        let (mut app, temp_dir) = create_test_app();
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 10;
        assert_eq!(app.tick_count, 0);

        // First 3 ticks (250ms, 500ms, 750ms) -> timer should remain at 10s
        app.on_tick();
        assert_eq!(app.tick_count, 1);
        assert_eq!(app.timer.time_remaining_secs, 10);

        app.on_tick();
        assert_eq!(app.tick_count, 2);
        assert_eq!(app.timer.time_remaining_secs, 10);

        app.on_tick();
        assert_eq!(app.tick_count, 3);
        assert_eq!(app.timer.time_remaining_secs, 10);

        // 4th tick (1000ms = 1s) -> timer decrements to 9s
        app.on_tick();
        assert_eq!(app.tick_count, 4);
        assert_eq!(app.timer.time_remaining_secs, 9);

        // 4 more ticks -> timer decrements to 8s
        for _ in 0..4 {
            app.on_tick();
        }
        assert_eq!(app.tick_count, 8);
        assert_eq!(app.timer.time_remaining_secs, 8);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_keypresses_do_not_decrement_timer() {
        let (mut app, temp_dir) = create_test_app();
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 25 * 60;
        let initial_remaining = app.timer.time_remaining_secs;
        let initial_tick_count = app.tick_count;

        // Simulate 50 rapid keypresses across different tabs
        for _ in 0..50 {
            app.on_key_event(make_key(KeyCode::Char('2'))); // Switch to Tasks tab
            app.on_key_event(make_key(KeyCode::Char('3'))); // Switch to Stats tab
            app.on_key_event(make_key(KeyCode::Char('1'))); // Switch to Timer tab
        }

        // Timer remaining seconds and tick_count must remain completely unaffected by keypresses
        assert_eq!(app.timer.time_remaining_secs, initial_remaining);
        assert_eq!(app.tick_count, initial_tick_count);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    // A session that runs to natural completion must be logged with the
    // duration it ACTUALLY ran for. The Settings tab allows editing durations
    // while the timer is running, so reading the current config at completion
    // time logs fabricated minutes whenever the user touched that row
    // mid-flight.
    #[test]
    fn test_completed_session_records_actual_duration_not_mutated_config() {
        let (mut app, temp_dir) = create_test_app();
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);
        app.config.desktop_notifications = false;

        // Start a 25-minute focus session
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.total_duration_secs = 25 * 60;
        app.timer.time_remaining_secs = 25 * 60;

        // Mid-flight the user raises Focus Duration to 90 minutes; a running
        // countdown is deliberately left untouched by adjust_setting
        app.config.work_duration_mins = 90;

        // Run the session to natural completion (loop anchored to the phase:
        // on completion the timer refills the countdown for the next phase)
        while app.timer.status == crate::timer::TimerStatus::Running
            && app.timer.phase == PomodoroPhase::Work
        {
            app.tick_second();
        }

        assert_eq!(app.stats.sessions.len(), 1);
        assert_eq!(
            app.stats.sessions[0].duration_mins, 25,
            "session must record the 25 minutes actually spent, not the mutated 90"
        );

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    // The inverse direction: shrinking the setting mid-session must not
    // under-report the minutes the user really focused.
    #[test]
    fn test_completed_session_records_actual_duration_when_config_shrunk() {
        let (mut app, temp_dir) = create_test_app();
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);
        app.config.desktop_notifications = false;

        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.total_duration_secs = 30 * 60;
        app.timer.time_remaining_secs = 30 * 60;

        app.config.work_duration_mins = 5;

        while app.timer.status == crate::timer::TimerStatus::Running
            && app.timer.phase == PomodoroPhase::Work
        {
            app.tick_second();
        }

        assert_eq!(app.stats.sessions.len(), 1);
        assert_eq!(
            app.stats.sessions[0].duration_mins, 30,
            "session must record the 30 minutes actually spent"
        );

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    // Break sessions follow the same rule: the recorded minutes come from the
    // phase that ran, not from whatever the config says at completion time.
    #[test]
    fn test_break_session_records_break_phase_duration() {
        let (mut app, temp_dir) = create_test_app();
        let _audio_mute_guard = crate::audio::audio_mute_guard_for_tests(true);
        app.config.desktop_notifications = false;

        app.timer.phase = PomodoroPhase::ShortBreak;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.total_duration_secs = 5 * 60;
        app.timer.time_remaining_secs = 5 * 60;

        // User lengthens breaks mid-break
        app.config.short_break_mins = 45;

        while app.timer.status == crate::timer::TimerStatus::Running
            && app.timer.phase == PomodoroPhase::ShortBreak
        {
            app.tick_second();
        }

        assert_eq!(app.stats.sessions.len(), 1);
        assert_eq!(app.stats.sessions[0].phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.stats.sessions[0].duration_mins, 5);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    // The confirmation toast must show the same cleaned-up title that was
    // stored, not the raw pre-trim buffer contents.
    #[test]
    fn test_modal_enter_shows_trimmed_title_in_status_message() {
        let (mut app, temp_dir) = create_test_app();
        app.open_task_modal();

        for c in "  Padded Task  ".chars() {
            app.on_key_event(make_key(KeyCode::Char(c)));
        }
        app.on_key_event(make_key(KeyCode::Enter));

        assert!(!app.show_task_modal);
        assert_eq!(app.tasks.tasks[0].title, "Padded Task");
        assert_eq!(
            app.status_message.as_deref(),
            Some("Task added: Padded Task"),
            "toast must echo the trimmed title users see in the list"
        );

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
