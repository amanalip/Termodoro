// Import serde serialization traits for saving configuration to disk
use serde::{Deserialize, Serialize};
// Import the ThemeChoice enum from our theme module
use crate::theme::ThemeChoice;

// Configuration structure representing all user-customizable preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // Duration in minutes for standard work focus sessions (default: 25)
    pub work_duration_mins: u32,
    // Duration in minutes for short breaks between sessions (default: 5)
    pub short_break_mins: u32,
    // Duration in minutes for long breaks after full cycles (default: 15)
    pub long_break_mins: u32,
    // Number of focus sessions required before triggering a long break (default: 4)
    pub long_break_interval: u32,
    // Whether to automatically start break timers when a work session ends
    pub auto_start_breaks: bool,
    // Whether to automatically start work timers when a break ends
    pub auto_start_work: bool,
    // Whether to emit terminal audio bells when timer phases complete
    pub sound_enabled: bool,
    // Whether to trigger OS-level desktop notifications when phases complete
    pub desktop_notifications: bool,
    // Active visual color theme selection
    pub theme: ThemeChoice,
}

// Implement Default trait to provide standard default settings for Termodoro
impl Default for Config {
    // Returns default instance of Config
    fn default() -> Self {
        // Construct standard Pomodoro 25/5/15 configuration
        Config {
            // Standard 25 minute focus session
            work_duration_mins: 25,
            // Standard 5 minute short break
            short_break_mins: 5,
            // Standard 15 minute long break
            long_break_mins: 15,
            // 4 work sessions per long break cycle
            long_break_interval: 4,
            // Do not auto-start breaks by default (allows user to rest at their pace)
            auto_start_breaks: false,
            // Do not auto-start work by default
            auto_start_work: false,
            // Sound notifications enabled by default
            sound_enabled: true,
            // Desktop notifications enabled by default
            desktop_notifications: true,
            // Default theme is Catppuccin Mocha
            theme: ThemeChoice::CatppuccinMocha,
        }
    }
}
