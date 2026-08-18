// Import standard file system, process, and path utilities
use std::fs;
use std::path::Path;
use std::process::Command;

// Import Ratatui and Crossterm Backend types
use ratatui::backend::TestBackend;
use ratatui::style::Modifier;
use ratatui::Terminal;

// Import core application components from termodoro library
use termodoro::app::{ActiveTab, App};
use termodoro::storage::Storage;
use termodoro::theme::ThemeChoice;
use termodoro::timer::PomodoroPhase;
use termodoro::ui;

// Renders the Ratatui cell buffer into a modern SVG terminal window mockup
fn render_buffer_to_svg(
    buffer: &ratatui::buffer::Buffer,
    title: &str,
    theme_choice: ThemeChoice,
) -> String {
    let theme = termodoro::theme::Theme::from_choice(theme_choice);
    let (bg_r, bg_g, bg_b) = match theme.bg {
        ratatui::style::Color::Rgb(r, g, b) => (r, g, b),
        _ => (30, 30, 46),
    };
    let (fg_r, fg_g, fg_b) = match theme.fg {
        ratatui::style::Color::Rgb(r, g, b) => (r, g, b),
        _ => (205, 214, 244),
    };

    let cols = buffer.area.width as usize;
    let rows = buffer.area.height as usize;

    let char_w = 9.5;
    let char_h = 20.0;
    let padding_x = 24.0;
    let padding_top = 48.0;
    let padding_bottom = 24.0;

    let term_w = (cols as f64 * char_w) + (padding_x * 2.0);
    let term_h = (rows as f64 * char_h) + padding_top + padding_bottom;

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {:.1} {:.1}\" width=\"{:.1}\" height=\"{:.1}\">\n",
        term_w, term_h, term_w, term_h
    ));
    svg.push_str("<defs>\n");
    svg.push_str("  <filter id=\"shadow\" x=\"-5%\" y=\"-5%\" width=\"110%\" height=\"110%\">\n");
    svg.push_str("    <feDropShadow dx=\"0\" dy=\"16\" stdDeviation=\"24\" flood-color=\"#000000\" flood-opacity=\"0.5\"/>\n");
    svg.push_str("  </filter>\n");
    svg.push_str("  <style>\n");
    svg.push_str("    .terminal-text {\n");
    svg.push_str("      font-family: 'JetBrains Mono', 'Fira Code', 'DejaVu Sans Mono', 'Cascadia Code', Menlo, Monaco, monospace;\n");
    svg.push_str("      font-size: 14px;\n");
    svg.push_str(&format!("      line-height: {:.1}px;\n", char_h));
    svg.push_str("      dominant-baseline: hanging;\n");
    svg.push_str("      white-space: pre;\n");
    svg.push_str("    }\n");
    svg.push_str("    .window-title {\n");
    svg.push_str("      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;\n");
    svg.push_str("      font-size: 13px;\n");
    svg.push_str("      font-weight: 500;\n");
    svg.push_str("      fill: #a6adc8;\n");
    svg.push_str("      text-anchor: middle;\n");
    svg.push_str("    }\n");
    svg.push_str("  </style>\n");
    svg.push_str("</defs>\n\n");

    // Outer shadow container
    svg.push_str("<g filter=\"url(#shadow)\">\n");
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"{:.1}\" rx=\"14\" ry=\"14\" fill=\"rgb({},{},{})\" stroke=\"rgba(255,255,255,0.12)\" stroke-width=\"1\"/>\n",
        term_w, term_h, bg_r, bg_g, bg_b
    ));
    svg.push_str("</g>\n\n");

    // Window title bar
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{:.1}\" height=\"40\" rx=\"14\" ry=\"14\" fill=\"rgba(0,0,0,0.25)\"/>\n",
        term_w
    ));
    svg.push_str(&format!(
        "  <line x1=\"0\" y1=\"40\" x2=\"{:.1}\" y2=\"40\" stroke=\"rgba(255,255,255,0.06)\" stroke-width=\"1\"/>\n",
        term_w
    ));

    // Traffic light buttons
    svg.push_str("  <circle cx=\"24\" cy=\"20\" r=\"6\" fill=\"#ff5f56\" stroke=\"#e0443e\" stroke-width=\"0.5\"/>\n");
    svg.push_str("  <circle cx=\"44\" cy=\"20\" r=\"6\" fill=\"#ffbd2e\" stroke=\"#dea123\" stroke-width=\"0.5\"/>\n");
    svg.push_str("  <circle cx=\"64\" cy=\"20\" r=\"6\" fill=\"#27c93f\" stroke=\"#1aab29\" stroke-width=\"0.5\"/>\n");

    // Window title
    let escaped_title = title
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");

    svg.push_str(&format!(
        "  <text x=\"{:.1}\" y=\"24\" class=\"window-title\">{}</text>\n\n",
        term_w / 2.0,
        escaped_title
    ));

    // Terminal text group
    svg.push_str("<g class=\"terminal-text\">\n");

    // Render cells
    for y in 0..rows {
        let cell_y = padding_top + (y as f64 * char_h);

        for x in 0..cols {
            let cell = buffer.cell((x as u16, y as u16)).unwrap();
            let cell_x = padding_x + (x as f64 * char_w);

            let (cell_fg_r, cell_fg_g, cell_fg_b) = match cell.fg {
                ratatui::style::Color::Rgb(r, g, b) => (r, g, b),
                ratatui::style::Color::Black => (24, 24, 37),
                ratatui::style::Color::Red => (243, 139, 168),
                ratatui::style::Color::Green => (166, 227, 161),
                ratatui::style::Color::Yellow => (249, 226, 175),
                ratatui::style::Color::Blue => (137, 180, 250),
                ratatui::style::Color::Magenta => (203, 166, 247),
                ratatui::style::Color::Cyan => (148, 226, 213),
                ratatui::style::Color::Gray => (186, 194, 222),
                ratatui::style::Color::DarkGray => (88, 91, 112),
                ratatui::style::Color::LightRed => (235, 111, 146),
                ratatui::style::Color::LightGreen => (156, 207, 216),
                ratatui::style::Color::LightYellow => (246, 193, 119),
                ratatui::style::Color::LightBlue => (180, 190, 254),
                ratatui::style::Color::LightMagenta => (196, 167, 231),
                ratatui::style::Color::LightCyan => (156, 207, 216),
                ratatui::style::Color::White => (205, 214, 244),
                ratatui::style::Color::Reset => (fg_r, fg_g, fg_b),
                _ => (fg_r, fg_g, fg_b),
            };

            let (cell_bg_r, cell_bg_g, cell_bg_b) = match cell.bg {
                ratatui::style::Color::Rgb(r, g, b) => (Some(r), Some(g), Some(b)),
                ratatui::style::Color::Reset => (None, None, None),
                _ => (None, None, None),
            };

            // Draw custom cell background if specified
            if let (Some(r), Some(g), Some(b)) = (cell_bg_r, cell_bg_g, cell_bg_b) {
                if (r, g, b) != (bg_r, bg_g, bg_b) {
                    svg.push_str(&format!(
                        "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"rgb({},{},{})\"/>\n",
                        cell_x, cell_y, char_w, char_h, r, g, b
                    ));
                }
            }

            let sym = cell.symbol();
            if !sym.trim().is_empty() {
                let escaped = sym
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;");

                let weight = if cell.modifier.contains(Modifier::BOLD) {
                    " font-weight=\"bold\""
                } else {
                    ""
                };

                let opacity = if cell.modifier.contains(Modifier::DIM) {
                    " opacity=\"0.6\""
                } else {
                    ""
                };

                svg.push_str(&format!(
                    "  <text x=\"{:.1}\" y=\"{:.1}\" fill=\"rgb({},{},{})\"{weight}{opacity}>{sym}</text>\n",
                    cell_x,
                    cell_y + 2.0,
                    cell_fg_r,
                    cell_fg_g,
                    cell_fg_b,
                    weight = weight,
                    opacity = opacity,
                    sym = escaped
                ));
            }
        }
    }

    svg.push_str("</g>\n</svg>");
    svg
}

fn main() {
    println!("🎨 Generating Termodoro High-Res Screenshots...");
    let out_dir = Path::new("assets/screenshots");
    fs::create_dir_all(out_dir).expect("Failed to create assets/screenshots directory");

    let width = 96;
    let height = 28;

    // Helper to create test app with isolated temporary storage
    let make_app = |theme: ThemeChoice| -> (App, std::path::PathBuf) {
        let temp_dir =
            std::env::temp_dir().join(format!("termodoro_screenshot_{}", uuid::Uuid::new_v4()));
        let file_path = temp_dir.join("data.json");
        let storage = Storage::with_path(file_path);
        let mut app = App::new_with_storage(storage);
        app.config.theme = theme;
        app.config.work_duration_mins = 25;
        app.config.short_break_mins = 5;
        app.config.long_break_mins = 15;
        app.config.long_break_interval = 4;
        (app, temp_dir)
    };

    // 1. Timer View (Focus Session Active)
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::CatppuccinMocha);
        app.active_tab = ActiveTab::Timer;
        app.tasks
            .add("⚡ Implement Core TUI State Machine in Rust".to_string(), 4);
        app.tasks.tasks[0].pomodoros_spent = 1;
        app.timer.status = termodoro::timer::TimerStatus::Running;
        app.timer.phase = PomodoroPhase::Work;
        app.timer.time_remaining_secs = 24 * 60 + 18; // 24:18
        app.timer.current_cycle = 2;
        app.timer.completed_pomodoros = 1;
        app.set_status_message("Focus mode active — Stay in the zone! 🔥".to_string());

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Pomodoro Focus Timer",
            app.config.theme,
        );
        let svg_path = out_dir.join("01_timer_view.svg");
        let png_path = out_dir.join("01_timer_view.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 01_timer_view.png");
    }

    // 2. Tasks View
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::TokyoNight);
        app.active_tab = ActiveTab::Tasks;
        app.tasks.add(
            "🦀 Build High-Performance Ratatui TUI Framework".to_string(),
            3,
        );
        app.tasks.tasks[0].pomodoros_spent = 3;
        app.tasks.tasks[0].completed = true;

        app.tasks.add(
            "🎵 Synthesize High-Fidelity 16-Bit Audio Chimes".to_string(),
            2,
        );
        app.tasks.tasks[1].pomodoros_spent = 2;
        app.tasks.tasks[1].completed = true;

        app.tasks.add(
            "🎨 Implement 18 Section A Color Schemes & WCAG Contrast".to_string(),
            4,
        );
        app.tasks.tasks[2].pomodoros_spent = 2;

        app.tasks.add(
            "📊 Design Productivity Heatmap & Streak Analytics".to_string(),
            2,
        );
        app.tasks.tasks[3].pomodoros_spent = 0;

        app.tasks.add(
            "🚀 Write CI/CD GitHub Actions Workflow (137 Tests)".to_string(),
            1,
        );
        app.tasks.tasks[4].pomodoros_spent = 0;

        app.tasks.selected_index = 2;
        app.tasks.set_selected_active();
        app.set_status_message("Target set to: Implement 18 Section A Color Schemes".to_string());

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Task Management & Targets",
            app.config.theme,
        );
        let svg_path = out_dir.join("02_tasks_view.svg");
        let png_path = out_dir.join("02_tasks_view.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 02_tasks_view.png");
    }

    // 3. Stats & Productivity Dashboard
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::Dracula);
        app.active_tab = ActiveTab::Stats;

        // Seed realistic historical sessions across last 7 days
        let today = chrono::Local::now().date_naive();
        for day_offset in 0..7 {
            let session_count = match day_offset {
                0 => 4, // Today
                1 => 6, // Yesterday
                2 => 5,
                3 => 8,
                4 => 6,
                5 => 7,
                6 => 4,
                _ => 3,
            };
            let date = today - chrono::Duration::days(day_offset);
            for s in 0..session_count {
                let dt = date
                    .and_hms_opt(9 + s, 15, 0)
                    .unwrap()
                    .and_local_timezone(chrono::Local)
                    .unwrap()
                    .with_timezone(&chrono::Utc);
                app.stats.sessions.push(termodoro::stats::CompletedSession {
                    timestamp: dt,
                    phase: PomodoroPhase::Work,
                    duration_mins: 25,
                    task_id: Some("sample-task-id".to_string()),
                    task_title: Some("Core Focus".to_string()),
                });
            }
        }

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Productivity Analytics & Streaks",
            app.config.theme,
        );
        let svg_path = out_dir.join("03_stats_view.svg");
        let png_path = out_dir.join("03_stats_view.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 03_stats_view.png");
    }

    // 4. Settings & 18 Color Schemes
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::Nord);
        app.active_tab = ActiveTab::Settings;
        app.settings_index = 8; // Highlight Theme selector
        app.set_status_message("Theme: Nord (18 built-in themes available)".to_string());

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Preferences & Color Themes",
            app.config.theme,
        );
        let svg_path = out_dir.join("04_settings_view.svg");
        let png_path = out_dir.join("04_settings_view.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 04_settings_view.png");
    }

    // 5. Add Task Modal Dialog
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::GruvboxDark);
        app.active_tab = ActiveTab::Tasks;
        app.tasks.add("Sample Task".to_string(), 2);
        app.open_task_modal();
        app.task_input_title = "🚀 Ship Termodoro v0.1.0 to Cargo Crates".to_string();
        app.task_input_estimated = 3;
        app.task_modal_focus = 0;

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Add Task Modal Dialog",
            app.config.theme,
        );
        let svg_path = out_dir.join("05_task_modal.svg");
        let png_path = out_dir.join("05_task_modal.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 05_task_modal.png");
    }

    // 6. Help Shortcuts Modal Dialog
    {
        let (mut app, temp_dir) = make_app(ThemeChoice::MonokaiPro);
        app.active_tab = ActiveTab::Timer;
        app.show_help = true;

        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| ui::render(f, &app)).unwrap();

        let svg = render_buffer_to_svg(
            terminal.backend().buffer(),
            "Termodoro — Keyboard Shortcuts & Navigation",
            app.config.theme,
        );
        let svg_path = out_dir.join("06_help_modal.svg");
        let png_path = out_dir.join("06_help_modal.png");
        fs::write(&svg_path, svg).unwrap();

        let _ = Command::new("rsvg-convert")
            .args([
                "-d",
                "144",
                "-p",
                "144",
                svg_path.to_str().unwrap(),
                "-o",
                png_path.to_str().unwrap(),
            ])
            .output();

        let _ = fs::remove_dir_all(temp_dir);
        println!("  ✓ Saved 06_help_modal.png");
    }

    println!("✨ All 6 high-res screenshots generated in assets/screenshots/");
}
