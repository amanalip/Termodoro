// Import ratatui layout types for structuring the tasks view UI
use ratatui::{
    // Layout constraints, direction, and bounding rectangles
    layout::{Constraint, Direction, Layout, Rect},
    // Styling attributes for colors, modifiers, and bold text
    style::{Modifier, Style},
    // Line and Span primitives for rich formatted text
    text::{Line, Span},
    // Table and block widgets
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    // Frame for rendering
    Frame,
};

// Import App state
use crate::app::App;
// Import TaskFilter enum from tasks module
use crate::tasks::TaskFilter;
// Import Theme structure for consistent theming
use crate::theme::Theme;

// Renders the Tasks list and task management interface
pub fn render(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    // Split vertical space into 3 panels: Filter Bar, Table, and Action Bar
    let chunks = Layout::default()
        // Split top-to-bottom
        .direction(Direction::Vertical)
        // Heights for each panel
        .constraints([
            // Filter selector bar
            Constraint::Length(3),
            // Tasks table taking remaining space
            Constraint::Min(5),
            // Bottom keybinding shortcut hints
            Constraint::Length(3),
        ])
        // Margin around edge
        .margin(1)
        // Compute chunks
        .split(area);

    // 1. Filter Bar Header
    let all_style = if app.tasks.filter == TaskFilter::All {
        // Highlight active filter with primary theme color
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        // Muted color for inactive filters
        Style::default().fg(theme.muted)
    };
    let act_style = if app.tasks.filter == TaskFilter::Active {
        // Highlight active filter
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        // Muted color
        Style::default().fg(theme.muted)
    };
    let comp_style = if app.tasks.filter == TaskFilter::Completed {
        // Highlight completed filter
        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
    } else {
        // Muted color
        Style::default().fg(theme.muted)
    };

    // Calculate task counts
    let total_tasks = app.tasks.tasks.len();
    // Count of completed tasks
    let completed_tasks = app.tasks.tasks.iter().filter(|t| t.completed).count();
    // Count of active uncompleted tasks
    let pending_tasks = total_tasks - completed_tasks;

    // Assemble styled filter bar line
    let filter_line = Line::from(vec![
        // Filter label
        Span::styled(" Filter: ", Style::default().fg(theme.fg)),
        // All filter button
        Span::styled("[1] All ", all_style),
        // All count
        Span::styled(format!("({}) ", total_tasks), Style::default().fg(theme.muted)),
        // Active filter button
        Span::styled(" [2] Active ", act_style),
        // Active count
        Span::styled(format!("({}) ", pending_tasks), Style::default().fg(theme.muted)),
        // Completed filter button
        Span::styled(" [3] Completed ", comp_style),
        // Completed count
        Span::styled(format!("({})", completed_tasks), Style::default().fg(theme.muted)),
    ]);
    // Create filter paragraph widget
    let filter_widget = Paragraph::new(filter_line).block(
        Block::default()
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border theme color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render filter widget into top chunk
    f.render_widget(filter_widget, chunks[0]);

    // 2. Tasks Table
    let indices = app.tasks.filtered_indices();

    // Define table header columns
    let header_cells = ["", "Status", "Task Title", "Est. Pomodoros", "Active Target"]
        .iter()
        // Map header column strings to styled cells
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
    // Build table header row
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Map filtered tasks to styled table rows
    let rows: Vec<Row> = indices
        .iter()
        // Enumerate over visible display index
        .enumerate()
        // Construct Row for each task
        .map(|(disp_idx, &real_idx)| {
            // Get reference to task struct
            let task = &app.tasks.tasks[real_idx];
            // Check if this row is currently selected by cursor
            let is_selected = disp_idx == app.tasks.selected_index;
            // Check if this task is assigned as active timer target
            let is_active = app.tasks.active_task_id.as_deref() == Some(&task.id);

            // Choose checkmark glyph based on completion
            let check_mark = if task.completed { " ✔ " } else { " ○ " };
            // Style checkmark glyph
            let check_style = if task.completed {
                // Green for completed
                Style::default().fg(theme.success)
            } else {
                // Muted for pending
                Style::default().fg(theme.muted)
            };

            // Style task title text
            let title_style = if task.completed {
                // Strike-through and muted for finished tasks
                Style::default().fg(theme.muted).add_modifier(Modifier::CROSSED_OUT)
            } else if is_selected {
                // Bold text for selected row
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)
            } else {
                // Normal foreground color
                Style::default().fg(theme.fg)
            };

            // Format pomodoros counter string
            let pomodoros_str = if task.pomodoros_estimated > 0 {
                // Show spent and estimated
                format!("🍅 {} / {}", task.pomodoros_spent, task.pomodoros_estimated)
            } else {
                // Show spent only
                format!("🍅 {}", task.pomodoros_spent)
            };

            // Active target indicator badge
            let active_label = if is_active { "🎯 ACTIVE" } else { "" };
            // Style active target badge
            let active_style = Style::default().fg(theme.work).add_modifier(Modifier::BOLD);

            // Selection arrow pointer
            let pointer = if is_selected { "▶" } else { " " };

            // Assemble row cells
            let row = Row::new(vec![
                // Selection pointer cell
                Cell::from(pointer).style(Style::default().fg(theme.primary)),
                // Checkmark status cell
                Cell::from(check_mark).style(check_style),
                // Task title cell
                Cell::from(task.title.clone()).style(title_style),
                // Pomodoro counter cell
                Cell::from(pomodoros_str).style(Style::default().fg(theme.secondary)),
                // Active badge cell
                Cell::from(active_label).style(active_style),
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
        // Collect into vector of rows
        .collect();

    // Construct the Table widget
    let table = Table::new(
        rows,
        // Define column widths
        [
            // Pointer column width
            Constraint::Length(2),
            // Status column width
            Constraint::Length(8),
            // Title column percentage
            Constraint::Percentage(55),
            // Pomodoro counter width
            Constraint::Length(18),
            // Active badge width
            Constraint::Length(15),
        ],
    )
    // Attach header row
    .header(header)
    // Attach rounded styled border block
    .block(
        Block::default()
            // Panel title
            .title(Span::styled(" 📋 Tasks ", Style::default().fg(theme.primary)))
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border theme color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render table into middle chunk
    f.render_widget(table, chunks[1]);

    // 3. Bottom Action Hints
    let action_spans = vec![
        Span::styled(" [a] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Add Task  ", Style::default().fg(theme.fg)),
        Span::styled("[Space] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Toggle Done  ", Style::default().fg(theme.fg)),
        Span::styled("[t] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Set Target  ", Style::default().fg(theme.fg)),
        Span::styled("[d] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Delete  ", Style::default().fg(theme.fg)),
        Span::styled("[↑/↓] ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate", Style::default().fg(theme.fg)),
    ];
    // Build action helper paragraph
    let action_widget = Paragraph::new(Line::from(action_spans)).block(
        Block::default()
            // All borders
            .borders(Borders::ALL)
            // Rounded corners
            .border_type(BorderType::Rounded)
            // Border theme color
            .border_style(Style::default().fg(theme.border)),
    );
    // Render action helper into bottom chunk
    f.render_widget(action_widget, chunks[2]);
}
