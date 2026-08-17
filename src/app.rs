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
        }
    }

    // Persists current application state (config, tasks, stats) to disk
    pub fn save_state(&self) {
        // Invoke storage save method
        self.storage.save(&self.config, &self.tasks, &self.stats);
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
        // Ring audio / terminal bell if enabled in configuration
        if self.config.sound_enabled {
            // Write ASCII bell character (\x07) to standard output
            let mut out = stdout();
            // Write bell byte
            let _ = out.write_all(b"\x07");
            // Flush standard output buffer
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

    // Periodic tick method invoked on every timer interval (~250ms)
    pub fn on_tick(&mut self) {
        // Tick countdown timer
        if let Some(event) = self.timer.tick(&self.config) {
            // Handle timer completion event
            match event {
                // Phase completed
                TimerEvent::PhaseCompleted { finished_phase, next_phase } => {
                    // Calculate duration in minutes for finished phase
                    let dur_mins = match finished_phase {
                        PomodoroPhase::Work => self.config.work_duration_mins,
                        PomodoroPhase::ShortBreak => self.config.short_break_mins,
                        PomodoroPhase::LongBreak => self.config.long_break_mins,
                    };

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
                    self.stats.record(finished_phase, dur_mins, task_id, task_title);
                    // Trigger audio bell and desktop notification
                    self.notify_phase_completed(finished_phase, next_phase);
                    // Update status banner
                    self.set_status_message(format!("{} completed! Next: {}", finished_phase.title(), next_phase.title()));
                    // Automatically persist state to disk
                    self.save_state();
                }
            }
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
                    // Add task to task manager
                    self.tasks.add(self.task_input_title.clone(), self.task_input_estimated);
                    // Close task modal
                    self.show_task_modal = false;
                    // Show confirmation notification
                    self.set_status_message(format!("Task added: {}", self.task_input_title));
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
                // Right arrow, '+', or 'l' increments estimated pomodoros
                KeyCode::Right | KeyCode::Char('+') | KeyCode::Char('l') => {
                    // Maximum limit 20 pomodoros
                    if self.task_input_estimated < 20 {
                        // Increment
                        self.task_input_estimated += 1;
                    }
                }
                // Left arrow, '-', or 'h' decrements estimated pomodoros
                KeyCode::Left | KeyCode::Char('-') | KeyCode::Char('h') => {
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
                // Set selected as active
                self.tasks.set_selected_active();
                // Set status notification
                if let Some(task) = self.tasks.active_task() {
                    // Status banner
                    self.set_status_message(format!("Target set to: {}", task.title));
                }
                // Persist state
                self.save_state();
            }
            // 'd' or 'x' deletes selected task
            KeyCode::Char('d') | KeyCode::Char('x') => {
                // Remove task
                self.tasks.remove_selected();
                // Set notification
                self.set_status_message("Task deleted.".to_string());
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
            // Right arrow, '+', or 'l' increments setting value
            KeyCode::Right | KeyCode::Char('+') | KeyCode::Char('l') => {
                // Adjust setting positive
                self.adjust_setting(1);
            }
            // Left arrow, '-', or 'h' decrements setting value
            KeyCode::Left | KeyCode::Char('-') | KeyCode::Char('h') => {
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
                if self.timer.status == crate::timer::TimerStatus::Stopped && self.timer.phase == PomodoroPhase::Work {
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
                if self.timer.status == crate::timer::TimerStatus::Stopped && self.timer.phase == PomodoroPhase::ShortBreak {
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
                if self.timer.status == crate::timer::TimerStatus::Stopped && self.timer.phase == PomodoroPhase::LongBreak {
                    self.timer.reset(&self.config);
                }
            }
            // Setting 3: Long break interval (sessions count)
            3 => {
                // New value clamped between 1 and 12 sessions
                let new_val = (self.config.long_break_interval as i32 + delta).clamp(1, 12) as u32;
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
                let cur_idx = all_themes.iter().position(|&t| t == self.config.theme).unwrap_or(0);
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
        let temp_dir = std::env::temp_dir().join(format!("termodoro_app_test_{}", uuid::Uuid::new_v4()));
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
        assert_eq!(app.config.theme, ThemeChoice::Nord);
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

        // Add task and test on_tick completion
        app.tasks.add("Focus Task".to_string(), 2);
        app.timer.phase = PomodoroPhase::Work;
        app.timer.status = crate::timer::TimerStatus::Running;
        app.timer.time_remaining_secs = 1;

        // Tick to complete phase
        app.on_tick();
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
        app.on_tick(); // Completes Work 1 -> ShortBreak 1 (cycle 2)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 2);
        assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);

        // Complete ShortBreak 1
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes ShortBreak 1 -> Work 2
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Switch target to Task B
        app.tasks.selected_index = 1;
        app.tasks.set_selected_active();
        assert_eq!(app.tasks.active_task().unwrap().title, "Task B");

        // Cycle 2: Work (Task B active)
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes Work 2 -> ShortBreak 2 (cycle 3)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 3);
        assert_eq!(app.tasks.tasks[1].pomodoros_spent, 1);

        // Complete ShortBreak 2
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes ShortBreak 2 -> Work 3
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Cycle 3: Work
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes Work 3 -> ShortBreak 3 (cycle 4)
        assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(app.timer.current_cycle, 4);

        // Complete ShortBreak 3
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes ShortBreak 3 -> Work 4
        assert_eq!(app.timer.phase, PomodoroPhase::Work);

        // Cycle 4: Work (at cycle 4)
        app.timer.time_remaining_secs = 1;
        app.on_tick(); // Completes Work 4 -> LongBreak! Cycle resets to 1
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
        let temp_dir = std::env::temp_dir().join(format!("termodoro_recovery_test_{}", uuid::Uuid::new_v4()));
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
            app1.stats.record(PomodoroPhase::Work, 45, Some("Persistent Task 1".to_string()), Some("Persistent Task 1".to_string()));

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
        assert!(app.status_message.as_ref().unwrap().contains("Target set to: Task 3"));

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
        for _ in 0..20 {
            app.on_key_event(make_key(KeyCode::Char('l')));
        }
        assert_eq!(app.config.long_break_interval, 12);
        for _ in 0..20 {
            app.on_key_event(make_key(KeyCode::Char('h')));
        }
        assert_eq!(app.config.long_break_interval, 1);

        // Theme wrap-around backwards
        app.settings_index = 8;
        app.config.theme = ThemeChoice::CatppuccinMocha;
        app.on_key_event(make_key(KeyCode::Char('h')));
        assert_eq!(app.config.theme, ThemeChoice::SolarizedDark);

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_status_message_expiration_on_ticks() {
        let (mut app, temp_dir) = create_test_app();
        app.set_status_message("Temporary Notification".to_string());
        assert_eq!(app.status_message.as_deref(), Some("Temporary Notification"));
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

        // Digit '0' should not change estimated to 0
        app.on_key_event(make_key(KeyCode::Char('0')));
        assert_eq!(app.task_input_estimated, 1);

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}



