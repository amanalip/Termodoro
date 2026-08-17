// Import layout and widget primitives from ratatui
use ratatui::{
    // Layout constraints and rectangular bounding boxes
    layout::{Constraint, Direction, Layout, Rect},
    // Styling attributes
    style::{Modifier, Style},
    // Text lines and spans
    text::{Line, Span},
    // Widgets for Block, BorderType, Borders, Cell, Paragraph, Row, and Table
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    // Current frame handle
    Frame,
};

// Import App state
use crate::app::App;
// Import Theme structure for consistent palette styling
use crate::theme::Theme;

// Renders the Preferences & Settings tab UI
pub fn render(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Split vertical space into 2 panels: Settings Table and Help Bar
    let chunks = Layout::default()
        // Top-to-bottom layout
        .direction(Direction::Vertical)
        // Heights for each panel
        .constraints([
            // Main settings table taking available space
            Constraint::Min(12),
            // Bottom keybinding control instructions
            Constraint::Length(3),
        ])
        // Outer margin
        .margin(1)
        // Split layout
        .split(area);

    // List of configurable settings items with their current values and descriptions
    let setting_items = [
        // Work focus duration in minutes
        ("Focus Duration", format!("{} mins", app.config.work_duration_mins), "Length of a standard work pomodoro (1 - 120 mins)"),
        // Short break duration in minutes
        ("Short Break", format!("{} mins", app.config.short_break_mins), "Duration of short breaks between sessions (1 - 60 mins)"),
        // Long break duration in minutes
        ("Long Break", format!("{} mins", app.config.long_break_mins), "Duration of long break after completing a full cycle (1 - 90 mins)"),
        // Number of focus sessions before long break
        ("Long Break Interval", format!("{} sessions", app.config.long_break_interval), "Number of focus sessions before a long break (1 - 12)"),
        // Auto-start breaks toggle
        ("Auto-start Breaks", if app.config.auto_start_breaks { "Enabled" } else { "Disabled" }.to_string(), "Automatically start countdown when entering a break"),
        // Auto-start work sessions toggle
        ("Auto-start Work", if app.config.auto_start_work { "Enabled" } else { "Disabled" }.to_string(), "Automatically start countdown after break finishes"),
        // Desktop notification toggle
        ("Desktop Notifications", if app.config.desktop_notifications { "Enabled" } else { "Disabled" }.to_string(), "Send native OS desktop notification on phase completion"),
        // Sound / terminal bell alert toggle
        ("Sound / Bell Alert", if app.config.sound_enabled { "Enabled" } else { "Disabled" }.to_string(), "Ring audio / terminal bell when a session finishes"),
        // Active visual color theme selection
        ("Color Theme", app.config.theme.name().to_string(), "Select your favorite TUI visual color scheme"),
    ];

    // Define table header columns
    let header_cells = ["", "Setting", "Current Value", "Description"]
        .iter()
        // Map header labels to styled cells
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
    // Build table header row
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Map setting items to styled rows
    let rows: Vec<Row> = setting_items
        .iter()
        // Enumerate with index to identify selected row
        .enumerate()
        // Convert to Row
        .map(|(idx, (name, val, desc))| {
            // Check if current row is highlighted
            let is_selected = idx == app.settings_index;
            // Pointer arrow for selected row
            let pointer = if is_selected { "▶" } else { " " };
            // Style for value cell
            let val_style = if is_selected {
                // Highlighted value
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
            } else {
                // Normal value
                Style::default().fg(theme.fg)
            };

            // Build Row with 4 columns
            let row = Row::new(vec![
                // Pointer cell
                Cell::from(pointer).style(Style::default().fg(theme.primary)),
                // Setting name cell
                Cell::from(*name).style(if is_selected {
                    Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.fg)
                }),
                // Setting value cell
                Cell::from(val.clone()).style(val_style),
                // Description cell
                Cell::from(*desc).style(Style::default().fg(theme.muted)),
            ]);

            // Apply highlight background if row is selected
            if is_selected {
                // Highlight background
                row.style(Style::default().bg(theme.highlight))
            } else {
                // Default background
                row
            }
        })
        // Collect into vector
        .collect();

    // Construct the Table widget
    let table = Table::new(
        rows,
        // Define column widths
        [
            // Pointer column width
            Constraint::Length(2),
            // Setting name width
            Constraint::Length(25),
            // Value width
            Constraint::Length(20),
            // Description percentage
            Constraint::Percentage(50),
        ],
    )
    // Attach header row
    .header(header)
    // Attach styled block
    .block(
        Block::default()
            // Panel title
            .title(Span::styled(" ⚙️ Preferences & Settings ", Style::default().fg(theme.primary)))
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render settings table into top chunk
    f.render_widget(table, chunks[0]);

    // Bottom navigation helper
    let helper = Paragraph::new(Line::from(vec![
        // Up/down keys
        Span::styled(" [↑/↓] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Select Setting  ", Style::default().fg(theme.fg)),
        // Left/Right or +/- keys
        Span::styled("[← / →] or [+/-] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Adjust Value / Change Theme  ", Style::default().fg(theme.fg)),
        // Space / Enter keys
        Span::styled("[Space/Enter] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Toggle", Style::default().fg(theme.fg)),
    ]))
    // Attach styled block
    .block(
        Block::default()
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render helper into bottom chunk
    f.render_widget(helper, chunks[1]);
}
