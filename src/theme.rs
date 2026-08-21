// Import Color from ratatui style module for rendering terminal colors
use ratatui::style::Color;
// Import Serialize trait from serde for configuration persistence; Deserialize
// is implemented manually below (fully qualified) rather than derived
use serde::Serialize;

// Enum representing the selectable visual color themes in the application
//
// NOTE: Deserialize is intentionally NOT derived here. It is implemented
// manually below so that an unknown theme string in data.json falls back to
// the default theme instead of failing the entire AppData parse (which would
// otherwise reset all user data). Serialize stays derived as before.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Default)]
pub enum ThemeChoice {
    // Catppuccin Mocha theme (modern pastel dark palette)
    #[default]
    CatppuccinMocha,
    // Catppuccin Macchiato theme (mid-tone dark pastel palette)
    CatppuccinMacchiato,
    // Catppuccin Frappé theme (soft low-contrast dark palette)
    CatppuccinFrappe,
    // Catppuccin Latte theme (clean, high-contrast light palette)
    CatppuccinLatte,
    // Nord theme (arctic, north-bluish palette)
    Nord,
    // Gruvbox Dark theme (retro groove warm palette)
    GruvboxDark,
    // Tokyo Night theme (vibrant neon Japanese city palette)
    TokyoNight,
    // Dracula theme (vampire dark purple & pink palette)
    Dracula,
    // Solarized Dark theme (low-contrast designer dark palette)
    SolarizedDark,
    // Solarized Light theme (warm designer light palette)
    SolarizedLight,
    // Rose Pine theme (natural pine, warm gold, iris and rose palette)
    RosePine,
    // One Dark theme (classic Atom Pro editor palette)
    OneDark,
    // Kanagawa theme (Japanese woodblock print inspired palette)
    Kanagawa,
    // Everforest Dark theme (organic, fatigue-free dark green palette)
    EverforestDark,
    // Everforest Light theme (soothing warm paper light green palette)
    EverforestLight,
    // Synthwave / Cyberpunk '84 theme (electric neon magenta & cyan palette)
    Synthwave84,
    // Monokai Pro theme (filtered spectrum high-contrast palette)
    MonokaiPro,
    // OLED Phosphor theme (pitch-black #000000 background with CRT green accents)
    OledPhosphor,
}

impl ThemeChoice {
    // Returns a static slice of all available theme variants for iteration
    pub fn all() -> &'static [ThemeChoice] {
        // Return array of all enum variants
        &[
            ThemeChoice::CatppuccinMocha,
            ThemeChoice::CatppuccinMacchiato,
            ThemeChoice::CatppuccinFrappe,
            ThemeChoice::CatppuccinLatte,
            ThemeChoice::Nord,
            ThemeChoice::GruvboxDark,
            ThemeChoice::TokyoNight,
            ThemeChoice::Dracula,
            ThemeChoice::SolarizedDark,
            ThemeChoice::SolarizedLight,
            ThemeChoice::RosePine,
            ThemeChoice::OneDark,
            ThemeChoice::Kanagawa,
            ThemeChoice::EverforestDark,
            ThemeChoice::EverforestLight,
            ThemeChoice::Synthwave84,
            ThemeChoice::MonokaiPro,
            ThemeChoice::OledPhosphor,
        ]
    }

    // Returns the human-readable display name for the theme
    pub fn name(&self) -> &'static str {
        // Match the current theme enum variant
        match self {
            ThemeChoice::CatppuccinMocha => "Catppuccin Mocha",
            ThemeChoice::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeChoice::CatppuccinFrappe => "Catppuccin Frappé",
            ThemeChoice::CatppuccinLatte => "Catppuccin Latte",
            ThemeChoice::Nord => "Nord",
            ThemeChoice::GruvboxDark => "Gruvbox Dark",
            ThemeChoice::TokyoNight => "Tokyo Night",
            ThemeChoice::Dracula => "Dracula",
            ThemeChoice::SolarizedDark => "Solarized Dark",
            ThemeChoice::SolarizedLight => "Solarized Light",
            ThemeChoice::RosePine => "Rose Pine",
            ThemeChoice::OneDark => "One Dark",
            ThemeChoice::Kanagawa => "Kanagawa",
            ThemeChoice::EverforestDark => "Everforest Dark",
            ThemeChoice::EverforestLight => "Everforest Light",
            ThemeChoice::Synthwave84 => "Synthwave '84",
            ThemeChoice::MonokaiPro => "Monokai Pro",
            ThemeChoice::OledPhosphor => "OLED Phosphor",
        }
    }
}

impl std::str::FromStr for ThemeChoice {
    type Err = ();

    // Parses a theme from its serialized variant name (for example
    // "CatppuccinMocha") or its human display name ("Catppuccin Mocha").
    // Matching is case-insensitive and ignores separators so hand-edited
    // config files still load. Unknown names yield Err, which the Deserialize
    // impl below converts into the default theme.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Normalization folds everything a hand-edited config might plausibly
        // contain: casing, space/hyphen/underscore separators, apostrophes
        // ("Synthwave '84"), and accented characters ("Catppuccin Frappé" —
        // both NFC é and NFD e + combining acute). Without this, copying the
        // exact display name shown in the UI footer into data.json silently
        // fell back to the default theme.
        let normalize = |v: &str| -> String {
            v.trim()
                .to_ascii_lowercase()
                .replace([' ', '-', '_', '\'', '\u{2018}', '\u{2019}'], "")
                .replace('é', "e")
                .replace('\u{0301}', "")
        };
        let wanted = normalize(s);
        for choice in ThemeChoice::all() {
            let variant = normalize(format!("{:?}", choice).as_str());
            let display = normalize(choice.name());
            if wanted == variant || wanted == display {
                return Ok(*choice);
            }
        }
        Err(())
    }
}

// Tolerant deserialization: any unrecognized theme name maps to the default
// variant instead of erroring out. This keeps one bad string in data.json
// from invalidating the whole file (and wiping the user's saved data).
impl<'de> serde::Deserialize<'de> for ThemeChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(raw.parse::<ThemeChoice>().unwrap_or_default())
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
                choice,
                bg: Color::Rgb(30, 30, 46),               // #1e1e2e
                fg: Color::Rgb(205, 214, 244),            // #cdd6f4
                primary: Color::Rgb(137, 180, 250),       // #89b4fa
                secondary: Color::Rgb(203, 166, 247),     // #cba6f7
                work: Color::Rgb(243, 139, 168),          // #f38ba8
                short_break: Color::Rgb(166, 227, 161),   // #a6e3a1
                long_break: Color::Rgb(148, 226, 213),    // #94e2d5
                success: Color::Rgb(166, 227, 161),       // #a6e3a1
                warning: Color::Rgb(249, 226, 175),       // #f9e2af
                border: Color::Rgb(88, 91, 112),          // #585b70
                border_active: Color::Rgb(137, 180, 250), // #89b4fa
                muted: Color::Rgb(108, 112, 134),         // #6c7086
                highlight: Color::Rgb(49, 50, 68),        // #313244
            },
            // Build Catppuccin Macchiato palette
            ThemeChoice::CatppuccinMacchiato => Theme {
                choice,
                bg: Color::Rgb(36, 39, 58),               // #24273a
                fg: Color::Rgb(202, 211, 245),            // #cad3f5
                primary: Color::Rgb(138, 173, 244),       // #8aadf4
                secondary: Color::Rgb(198, 160, 246),     // #c6a0f6
                work: Color::Rgb(237, 135, 150),          // #ed8796
                short_break: Color::Rgb(166, 218, 149),   // #a6da95
                long_break: Color::Rgb(139, 213, 202),    // #8bd5ca
                success: Color::Rgb(166, 218, 149),       // #a6da95
                warning: Color::Rgb(238, 212, 159),       // #eed49f
                border: Color::Rgb(91, 96, 120),          // #5b6078
                border_active: Color::Rgb(138, 173, 244), // #8aadf4
                muted: Color::Rgb(110, 115, 141),         // #6e738d
                highlight: Color::Rgb(54, 58, 79),        // #363a4f
            },
            // Build Catppuccin Frappé palette
            ThemeChoice::CatppuccinFrappe => Theme {
                choice,
                bg: Color::Rgb(48, 52, 70),               // #303446
                fg: Color::Rgb(198, 208, 245),            // #c6d0f5
                primary: Color::Rgb(140, 170, 238),       // #8caaee
                secondary: Color::Rgb(202, 158, 230),     // #ca9ee6
                work: Color::Rgb(231, 130, 132),          // #e78284
                short_break: Color::Rgb(163, 209, 140),   // #a3d18c
                long_break: Color::Rgb(129, 200, 190),    // #81c8be
                success: Color::Rgb(163, 209, 140),       // #a3d18c
                warning: Color::Rgb(229, 200, 144),       // #e5c890
                border: Color::Rgb(98, 104, 128),         // #626880
                border_active: Color::Rgb(140, 170, 238), // #8caaee
                muted: Color::Rgb(115, 121, 148),         // #737994
                highlight: Color::Rgb(65, 69, 89),        // #414559
            },
            // Build Catppuccin Latte palette (Light)
            ThemeChoice::CatppuccinLatte => Theme {
                choice,
                bg: Color::Rgb(239, 241, 245),           // #eff1f5
                fg: Color::Rgb(76, 79, 105),             // #4c4f69
                primary: Color::Rgb(30, 102, 245),       // #1e66f5
                secondary: Color::Rgb(136, 57, 239),     // #8839ef
                work: Color::Rgb(210, 15, 57),           // #d20f39
                short_break: Color::Rgb(64, 160, 43),    // #40a02b
                long_break: Color::Rgb(23, 146, 153),    // #179299
                success: Color::Rgb(64, 160, 43),        // #40a02b
                warning: Color::Rgb(223, 142, 29),       // #df8e1d
                border: Color::Rgb(188, 192, 204),       // #bcc0cc
                border_active: Color::Rgb(30, 102, 245), // #1e66f5
                muted: Color::Rgb(140, 143, 161),        // #8c8fa1
                highlight: Color::Rgb(204, 208, 218),    // #ccd0da
            },
            // Build Nord palette
            ThemeChoice::Nord => Theme {
                choice,
                bg: Color::Rgb(46, 52, 64),               // #2e3440
                fg: Color::Rgb(236, 239, 244),            // #eceff4
                primary: Color::Rgb(136, 192, 208),       // #88c0d0
                secondary: Color::Rgb(129, 161, 193),     // #81a1c1
                work: Color::Rgb(191, 97, 106),           // #bf616a
                short_break: Color::Rgb(163, 190, 140),   // #a3be8c
                long_break: Color::Rgb(143, 188, 187),    // #8fbcbb
                success: Color::Rgb(163, 190, 140),       // #a3be8c
                warning: Color::Rgb(235, 203, 139),       // #ebcb8b
                border: Color::Rgb(76, 86, 106),          // #4c566a
                border_active: Color::Rgb(136, 192, 208), // #88c0d0
                muted: Color::Rgb(94, 129, 172),          // #5e81ac
                highlight: Color::Rgb(59, 66, 82),        // #3b4252
            },
            // Build Gruvbox Dark palette
            ThemeChoice::GruvboxDark => Theme {
                choice,
                bg: Color::Rgb(40, 40, 40),              // #282828
                fg: Color::Rgb(235, 219, 178),           // #ebdbb2
                primary: Color::Rgb(250, 189, 47),       // #fabd2f
                secondary: Color::Rgb(254, 128, 25),     // #fe8019
                work: Color::Rgb(251, 73, 52),           // #fb4934
                short_break: Color::Rgb(184, 187, 38),   // #b8bb26
                long_break: Color::Rgb(142, 192, 124),   // #8ec07c
                success: Color::Rgb(184, 187, 38),       // #b8bb26
                warning: Color::Rgb(250, 189, 47),       // #fabd2f
                border: Color::Rgb(80, 73, 69),          // #504945
                border_active: Color::Rgb(250, 189, 47), // #fabd2f
                muted: Color::Rgb(146, 131, 116),        // #928374
                highlight: Color::Rgb(60, 56, 54),       // #3c3836
            },
            // Build Tokyo Night palette
            ThemeChoice::TokyoNight => Theme {
                choice,
                bg: Color::Rgb(26, 27, 38),               // #1a1b26
                fg: Color::Rgb(192, 202, 245),            // #c0caf5
                primary: Color::Rgb(122, 162, 247),       // #7aa2f7
                secondary: Color::Rgb(187, 154, 247),     // #bb9af7
                work: Color::Rgb(247, 118, 142),          // #f7768e
                short_break: Color::Rgb(158, 206, 106),   // #9ece6a
                long_break: Color::Rgb(115, 218, 202),    // #73daca
                success: Color::Rgb(158, 206, 106),       // #9ece6a
                warning: Color::Rgb(224, 175, 104),       // #e0af68
                border: Color::Rgb(65, 72, 104),          // #414868
                border_active: Color::Rgb(122, 162, 247), // #7aa2f7
                muted: Color::Rgb(86, 95, 137),           // #565f89
                highlight: Color::Rgb(36, 40, 59),        // #24283b
            },
            // Build Dracula palette
            ThemeChoice::Dracula => Theme {
                choice,
                bg: Color::Rgb(40, 42, 54),            // #282a36
                fg: Color::Rgb(248, 248, 242),         // #f8f8f2
                primary: Color::Rgb(189, 147, 249),    // #bd93f9
                secondary: Color::Rgb(139, 233, 253),  // #8be9fd
                work: Color::Rgb(255, 85, 85),         // #ff5555
                short_break: Color::Rgb(80, 250, 123), // #50fa7b
                // Dracula orange instead of cyan: cyan duplicated the secondary
                // accent, defeating phase color-coding
                long_break: Color::Rgb(255, 184, 108), // #ffb86c
                success: Color::Rgb(80, 250, 123),     // #50fa7b
                warning: Color::Rgb(241, 250, 140),    // #f1fa8c
                border: Color::Rgb(68, 71, 90),        // #44475a
                border_active: Color::Rgb(189, 147, 249), // #bd93f9
                muted: Color::Rgb(98, 114, 164),       // #6272a4
                // Slightly lighter than the border so selected rows stand out
                highlight: Color::Rgb(78, 82, 102), // #4e5266
            },
            // Build Solarized Dark palette
            ThemeChoice::SolarizedDark => Theme {
                choice,
                bg: Color::Rgb(0, 43, 54),            // #002b36
                fg: Color::Rgb(131, 148, 150),        // #839496
                primary: Color::Rgb(38, 139, 210),    // #268bd2
                secondary: Color::Rgb(42, 161, 152),  // #2aa198
                work: Color::Rgb(220, 50, 47),        // #dc322f
                short_break: Color::Rgb(133, 153, 0), // #859900
                // Solarized violet instead of cyan: cyan duplicated the
                // secondary accent, defeating phase color-coding
                long_break: Color::Rgb(108, 113, 196), // #6c71c4
                success: Color::Rgb(133, 153, 0),      // #859900
                warning: Color::Rgb(181, 137, 0),      // #b58900
                border: Color::Rgb(7, 54, 66),         // #073642
                border_active: Color::Rgb(38, 139, 210), // #268bd2
                muted: Color::Rgb(88, 110, 117),       // #586e75
                // Slightly lighter than the border so selected rows stand out
                highlight: Color::Rgb(13, 74, 89), // #0d4a59
            },
            // Build Solarized Light palette (Light)
            ThemeChoice::SolarizedLight => Theme {
                choice,
                bg: Color::Rgb(253, 246, 227),        // #fdf6e3
                fg: Color::Rgb(101, 123, 131),        // #657b83
                primary: Color::Rgb(38, 139, 210),    // #268bd2
                secondary: Color::Rgb(108, 113, 196), // #6c71c4
                work: Color::Rgb(220, 50, 47),        // #dc322f
                short_break: Color::Rgb(133, 153, 0), // #859900
                // Solarized magenta instead of cyan: cyan duplicated the
                // secondary accent (violet is taken here), defeating color-coding
                long_break: Color::Rgb(211, 54, 130), // #d33682
                success: Color::Rgb(133, 153, 0),     // #859900
                warning: Color::Rgb(181, 137, 0),     // #b58900
                border: Color::Rgb(238, 232, 213),    // #eee8d5
                border_active: Color::Rgb(38, 139, 210), // #268bd2
                muted: Color::Rgb(147, 161, 161),     // #93a1a1
                // Slightly darker than the border so selected rows stand out
                highlight: Color::Rgb(227, 220, 195), // #e3dcc3
            },
            // Build Rose Pine palette
            ThemeChoice::RosePine => Theme {
                choice,
                bg: Color::Rgb(25, 23, 36),            // #191724
                fg: Color::Rgb(224, 222, 244),         // #e0def4
                primary: Color::Rgb(156, 207, 216),    // #9ccfd8
                secondary: Color::Rgb(196, 167, 231),  // #c4a7e7
                work: Color::Rgb(235, 111, 146),       // #eb6f92
                short_break: Color::Rgb(49, 116, 143), // #31748f
                // Rose Pine "rose" instead of foam: foam duplicated the primary
                // accent, defeating phase color-coding
                long_break: Color::Rgb(235, 188, 186), // #ebbcba
                success: Color::Rgb(49, 116, 143),     // #31748f
                warning: Color::Rgb(246, 193, 119),    // #f6c177
                border: Color::Rgb(38, 35, 58),        // #26233a
                border_active: Color::Rgb(196, 167, 231), // #c4a7e7
                muted: Color::Rgb(110, 106, 134),      // #6e6a86
                highlight: Color::Rgb(42, 40, 62),     // #2a283e
            },
            // Build One Dark (Atom Pro) palette
            ThemeChoice::OneDark => Theme {
                choice,
                bg: Color::Rgb(40, 44, 52),              // #282c34
                fg: Color::Rgb(171, 178, 191),           // #abb2bf
                primary: Color::Rgb(97, 175, 239),       // #61afef
                secondary: Color::Rgb(198, 120, 221),    // #c678dd
                work: Color::Rgb(224, 108, 117),         // #e06c75
                short_break: Color::Rgb(152, 195, 121),  // #98c379
                long_break: Color::Rgb(86, 182, 194),    // #56b6c2
                success: Color::Rgb(152, 195, 121),      // #98c379
                warning: Color::Rgb(229, 192, 123),      // #e5c07b
                border: Color::Rgb(62, 68, 82),          // #3e4452
                border_active: Color::Rgb(97, 175, 239), // #61afef
                muted: Color::Rgb(92, 99, 112),          // #5c6370
                highlight: Color::Rgb(53, 59, 69),       // #353b45
            },
            // Build Kanagawa (Wave) palette
            ThemeChoice::Kanagawa => Theme {
                choice,
                bg: Color::Rgb(31, 31, 40),               // #1f1f28
                fg: Color::Rgb(220, 215, 186),            // #dcd7ba
                primary: Color::Rgb(126, 156, 216),       // #7e9cd8
                secondary: Color::Rgb(149, 127, 184),     // #957fb8
                work: Color::Rgb(228, 104, 118),          // #e46876
                short_break: Color::Rgb(118, 148, 106),   // #76946a
                long_break: Color::Rgb(106, 149, 137),    // #6a9589
                success: Color::Rgb(118, 148, 106),       // #76946a
                warning: Color::Rgb(255, 160, 102),       // #ffa066
                border: Color::Rgb(54, 54, 70),           // #363646
                border_active: Color::Rgb(126, 156, 216), // #7e9cd8
                muted: Color::Rgb(114, 113, 105),         // #727169
                highlight: Color::Rgb(42, 42, 55),        // #2a2a37
            },
            // Build Everforest Dark palette
            ThemeChoice::EverforestDark => Theme {
                choice,
                bg: Color::Rgb(45, 53, 59),               // #2d353b
                fg: Color::Rgb(211, 198, 170),            // #d3c6aa
                primary: Color::Rgb(127, 187, 179),       // #7fbbb3
                secondary: Color::Rgb(214, 153, 182),     // #d699b6
                work: Color::Rgb(230, 126, 128),          // #e67e80
                short_break: Color::Rgb(167, 192, 128),   // #a7c080
                long_break: Color::Rgb(131, 192, 146),    // #83c092
                success: Color::Rgb(167, 192, 128),       // #a7c080
                warning: Color::Rgb(219, 188, 127),       // #dbbc7f
                border: Color::Rgb(71, 82, 88),           // #475258
                border_active: Color::Rgb(167, 192, 128), // #a7c080
                muted: Color::Rgb(133, 146, 137),         // #859289
                highlight: Color::Rgb(52, 63, 68),        // #343f44
            },
            // Build Everforest Light palette (Light)
            ThemeChoice::EverforestLight => Theme {
                choice,
                bg: Color::Rgb(253, 246, 227),           // #fdf6e3
                fg: Color::Rgb(92, 106, 114),            // #5c6a72
                primary: Color::Rgb(58, 148, 134),       // #3a9486
                secondary: Color::Rgb(223, 105, 186),    // #df69ba
                work: Color::Rgb(248, 85, 82),           // #f85552
                short_break: Color::Rgb(141, 161, 1),    // #8da101
                long_break: Color::Rgb(53, 167, 124),    // #35a77c
                success: Color::Rgb(141, 161, 1),        // #8da101
                warning: Color::Rgb(223, 160, 0),        // #dfa000
                border: Color::Rgb(234, 228, 203),       // #eae4cb
                border_active: Color::Rgb(58, 148, 134), // #3a9486
                muted: Color::Rgb(147, 170, 159),        // #93aa9f
                // Slightly darker than the border so selected rows stand out
                highlight: Color::Rgb(223, 216, 190), // #dfd8be
            },
            // Build Synthwave '84 palette
            ThemeChoice::Synthwave84 => Theme {
                choice,
                bg: Color::Rgb(38, 35, 53),             // #262335
                fg: Color::Rgb(240, 239, 241),          // #f0eff1
                primary: Color::Rgb(54, 249, 246),      // #36f9f6
                secondary: Color::Rgb(255, 126, 219),   // #ff7edb
                work: Color::Rgb(254, 68, 80),          // #fe4450
                short_break: Color::Rgb(114, 241, 184), // #72f1b8
                // Synthwave orange instead of cyan: cyan duplicated the primary
                // accent, defeating phase color-coding
                long_break: Color::Rgb(255, 139, 57), // #ff8b39
                success: Color::Rgb(114, 241, 184),   // #72f1b8
                warning: Color::Rgb(254, 222, 93),    // #fede5d
                border: Color::Rgb(73, 67, 99),       // #494363
                border_active: Color::Rgb(255, 126, 219), // #ff7edb
                muted: Color::Rgb(132, 139, 189),     // #848bbd
                highlight: Color::Rgb(52, 41, 79),    // #34294f
            },
            // Build Monokai Pro palette
            ThemeChoice::MonokaiPro => Theme {
                choice,
                bg: Color::Rgb(45, 42, 46),             // #2d2a2e
                fg: Color::Rgb(252, 252, 250),          // #fcfcfa
                primary: Color::Rgb(120, 220, 232),     // #78dce8
                secondary: Color::Rgb(171, 157, 242),   // #ab9df2
                work: Color::Rgb(255, 97, 136),         // #ff6188
                short_break: Color::Rgb(169, 220, 118), // #a9dc76
                // Monokai orange instead of blue: blue duplicated the primary
                // accent, defeating phase color-coding
                long_break: Color::Rgb(252, 152, 103), // #fc9867
                success: Color::Rgb(169, 220, 118),    // #a9dc76
                warning: Color::Rgb(255, 216, 102),    // #ffd866
                border: Color::Rgb(64, 62, 65),        // #403e41
                border_active: Color::Rgb(255, 216, 102), // #ffd866
                muted: Color::Rgb(114, 112, 114),      // #727072
                highlight: Color::Rgb(58, 56, 59),     // #3a383b
            },
            // Build OLED Phosphor palette
            ThemeChoice::OledPhosphor => Theme {
                choice,
                bg: Color::Rgb(0, 0, 0),                // #000000
                fg: Color::Rgb(51, 255, 102),           // #33ff66
                primary: Color::Rgb(0, 255, 102),       // #00ff66
                secondary: Color::Rgb(0, 204, 85),      // #00cc55
                work: Color::Rgb(255, 51, 51),          // #ff3333
                short_break: Color::Rgb(51, 255, 102),  // #33ff66
                long_break: Color::Rgb(0, 255, 255),    // #00ffff
                success: Color::Rgb(51, 255, 102),      // #33ff66
                warning: Color::Rgb(255, 255, 51),      // #ffff33
                border: Color::Rgb(26, 51, 26),         // #1a331a
                border_active: Color::Rgb(0, 255, 102), // #00ff66
                muted: Color::Rgb(26, 102, 42),         // #1a662a
                highlight: Color::Rgb(10, 31, 10),      // #0a1f0a
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
        assert_eq!(all.len(), 18);
        assert!(all.contains(&ThemeChoice::CatppuccinMocha));
        assert!(all.contains(&ThemeChoice::CatppuccinMacchiato));
        assert!(all.contains(&ThemeChoice::CatppuccinFrappe));
        assert!(all.contains(&ThemeChoice::CatppuccinLatte));
        assert!(all.contains(&ThemeChoice::Nord));
        assert!(all.contains(&ThemeChoice::GruvboxDark));
        assert!(all.contains(&ThemeChoice::TokyoNight));
        assert!(all.contains(&ThemeChoice::Dracula));
        assert!(all.contains(&ThemeChoice::SolarizedDark));
        assert!(all.contains(&ThemeChoice::SolarizedLight));
        assert!(all.contains(&ThemeChoice::RosePine));
        assert!(all.contains(&ThemeChoice::OneDark));
        assert!(all.contains(&ThemeChoice::Kanagawa));
        assert!(all.contains(&ThemeChoice::EverforestDark));
        assert!(all.contains(&ThemeChoice::EverforestLight));
        assert!(all.contains(&ThemeChoice::Synthwave84));
        assert!(all.contains(&ThemeChoice::MonokaiPro));
        assert!(all.contains(&ThemeChoice::OledPhosphor));
    }

    #[test]
    fn test_theme_names() {
        assert_eq!(ThemeChoice::CatppuccinMocha.name(), "Catppuccin Mocha");
        assert_eq!(
            ThemeChoice::CatppuccinMacchiato.name(),
            "Catppuccin Macchiato"
        );
        assert_eq!(ThemeChoice::CatppuccinFrappe.name(), "Catppuccin Frappé");
        assert_eq!(ThemeChoice::CatppuccinLatte.name(), "Catppuccin Latte");
        assert_eq!(ThemeChoice::Nord.name(), "Nord");
        assert_eq!(ThemeChoice::GruvboxDark.name(), "Gruvbox Dark");
        assert_eq!(ThemeChoice::TokyoNight.name(), "Tokyo Night");
        assert_eq!(ThemeChoice::Dracula.name(), "Dracula");
        assert_eq!(ThemeChoice::SolarizedDark.name(), "Solarized Dark");
        assert_eq!(ThemeChoice::SolarizedLight.name(), "Solarized Light");
        assert_eq!(ThemeChoice::RosePine.name(), "Rose Pine");
        assert_eq!(ThemeChoice::OneDark.name(), "One Dark");
        assert_eq!(ThemeChoice::Kanagawa.name(), "Kanagawa");
        assert_eq!(ThemeChoice::EverforestDark.name(), "Everforest Dark");
        assert_eq!(ThemeChoice::EverforestLight.name(), "Everforest Light");
        assert_eq!(ThemeChoice::Synthwave84.name(), "Synthwave '84");
        assert_eq!(ThemeChoice::MonokaiPro.name(), "Monokai Pro");
        assert_eq!(ThemeChoice::OledPhosphor.name(), "OLED Phosphor");
    }

    #[test]
    fn test_theme_from_choice_all_variants() {
        for choice in ThemeChoice::all() {
            let theme = Theme::from_choice(*choice);
            assert_eq!(theme.choice, *choice);
            // Verify that all colors are valid non-empty values
            assert_ne!(theme.bg, theme.fg);
            assert_ne!(theme.work, theme.bg);
            assert_ne!(theme.primary, theme.bg);
            assert_ne!(theme.short_break, theme.bg);
            assert_ne!(theme.long_break, theme.bg);
            assert_ne!(theme.success, theme.bg);
            assert_ne!(theme.warning, theme.bg);
            assert_ne!(theme.border_active, theme.border);
        }
    }

    #[test]
    fn test_theme_choice_serde_roundtrip_all() {
        for choice in ThemeChoice::all() {
            let serialized = serde_json::to_string(choice).expect("Serialization failed");
            let deserialized: ThemeChoice =
                serde_json::from_str(&serialized).expect("Deserialization failed");
            assert_eq!(*choice, deserialized);
        }
    }

    #[test]
    fn test_theme_choice_unknown_name_falls_back_to_default() {
        // An unknown theme string must not fail the whole AppData parse; it
        // should silently map to the default theme instead.
        for bogus in ["\"NotATheme\"", "\"\"", "\"catppuccin mocha typo\""] {
            let parsed: ThemeChoice = serde_json::from_str(bogus).expect("must not error");
            assert_eq!(parsed, ThemeChoice::default(), "bogus input: {}", bogus);
        }
        // Case and separator tolerance still resolves known names
        let parsed: ThemeChoice = serde_json::from_str("\"dracula\"").expect("must parse");
        assert_eq!(parsed, ThemeChoice::Dracula);
        let parsed: ThemeChoice = serde_json::from_str("\"Synthwave 84\"").expect("must parse");
        assert_eq!(parsed, ThemeChoice::Synthwave84);
    }

    #[test]
    fn test_theme_luminance_contrast_across_all_18_palettes() {
        for choice in ThemeChoice::all() {
            let theme = Theme::from_choice(*choice);
            let (bg_r, bg_g, bg_b) = match theme.bg {
                Color::Rgb(r, g, b) => (r as f32, g as f32, b as f32),
                _ => (0.0, 0.0, 0.0),
            };
            let (fg_r, fg_g, fg_b) = match theme.fg {
                Color::Rgb(r, g, b) => (r as f32, g as f32, b as f32),
                _ => (255.0, 255.0, 255.0),
            };

            let lum_bg = (0.2126 * bg_r + 0.7152 * bg_g + 0.0722 * bg_b) / 255.0;
            let lum_fg = (0.2126 * fg_r + 0.7152 * fg_g + 0.0722 * fg_b) / 255.0;
            let contrast_diff = (lum_bg - lum_fg).abs();

            // All 18 palettes must maintain a significant luminance difference between fg and bg
            assert!(
                contrast_diff > 0.20,
                "Theme {:?} has insufficient luminance contrast: {}",
                choice,
                contrast_diff
            );
        }
    }

    #[test]
    fn test_theme_palette_index_cycling() {
        let all = ThemeChoice::all();

        // Expected palette count keeps accidental variant additions/removals visible
        assert_eq!(all.len(), 18);

        // No duplicates: cycling would otherwise skip or repeat themes
        for (i, choice) in all.iter().enumerate() {
            assert!(
                !all[..i].contains(choice),
                "ThemeChoice::all() contains duplicate {:?}",
                choice
            );
        }

        // Simulate the UI's forward cycle ((i + 1) % len): starting anywhere,
        // stepping all.len() times must visit every theme exactly once and
        // land back on the starting theme.
        for start in 0..all.len() {
            let mut idx = start;
            let mut visited: Vec<ThemeChoice> = Vec::with_capacity(all.len());
            for _ in 0..all.len() {
                assert!(
                    !visited.contains(&all[idx]),
                    "cycle revisited {:?}",
                    all[idx]
                );
                visited.push(all[idx]);
                idx = (idx + 1) % all.len();
            }
            assert_eq!(
                idx, start,
                "cycle from {} did not return to its start",
                start
            );
        }

        // The last element's forward neighbor is the first element, and the
        // two must be distinct so pressing Next on the last theme visibly moves
        assert_ne!(all[all.len() - 1], all[0]);
    }

    #[test]
    fn test_theme_phase_colors_distinctness() {
        for choice in ThemeChoice::all() {
            let theme = Theme::from_choice(*choice);
            // Work color and ShortBreak color should never be identical in any theme
            assert_ne!(
                format!("{:?}", theme.work),
                format!("{:?}", theme.short_break),
                "Theme {:?} work and short_break colors collide",
                choice
            );
            // Long break must be visually distinct from both accent colors,
            // otherwise the phase loses its own color-coding
            assert_ne!(
                format!("{:?}", theme.long_break),
                format!("{:?}", theme.primary),
                "Theme {:?} long_break and primary colors collide",
                choice
            );
            assert_ne!(
                format!("{:?}", theme.long_break),
                format!("{:?}", theme.secondary),
                "Theme {:?} long_break and secondary colors collide",
                choice
            );
            // Selected rows would be invisible if highlight matched the border
            assert_ne!(
                format!("{:?}", theme.highlight),
                format!("{:?}", theme.border),
                "Theme {:?} highlight and border colors collide",
                choice
            );
        }
    }

    #[test]
    fn test_theme_default_fallback_is_catppuccin_mocha() {
        assert_eq!(ThemeChoice::default(), ThemeChoice::CatppuccinMocha);
        let theme = Theme::from_choice(ThemeChoice::default());
        assert_eq!(theme.choice, ThemeChoice::CatppuccinMocha);
    }

    #[test]
    fn test_theme_rgb_components_within_byte_bounds() {
        for choice in ThemeChoice::all() {
            let theme = Theme::from_choice(*choice);
            let colors = [
                theme.bg,
                theme.fg,
                theme.primary,
                theme.secondary,
                theme.work,
                theme.short_break,
                theme.long_break,
                theme.success,
                theme.warning,
                theme.border,
                theme.border_active,
                theme.muted,
                theme.highlight,
            ];
            for c in colors {
                match c {
                    Color::Rgb(r, g, b) => {
                        // Pattern match verifies 24-bit TrueColor RGB encoding
                        let _ = (r, g, b);
                    }
                    _ => panic!("Expected RGB color in theme"),
                }
            }
        }
    }

    #[test]
    fn test_theme_choice_clone_and_copy() {
        let original = ThemeChoice::TokyoNight;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(format!("{:?}", original), "TokyoNight");
    }

    // The Settings UI and footer display `name()` strings. A user who copies
    // that exact string into data.json must get the same theme back, not a
    // silent fallback to the default palette.
    #[test]
    fn test_from_str_parses_every_display_name() {
        for choice in ThemeChoice::all() {
            let parsed: ThemeChoice = choice
                .name()
                .parse()
                .unwrap_or_else(|_| panic!("display name {:?} must parse", choice.name()));
            assert_eq!(
                parsed,
                *choice,
                "display name {:?} parsed to the wrong variant",
                choice.name()
            );
        }
    }

    // Serialization writes the Debug variant name; it must always round-trip.
    #[test]
    fn test_from_str_parses_every_variant_name() {
        for choice in ThemeChoice::all() {
            let variant = format!("{:?}", choice);
            let parsed: ThemeChoice = variant
                .parse()
                .unwrap_or_else(|_| panic!("variant name {:?} must parse", variant));
            assert_eq!(parsed, *choice);
        }
    }

    // Names shown in the UI contain an accented character (Catppuccin Frappé)
    // and an apostrophe (Synthwave '84). Both spellings must resolve instead
    // of silently degrading to the default theme.
    #[test]
    fn test_from_str_handles_accented_and_apostrophe_names() {
        let frappe: ThemeChoice = "Catppuccin Frappé".parse().expect("accented é must fold");
        assert_eq!(frappe, ThemeChoice::CatppuccinFrappe);

        let synthwave: ThemeChoice = "Synthwave '84".parse().expect("apostrophe must be ignored");
        assert_eq!(synthwave, ThemeChoice::Synthwave84);

        // Curly-quote variant some editors substitute when hand-editing JSON
        let curly: ThemeChoice = "Synthwave \u{2019}84"
            .parse()
            .expect("curly quote tolerated");
        assert_eq!(curly, ThemeChoice::Synthwave84);
    }

    // Hand-edited configs are frequently sloppy: random casing, stray
    // separators, padding whitespace. All of it must still resolve.
    #[test]
    fn test_from_str_loose_formatting_tolerated() {
        let cases = [
            ("  nord  ", ThemeChoice::Nord),
            ("DRACULA", ThemeChoice::Dracula),
            ("tokyo-night", ThemeChoice::TokyoNight),
            ("tokyo_night", ThemeChoice::TokyoNight),
            ("catppuccin latte", ThemeChoice::CatppuccinLatte),
            ("one-dark", ThemeChoice::OneDark),
            ("oled phosphor", ThemeChoice::OledPhosphor),
            ("gruvboxdark", ThemeChoice::GruvboxDark),
        ];
        for (input, expected) in cases {
            let parsed: ThemeChoice = input
                .parse()
                .unwrap_or_else(|_| panic!("{:?} should parse", input));
            assert_eq!(parsed, expected, "{:?} resolved to the wrong theme", input);
        }
    }

    // Genuinely unknown names must keep falling back to the default rather
    // than panicking or inventing a variant.
    #[test]
    fn test_deserialize_unknown_theme_falls_back_to_default() {
        let json = r#""NotARealTheme""#;
        let parsed: ThemeChoice = serde_json::from_str(json).expect("unknown names never error");
        assert_eq!(parsed, ThemeChoice::default());
    }
}
