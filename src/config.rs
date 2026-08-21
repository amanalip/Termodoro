// Import serde serialization traits for saving configuration to disk
use serde::{Deserialize, Serialize};
// Import the ThemeChoice enum from our theme module
use crate::theme::ThemeChoice;

// Configuration structure representing all user-customizable preferences
//
// #[serde(default)] at the CONTAINER level fills every missing field from
// Config::default(), so a data.json written by an older or newer version of
// the app (missing or extra fields, or a partial config object) parses into
// sensible Pomodoro values. Field-level defaults were previously used, whose
// u32::default() is ZERO — a hand-edited {"config":{}} loaded 0-minute focus
// sessions instead of the documented 25/5/15 defaults.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
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
    // Active visual color theme selection (unknown names fall back to default
    // via ThemeChoice's tolerant Deserialize impl)
    pub theme: ThemeChoice,
}

// Inclusive bounds for every numeric setting. These mirror the clamps in the
// Settings UI handlers exactly so a hand-edited data.json cannot smuggle in
// values the interface itself would never produce.
const WORK_MINS_RANGE: (u32, u32) = (1, 120);
const SHORT_BREAK_MINS_RANGE: (u32, u32) = (1, 60);
const LONG_BREAK_MINS_RANGE: (u32, u32) = (1, 90);
const LONG_BREAK_INTERVAL_RANGE: (u32, u32) = (1, 24);

impl Config {
    // Clamps every numeric field into its valid range in place.
    //
    // Called on every load from disk. A corrupted or hand-edited file
    // containing zero durations previously caused instant phase-completion
    // loops (stats inflation plus a disk write every second), and absurdly
    // large values overflowed the minutes-to-seconds conversion at startup.
    pub fn sanitize(&mut self) {
        // Clamp helper: saturates value into the inclusive [min, max] range
        let clamp = |v: u32, range: (u32, u32)| v.clamp(range.0, range.1);
        self.work_duration_mins = clamp(self.work_duration_mins, WORK_MINS_RANGE);
        self.short_break_mins = clamp(self.short_break_mins, SHORT_BREAK_MINS_RANGE);
        self.long_break_mins = clamp(self.long_break_mins, LONG_BREAK_MINS_RANGE);
        self.long_break_interval = clamp(self.long_break_interval, LONG_BREAK_INTERVAL_RANGE);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_values() {
        let config = Config::default();
        assert_eq!(config.work_duration_mins, 25);
        assert_eq!(config.short_break_mins, 5);
        assert_eq!(config.long_break_mins, 15);
        assert_eq!(config.long_break_interval, 4);
        assert!(!config.auto_start_breaks);
        assert!(!config.auto_start_work);
        assert!(config.sound_enabled);
        assert!(config.desktop_notifications);
        assert_eq!(config.theme, ThemeChoice::CatppuccinMocha);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = Config {
            work_duration_mins: 50,
            short_break_mins: 10,
            long_break_mins: 30,
            long_break_interval: 2,
            auto_start_breaks: true,
            auto_start_work: true,
            sound_enabled: false,
            desktop_notifications: false,
            theme: ThemeChoice::Nord,
        };

        let serialized = serde_json::to_string(&config).expect("Serialization failed");
        let deserialized: Config =
            serde_json::from_str(&serialized).expect("Deserialization failed");
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_mutation_and_cloned_equality() {
        let mut config1 = Config {
            work_duration_mins: 45,
            long_break_interval: 6,
            theme: ThemeChoice::TokyoNight,
            ..Default::default()
        };

        let config2 = config1.clone();
        assert_eq!(config1, config2);

        config1.sound_enabled = false;
        assert_ne!(config1, config2);
    }

    #[test]
    fn test_config_all_theme_variant_serialization() {
        for theme in ThemeChoice::all() {
            let config = Config {
                theme: *theme,
                ..Default::default()
            };
            let json = serde_json::to_string(&config).unwrap();
            let parsed: Config = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.theme, *theme);
        }
    }

    #[test]
    fn test_config_boolean_flag_combinations() {
        let config = Config {
            auto_start_breaks: true,
            auto_start_work: true,
            sound_enabled: false,
            desktop_notifications: false,
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert!(loaded.auto_start_breaks);
        assert!(loaded.auto_start_work);
        assert!(!loaded.sound_enabled);
        assert!(!loaded.desktop_notifications);
    }

    #[test]
    fn test_config_extreme_duration_values_serde() {
        let config = Config {
            work_duration_mins: 120,
            short_break_mins: 60,
            long_break_mins: 90,
            long_break_interval: 24,
            ..Default::default()
        };
        let json = serde_json::to_string(&config).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.work_duration_mins, 120);
        assert_eq!(loaded.short_break_mins, 60);
        assert_eq!(loaded.long_break_mins, 90);
        assert_eq!(loaded.long_break_interval, 24);
    }

    #[test]
    fn test_config_debug_formatting() {
        let config = Config::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("work_duration_mins: 25"));
        assert!(debug_str.contains("short_break_mins: 5"));
        assert!(debug_str.contains("CatppuccinMocha"));
    }

    #[test]
    fn test_config_custom_initialization_builder_pattern() {
        let config = Config {
            work_duration_mins: 45,
            theme: ThemeChoice::GruvboxDark,
            ..Default::default()
        };
        assert_eq!(config.work_duration_mins, 45);
        assert_eq!(config.theme, ThemeChoice::GruvboxDark);
        assert_eq!(config.short_break_mins, 5);
    }
}
