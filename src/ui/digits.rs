// Renders minutes and seconds as a 5-line tall ASCII/block digit graphic banner
pub fn render_big_time(mins: u32, secs: u32) -> Vec<String> {
    // Format minutes and seconds into a 5-character string e.g. "25:00"
    let str_time = format!("{:02}:{:02}", mins, secs);
    // Initialize 5 empty lines for the 5-row block font
    let mut lines = vec![
        // Row 0
        String::new(),
        // Row 1
        String::new(),
        // Row 2
        String::new(),
        // Row 3
        String::new(),
        // Row 4
        String::new(),
    ];

    // Iterate through each character in the formatted time string with its character index
    for (i, ch) in str_time.chars().enumerate() {
        // Retrieve 5-row pattern matrix for this character
        let pattern = char_pattern(ch);
        // Loop over the 5 rows
        for row in 0..5 {
            // Add a single spacing space between consecutive characters
            if i > 0 {
                // Append space delimiter
                lines[row].push(' ');
            }
            // Append the row segment for this character
            lines[row].push_str(pattern[row]);
        }
    }

    // Return the completed 5 lines of big text
    lines
}

// Maps an individual character ('0'..'9', ':') to its 5-row 4-column block graphic slice
fn char_pattern(ch: char) -> [&'static str; 5] {
    // Match the character
    match ch {
        // Digit 0 pattern
        '0' => [
            // Row 0
            "████",
            // Row 1
            "█  █",
            // Row 2
            "█  █",
            // Row 3
            "█  █",
            // Row 4
            "████",
        ],
        // Digit 1 pattern
        '1' => [
            // Row 0
            "  ██", // Row 1
            "  ██", // Row 2
            "  ██", // Row 3
            "  ██", // Row 4
            "  ██",
        ],
        // Digit 2 pattern
        '2' => [
            // Row 0
            "████",
            // Row 1
            "   █",
            // Row 2
            "████",
            // Row 3
            "█   ",
            // Row 4
            "████",
        ],
        // Digit 3 pattern
        '3' => [
            // Row 0
            "████",
            // Row 1
            "   █",
            // Row 2
            "████",
            // Row 3
            "   █",
            // Row 4
            "████",
        ],
        // Digit 4 pattern
        '4' => [
            // Row 0
            "█  █",
            // Row 1
            "█  █",
            // Row 2
            "████",
            // Row 3
            "   █",
            // Row 4
            "   █",
        ],
        // Digit 5 pattern
        '5' => [
            // Row 0
            "████",
            // Row 1
            "█   ",
            // Row 2
            "████",
            // Row 3
            "   █",
            // Row 4
            "████",
        ],
        // Digit 6 pattern
        '6' => [
            // Row 0
            "████",
            // Row 1
            "█   ",
            // Row 2
            "████",
            // Row 3
            "█  █",
            // Row 4
            "████",
        ],
        // Digit 7 pattern
        '7' => [
            // Row 0
            "████",
            // Row 1
            "   █",
            // Row 2
            "   █",
            // Row 3
            "   █",
            // Row 4
            "   █",
        ],
        // Digit 8 pattern
        '8' => [
            // Row 0
            "████",
            // Row 1
            "█  █",
            // Row 2
            "████",
            // Row 3
            "█  █",
            // Row 4
            "████",
        ],
        // Digit 9 pattern
        '9' => [
            // Row 0
            "████",
            // Row 1
            "█  █",
            // Row 2
            "████",
            // Row 3
            "   █",
            // Row 4
            "████",
        ],
        // Colon ':' separator pattern
        ':' => [
            // Row 0 (empty)
            " ", // Row 1 (top dot)
            "█", // Row 2 (middle space)
            " ", // Row 3 (bottom dot)
            "█", // Row 4 (empty)
            " ",
        ],
        // Fallback for unexpected characters
        _ => [
            // Row 0
            "    ", // Row 1
            "    ", // Row 2
            "    ", // Row 3
            "    ", // Row 4
            "    ",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_big_time_structure() {
        let lines = render_big_time(25, 0);
        assert_eq!(lines.len(), 5);
        // All 5 rows should have the exact same character length
        let len0 = lines[0].chars().count();
        assert!(len0 > 0);
        for line in &lines {
            assert_eq!(line.chars().count(), len0);
        }
    }

    #[test]
    fn test_render_big_time_various_values() {
        let cases = [(0, 0), (5, 9), (25, 30), (99, 59)];
        for (m, s) in cases {
            let lines = render_big_time(m, s);
            assert_eq!(lines.len(), 5);
            for row in lines {
                assert!(!row.is_empty());
            }
        }
    }

    #[test]
    fn test_char_pattern_all_valid_chars() {
        let digits = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '?', 'X', ' ',
        ];
        for c in digits {
            let pattern = char_pattern(c);
            assert_eq!(pattern.len(), 5);
            for row in pattern {
                assert_eq!(row.chars().count(), 4);
            }
        }

        let colon_pattern = char_pattern(':');
        assert_eq!(colon_pattern.len(), 5);
        for row in colon_pattern {
            assert_eq!(row.chars().count(), 1);
        }
    }

    #[test]
    fn test_render_big_time_boundary_values() {
        // High minutes
        let lines = render_big_time(120, 0);
        assert_eq!(lines.len(), 5);
        // All rows same width
        let len0 = lines[0].chars().count();
        for line in &lines {
            assert_eq!(line.chars().count(), len0);
        }
    }

    #[test]
    fn test_big_digits_various_large_minutes_formatting() {
        for (m, s) in [(0, 0), (1, 5), (25, 30), (99, 59), (999, 0)] {
            let lines = render_big_time(m, s);
            assert_eq!(lines.len(), 5);
            let first_width = lines[0].chars().count();
            assert!(first_width > 0);
            for line in &lines {
                assert_eq!(line.chars().count(), first_width);
            }
        }
    }
}
