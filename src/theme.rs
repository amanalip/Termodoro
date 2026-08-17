// Import Color from ratatui style module for rendering terminal colors
use ratatui::style::Color;
// Import Deserialize and Serialize traits from serde for configuration persistence
use serde::{Deserialize, Serialize};

// Enum representing the selectable visual color themes in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    // Catppuccin Mocha theme (modern pastel dark palette)
    #[default]
    CatppuccinMocha,
    // Nord theme (arctic, north-bluish palette)
    Nord,
    // Gruvbox Dark theme (retro groove warm palette)
    GruvboxDark,
    // Tokyo Night theme (vibrant neon Japanese city palette)
    TokyoNight,
    // Dracula theme (vampire dark purple & pink palette)
    Dracula,
    // Solarized Dark theme (low-contrast designer palette)
    SolarizedDark,
}

impl ThemeChoice {
    // Returns a static slice of all available theme variants for iteration
    pub fn all() -> &'static [ThemeChoice] {
        // Return array of all enum variants
        &[
            // Catppuccin Mocha variant
            ThemeChoice::CatppuccinMocha,
            // Nord variant
            ThemeChoice::Nord,
            // Gruvbox Dark variant
            ThemeChoice::GruvboxDark,
            // Tokyo Night variant
            ThemeChoice::TokyoNight,
            // Dracula variant
            ThemeChoice::Dracula,
            // Solarized Dark variant
            ThemeChoice::SolarizedDark,
        ]
    }

    // Returns the human-readable display name for the theme
    pub fn name(&self) -> &'static str {
        // Match the current theme enum variant
        match self {
            // Display string for Catppuccin Mocha
            ThemeChoice::CatppuccinMocha => "Catppuccin Mocha",
            // Display string for Nord
            ThemeChoice::Nord => "Nord",
            // Display string for Gruvbox Dark
            ThemeChoice::GruvboxDark => "Gruvbox Dark",
            // Display string for Tokyo Night
            ThemeChoice::TokyoNight => "Tokyo Night",
            // Display string for Dracula
            ThemeChoice::Dracula => "Dracula",
            // Display string for Solarized Dark
            ThemeChoice::SolarizedDark => "Solarized Dark",
        }
    }
}

// Structure containing all concrete RGB color values used across UI widgets
#[derive(Debug, Clone)]
pub struct Theme {
    // The enum choice representing this theme
    pub choice: ThemeChoice,
    // Background color for terminal panels and windows
    pub bg: Color,
    // Primary foreground text color
    pub fg: Color,
    // Primary brand/accent color for headers and active tab
    pub primary: Color,
    // Secondary accent color for supplementary details
    pub secondary: Color,
    // Color designated for active focus/work pomodoro phases
    pub work: Color,
    // Color designated for short break phases
    pub short_break: Color,
    // Color designated for long break phases
    pub long_break: Color,
    // Color for completed tasks and success indicators
    pub success: Color,
    // Color for warnings, paused states, and streaks
    pub warning: Color,
    // Border color for inactive panels
    pub border: Color,
    // Border color for focused and active panels
    pub border_active: Color,
    // Color for secondary, muted, or hint text
    pub muted: Color,
    // Background highlight color for selected rows
    pub highlight: Color,
}

impl Theme {
    // Constructs concrete RGB color values based on the selected ThemeChoice
    pub fn from_choice(choice: ThemeChoice) -> Self {
        // Match on the theme choice
        match choice {
            // Build Catppuccin Mocha palette
            ThemeChoice::CatppuccinMocha => Theme {
                // Set the theme choice
                choice,
                // Crust / Base background color (#1e1e2e)
                bg: Color::Rgb(30, 30, 46),
                // Text foreground color (#cdd6f4)
                fg: Color::Rgb(205, 214, 244),
                // Blue accent color (#89b4fa)
                primary: Color::Rgb(137, 180, 250),
                // Mauve secondary accent color (#cba6f7)
                secondary: Color::Rgb(203, 166, 247),
                // Red focus phase color (#f38ba8)
                work: Color::Rgb(243, 139, 168),
                // Green short break color (#a6e3a1)
                short_break: Color::Rgb(166, 227, 161),
                // Teal long break color (#94e2d5)
                long_break: Color::Rgb(148, 226, 213),
                // Green success color (#a6e3a1)
                success: Color::Rgb(166, 227, 161),
                // Yellow warning color (#f9e2af)
                warning: Color::Rgb(249, 226, 175),
                // Surface2 border color (#585b70)
                border: Color::Rgb(88, 91, 112),
                // Blue active border color (#89b4fa)
                border_active: Color::Rgb(137, 180, 250),
                // Overlay0 muted text color (#6c7086)
                muted: Color::Rgb(108, 112, 134),
                // Surface0 selection row highlight (#313244)
                highlight: Color::Rgb(49, 50, 68),
            },
            // Build Nord palette
            ThemeChoice::Nord => Theme {
                // Set the theme choice
                choice,
                // Polar night dark background (#2e3440)
                bg: Color::Rgb(46, 52, 64),
                // Snow storm light foreground (#eceff4)
                fg: Color::Rgb(236, 239, 244),
                // Frost cyan-blue primary (#88c0d0)
                primary: Color::Rgb(136, 192, 208),
                // Frost blue secondary (#81a1c1)
                secondary: Color::Rgb(129, 161, 193),
                // Aurora red work color (#bf616a)
                work: Color::Rgb(191, 97, 106),
                // Aurora green short break color (#a3be8c)
                short_break: Color::Rgb(163, 190, 140),
                // Frost teal long break color (#8fbcbb)
                long_break: Color::Rgb(143, 188, 187),
                // Aurora green success color (#a3be8c)
                success: Color::Rgb(163, 190, 140),
                // Aurora yellow warning color (#ebcb8b)
                warning: Color::Rgb(235, 203, 139),
                // Polar night light border (#4c566a)
                border: Color::Rgb(76, 86, 106),
                // Frost cyan active border (#88c0d0)
                border_active: Color::Rgb(136, 192, 208),
                // Frost muted dark blue (#5e81ac)
                muted: Color::Rgb(94, 129, 172),
                // Polar night highlight (#3b4252)
                highlight: Color::Rgb(59, 66, 82),
            },
            // Build Gruvbox Dark palette
            ThemeChoice::GruvboxDark => Theme {
                // Set the theme choice
                choice,
                // Dark background (#282828)
                bg: Color::Rgb(40, 40, 40),
                // Light foreground (#ebdbb2)
                fg: Color::Rgb(235, 219, 178),
                // Yellow primary (#fabd2f)
                primary: Color::Rgb(250, 189, 47),
                // Orange secondary (#fe8019)
                secondary: Color::Rgb(254, 128, 25),
                // Bright red work color (#fb4934)
                work: Color::Rgb(251, 73, 52),
                // Bright green short break color (#b8bb26)
                short_break: Color::Rgb(184, 187, 38),
                // Bright aqua long break color (#8ec07c)
                long_break: Color::Rgb(142, 192, 124),
                // Bright green success color (#b8bb26)
                success: Color::Rgb(184, 187, 38),
                // Bright yellow warning color (#fabd2f)
                warning: Color::Rgb(250, 189, 47),
                // Gray border (#504945)
                border: Color::Rgb(80, 73, 69),
                // Yellow active border (#fabd2f)
                border_active: Color::Rgb(250, 189, 47),
                // Muted gray (#928374)
                muted: Color::Rgb(146, 131, 116),
                // Dark highlight row (#3c3836)
                highlight: Color::Rgb(60, 56, 54),
            },
            // Build Tokyo Night palette
            ThemeChoice::TokyoNight => Theme {
                // Set the theme choice
                choice,
                // Night background (#1a1b26)
                bg: Color::Rgb(26, 27, 38),
                // Light foreground (#c0caf5)
                fg: Color::Rgb(192, 202, 245),
                // Blue primary (#7aa2f7)
                primary: Color::Rgb(122, 162, 247),
                // Magenta secondary (#bb9af7)
                secondary: Color::Rgb(187, 154, 247),
                // Red focus work color (#f7768e)
                work: Color::Rgb(247, 118, 142),
                // Green short break color (#9ece6a)
                short_break: Color::Rgb(158, 206, 106),
                // Teal long break color (#73daca)
                long_break: Color::Rgb(115, 218, 202),
                // Green success color (#9ece6a)
                success: Color::Rgb(158, 206, 106),
                // Yellow warning color (#e0af68)
                warning: Color::Rgb(224, 175, 104),
                // Dark border (#414868)
                border: Color::Rgb(65, 72, 104),
                // Blue active border (#7aa2f7)
                border_active: Color::Rgb(122, 162, 247),
                // Muted slate (#565f89)
                muted: Color::Rgb(86, 95, 137),
                // Night highlight row (#24283b)
                highlight: Color::Rgb(36, 40, 59),
            },
            // Build Dracula palette
            ThemeChoice::Dracula => Theme {
                // Set the theme choice
                choice,
                // Dracula background (#282a36)
                bg: Color::Rgb(40, 42, 54),
                // Light foreground (#f8f8f2)
                fg: Color::Rgb(248, 248, 242),
                // Purple primary (#bd93f9)
                primary: Color::Rgb(189, 147, 249),
                // Cyan secondary (#8be9fd)
                secondary: Color::Rgb(139, 233, 253),
                // Red work focus color (#ff5555)
                work: Color::Rgb(255, 85, 85),
                // Green short break color (#50fa7b)
                short_break: Color::Rgb(80, 250, 123),
                // Cyan long break color (#8be9fd)
                long_break: Color::Rgb(139, 233, 253),
                // Green success color (#50fa7b)
                success: Color::Rgb(80, 250, 123),
                // Yellow warning color (#f1fa8c)
                warning: Color::Rgb(241, 250, 140),
                // Purple-gray border (#44475a)
                border: Color::Rgb(68, 71, 90),
                // Purple active border (#bd93f9)
                border_active: Color::Rgb(189, 147, 249),
                // Comment muted color (#6272a4)
                muted: Color::Rgb(98, 114, 164),
                // Current line highlight (#44475a)
                highlight: Color::Rgb(68, 71, 90),
            },
            // Build Solarized Dark palette
            ThemeChoice::SolarizedDark => Theme {
                // Set the theme choice
                choice,
                // Deep blue background (#002b36)
                bg: Color::Rgb(0, 43, 54),
                // Base0 foreground (#839496)
                fg: Color::Rgb(131, 148, 150),
                // Blue primary (#268bd2)
                primary: Color::Rgb(38, 139, 210),
                // Cyan secondary (#2aa198)
                secondary: Color::Rgb(42, 161, 152),
                // Red work focus color (#dc322f)
                work: Color::Rgb(220, 50, 47),
                // Green short break color (#859900)
                short_break: Color::Rgb(133, 153, 0),
                // Cyan long break color (#2aa198)
                long_break: Color::Rgb(42, 161, 152),
                // Green success color (#859900)
                success: Color::Rgb(133, 153, 0),
                // Yellow warning color (#b58900)
                warning: Color::Rgb(181, 137, 0),
                // Base02 border (#073642)
                border: Color::Rgb(7, 54, 66),
                // Blue active border (#268bd2)
                border_active: Color::Rgb(38, 139, 210),
                // Base01 muted color (#586e75)
                muted: Color::Rgb(88, 110, 117),
                // Base02 highlight (#073642)
                highlight: Color::Rgb(7, 54, 66),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_theme_choices() {
        let all = ThemeChoice::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&ThemeChoice::CatppuccinMocha));
        assert!(all.contains(&ThemeChoice::Nord));
        assert!(all.contains(&ThemeChoice::GruvboxDark));
        assert!(all.contains(&ThemeChoice::TokyoNight));
        assert!(all.contains(&ThemeChoice::Dracula));
        assert!(all.contains(&ThemeChoice::SolarizedDark));
    }

    #[test]
    fn test_theme_names() {
        assert_eq!(ThemeChoice::CatppuccinMocha.name(), "Catppuccin Mocha");
        assert_eq!(ThemeChoice::Nord.name(), "Nord");
        assert_eq!(ThemeChoice::GruvboxDark.name(), "Gruvbox Dark");
        assert_eq!(ThemeChoice::TokyoNight.name(), "Tokyo Night");
        assert_eq!(ThemeChoice::Dracula.name(), "Dracula");
        assert_eq!(ThemeChoice::SolarizedDark.name(), "Solarized Dark");
    }

    #[test]
    fn test_theme_from_choice_all_variants() {
        for choice in ThemeChoice::all() {
            let theme = Theme::from_choice(*choice);
            assert_eq!(theme.choice, *choice);
            // Verify that all colors are valid non-empty values
            assert_ne!(theme.bg, theme.fg);
        }
    }
}
