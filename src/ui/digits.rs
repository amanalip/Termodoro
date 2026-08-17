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
            "  ██",
            // Row 1
            "  ██",
            // Row 2
            "  ██",
            // Row 3
            "  ██",
            // Row 4
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
            " ",
            // Row 1 (top dot)
            "█",
            // Row 2 (middle space)
            " ",
            // Row 3 (bottom dot)
            "█",
            // Row 4 (empty)
            " ",
        ],
        // Fallback for unexpected characters
        _ => [
            // Row 0
            "    ",
            // Row 1
            "    ",
            // Row 2
            "    ",
            // Row 3
            "    ",
            // Row 4
            "    ",
        ],
    }
}
