// Import ratatui layout and styling primitives for rendering floating popup dialog
use ratatui::{
    // Alignment and rectangular layout dimensions
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    // Styling attributes
    style::{Modifier, Style},
    // Line and Span text building blocks
    text::{Line, Span},
    // Block, BorderType, Borders, Clear, and Paragraph widgets
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    // Current frame handle
    Frame,
};

// Import Theme struct for styling
use crate::theme::Theme;

// Helper function to calculate a centered rectangular area for modal popups
//
// Percentages above 100 are clamped to 100: the raw arithmetic
// `(100 - percent)` would underflow u16 and panic in debug builds for
// out-of-range inputs from future callers or refactors.
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Clamp both axes into the legal percentage domain
    let percent_x = percent_x.min(100);
    let percent_y = percent_y.min(100);

    // Split vertical space to center content vertically
    let popup_layout = Layout::default()
        // Top-to-bottom layout
        .direction(Direction::Vertical)
        // Three constraints: top margin, content height, bottom margin
        .constraints([
            // Top padding
            Constraint::Percentage((100 - percent_y) / 2),
            // Popup content height
            Constraint::Percentage(percent_y),
            // Bottom padding
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        // Split given area
        .split(r);

    // Split horizontal space inside the middle vertical slice
    Layout::default()
        // Left-to-right layout
        .direction(Direction::Horizontal)
        // Three constraints: left margin, content width, right margin
        .constraints([
            // Left padding
            Constraint::Percentage((100 - percent_x) / 2),
            // Popup content width
            Constraint::Percentage(percent_x),
            // Right padding
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        // Split middle vertical slice
        .split(popup_layout[1])[1]
}

// Renders the Help & Keybindings popup modal overlay
pub fn render(f: &mut Frame, theme: &Theme, area: Rect) {
    // Calculate centered rectangular area (84% width, 94% height for comfortable reading)
    let popup_area = centered_rect(84, 94, area);
    // Clear underlying background beneath popup area to avoid visual bleeding
    f.render_widget(Clear, popup_area);

    // Define table of keybindings and descriptions
    let help_lines = vec![
        // Section: Navigation
        Line::from(Span::styled(
            "Navigation & Tabs",
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  [Tab] / [Shift+Tab]   Cycle tabs (Timer, Tasks, Stats, Settings)"),
        Line::from("  [1] - [4]             Jump to tab 1-4 (Timer, Tasks, Stats, Settings)"),
        Line::from("  [?]                   Toggle this Help dialog"),
        Line::from("  [q] / [Esc]           Close popup / Quit application"),
        Line::from(""),
        // Section: Pomodoro Timer Controls
        Line::from(Span::styled(
            "Timer Controls (Tab 1)",
            Style::default().fg(theme.work).add_modifier(Modifier::BOLD),
        )),
        Line::from("  [Space]               Start / Pause timer countdown"),
        Line::from("  [r]                   Reset timer to beginning of current phase"),
        Line::from("  [s]                   Skip current phase (not counted as completed)"),
        Line::from("  [a]                   Quickly add a new task"),
        Line::from(""),
        // Section: Task Management Controls
        Line::from(Span::styled(
            "Task Management (Tab 2)",
            Style::default()
                .fg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  [a]                   Create a new task"),
        Line::from("  [Space] / [Enter]     Toggle task completion status"),
        Line::from("  [t]                   Set selected task as active target for timer"),
        Line::from("  [d] / [x]             Delete selected task"),
        Line::from("  [↑ / k], [↓ / j]      Navigate tasks list"),
        Line::from("  [1] / [2] / [3]       Filter tasks (All, Active, Completed)"),
        Line::from(""),
        // Section: Settings & Preferences Controls
        Line::from(Span::styled(
            "Settings Controls (Tab 4)",
            Style::default()
                .fg(theme.warning)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  [↑ / k], [↓ / j]      Select setting option"),
        Line::from("  [← / h], [→ / l]      Adjust duration / cycle through color themes"),
        Line::from("  [+], [-]              Increment / Decrement values"),
        Line::from("  [Space] / [Enter]     Toggle feature flags on / off"),
    ];

    // Build the popup block widget
    let help_paragraph = Paragraph::new(help_lines)
        // Attach styled block
        .block(
            Block::default()
                // Title
                .title(Span::styled(
                    " ❓ Termodoro Keybindings & Help ",
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                ))
                // All borders
                .borders(Borders::ALL)
                // Rounded corners
                .border_type(BorderType::Rounded)
                // Active border color
                .border_style(Style::default().fg(theme.border_active))
                // Background fill
                .style(Style::default().bg(theme.bg)),
        )
        // Left alignment with left padding
        .alignment(Alignment::Left);

    // Render the help modal into the popup area
    f.render_widget(help_paragraph, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    // The popup must always sit strictly inside its parent area for a matrix
    // of sane percentages and terminal geometries, including degenerate ones.
    #[test]
    fn centered_rect_stays_inside_parent_across_matrix() {
        let geometries = [(80u16, 24u16), (40, 12), (25, 8), (200, 60), (1, 1)];
        let percents = [(84u16, 94u16), (60, 35), (100, 100), (10, 10), (99, 1)];
        for (w, h) in geometries {
            let parent = Rect::new(0, 0, w, h);
            for (px, py) in percents {
                let popup = centered_rect(px, py, parent);
                assert!(
                    popup.x >= parent.x
                        && popup.y >= parent.y
                        && popup.right() <= parent.right()
                        && popup.bottom() <= parent.bottom(),
                    "popup {px}%x{py}% escaped parent {}x{}: {popup:?}",
                    w,
                    h
                );
            }
        }
    }

    // 100% x 100% must cover the whole parent (integer layout rounding may
    // leave it one row/column short at odd sizes; it may never exceed it).
    #[test]
    fn centered_rect_full_percent_covers_parent() {
        let parent = Rect::new(0, 0, 80, 24);
        let popup = centered_rect(100, 100, parent);
        assert!(popup.width >= parent.width.saturating_sub(1));
        assert!(popup.height >= parent.height.saturating_sub(1));
    }

    // Percentages above 100 previously underflowed `(100 - p)` and panicked
    // in debug builds. They now clamp to the full-area behavior of 100%.
    #[test]
    fn centered_rect_clamps_percentages_over_100_without_panicking() {
        let parent = Rect::new(0, 0, 80, 24);
        let clamped = centered_rect(100, 100, parent);
        let over = centered_rect(u16::MAX, u16::MAX, parent);
        assert_eq!(clamped, over, "values above 100 clamp to 100");

        // Mixed over/under inputs are equally safe.
        let _ = centered_rect(u16::MAX, 50, parent);
        let _ = centered_rect(50, u16::MAX, parent);
    }

    // A zero-sized parent must yield a zero-sized popup, not a panic.
    #[test]
    fn centered_rect_zero_sized_parent_is_safe() {
        let parent = Rect::new(5, 5, 0, 0);
        let popup = centered_rect(84, 94, parent);
        assert_eq!(popup.width, 0);
        assert_eq!(popup.height, 0);
    }

    // Horizontal centering: left and right margins must match within one
    // column (integer percentage rounding can differ by exactly one).
    #[test]
    fn centered_rect_horizontal_margins_symmetric_within_one_column() {
        for w in [20u16, 33, 47, 80, 101, 120] {
            let parent = Rect::new(0, 0, w, 24);
            let popup = centered_rect(60, 35, parent);
            let left = popup.x - parent.x;
            let right = parent.right() - popup.right();
            assert!(
                left.abs_diff(right) <= 1,
                "width {w}: margins l={left} r={right} diverge by more than rounding"
            );
        }
    }

    // Larger percentages must never produce a smaller popup.
    #[test]
    fn centered_rect_monotonic_in_percentage() {
        let parent = Rect::new(0, 0, 100, 40);
        let mut prev_area = 0u32;
        for p in [20u16, 40, 60, 80, 100] {
            let popup = centered_rect(p, p, parent);
            let area = popup.width as u32 * popup.height as u32;
            assert!(
                area >= prev_area,
                "popup area shrank when percentage grew to {p}"
            );
            prev_area = area;
        }
    }
}
