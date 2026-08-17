// Import ratatui layout, style, and widget modules
use ratatui::{
    // Layout constraints and rectangular bounding boxes
    layout::{Constraint, Direction, Layout, Rect},
    // Styling attributes
    style::{Modifier, Style},
    // Text lines and spans
    text::{Line, Span},
    // Widgets for Block, BorderType, Borders, Clear, and Paragraph
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    // Current frame handle
    Frame,
};

// Import App state
use crate::app::App;
// Import Theme structure for styling
use crate::theme::Theme;
// Import centered_rect helper from help_popup module
use crate::ui::help_popup::centered_rect;

// Renders the Add Task interactive modal popup
pub fn render(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Calculate centered rectangular area (60% width, 35% height)
    let popup_area = centered_rect(60, 35, area);
    // Clear background beneath popup area
    f.render_widget(Clear, popup_area);

    // Split popup vertically into Title Input, Estimated Pomodoros Input, and Buttons/Help
    let chunks = Layout::default()
        // Top-to-bottom layout
        .direction(Direction::Vertical)
        // Heights for input fields and buttons
        .constraints([
            // Task Title text input field
            Constraint::Length(3),
            // Estimated Pomodoro count input field
            Constraint::Length(3),
            // Bottom control instructions
            Constraint::Length(3),
        ])
        // Margin inside the popup
        .margin(1)
        // Split popup area
        .split(popup_area);

    // Render outer modal frame
    let modal_block = Block::default()
        // Modal title
        .title(Span::styled(" 📝 Add New Task ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)))
        // All borders
        .borders(Borders::ALL)
        // Rounded corners
        .border_type(BorderType::Rounded)
        // Active border color
        .border_style(Style::default().fg(theme.border_active))
        // Background fill
        .style(Style::default().bg(theme.bg));
    // Render outer modal block
    f.render_widget(modal_block, popup_area);

    // 1. Task Title Input Field
    let title_border_color = if app.task_modal_focus == 0 {
        // Active focus border color
        theme.primary
    } else {
        // Inactive border color
        theme.border
    };

    // Build title display text with cursor
    let title_text = if app.task_input_title.is_empty() && app.task_modal_focus != 0 {
        // Placeholder text
        Span::styled("Enter task description...", Style::default().fg(theme.muted).add_modifier(Modifier::ITALIC))
    } else if app.task_modal_focus == 0 {
        // Show entered text with blinking cursor block
        Span::styled(format!("{}█", app.task_input_title), Style::default().fg(theme.fg))
    } else {
        // Show entered text without cursor
        Span::styled(&app.task_input_title, Style::default().fg(theme.fg))
    };

    // Build title input paragraph widget
    let title_widget = Paragraph::new(Line::from(title_text)).block(
        Block::default()
            // Title field label
            .title(Span::styled(" Task Description ", Style::default().fg(title_border_color)))
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Dynamic border style
            .border_style(Style::default().fg(title_border_color)),
    );
    // Render title field into top chunk
    f.render_widget(title_widget, chunks[0]);

    // 2. Estimated Pomodoros Input Field
    let est_border_color = if app.task_modal_focus == 1 {
        // Active focus border color
        theme.primary
    } else {
        // Inactive border color
        theme.border
    };

    // Estimated pomodoros display
    let est_text = format!("🍅 {} pomodoros (Use ← / → or +/- to adjust)", app.task_input_estimated);
    // Build estimated pomodoros paragraph widget
    let est_widget = Paragraph::new(Line::from(Span::styled(est_text, Style::default().fg(theme.fg)))).block(
        Block::default()
            // Estimated field label
            .title(Span::styled(" Estimated Focus Sessions ", Style::default().fg(est_border_color)))
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Dynamic border style
            .border_style(Style::default().fg(est_border_color)),
    );
    // Render estimated field into middle chunk
    f.render_widget(est_widget, chunks[1]);

    // 3. Bottom Button Controls
    let help_line = Line::from(vec![
        // Tab key
        Span::styled("[Tab] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Next Field  ", Style::default().fg(theme.fg)),
        // Enter key
        Span::styled("[Enter] ", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
        Span::styled("Save Task  ", Style::default().fg(theme.fg)),
        // Esc key
        Span::styled("[Esc] ", Style::default().fg(theme.work).add_modifier(Modifier::BOLD)),
        Span::styled("Cancel", Style::default().fg(theme.fg)),
    ]);
    // Build help paragraph widget
    let help_widget = Paragraph::new(help_line);
    // Render help into bottom chunk
    f.render_widget(help_widget, chunks[2]);
}
