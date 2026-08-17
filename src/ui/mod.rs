// Export digits module for big number rendering
pub mod digits;
// Export help popup module
pub mod help_popup;
// Export settings view module
pub mod settings_view;
// Export stats view module
pub mod stats_view;
// Export task creation modal module
pub mod task_modal;
// Export tasks list view module
pub mod tasks_view;
// Export timer view module
pub mod timer_view;

// Import ratatui layout and styling types
use ratatui::{
    // Layout constraints and rectangular bounding boxes
    layout::{Constraint, Direction, Layout, Rect},
    // Styling attributes for colors and text formatting
    style::{Modifier, Style},
    // Rich text Line and Span types
    text::{Line, Span},
    // Block, BorderType, Borders, Paragraph, and Tabs widgets
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    // Current frame handle
    Frame,
};

// Import App state and ActiveTab enum
use crate::app::{ActiveTab, App};
// Import Theme structure
use crate::theme::Theme;

// Root render function coordinating header tabs, view routing, footer status, and active modals
pub fn render(f: &mut Frame, app: &App) {
    // Resolve active theme palette based on configuration
    let theme = Theme::from_choice(app.config.theme);
    // Get full terminal window area
    let size = f.area();

    // Fill entire terminal background with theme background color
    let background_block = Block::default().style(Style::default().bg(theme.bg));
    // Render base background
    f.render_widget(background_block, size);

    // Split vertical window into Header (Tabs), Main Content, and Footer (Status Bar)
    let chunks = Layout::default()
        // Top-to-bottom layout
        .direction(Direction::Vertical)
        // Heights for header, body, and footer
        .constraints([
            // Header bar containing App title and navigation tabs
            Constraint::Length(3),
            // Body area for the currently active tab view
            Constraint::Min(10),
            // Footer status bar for notifications and global hints
            Constraint::Length(3),
        ])
        // Compute split layout chunks
        .split(size);

    // 1. Header with Tabs
    render_header(f, app, &theme, chunks[0]);

    // 2. Body: Route to active tab view
    match app.active_tab {
        // Tab 1: Pomodoro Countdown Timer
        ActiveTab::Timer => timer_view::render(f, app, &theme, chunks[1]),
        // Tab 2: Task Management & Todo List
        ActiveTab::Tasks => tasks_view::render(f, app, &theme, chunks[1]),
        // Tab 3: Productivity Analytics & Streaks
        ActiveTab::Stats => stats_view::render(f, app, &theme, chunks[1]),
        // Tab 4: Preferences & Color Themes
        ActiveTab::Settings => settings_view::render(f, app, &theme, chunks[1]),
    }

    // 3. Footer: Global Key Hints & Status Message
    render_footer(f, app, &theme, chunks[2]);

    // 4. Overlays: Render active modal dialogs if open
    if app.show_help {
        // Render Help modal on top of screen
        help_popup::render(f, &theme, size);
    } else if app.show_task_modal {
        // Render Add Task modal on top of screen
        task_modal::render(f, app, &theme, size);
    }
}

// Renders header bar featuring application branding and tab selector
fn render_header(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Split header horizontally into Title Logo and Tab Navigation
    let header_chunks = Layout::default()
        // Left-to-right arrangement
        .direction(Direction::Horizontal)
        // Column constraints
        .constraints([
            // App branding column
            Constraint::Length(18),
            // Tabs navigation column
            Constraint::Min(20),
        ])
        // Split header area
        .split(area);

    // App title logo widget
    let title_line = Line::from(vec![
        // Tomato emoji
        Span::styled(" 🍅 ", Style::default()),
        // App name in bold primary color
        Span::styled("Termodoro", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
    ]);
    // Build title paragraph
    let title_widget = Paragraph::new(title_line).block(
        Block::default()
            // Borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render title widget
    f.render_widget(title_widget, header_chunks[0]);

    // Tab titles
    let tab_titles = vec![
        // Tab 1: Timer
        Line::from(vec![Span::styled("[1] ", Style::default().fg(theme.primary)), Span::raw("Timer")]),
        // Tab 2: Tasks
        Line::from(vec![Span::styled("[2] ", Style::default().fg(theme.primary)), Span::raw("Tasks")]),
        // Tab 3: Stats
        Line::from(vec![Span::styled("[3] ", Style::default().fg(theme.primary)), Span::raw("Stats")]),
        // Tab 4: Settings
        Line::from(vec![Span::styled("[4] ", Style::default().fg(theme.primary)), Span::raw("Settings")]),
    ];

    // Determine numerical index of active tab
    let selected_tab_idx = match app.active_tab {
        ActiveTab::Timer => 0,
        ActiveTab::Tasks => 1,
        ActiveTab::Stats => 2,
        ActiveTab::Settings => 3,
    };

    // Construct Tabs widget
    let tabs = Tabs::new(tab_titles)
        // Attach styled block
        .block(
            Block::default()
                // All borders
                .borders(Borders::ALL)
                // Rounded corners
                .border_type(BorderType::Rounded)
                // Border color
                .border_style(Style::default().fg(theme.border)),
        )
        // Highlight active tab with primary color, bold, and underline
        .highlight_style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        )
        // Inactive tab text style
        .style(Style::default().fg(theme.muted))
        // Select active tab index
        .select(selected_tab_idx)
        // Tab separator string
        .divider(" │ ");

    // Render tabs widget into navigation chunk
    f.render_widget(tabs, header_chunks[1]);
}

// Renders footer bar displaying temporary notifications or global shortcut hints
fn render_footer(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Construct footer message line
    let footer_line = if let Some(ref msg) = app.status_message {
        // Display active notification message with accent styling
        Line::from(vec![
            Span::styled(" 📢 ", Style::default()),
            Span::styled(msg, Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
        ])
    } else {
        // Display standard global keybinding shortcuts
        Line::from(vec![
            Span::styled(" [Tab] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled("Switch Tab  ", Style::default().fg(theme.fg)),
            Span::styled("[?] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled("Help Modal  ", Style::default().fg(theme.fg)),
            Span::styled("[q] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled("Quit  ", Style::default().fg(theme.fg)),
            Span::styled("│ Theme: ", Style::default().fg(theme.muted)),
            Span::styled(theme.choice.name(), Style::default().fg(theme.secondary)),
        ])
    };

    // Build footer paragraph widget
    let footer_widget = Paragraph::new(footer_line).block(
        Block::default()
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render footer widget
    f.render_widget(footer_widget, area);
}
