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
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
    // Calculate centered rectangular area (60% width, 70% height)
    let popup_area = centered_rect(65, 75, area);
    // Clear underlying background beneath popup area to avoid visual bleeding
    f.render_widget(Clear, popup_area);

    // Define table of keybindings and descriptions
    let help_lines = vec![
        // Section: Navigation
        Line::from(Span::styled("Navigation & Tabs", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
        Line::from("  [Tab] / [Shift+Tab]   Cycle between tabs (Timer, Tasks, Stats, Settings)"),
        Line::from("  [1] - [4]             Jump directly to tab (1: Timer, 2: Tasks, 3: Stats, 4: Settings)"),
        Line::from("  [?]                   Toggle this Help dialog"),
        Line::from("  [q] / [Esc]           Close popup / Quit application"),
        Line::from(""),
        // Section: Pomodoro Timer Controls
        Line::from(Span::styled("Timer Controls (Tab 1)", Style::default().fg(theme.work).add_modifier(Modifier::BOLD))),
        Line::from("  [Space]               Start / Pause timer countdown"),
        Line::from("  [r]                   Reset timer to beginning of current phase"),
        Line::from("  [s]                   Skip current phase and advance to next"),
        Line::from("  [a]                   Quickly add a new task"),
        Line::from(""),
        // Section: Task Management Controls
        Line::from(Span::styled("Task Management (Tab 2)", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD))),
        Line::from("  [a]                   Create a new task"),
        Line::from("  [Space] / [Enter]     Toggle task completion status"),
        Line::from("  [t]                   Set selected task as active target for timer"),
        Line::from("  [d] / [x]             Delete selected task"),
        Line::from("  [↑ / k], [↓ / j]      Navigate tasks list"),
        Line::from("  [1] / [2] / [3]       Filter tasks (All, Active, Completed)"),
        Line::from(""),
        // Section: Settings & Preferences Controls
        Line::from(Span::styled("Settings Controls (Tab 4)", Style::default().fg(theme.warning).add_modifier(Modifier::BOLD))),
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
