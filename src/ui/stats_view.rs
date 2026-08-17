// Import Local timezone from chrono for formatting local dates and times
use chrono::Local;
// Import ratatui layout and widget primitives
use ratatui::{
    // Layout constraints and rectangular areas
    layout::{Constraint, Direction, Layout, Rect},
    // Styling attributes
    style::{Modifier, Style},
    // Text Line and Span primitives
    text::{Line, Span},
    // Widgets for BarChart, Block, Table, and Paragraphs
    widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    // Current frame handle
    Frame,
};

// Import App state
use crate::app::App;
// Import Theme structure for styling
use crate::theme::Theme;

// Renders the productivity statistics, daily streak tracker, and activity charts
pub fn render(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Split vertical space into 3 sections: Summary Cards, Activity BarChart, Recent History Table
    let chunks = Layout::default()
        // Vertical layout
        .direction(Direction::Vertical)
        // Heights for each panel
        .constraints([
            // Top metric summary cards
            Constraint::Length(6),
            // Middle weekly activity bar chart
            Constraint::Length(10),
            // Bottom recent completed sessions list
            Constraint::Min(6),
        ])
        // Outer margin
        .margin(1)
        // Split layout
        .split(area);

    // 1. Metric Summary Cards (Horizontal Split into 3 columns)
    let card_chunks = Layout::default()
        // Horizontal arrangement
        .direction(Direction::Horizontal)
        // Three equal columns
        .constraints([
            // Today's focus card
            Constraint::Percentage(33),
            // Current streak card
            Constraint::Percentage(33),
            // All-time focus card
            Constraint::Percentage(34),
        ])
        // Split top chunk horizontally
        .split(chunks[0]);

    // Card 1: Today's Focus Metrics
    let today_sessions = app.stats.today_work_sessions();
    // Calculate today's focus minutes
    let today_mins = app.stats.today_focus_minutes();
    // Build paragraph widget for Card 1
    let card1 = Paragraph::new(vec![
        // Line showing pomodoro count
        Line::from(Span::styled(
            format!("{} 🍅", today_sessions),
            Style::default().fg(theme.work).add_modifier(Modifier::BOLD),
        )),
        // Line showing minutes
        Line::from(Span::styled(
            format!("{} minutes focused", today_mins),
            Style::default().fg(theme.muted),
        )),
    ])
    // Attach styled block
    .block(
        Block::default()
            // Title
            .title(Span::styled(" 📅 Today's Focus ", Style::default().fg(theme.primary)))
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render Card 1
    f.render_widget(card1, card_chunks[0]);

    // Card 2: Daily Streak Metrics
    let cur_streak = app.stats.current_streak_days();
    // Calculate longest personal streak
    let max_streak = app.stats.longest_streak_days();
    // Build paragraph widget for Card 2
    let card2 = Paragraph::new(vec![
        // Line showing current streak with flame emoji
        Line::from(Span::styled(
            format!("🔥 {} Days", cur_streak),
            Style::default().fg(theme.warning).add_modifier(Modifier::BOLD),
        )),
        // Line showing personal best record
        Line::from(Span::styled(
            format!("Personal Best: {} Days", max_streak),
            Style::default().fg(theme.muted),
        )),
    ])
    // Attach styled block
    .block(
        Block::default()
            // Title
            .title(Span::styled(" ⚡ Current Streak ", Style::default().fg(theme.primary)))
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render Card 2
    f.render_widget(card2, card_chunks[1]);

    // Card 3: All-Time Statistics
    let total_sessions = app.stats.total_work_sessions();
    // Calculate total focus minutes
    let total_mins = app.stats.total_focus_minutes();
    // Convert total minutes to hours
    let total_hours = (total_mins as f64) / 60.0;
    // Build paragraph widget for Card 3
    let card3 = Paragraph::new(vec![
        // Line showing total sessions
        Line::from(Span::styled(
            format!("{} Sessions", total_sessions),
            Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
        )),
        // Line showing total hours focused
        Line::from(Span::styled(
            format!("{:.1} Total Focus Hours", total_hours),
            Style::default().fg(theme.muted),
        )),
    ])
    // Attach styled block
    .block(
        Block::default()
            // Title
            .title(Span::styled(" 🏆 All-Time Focus ", Style::default().fg(theme.primary)))
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render Card 3
    f.render_widget(card3, card_chunks[2]);

    // 2. Weekly Bar Chart
    let dist = app.stats.last_days_distribution(7);
    // Map distribution to Ratatui Bar objects
    let bars: Vec<Bar> = dist
        .iter()
        // Convert tuple into Bar
        .map(|(day_label, count)| {
            // Build Bar
            Bar::default()
                // Day label
                .label(Line::from(day_label.as_str()))
                // Pomodoro count value
                .value(*count)
                // Bar color
                .style(Style::default().fg(theme.work))
                // Value text style
                .value_style(Style::default().fg(theme.fg).add_modifier(Modifier::BOLD))
        })
        // Collect into vector
        .collect();

    // Group bars into BarGroup
    let bar_group = BarGroup::default().bars(&bars);
    // Calculate maximum value for chart scaling
    let max_val = dist.iter().map(|(_, c)| *c).max().unwrap_or(5).max(5);

    // Build BarChart widget
    let chart = BarChart::default()
        // Attach styled block
        .block(
            Block::default()
                // Chart title
                .title(Span::styled(" 📊 Daily Activity (Past 7 Days) ", Style::default().fg(theme.primary)))
                // All borders
                .borders(Borders::ALL)
                // Rounded corners
                .border_type(BorderType::Rounded)
                // Border color
                .border_style(Style::default().fg(theme.border)),
        )
        // Set chart data group
        .data(bar_group)
        // Width of each bar
        .bar_width(6)
        // Gap between adjacent bars
        .bar_gap(2)
        // Y-axis maximum scale
        .max(max_val);
    // Render bar chart into middle chunk
    f.render_widget(chart, chunks[1]);

    // 3. Recent Sessions History Table
    let header_cells = ["Time", "Phase", "Duration", "Task Assigned"]
        .iter()
        // Map header cells to styled text
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
    // Build table header row
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Get up to 6 most recent completed sessions in reverse chronological order
    let recent: Vec<&crate::stats::CompletedSession> = app.stats.sessions.iter().rev().take(6).collect();
    // Map sessions to table rows
    let rows: Vec<Row> = recent
        .iter()
        // Convert session struct to styled table Row
        .map(|s| {
            // Format local timestamp string
            let time_str = s.timestamp.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string();
            // Format phase string with emoji
            let phase_str = format!("{} {}", s.phase.emoji(), s.phase.title());
            // Format duration string
            let dur_str = format!("{} mins", s.duration_mins);
            // Format task name or "-" if no task was assigned
            let task_str = s.task_title.clone().unwrap_or_else(|| "-".to_string());

            // Choose phase accent color
            let phase_style = match s.phase {
                // Work color
                crate::timer::PomodoroPhase::Work => Style::default().fg(theme.work),
                // Short break color
                crate::timer::PomodoroPhase::ShortBreak => Style::default().fg(theme.short_break),
                // Long break color
                crate::timer::PomodoroPhase::LongBreak => Style::default().fg(theme.long_break),
            };

            // Build Row with 4 cells
            Row::new(vec![
                // Timestamp cell
                Cell::from(time_str).style(Style::default().fg(theme.muted)),
                // Phase cell
                Cell::from(phase_str).style(phase_style),
                // Duration cell
                Cell::from(dur_str).style(Style::default().fg(theme.fg)),
                // Task cell
                Cell::from(task_str).style(Style::default().fg(theme.fg)),
            ])
        })
        // Collect into row vector
        .collect();

    // Construct the Table widget
    let table = Table::new(
        rows,
        // Define column widths
        [
            // Time column width
            Constraint::Length(18),
            // Phase column width
            Constraint::Length(20),
            // Duration column width
            Constraint::Length(12),
            // Task title column percentage
            Constraint::Percentage(45),
        ],
    )
    // Attach header row
    .header(header)
    // Attach styled block
    .block(
        Block::default()
            // Panel title
            .title(Span::styled(" 🕒 Recent Completed Sessions ", Style::default().fg(theme.primary)))
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render table into bottom chunk
    f.render_widget(table, chunks[2]);
}
