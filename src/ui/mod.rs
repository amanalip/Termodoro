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
        Span::styled(
            "Termodoro",
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
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
        Line::from(vec![
            Span::styled("[1] ", Style::default().fg(theme.primary)),
            Span::raw("Timer"),
        ]),
        // Tab 2: Tasks
        Line::from(vec![
            Span::styled("[2] ", Style::default().fg(theme.primary)),
            Span::raw("Tasks"),
        ]),
        // Tab 3: Stats
        Line::from(vec![
            Span::styled("[3] ", Style::default().fg(theme.primary)),
            Span::raw("Stats"),
        ]),
        // Tab 4: Settings
        Line::from(vec![
            Span::styled("[4] ", Style::default().fg(theme.primary)),
            Span::raw("Settings"),
        ]),
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
            Span::styled(
                msg,
                Style::default()
                    .fg(theme.warning)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        // Display standard global keybinding shortcuts
        Line::from(vec![
            Span::styled(
                " [Tab] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Switch Tab  ", Style::default().fg(theme.fg)),
            Span::styled(
                "[?] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Help Modal  ", Style::default().fg(theme.fg)),
            Span::styled(
                "[q] ",
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Storage;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn create_test_app() -> (App, std::path::PathBuf) {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_ui_test_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path.clone());
        let app = App::new_with_storage(storage);
        (app, temp_dir)
    }

    #[test]
    fn test_render_all_tabs_without_panic() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        // Render Timer tab
        app.active_tab = ActiveTab::Timer;
        terminal.draw(|f| render(f, &app)).unwrap();

        // Render Tasks tab with tasks
        app.active_tab = ActiveTab::Tasks;
        app.tasks.add("Design Architecture".to_string(), 3);
        app.tasks.add("Implement Core".to_string(), 2);
        terminal.draw(|f| render(f, &app)).unwrap();

        // Render Stats tab with sessions
        app.active_tab = ActiveTab::Stats;
        app.stats
            .record(crate::timer::PomodoroPhase::Work, 25, None, None);
        terminal.draw(|f| render(f, &app)).unwrap();

        // Render Settings tab
        app.active_tab = ActiveTab::Settings;
        terminal.draw(|f| render(f, &app)).unwrap();

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_modals_and_status_message() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        // Status banner
        app.set_status_message("Phase Completed!".to_string());
        terminal.draw(|f| render(f, &app)).unwrap();

        // Help Modal overlay
        app.show_help = true;
        terminal.draw(|f| render(f, &app)).unwrap();
        app.show_help = false;

        // Add Task Modal overlay
        app.open_task_modal();
        app.task_input_title = "New Task Name".to_string();
        terminal.draw(|f| render(f, &app)).unwrap();

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_all_terminal_dimensions() {
        let (app, temp_dir) = create_test_app();
        let dimensions = [(60, 20), (80, 24), (100, 30), (140, 45), (200, 60)];

        for (width, height) in dimensions {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.draw(|f| render(f, &app)).unwrap();
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_all_color_themes() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        for choice in crate::theme::ThemeChoice::all() {
            app.config.theme = *choice;
            terminal.draw(|f| render(f, &app)).unwrap();
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_all_timer_phases_and_statuses() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let phases = [
            crate::timer::PomodoroPhase::Work,
            crate::timer::PomodoroPhase::ShortBreak,
            crate::timer::PomodoroPhase::LongBreak,
        ];
        let statuses = [
            crate::timer::TimerStatus::Stopped,
            crate::timer::TimerStatus::Running,
            crate::timer::TimerStatus::Paused,
        ];

        for phase in phases {
            for status in statuses {
                app.timer.phase = phase;
                app.timer.status = status;

                // Render without active task
                app.tasks.active_task_id = None;
                terminal.draw(|f| render(f, &app)).unwrap();

                // Render with active task
                app.tasks.add("Design System".to_string(), 4);
                terminal.draw(|f| render(f, &app)).unwrap();
            }
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_all_settings_rows_highlighted() {
        let (mut app, temp_dir) = create_test_app();
        app.active_tab = ActiveTab::Settings;
        let backend = TestBackend::new(90, 25);
        let mut terminal = Terminal::new(backend).unwrap();

        for i in 0..=8 {
            app.settings_index = i;
            terminal.draw(|f| render(f, &app)).unwrap();
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_task_modal_both_focus_states() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        app.open_task_modal();

        // Focus 0 (Title input) with text
        app.task_modal_focus = 0;
        app.task_input_title = "My Task".to_string();
        terminal.draw(|f| render(f, &app)).unwrap();

        // Focus 0 (Title input) empty (cursor)
        app.task_input_title.clear();
        terminal.draw(|f| render(f, &app)).unwrap();

        // Focus 1 (Estimated Pomodoros)
        app.task_modal_focus = 1;
        app.task_input_estimated = 5;
        terminal.draw(|f| render(f, &app)).unwrap();

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_empty_views() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        // Empty tasks tab
        app.active_tab = ActiveTab::Tasks;
        assert_eq!(app.tasks.tasks.len(), 0);
        terminal.draw(|f| render(f, &app)).unwrap();

        // Empty stats tab
        app.active_tab = ActiveTab::Stats;
        assert_eq!(app.stats.sessions.len(), 0);
        terminal.draw(|f| render(f, &app)).unwrap();

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_extreme_small_terminals() {
        let (app, temp_dir) = create_test_app();
        let micro_dimensions = [(40, 15), (50, 15), (35, 12)];

        for (width, height) in micro_dimensions {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.draw(|f| render(f, &app)).unwrap();
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_twenty_four_cycle_dots_timer_view() {
        let (mut app, temp_dir) = create_test_app();
        let backend = TestBackend::new(120, 30);
        let mut terminal = Terminal::new(backend).unwrap();

        app.config.long_break_interval = 24;
        app.active_tab = ActiveTab::Timer;

        // Cycle 1 of 24
        app.timer.current_cycle = 1;
        terminal.draw(|f| render(f, &app)).unwrap();

        // Cycle 12 of 24
        app.timer.current_cycle = 12;
        terminal.draw(|f| render(f, &app)).unwrap();

        // Cycle 24 of 24
        app.timer.current_cycle = 24;
        terminal.draw(|f| render(f, &app)).unwrap();

        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_render_varied_terminal_geometries_stress() {
        let (mut app, temp_dir) = create_test_app();
        app.tasks.add("Task 1".to_string(), 2);
        app.stats
            .record(crate::timer::PomodoroPhase::Work, 25, None, None);

        let geometries = [
            (50, 18),
            (60, 20),
            (70, 22),
            (80, 24),
            (90, 28),
            (100, 30),
            (120, 35),
            (140, 40),
            (160, 45),
            (200, 50),
            (250, 60),
        ];

        for (w, h) in geometries {
            let backend = TestBackend::new(w, h);
            let mut terminal = Terminal::new(backend).unwrap();

            for tab in [
                ActiveTab::Timer,
                ActiveTab::Tasks,
                ActiveTab::Stats,
                ActiveTab::Settings,
            ] {
                app.active_tab = tab;
                terminal.draw(|f| render(f, &app)).unwrap();
            }
        }

        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
