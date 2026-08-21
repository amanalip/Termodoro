// Import ratatui layout modules for positioning widgets in the terminal
use ratatui::{
    // Import Alignment enum for text alignment (Left, Center, Right)
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    // Import Style and Modifier for colors and font attributes
    style::{Modifier, Style},
    // Import Line and Span for rich styled text segments
    text::{Line, Span},
    // Import visual widgets: Block, BorderType, Borders, Gauge progress bar, and Paragraph
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    // Import Frame representing the current render frame
    Frame,
};

// Import App state from our app module
use crate::app::App;
// Import Theme structure for styling
use crate::theme::Theme;
// Import PomodoroPhase and TimerStatus enums from timer module
use crate::timer::{PomodoroPhase, TimerStatus};
// Import the render_big_time helper for drawing large digits
use crate::ui::digits::render_big_time;

// Builds the visual cycle dot indicator, e.g. "● ● ◉ ○".
//
// Extracted from render() as a pure function so the dot-state machine is
// directly unit-testable. The interval is clamped to the same 1..=24 range
// the Settings UI enforces: rendering trusts config values, and a corrupt
// data.json with a huge interval previously looped billions of times per
// frame and froze the UI before storage sanitization could help.
//
// KNOWN EDGE CASE (pinned by tests): when `current_cycle` exceeds
// `interval` — possible if the user shrinks the long-break interval below
// their live cycle position mid-session — every dot renders filled until the
// next natural completion recovers via advance_phase's >= check.
fn build_cycle_dots(current_cycle: u32, interval: u32, phase: PomodoroPhase) -> String {
    // Clamp to the display maximum
    let cycle_display_max = interval.clamp(1, 24);
    let mut cycle_dots = String::new();
    // Iterate from 1 to the clamped cycle count
    for i in 1..=cycle_display_max {
        // Check if cycle is completed, in progress, or upcoming
        if i < current_cycle {
            // Completed cycle dot
            cycle_dots.push('●');
        } else if i == current_cycle && phase == PomodoroPhase::Work {
            // Active focus session dot
            cycle_dots.push('◉');
        } else {
            // Upcoming cycle dot
            cycle_dots.push('○');
        }
        // Add space separator between dots
        if i < cycle_display_max {
            // Append space
            cycle_dots.push(' ');
        }
    }
    cycle_dots
}

// Renders the main Pomodoro Timer tab UI inside the given bounding area
pub fn render(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Split area vertically into 5 sections: Header, Big Digits, Progress Gauge, Target Box, Controls
    let chunks = Layout::default()
        // Arrange items vertically from top to bottom
        .direction(Direction::Vertical)
        // Define sizes for each vertical segment
        .constraints([
            // Header bar for phase name, running status, and cycle indicators
            Constraint::Length(3),
            // Big digit countdown banner
            Constraint::Length(7),
            // Progress gauge bar
            Constraint::Length(3),
            // Active task target information panel
            Constraint::Length(4),
            // Keybindings and daily summary controls
            Constraint::Min(2),
        ])
        // Margin around the container
        .margin(1)
        // Compute split layout chunks
        .split(area);

    // Determine the theme accent color corresponding to the active phase
    let phase_color = match app.timer.phase {
        // Red / warm color for work sessions
        PomodoroPhase::Work => theme.work,
        // Green / cool color for short breaks
        PomodoroPhase::ShortBreak => theme.short_break,
        // Teal / calm color for long breaks
        PomodoroPhase::LongBreak => theme.long_break,
    };

    // 1. Phase, Status & Cycle Header
    let status_str = match app.timer.status {
        // Status string when countdown is active
        TimerStatus::Running => "● RUNNING",
        // Status string when countdown is paused
        TimerStatus::Paused => "❚❚ PAUSED",
        // Status string when countdown is stopped
        TimerStatus::Stopped => "■ READY",
    };

    // Determine text style for current status
    let status_style = match app.timer.status {
        // Green bold text when running
        TimerStatus::Running => Style::default()
            .fg(theme.success)
            .add_modifier(Modifier::BOLD),
        // Yellow bold text when paused
        TimerStatus::Paused => Style::default()
            .fg(theme.warning)
            .add_modifier(Modifier::BOLD),
        // Muted text when stopped/ready
        TimerStatus::Stopped => Style::default().fg(theme.muted),
    };

    // Construct visual cycle dot indicator e.g. "● ● ◉ ○"
    let cycle_dots = build_cycle_dots(
        app.timer.current_cycle,
        app.config.long_break_interval,
        app.timer.phase,
    );

    // Build the centered header paragraph
    let phase_header = Paragraph::new(vec![
        // Single line containing Phase Emoji + Title, Status badge, and Cycle tracker
        Line::from(vec![
            // Phase emoji and title
            Span::styled(
                format!("{} {}  ", app.timer.phase.emoji(), app.timer.phase.title()),
                Style::default()
                    .fg(phase_color)
                    .add_modifier(Modifier::BOLD),
            ),
            // Status text badge
            Span::styled(format!("[{}]  ", status_str), status_style),
            // Cycle counter and dots
            Span::styled(
                format!(
                    "Cycle {}/{} [{}]",
                    app.timer.current_cycle, app.config.long_break_interval, cycle_dots
                ),
                Style::default().fg(theme.fg),
            ),
        ]),
    ])
    // Center-align the header line
    .alignment(Alignment::Center);
    // Render the header into the top chunk
    f.render_widget(phase_header, chunks[0]);

    // 2. Big Digits Countdown Banner
    let (mins, secs) = app.timer.formatted_time();
    // Render the 5 lines of ASCII block font
    let big_lines = render_big_time(mins, secs);
    // Convert string lines to styled Lines with the active phase color
    let digit_spans: Vec<Line> = big_lines
        .into_iter()
        // Map each string line to a Line widget
        .map(|line| {
            // Construct styled line
            Line::from(Span::styled(
                line,
                Style::default()
                    .fg(phase_color)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        // Collect into vector
        .collect();

    // Create center-aligned paragraph widget with the big digits
    let digits_widget = Paragraph::new(digit_spans).alignment(Alignment::Center);
    // Render digits into the second chunk
    f.render_widget(digits_widget, chunks[1]);

    // 3. Gauge Progress Bar
    let ratio = app.timer.progress_ratio().clamp(0.0, 1.0);
    // Convert ratio to percentage integer
    let percent = (ratio * 100.0) as u16;
    // Build the Gauge widget
    let gauge = Gauge::default()
        // Attach rounded border block
        .block(
            Block::default()
                // Enable all borders
                .borders(Borders::ALL)
                // Use smooth rounded border corners
                .border_type(BorderType::Rounded)
                // Border color from theme
                .border_style(Style::default().fg(theme.border)),
        )
        // Style the filled bar with phase color and empty bar with highlight color
        .gauge_style(Style::default().fg(phase_color).bg(theme.highlight))
        // Set filled percentage
        .percent(percent)
        // Display percentage label inside the bar
        .label(format!("{:.0}%", percent));
    // Render gauge into the third chunk
    f.render_widget(gauge, chunks[2]);

    // 4. Active Task Box
    let active_task_content = if let Some(task) = app.tasks.active_task() {
        // Format spent vs estimated pomodoros string
        let est_str = if task.pomodoros_estimated > 0 {
            // Show ratio e.g. "2/4"
            format!("{}/{}", task.pomodoros_spent, task.pomodoros_estimated)
        } else {
            // Show spent count only
            format!("{}", task.pomodoros_spent)
        };
        // Construct styled text line for active task
        vec![Line::from(vec![
            // Label prefix
            Span::styled("  Active Focus: ", Style::default().fg(theme.muted)),
            // Task title in bold text
            Span::styled(
                &task.title,
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            // Tomato count badge
            Span::styled(
                format!("  (🍅 {})", est_str),
                Style::default().fg(theme.secondary),
            ),
        ])]
    } else {
        // Placeholder message when no task is selected
        vec![Line::from(vec![Span::styled(
            "  No active task selected. Press [Tab] to view Tasks, or [a] to quickly add one.",
            Style::default()
                .fg(theme.muted)
                .add_modifier(Modifier::ITALIC),
        )])]
    };

    // Create target box panel
    let active_task_box = Paragraph::new(active_task_content).block(
        Block::default()
            // Panel title
            .title(Span::styled(
                " 🎯 Current Target ",
                Style::default().fg(theme.primary),
            ))
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border style
            .border_style(Style::default().fg(theme.border)),
    );
    // Render active task box into the fourth chunk
    f.render_widget(active_task_box, chunks[3]);

    // 5. Quick Controls & Daily Stats Summary
    let today_done = app.stats.today_work_sessions();
    // Today's focus minutes
    let today_mins = app.stats.today_focus_minutes();
    // Current daily streak
    let streak = app.stats.current_streak_days();

    // Assemble controls and summary lines
    let controls_text = vec![
        // First line: Today's productivity and streak
        Line::from(vec![
            Span::styled("  Today's Sessions: ", Style::default().fg(theme.muted)),
            Span::styled(
                format!("{} pomodoros ({} mins)", today_done, today_mins),
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("   │   Streak: ", Style::default().fg(theme.muted)),
            Span::styled(
                format!("🔥 {} days", streak),
                Style::default()
                    .fg(theme.warning)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        // Blank separator line
        Line::from(""),
        // Second line: Keybinding quick reference
        Line::from(vec![
            Span::styled(
                "  [Space] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if app.timer.status == TimerStatus::Running {
                    "Pause"
                } else {
                    "Start"
                },
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                "   [r] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Reset", Style::default().fg(theme.fg)),
            Span::styled(
                "   [s] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Skip Phase", Style::default().fg(theme.fg)),
            Span::styled(
                "   [a] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Quick Add Task", Style::default().fg(theme.fg)),
        ]),
    ];

    // Build controls paragraph
    let controls_widget = Paragraph::new(controls_text);
    // Render controls widget into bottom chunk
    f.render_widget(controls_widget, chunks[4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mid-cycle during a Work phase: completed dots, one active dot, rest
    // upcoming.
    #[test]
    fn cycle_dots_mid_cycle_during_work() {
        assert_eq!(build_cycle_dots(3, 4, PomodoroPhase::Work), "● ● ◉ ○");
        assert_eq!(build_cycle_dots(1, 4, PomodoroPhase::Work), "◉ ○ ○ ○");
    }

    // During any break there is no active focus dot: the current position
    // renders as upcoming, not in-progress.
    #[test]
    fn cycle_dots_show_no_active_dot_during_breaks() {
        assert_eq!(build_cycle_dots(2, 4, PomodoroPhase::ShortBreak), "● ○ ○ ○");
        assert_eq!(build_cycle_dots(1, 4, PomodoroPhase::LongBreak), "○ ○ ○ ○");
    }

    // Final cycle slot fully completed: every dot filled.
    #[test]
    fn cycle_dots_all_completed_at_interval_boundary() {
        assert_eq!(build_cycle_dots(5, 4, PomodoroPhase::Work), "● ● ● ●");
    }

    // KNOWN DISPLAY DESYNC (pinned): shrinking long_break_interval below the
    // live cycle position mid-session shows e.g. "Cycle 5/2" with all dots
    // filled. No panic occurs and advance_phase's >= check recovers on the
    // next completion; this test documents the rendering so a future fix
    // (clamping the displayed numerator) is a conscious choice.
    #[test]
    fn cycle_dots_desync_when_interval_shrinks_below_current_cycle() {
        let dots = build_cycle_dots(5, 2, PomodoroPhase::Work);
        assert_eq!(dots, "● ●", "all visible dots render filled during desync");

        let dots = build_cycle_dots(25, 24, PomodoroPhase::Work);
        assert_eq!(dots.split(' ').count(), 24);
        assert!(!dots.contains('○'), "no upcoming dots remain at max desync");
    }

    // The interval is clamped to 1..=24 for display: corrupt huge values
    // render exactly 24 dots instead of freezing the frame loop.
    #[test]
    fn cycle_dots_clamp_huge_and_zero_intervals() {
        let huge = build_cycle_dots(1, u32::MAX, PomodoroPhase::Work);
        assert_eq!(
            huge.split(' ').count(),
            24,
            "huge interval clamps to 24 dots"
        );
        assert!(huge.starts_with("◉ "));

        let zero = build_cycle_dots(1, 0, PomodoroPhase::Work);
        assert_eq!(zero, "◉", "zero interval clamps to a single dot");
    }

    // Dot count always equals clamped interval, whatever the inputs.
    #[test]
    fn cycle_dots_count_always_matches_clamped_interval() {
        for interval in [0u32, 1, 2, 7, 23, 24, 25, 1000] {
            let dots = build_cycle_dots(3, interval, PomodoroPhase::Work);
            let expected = interval.clamp(1, 24) as usize;
            assert_eq!(
                dots.split(' ').count(),
                expected,
                "interval {interval} must render {expected} dots"
            );
        }
    }
}
