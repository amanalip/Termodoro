// ==============================================================================
// Termodoro rigorous end-to-end integration suite (permanent).
//
// These tests exercise the PUBLIC crate API exactly the way main.rs drives it:
// real Storage files on disk, real App key dispatch, real sub-second tick
// pacing, and real ratatui TestBackend rendering. Unit tests inside src/
// cover module internals; this file covers the seams BETWEEN modules where
// regressions actually ship: persistence round-trips across restarts,
// schema-compatibility with older data.json files, session accounting when
// settings change mid-flight, task/target lifecycle under filters, streak
// math at calendar boundaries, and UI rendering at hostile geometries.
//
// Run with: cargo test --test e2e_rigorous
// ==============================================================================

use chrono::{Duration as ChronoDuration, Utc};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use termodoro::app::{ActiveTab, App};
use termodoro::config::Config;
use termodoro::stats::{CompletedSession, StatsHistory};
use termodoro::storage::Storage;
use termodoro::tasks::TaskFilter;
use termodoro::theme::ThemeChoice;
use termodoro::timer::{PomodoroPhase, TimerStatus};

// ------------------------------------------------------------------------------
// Helpers
// ------------------------------------------------------------------------------

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::empty(),
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    }
}

// Unique per-test temp directory so parallel tests never share state files.
fn temp_dir(tag: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("termodoro_e2e_{}_{}", tag, uuid::Uuid::new_v4()))
}

// App wired to an isolated storage file; audio muted for the whole process
// (integration tests never unmute) and desktop notifications disabled so no
// OS notification threads are spawned headless.
fn make_app(tag: &str) -> (App, std::path::PathBuf) {
    let dir = temp_dir(tag);
    let storage = Storage::with_path(dir.join("data.json"));
    let mut app = App::new_with_storage(storage);
    app.config.desktop_notifications = false;
    termodoro::audio::set_audio_muted_for_tests(true);
    (app, dir)
}

// Fresh App instance over an EXISTING data file, simulating a relaunch.
fn relaunch(dir: &std::path::Path) -> App {
    let storage = Storage::with_path(dir.join("data.json"));
    let mut app = App::new_with_storage(storage);
    app.config.desktop_notifications = false;
    app
}

// Drives one full second through the same 4x250ms pacing main.rs uses.
fn advance_one_second(app: &mut App) {
    for _ in 0..4 {
        app.on_tick();
    }
}

// Runs the current phase to natural completion via real seconds. The loop is
// anchored to phase+status rather than the countdown: on completion the timer
// auto-advances and refills time_remaining_secs with the NEXT phase's full
// duration, which would otherwise spin this helper forever.
fn complete_phase(app: &mut App) {
    let start_phase = app.timer.phase;
    assert_eq!(
        app.timer.status,
        TimerStatus::Running,
        "phase must be running"
    );
    while app.timer.status == TimerStatus::Running && app.timer.phase == start_phase {
        advance_one_second(app);
    }
    assert_ne!(app.timer.phase, start_phase, "phase never completed");
}

fn cleanup(dir: std::path::PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

// Convenience: start a fresh Work phase running at full configured duration.
fn start_work(app: &mut App) {
    app.timer.phase = PomodoroPhase::Work;
    app.timer.status = TimerStatus::Running;
    app.timer.total_duration_secs = app.config.work_duration_mins * 60;
    app.timer.time_remaining_secs = app.config.work_duration_mins * 60;
}

fn start_break(app: &mut App, phase: PomodoroPhase) {
    app.timer.phase = phase;
    app.timer.status = TimerStatus::Running;
    let total = match phase {
        PomodoroPhase::ShortBreak => app.config.short_break_mins,
        _ => app.config.long_break_mins,
    } * 60;
    app.timer.total_duration_secs = total;
    app.timer.time_remaining_secs = total;
}

// ==============================================================================
// Section 1 — Persistence & schema compatibility
// ==============================================================================

// A data.json written by a build that predates the stats section must load
// with the user's tasks intact instead of being quarantined as "corrupt".
#[test]
fn storage_loads_legacy_file_missing_stats_section_and_keeps_tasks() {
    let dir = temp_dir("legacy_stats");
    let path = dir.join("data.json");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        &path,
        r#"{"config":{"theme":"Nord","work_duration_mins":50},"tasks":{"tasks":[{"id":"legacy-1","title":"Surviving Task","completed":false,"pomodoros_spent":2,"pomodoros_estimated":5,"created_at":"2026-01-01T00:00:00Z"}],"active_task_id":"legacy-1"}}"#,
    )
    .unwrap();

    let app = relaunch(&dir);
    assert_eq!(app.tasks.tasks.len(), 1, "tasks must survive the load");
    assert_eq!(app.tasks.tasks[0].title, "Surviving Task");
    assert_eq!(app.tasks.tasks[0].pomodoros_spent, 2);
    assert_eq!(
        app.tasks.active_task().map(|t| t.title.as_str()),
        Some("Surviving Task")
    );
    assert_eq!(app.stats.sessions.len(), 0);
    assert_eq!(app.config.work_duration_mins, 50);

    // Nothing was corrupt, so no quarantine side file may appear.
    let quarantined = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .any(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("data.json.corrupt-")
        });
    assert!(!quarantined, "valid legacy file must not be quarantined");

    cleanup(dir);
}

#[test]
fn storage_loads_file_missing_tasks_section() {
    let dir = temp_dir("legacy_tasks");
    let path = dir.join("data.json");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        &path,
        r#"{"config":{"work_duration_mins":42},"stats":{"sessions":[]}}"#,
    )
    .unwrap();

    let app = relaunch(&dir);
    assert_eq!(app.config.work_duration_mins, 42);
    assert_eq!(app.tasks.tasks.len(), 0);
    assert_eq!(app.stats.sessions.len(), 0);

    cleanup(dir);
}

#[test]
fn storage_loads_file_missing_config_section() {
    let dir = temp_dir("legacy_config");
    let path = dir.join("data.json");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        &path,
        r#"{"tasks":{"tasks":[],"active_task_id":null},"stats":{"sessions":[]}}"#,
    )
    .unwrap();

    let app = relaunch(&dir);
    // relaunch() silences desktop notifications for hermeticity; everything
    // else must equal factory defaults.
    let expected = Config {
        desktop_notifications: false,
        ..Config::default()
    };
    assert_eq!(app.config, expected);
    assert_eq!(app.timer.time_remaining_secs, 25 * 60);

    cleanup(dir);
}

// The Settings footer displays names like "Catppuccin Frappé" and
// "Synthwave '84". A user copying that exact string into data.json must get
// the same theme back, not a silent fallback to Catppuccin Mocha.
#[test]
fn hand_edited_theme_display_names_survive_reload() {
    for (display, expected) in [
        ("Catppuccin Frappé", ThemeChoice::CatppuccinFrappe),
        ("Synthwave '84", ThemeChoice::Synthwave84),
        ("OLED Phosphor", ThemeChoice::OledPhosphor),
        ("Solarized Dark", ThemeChoice::SolarizedDark),
    ] {
        let dir = temp_dir("theme_display");
        let path = dir.join("data.json");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            &path,
            format!(
                r#"{{"config":{{"theme":"{}"}},"tasks":{{"tasks":[],"active_task_id":null}},"stats":{{"sessions":[]}}}}"#,
                display
            ),
        )
        .unwrap();

        let app = relaunch(&dir);
        assert_eq!(
            app.config.theme, expected,
            "hand-edited display name {:?} must resolve to {:?}",
            display, expected
        );
        cleanup(dir);
    }
}

// Every theme must round-trip through save -> reload byte-for-byte.
#[test]
fn every_theme_survives_save_reload_cycle() {
    for choice in ThemeChoice::all() {
        let (mut app, dir) = make_app("theme_roundtrip");
        app.config.theme = *choice;
        app.save_state();

        let reloaded = relaunch(&dir);
        assert_eq!(
            reloaded.config.theme, *choice,
            "theme {:?} lost on reload",
            choice
        );
        cleanup(dir);
    }
}

#[test]
fn save_reload_roundtrip_preserves_unicode_titles_and_target() {
    let (mut app, dir) = make_app("unicode_roundtrip");
    app.active_tab = ActiveTab::Tasks;
    app.tasks.add("🦀 Refactor 中文 module".to_string(), 7);
    app.tasks.add("العربية task 🚀".to_string(), 3);
    app.tasks.selected_index = 1;
    app.on_key_event(key(KeyCode::Char('t'))); // target the Arabic title
    app.save_state();

    let reloaded = relaunch(&dir);
    assert_eq!(reloaded.tasks.tasks.len(), 2);
    assert_eq!(reloaded.tasks.tasks[0].title, "🦀 Refactor 中文 module");
    assert_eq!(reloaded.tasks.tasks[1].title, "العربية task 🚀");
    assert_eq!(
        reloaded.tasks.active_task().map(|t| t.title.as_str()),
        Some("العربية task 🚀"),
        "active target binding must persist"
    );
    cleanup(dir);
}

#[test]
fn save_reload_roundtrip_preserves_spent_counts_and_stats() {
    let (mut app, dir) = make_app("counts_roundtrip");
    app.tasks.add("Counted Task".to_string(), 10);

    // Two completed focus sessions credited to the active task.
    for _ in 0..2 {
        start_work(&mut app);
        complete_phase(&mut app);
        start_break(&mut app, PomodoroPhase::ShortBreak);
        complete_phase(&mut app);
    }

    app.save_state();
    let reloaded = relaunch(&dir);

    assert_eq!(reloaded.tasks.tasks[0].pomodoros_spent, 2);
    assert_eq!(reloaded.stats.total_work_sessions(), 2);
    assert_eq!(reloaded.stats.total_focus_minutes(), 50); // 2 x default 25m
    assert_eq!(reloaded.stats.sessions.len(), 4); // works + breaks recorded
    cleanup(dir);
}

#[test]
fn repeated_saves_leave_no_tmp_litter_and_stay_parseable() {
    let (mut app, dir) = make_app("tmp_litter");
    for i in 0..25 {
        app.tasks.add(format!("Save Storm {}", i), 1);
        app.save_state();

        // The live file must parse after EVERY save, not just the last.
        let raw = std::fs::read_to_string(dir.join("data.json")).unwrap();
        let parsed: termodoro::storage::AppData = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("save {} produced unparseable file: {}", i, e));
        assert_eq!(parsed.tasks.tasks.len(), i + 1);
    }

    let litter: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n.ends_with(".tmp"))
        .collect();
    assert!(
        litter.is_empty(),
        "atomic staging files leaked: {:?}",
        litter
    );

    cleanup(dir);
}

#[test]
fn corrupt_state_file_is_quarantined_with_bytes_intact() {
    let dir = temp_dir("quarantine_bytes");
    let path = dir.join("data.json");
    std::fs::create_dir_all(&dir).unwrap();
    let garbage = "{{{ definitely not json \u{1F400}";
    std::fs::write(&path, garbage).unwrap();

    let app = relaunch(&dir);
    let expected = Config {
        desktop_notifications: false, // relaunch() hermeticity override
        ..Config::default()
    };
    assert_eq!(app.config, expected, "corrupt input yields defaults");

    let entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let quarantine = entries
        .iter()
        .find(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("data.json.corrupt-")
        })
        .expect("corrupt file must be moved aside, not deleted");
    let preserved = std::fs::read_to_string(quarantine.path()).unwrap();
    assert_eq!(
        preserved, garbage,
        "original bytes must be preserved verbatim"
    );

    cleanup(dir);
}

#[test]
fn large_task_list_survives_restart_roundtrip() {
    let (mut app, dir) = make_app("large_list");
    for i in 0..300 {
        app.tasks.add(format!("Bulk Task {:03}", i), (i % 20) + 1);
    }
    // Complete every third task.
    for idx in (0..300).step_by(3) {
        app.tasks.selected_index = idx;
        app.tasks.toggle_selected();
    }
    app.save_state();

    let reloaded = relaunch(&dir);
    assert_eq!(reloaded.tasks.tasks.len(), 300);
    assert_eq!(
        reloaded.tasks.tasks.iter().filter(|t| t.completed).count(),
        100
    );
    assert_eq!(
        reloaded.tasks.filter,
        TaskFilter::All,
        "transient filter must reset to All on restart"
    );
    assert_eq!(reloaded.tasks.selected_index, 0);
    cleanup(dir);
}

#[test]
fn transient_ui_state_resets_on_restart() {
    let (mut app, dir) = make_app("transient_reset");
    app.tasks.add("One".to_string(), 1);
    app.tasks.add("Two".to_string(), 1);
    app.on_key_event(key_mod(KeyCode::Tab, KeyModifiers::SHIFT)); // navigate somewhere
    app.active_tab = ActiveTab::Stats;
    app.on_key_event(key(KeyCode::Char('3'))); // Stats tab: global digit jump guard
    app.active_tab = ActiveTab::Tasks;
    app.on_key_event(key(KeyCode::Char('3'))); // Completed filter
    app.tasks.selected_index = 1;
    app.set_status_message("Doomed Toast".to_string());
    app.save_state();

    let reloaded = relaunch(&dir);
    assert_eq!(
        reloaded.tasks.filter,
        TaskFilter::All,
        "filter is transient"
    );
    assert_eq!(reloaded.tasks.selected_index, 0, "selection is transient");
    assert_eq!(
        reloaded.active_tab,
        ActiveTab::Timer,
        "always launch on Timer tab"
    );
    assert_eq!(reloaded.status_message, None, "toasts never persist");
    assert!(!reloaded.show_help);
    assert!(!reloaded.show_task_modal);
    cleanup(dir);
}

// ==============================================================================
// Section 2 — Full session workflows
// ==============================================================================

// A realistic workday: two focus/break pairs with a restart in the middle,
// long break due at interval=2, everything driven through keys and ticks.
#[test]
fn full_workday_simulation_with_restart_between_sessions() {
    let (mut app, dir) = make_app("workday");
    app.config.long_break_interval = 2;
    app.tasks.add("Morning Focus".to_string(), 4);

    // Pomodoro 1 -> short break.
    app.on_key_event(key(KeyCode::Char(' '))); // Space starts the timer
    assert_eq!(app.timer.status, TimerStatus::Running);
    complete_phase(&mut app);
    assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
    assert_eq!(app.timer.current_cycle, 2);

    // Restart mid-day (user closes the laptop, comes back later).
    app.save_state();
    let mut app = relaunch(&dir);
    assert_eq!(
        app.timer.phase,
        PomodoroPhase::Work,
        "timer resets on relaunch"
    );
    assert_eq!(app.timer.status, TimerStatus::Stopped);
    assert_eq!(
        app.stats.total_work_sessions(),
        1,
        "completed work survives restart"
    );

    // Finish the break manually. NOTE: the cycle counter is session-local
    // (timer state is deliberately not persisted), so the afternoon starts
    // fresh at cycle 1 even though one pomodoro is already logged today.
    start_break(&mut app, PomodoroPhase::ShortBreak);
    complete_phase(&mut app);
    assert_eq!(app.timer.phase, PomodoroPhase::Work);
    assert_eq!(
        app.timer.current_cycle, 1,
        "cycle restarts with the session"
    );

    // Afternoon pomodoro 1 -> short break (cycle 1 -> 2).
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
    assert_eq!(app.timer.current_cycle, 2);

    // Finish that break, then afternoon pomodoro 2 -> LONG break (interval=2).
    start_break(&mut app, PomodoroPhase::ShortBreak);
    complete_phase(&mut app);
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(
        app.timer.phase,
        PomodoroPhase::LongBreak,
        "interval=2 triggers long break"
    );
    assert_eq!(
        app.timer.current_cycle, 1,
        "cycle counter wraps after long break"
    );
    assert_eq!(
        app.timer.completed_pomodoros, 2,
        "session-local count for this half"
    );

    // Long break completes back into Work.
    start_break(&mut app, PomodoroPhase::LongBreak);
    complete_phase(&mut app);
    assert_eq!(app.timer.phase, PomodoroPhase::Work);

    // Day totals span the restart: 1 morning + 2 afternoon focus sessions.
    assert_eq!(app.stats.total_work_sessions(), 3, "stats span restarts");
    assert_eq!(app.tasks.tasks[0].pomodoros_spent, 3);
    assert_eq!(app.stats.today_focus_minutes(), 75);
    cleanup(dir);
}

// Skipping is an explicit user action: it advances the cycle machinery but
// must NOT fabricate statistics or credit the active task.
#[test]
fn skip_advances_cycle_without_recording_stats_or_credit() {
    let (mut app, dir) = make_app("skip_semantics");
    app.tasks.add("Untouched Task".to_string(), 3);

    // Skip straight out of a barely-started work phase.
    app.on_key_event(key(KeyCode::Char(' ')));
    advance_one_second(&mut app); // 1s of real progress only
    app.on_key_event(key(KeyCode::Char('s'))); // skip
    assert_eq!(app.timer.phase, PomodoroPhase::ShortBreak);
    assert_eq!(app.timer.current_cycle, 2);

    // Skip the break too.
    app.on_key_event(key(KeyCode::Char('s')));
    assert_eq!(app.timer.phase, PomodoroPhase::Work);

    assert_eq!(
        app.stats.sessions.len(),
        0,
        "skips must not create sessions"
    );
    assert_eq!(
        app.tasks.tasks[0].pomodoros_spent, 0,
        "skips must not credit tasks"
    );
    assert_eq!(app.stats.today_focus_minutes(), 0);
    cleanup(dir);
}

#[test]
fn reset_keeps_cycle_position_and_clears_countdown() {
    let (mut app, dir) = make_app("reset_position");
    app.config.long_break_interval = 4;

    // Burn one full pomodoro to reach cycle 2.
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(app.timer.current_cycle, 2);

    // Partially run the next work phase, then reset.
    start_work(&mut app);
    for _ in 0..30 {
        advance_one_second(&mut app);
    }
    assert!(app.timer.time_remaining_secs < 25 * 60);
    app.on_key_event(key(KeyCode::Char('r')));

    assert_eq!(app.timer.status, TimerStatus::Stopped);
    assert_eq!(app.timer.time_remaining_secs, 25 * 60, "countdown restored");
    assert_eq!(
        app.timer.current_cycle, 2,
        "reset must not rewind the cycle"
    );
    // Exactly one recorded session exists: the fully-completed first pomodoro.
    // The partial progress of the aborted second session was discarded.
    assert_eq!(app.stats.sessions.len(), 1);
    assert_eq!(app.stats.total_work_sessions(), 1);
    cleanup(dir);
}

#[test]
fn pause_resume_across_many_toggles_never_drifts() {
    let (mut app, dir) = make_app("pause_drift");
    app.on_key_event(key(KeyCode::Char(' ')));

    let full = app.timer.time_remaining_secs;
    let mut expected = full;

    for _ in 0..10 {
        // Run 7 real seconds.
        for _ in 0..7 {
            advance_one_second(&mut app);
            expected -= 1;
        }
        // Pause: 12 seconds of ticks must change nothing.
        app.on_key_event(key(KeyCode::Char(' ')));
        assert_eq!(app.timer.status, TimerStatus::Paused);
        for _ in 0..12 {
            advance_one_second(&mut app);
        }
        assert_eq!(
            app.timer.time_remaining_secs, expected,
            "paused time leaked"
        );
        // Resume.
        app.on_key_event(key(KeyCode::Char(' ')));
        assert_eq!(app.timer.status, TimerStatus::Running);
    }

    assert_eq!(app.timer.time_remaining_secs, full - 70);
    assert_eq!(app.stats.sessions.len(), 0, "no phase completed");
    cleanup(dir);
}

// Every legal interval value must produce exactly `interval` pomodoros per
// long-break macro-cycle, driven through real App ticks.
#[test]
fn long_break_interval_exhaustive_one_to_twenty_four() {
    for interval in 1u32..=24 {
        let (mut app, dir) = make_app(&format!("interval_{}", interval));
        app.config.long_break_interval = interval;

        for cycle in 1..=interval {
            assert_eq!(app.timer.current_cycle, cycle, "interval {}", interval);
            start_work(&mut app);
            complete_phase(&mut app);

            if cycle < interval {
                assert_eq!(
                    app.timer.phase,
                    PomodoroPhase::ShortBreak,
                    "interval {}",
                    interval
                );
                start_break(&mut app, PomodoroPhase::ShortBreak);
                complete_phase(&mut app);
                assert_eq!(app.timer.phase, PomodoroPhase::Work);
            } else {
                assert_eq!(
                    app.timer.phase,
                    PomodoroPhase::LongBreak,
                    "interval {}",
                    interval
                );
                assert_eq!(app.timer.current_cycle, 1, "interval {}", interval);
            }
        }
        assert_eq!(app.timer.completed_pomodoros, interval);
        assert_eq!(app.stats.total_work_sessions(), interval as usize);
        cleanup(dir);
    }
}

// Direct struct construction can bypass config sanitization (zero durations).
// The timer must degrade gracefully: instant completion, no auto-start of a
// zero-length phase, no u32 underflow, bounded tick loop terminates.
#[test]
fn zero_duration_direct_construction_never_panics_or_underflows() {
    let dir = temp_dir("zero_duration");
    let config = Config {
        work_duration_mins: 0, // bypasses sanitize() on purpose
        short_break_mins: 0,
        long_break_mins: 0,
        ..Config::default()
    };

    let mut timer = termodoro::timer::PomodoroTimer::new(&config);
    assert_eq!(timer.time_remaining_secs, 0);
    timer.status = TimerStatus::Running;

    // A bounded burst of ticks must terminate and leave sane state.
    for _ in 0..100 {
        let _ = timer.tick(&config);
    }
    assert_eq!(timer.time_remaining_secs, 0, "never below zero");
    assert_eq!(
        timer.status,
        TimerStatus::Stopped,
        "zero-length phases never auto-start"
    );
    assert!(timer.progress_ratio() <= 1.0 && timer.progress_ratio() >= 0.0);

    cleanup(dir);
}

#[test]
fn timer_state_is_not_persisted_across_restart() {
    let (mut app, dir) = make_app("timer_not_persisted");
    app.on_key_event(key(KeyCode::Char(' ')));
    for _ in 0..40 {
        advance_one_second(&mut app); // burn 40s
    }
    app.timer.completed_pomodoros = 9;
    app.timer.current_cycle = 3;
    app.save_state();

    let reloaded = relaunch(&dir);
    assert_eq!(reloaded.timer.time_remaining_secs, 25 * 60);
    assert_eq!(reloaded.timer.status, TimerStatus::Stopped);
    assert_eq!(
        reloaded.timer.completed_pomodoros, 0,
        "session-local counters reset"
    );
    assert_eq!(reloaded.timer.current_cycle, 1);
    cleanup(dir);
}

// ==============================================================================
// Section 3 — Session accounting
// ==============================================================================

#[test]
fn work_session_credits_only_the_active_task_across_retargeting() {
    let (mut app, dir) = make_app("credit_retarget");
    app.active_tab = ActiveTab::Tasks;
    app.tasks.add("Task Alpha".to_string(), 5);
    app.tasks.add("Task Beta".to_string(), 5);

    // Session 1 credits Alpha (auto-active as first task).
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);
    assert_eq!(app.tasks.tasks[1].pomodoros_spent, 0);

    // Retarget Beta mid-day, session 2 credits ONLY Beta.
    app.tasks.selected_index = 1;
    app.on_key_event(key(KeyCode::Char('t')));
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(
        app.tasks.tasks[0].pomodoros_spent, 1,
        "Alpha must stay untouched"
    );
    assert_eq!(app.tasks.tasks[1].pomodoros_spent, 1);

    // Stats attribute each session to the right task title.
    let work_sessions: Vec<_> = app
        .stats
        .sessions
        .iter()
        .filter(|s| s.phase == PomodoroPhase::Work)
        .collect();
    assert_eq!(work_sessions.len(), 2);
    assert_eq!(work_sessions[0].task_title.as_deref(), Some("Task Alpha"));
    assert_eq!(work_sessions[1].task_title.as_deref(), Some("Task Beta"));
    cleanup(dir);
}

#[test]
fn deleting_active_task_mid_day_reassigns_credit_to_next() {
    let (mut app, dir) = make_app("delete_reassign");
    app.active_tab = ActiveTab::Tasks;
    app.tasks.add("Doomed".to_string(), 2);
    app.tasks.add("Successor".to_string(), 2);

    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);

    // Delete the ACTIVE task (cursor sits on row 1 after the second add, so
    // select row 0 first); target must fall through to the survivor.
    app.tasks.selected_index = 0;
    app.on_key_event(key(KeyCode::Char('d')));
    assert_eq!(app.tasks.tasks.len(), 1);
    assert_eq!(
        app.tasks.active_task().map(|t| t.title.as_str()),
        Some("Successor")
    );

    // Next session credits the successor, not the ghost of the deleted task.
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(app.tasks.tasks[0].title, "Successor");
    assert_eq!(app.tasks.tasks[0].pomodoros_spent, 1);
    assert_eq!(app.stats.total_work_sessions(), 2);
    cleanup(dir);
}

// Settings allow editing durations while a countdown runs. A session that
// runs to natural completion must log the minutes it ACTUALLY spent, both
// live and after a restart round-trip.
#[test]
fn completed_sessions_record_actual_elapsed_duration_even_when_config_mutated() {
    let (mut app, dir) = make_app("elapsed_truth");
    app.config.work_duration_mins = 25;
    start_work(&mut app);

    // Mid-flight the user cranks Focus Duration to 90.
    app.config.work_duration_mins = 90;
    assert_eq!(
        app.timer.time_remaining_secs,
        25 * 60,
        "running countdown must not be resized mid-flight"
    );

    complete_phase(&mut app);
    assert_eq!(
        app.stats.sessions[0].duration_mins, 25,
        "log actual minutes, not 90"
    );

    app.save_state();
    let reloaded = relaunch(&dir);
    assert_eq!(
        reloaded.stats.sessions[0].duration_mins, 25,
        "truth survives restart"
    );
    assert_eq!(
        reloaded.config.work_duration_mins, 90,
        "user's new preference persists"
    );
    cleanup(dir);
}

// ==============================================================================
// Section 4 — Task lifecycle
// ==============================================================================

// Pressing 't' on a finished task must refuse: finished work must never
// become the focus target and accrue new pomodoros.
#[test]
fn completed_task_cannot_become_active_target_via_keys() {
    let (mut app, dir) = make_app("target_completed");
    app.active_tab = ActiveTab::Tasks;
    app.tasks.add("Finished Thing".to_string(), 2);
    app.tasks.add("Live Thing".to_string(), 2);

    // Complete the first task.
    app.tasks.selected_index = 0;
    app.on_key_event(key(KeyCode::Char(' ')));
    assert!(app.tasks.tasks[0].completed);
    // Completing the active task auto-reassigns the target away from it.
    let active_after_toggle = app.tasks.active_task_id.clone();
    assert_ne!(
        active_after_toggle.as_deref(),
        Some(app.tasks.tasks[0].id.as_str())
    );

    // Now try to force-target the completed task through the Completed filter.
    app.on_key_event(key(KeyCode::Char('3'))); // Completed filter
    app.tasks.selected_index = 0;
    app.on_key_event(key(KeyCode::Char('t')));

    assert_ne!(
        app.tasks.active_task_id.as_deref(),
        Some(app.tasks.tasks[0].id.as_str()),
        "a completed task must never become the active target"
    );

    // And a work session must not credit it even if stale legacy state did.
    app.tasks.active_task_id = Some(app.tasks.tasks[0].id.clone()); // simulate legacy file
    start_work(&mut app);
    complete_phase(&mut app);
    assert_eq!(
        app.tasks.tasks[0].pomodoros_spent, 0,
        "completed tasks must not accrue pomodoros"
    );
    cleanup(dir);
}

#[test]
fn filter_matrix_add_toggle_delete_maintains_integrity() {
    let (mut app, dir) = make_app("filter_matrix");
    app.active_tab = ActiveTab::Tasks;

    for i in 1..=6 {
        app.tasks.add(format!("Matrix {}", i), 1);
    }
    // Complete items 2 and 5 (visible indices 1 and 4 under All).
    app.tasks.selected_index = 1;
    app.on_key_event(key(KeyCode::Char(' ')));
    app.tasks.selected_index = 4;
    app.on_key_event(key(KeyCode::Char(' ')));

    // Active filter shows exactly the 4 incomplete tasks.
    app.on_key_event(key(KeyCode::Char('2')));
    assert_eq!(app.tasks.filtered_indices().len(), 4);
    // Delete the first visible active task.
    app.on_key_event(key(KeyCode::Char('d')));
    assert_eq!(app.tasks.filtered_indices().len(), 3);
    assert_eq!(app.tasks.tasks.len(), 5);

    // Completed filter still shows exactly the 2 done tasks.
    app.on_key_event(key(KeyCode::Char('3')));
    assert_eq!(app.tasks.filtered_indices().len(), 2);
    // Untoggle one from within the Completed view; the view empties by one.
    app.on_key_event(key(KeyCode::Char(' ')));
    assert_eq!(app.tasks.filtered_indices().len(), 1);

    // Back to All: 5 tasks remain, none lost, selection valid.
    app.on_key_event(key(KeyCode::Char('1')));
    assert_eq!(app.tasks.tasks.len(), 5);
    assert!(app.tasks.selected_index < app.tasks.filtered_indices().len());

    // Delete down to empty through the UI; every step stays consistent.
    while !app.tasks.tasks.is_empty() {
        app.on_key_event(key(KeyCode::Char('d')));
    }
    assert_eq!(app.tasks.active_task_id, None);
    app.on_key_event(key(KeyCode::Char('d'))); // deleting from empty is a safe no-op
    cleanup(dir);
}

#[test]
fn selection_wrapping_under_all_filters_never_panics() {
    let (mut app, dir) = make_app("wrap_filters");
    app.active_tab = ActiveTab::Tasks;

    for i in 1..=5 {
        app.tasks.add(format!("Wrap {}", i), 1);
    }
    app.tasks.selected_index = 2;
    app.on_key_event(key(KeyCode::Char(' '))); // complete middle task

    for filter_key in [KeyCode::Char('1'), KeyCode::Char('2'), KeyCode::Char('3')] {
        app.on_key_event(key(filter_key));
        let count = app.tasks.filtered_indices().len();
        if count == 0 {
            continue;
        }
        // Hammer navigation in both directions across the wrap boundary.
        for _ in 0..(count * 3) {
            app.on_key_event(key(KeyCode::Char('j')));
            assert!(app.tasks.selected_index < count);
        }
        for _ in 0..(count * 3) {
            app.on_key_event(key(KeyCode::Char('k')));
            assert!(app.tasks.selected_index < count);
        }
        // Actions at arbitrary wrapped positions stay in-bounds.
        app.on_key_event(key(KeyCode::Char(' ')));
        app.on_key_event(key(KeyCode::Char('t')));
    }
    assert!(
        !app.tasks.tasks.is_empty(),
        "chaotic toggles may complete but never vanish tasks"
    );
    cleanup(dir);
}

#[test]
fn modal_estimate_input_mashing_stays_in_bounds() {
    let (mut app, dir) = make_app("estimate_mash");
    app.open_task_modal();
    app.task_input_title = "Mashed".to_string();
    app.on_key_event(key(KeyCode::Down)); // focus estimate field

    let mash = [
        KeyCode::Right,
        KeyCode::Left,
        KeyCode::Char('+'),
        KeyCode::Char('-'),
        KeyCode::Char('='),
        KeyCode::Char('_'),
        KeyCode::Char('9'),
        KeyCode::Char('0'),
        KeyCode::Char('l'),
        KeyCode::Char('h'),
    ];
    let mut rng: u64 = 0xC0FFEE;
    for _ in 0..500 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let code = mash[(rng % mash.len() as u64) as usize];
        app.on_key_event(key(code));
        assert!(
            (1..=20).contains(&app.task_input_estimated),
            "estimate escaped bounds: {}",
            app.task_input_estimated
        );
    }

    app.on_key_event(key(KeyCode::Enter));
    assert!(!app.show_task_modal);
    assert_eq!(app.tasks.tasks.len(), 1);
    assert!((1..=20).contains(&app.tasks.tasks[0].pomodoros_estimated));
    cleanup(dir);
}

#[test]
fn whitespace_only_title_rejected_then_real_title_accepted() {
    let (mut app, dir) = make_app("whitespace_title");
    app.open_task_modal();

    for c in "   ".chars() {
        app.on_key_event(key(KeyCode::Char(c)));
    }
    app.on_key_event(key(KeyCode::Enter));
    assert!(app.show_task_modal, "blank submission keeps the modal open");
    assert_eq!(app.tasks.tasks.len(), 0);

    // Keep typing over the spaces; submission trims and accepts.
    for c in "Real Task".chars() {
        app.on_key_event(key(KeyCode::Char(c)));
    }
    app.on_key_event(key(KeyCode::Enter));
    assert!(!app.show_task_modal);
    assert_eq!(app.tasks.tasks.len(), 1);
    assert_eq!(
        app.tasks.tasks[0].title, "Real Task",
        "stored title is trimmed"
    );
    assert_eq!(
        app.status_message.as_deref(),
        Some("Task added: Real Task"),
        "toast echoes the trimmed title"
    );
    cleanup(dir);
}

#[test]
fn delete_on_empty_and_filtered_out_lists_reports_noop() {
    let (mut app, dir) = make_app("noop_delete");
    app.active_tab = ActiveTab::Tasks;

    // Empty list: delete reports a no-op rather than a fake success.
    app.on_key_event(key(KeyCode::Char('d')));
    assert_eq!(
        app.status_message.as_deref(),
        Some("No task selected to delete.")
    );

    // Filtered-out list: task exists but is invisible under Completed.
    app.tasks.add("Invisible".to_string(), 1);
    app.on_key_event(key(KeyCode::Char('3'))); // Completed filter (empty)
    app.on_key_event(key(KeyCode::Char('d')));
    assert_eq!(
        app.tasks.tasks.len(),
        1,
        "invisible task must not be deletable"
    );
    assert_eq!(
        app.status_message.as_deref(),
        Some("No task selected to delete.")
    );
    cleanup(dir);
}

// ==============================================================================
// Section 5 — Stats & streaks
// ==============================================================================

#[test]
fn streak_ignores_future_dated_entries_gracefully() {
    let mut stats = StatsHistory::new();

    // Solid 3-day chain ending today...
    for offset in 0..3 {
        stats.sessions.push(CompletedSession {
            timestamp: Utc::now()
                .checked_sub_signed(ChronoDuration::days(offset))
                .unwrap(),
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });
    }
    // ...plus clock-skew junk dated tomorrow and next week.
    for offset in [1i64, 7] {
        stats.sessions.push(CompletedSession {
            timestamp: Utc::now()
                .checked_add_signed(ChronoDuration::days(offset))
                .unwrap(),
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });
    }

    assert_eq!(
        stats.current_streak_days(),
        3,
        "future-dated junk must neither inflate nor break the live streak"
    );
    // Of the 3-day chain only the offset-0 session falls on today's calendar
    // date; offsets 1 and 2 are yesterday and two days ago.
    assert_eq!(
        stats.today_work_sessions(),
        1,
        "only today's own session counts"
    );
    assert_eq!(
        stats.total_work_sessions(),
        5,
        "all-time totals include everything"
    );
}

#[test]
fn streak_continuity_across_month_and_year_boundary() {
    let mut stats = StatsHistory::new();
    // Dec 30 (Tue) -> Dec 31 -> Jan 1 -> Jan 2: four consecutive days spanning
    // a month AND a year boundary must read as one unbroken 4-day streak.
    let dates = [
        chrono::NaiveDate::from_ymd_opt(2025, 12, 30).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2026, 1, 2).unwrap(),
    ];
    for date in dates {
        stats.sessions.push(CompletedSession {
            timestamp: date
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_local_timezone(chrono::Local)
                .single()
                .expect("noon is DST-safe")
                .with_timezone(&Utc),
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });
    }
    assert_eq!(stats.longest_streak_days(), 4);
    assert_eq!(stats.distinct_work_dates().len(), 4);
}

#[test]
fn distribution_window_sums_match_recorded_sessions() {
    let mut stats = StatsHistory::new();
    for _ in 0..9 {
        stats.record(PomodoroPhase::Work, 25, None, None);
    }
    for _ in 0..4 {
        stats.record(PomodoroPhase::ShortBreak, 5, None, None);
    }

    let dist = stats.last_days_distribution(14);
    assert_eq!(dist.len(), 14);
    let work_sum: u64 = dist.iter().map(|(_, c)| *c).sum();
    assert_eq!(work_sum, 9, "breaks must not leak into the activity chart");
}

#[test]
fn today_metrics_exclude_other_days() {
    let mut stats = StatsHistory::new();
    let now = Utc::now();
    // Yesterday: 3 works + 1 break. Today: 2 works.
    for offset_hours in [30i64, 28, 26] {
        stats.sessions.push(CompletedSession {
            timestamp: now - ChronoDuration::hours(offset_hours),
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });
    }
    stats.sessions.push(CompletedSession {
        timestamp: now - ChronoDuration::hours(27),
        phase: PomodoroPhase::ShortBreak,
        duration_mins: 5,
        task_id: None,
        task_title: None,
    });
    for _ in 0..2 {
        stats.record(PomodoroPhase::Work, 50, None, None);
    }

    assert_eq!(stats.today_work_sessions(), 2, "only today's works counted");
    assert_eq!(stats.today_focus_minutes(), 100);
    assert_eq!(
        stats.total_work_sessions(),
        5,
        "all-time totals include history"
    );
    assert_eq!(stats.total_focus_minutes(), 175);
}

// ==============================================================================
// Section 6 — UI end-to-end rendering
// ==============================================================================

#[test]
fn render_every_tab_modal_geometry_matrix_with_content() {
    let (mut app, dir) = make_app("render_matrix");
    // Rich content: long unicode titles, deep history, extreme settings.
    app.tasks.add(
        "🔥 Extremely Long Task Title With Emoji 🚀 & Symbols #1234567890".to_string(),
        20,
    );
    for i in 0..40 {
        app.stats.record(
            PomodoroPhase::Work,
            120,
            Some(format!("id-{}", i)),
            Some("History".to_string()),
        );
    }
    app.config.work_duration_mins = 120;
    app.config.long_break_interval = 24;

    let geometries = [
        (20usize, 10usize),
        (35, 12),
        (80, 24),
        (120, 40),
        (350, 120),
    ];
    for (w, h) in geometries {
        let backend = TestBackend::new(w as u16, h as u16);
        let mut terminal = Terminal::new(backend).unwrap();
        for tab in [
            ActiveTab::Timer,
            ActiveTab::Tasks,
            ActiveTab::Stats,
            ActiveTab::Settings,
        ] {
            app.active_tab = tab;
            terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
        }
        app.show_help = true;
        terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
        app.show_help = false;
        app.open_task_modal();
        terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
        app.show_task_modal = false;
    }
    cleanup(dir);
}

#[test]
fn scrolled_task_list_keeps_selection_visible() {
    let (mut app, dir) = make_app("scroll_visible");
    app.active_tab = ActiveTab::Tasks;
    for i in 0..30 {
        app.tasks.add(format!("Alpha{:02}", i), 1);
    }
    app.tasks.selected_index = 25;

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();

    let buf = format!("{:?}", terminal.backend().buffer());
    assert!(
        buf.contains("Alpha25"),
        "selected row must scroll into view"
    );
    assert!(
        !buf.contains("Alpha00"),
        "rows far above the window must scroll off"
    );
    cleanup(dir);
}

#[test]
fn help_popup_renders_on_tiny_terminal() {
    let (mut app, dir) = make_app("help_tiny");
    app.show_help = true;
    for (w, h) in [(40u16, 12u16), (30, 10), (25, 8)] {
        let backend = TestBackend::new(w, h);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
        // Below ~35 columns the popup is narrower than its own title, so the
        // title truncates; there we only require a panic-free draw. From 40
        // columns up the branding must be visible.
        if w >= 40 {
            let buf = format!("{:?}", terminal.backend().buffer());
            assert!(
                buf.contains("Termodoro") || buf.contains("Keybindings"),
                "popup missing at {}x{}",
                w,
                h
            );
        }
    }
    cleanup(dir);
}

#[test]
fn big_digits_three_digit_minutes_uniform_width() {
    // 120-minute focus renders three minute digits; every row must stay the
    // same width or the banner shears horizontally.
    let lines = termodoro::ui::digits::render_big_time(120, 59);
    assert_eq!(lines.len(), 5);
    let width = lines[0].chars().count();
    assert!(width > 0);
    for line in &lines {
        assert_eq!(line.chars().count(), width, "digit rows diverged in width");
    }
    // Colon column aligns identically at any width.
    let small = termodoro::ui::digits::render_big_time(9, 5);
    assert_eq!(small.len(), 5);
    let w2 = small[0].chars().count();
    for line in &small {
        assert_eq!(line.chars().count(), w2);
    }
}

#[test]
fn footer_swaps_status_banner_back_to_hints_after_expiry() {
    let (mut app, dir) = make_app("footer_expiry");
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    app.set_status_message("FOCUS SESSION completed!".to_string());
    terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
    let buf = format!("{:?}", terminal.backend().buffer());
    assert!(
        buf.contains("FOCUS SESSION completed!"),
        "banner visible while fresh"
    );

    // Exactly 40 ticks of lifetime, matching set_status_message's contract.
    for _ in 0..39 {
        app.on_tick();
    }
    assert!(app.status_message.is_some(), "banner must survive 39 ticks");
    app.on_tick();
    assert_eq!(app.status_message, None);

    terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
    let buf = format!("{:?}", terminal.backend().buffer());
    assert!(buf.contains("Switch Tab"), "key hints return after expiry");
    assert!(!buf.contains("FOCUS SESSION completed!"));
    cleanup(dir);
}

#[test]
fn timer_view_renders_extreme_countdown_values() {
    let (mut app, dir) = make_app("extreme_countdown");
    app.active_tab = ActiveTab::Timer;
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    for remaining in [0u32, 1, 59, 60, 3599, 3600, 120 * 60, u32::MAX] {
        app.timer.time_remaining_secs = remaining;
        terminal.draw(|f| termodoro::ui::render(f, &app)).unwrap();
    }
    cleanup(dir);
}

// ==============================================================================
// Section 7 — Chaos fuzzing with persistence
// ==============================================================================

// 2000 mixed events (keys, sub-second ticks, saves, reloads). After every
// reload the persisted state must obey the same invariants as the live app.
#[test]
fn chaos_fuzz_mixed_events_with_periodic_persistence_invariants() {
    let (mut app, dir) = make_app("chaos_persist");
    app.tasks.add("Chaos One".to_string(), 3);
    app.tasks.add("Chaos Two".to_string(), 1);

    let pool = [
        KeyCode::Char(' '),
        KeyCode::Char('a'),
        KeyCode::Char('s'),
        KeyCode::Char('r'),
        KeyCode::Char('t'),
        KeyCode::Char('d'),
        KeyCode::Char('x'),
        KeyCode::Char('j'),
        KeyCode::Char('k'),
        KeyCode::Char('h'),
        KeyCode::Char('l'),
        KeyCode::Char('+'),
        KeyCode::Char('-'),
        KeyCode::Char('1'),
        KeyCode::Char('2'),
        KeyCode::Char('3'),
        KeyCode::Char('4'),
        KeyCode::Char('?'),
        KeyCode::Tab,
        KeyCode::BackTab,
        KeyCode::Esc,
        KeyCode::Enter,
        KeyCode::Backspace,
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
    ];
    let mut rng: u64 = 0x5EED_2026_0821;

    for step in 0..2000 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;

        if app.should_quit {
            app.should_quit = false; // keep hammering
        }
        app.on_key_event(key(pool[(rng % pool.len() as u64) as usize]));

        // Interleave wall-clock progress every other step.
        if step % 2 == 0 {
            app.on_tick();
        }
        // Periodically persist and verify the reloaded world is coherent.
        if step % 250 == 0 {
            app.save_state();
            let reloaded = relaunch(&dir);
            assert!(
                (1..=120).contains(&reloaded.config.work_duration_mins),
                "work duration escaped clamp in persisted state"
            );
            assert!(
                (1..=24).contains(&reloaded.config.long_break_interval),
                "interval escaped clamp in persisted state"
            );
            assert_eq!(reloaded.tasks.filter, TaskFilter::All);
            for task in &reloaded.tasks.tasks {
                assert!(!task.id.is_empty());
                assert!(!task.title.trim().is_empty(), "blank title persisted");
            }
            if let Some(active) = &reloaded.tasks.active_task_id {
                assert!(
                    reloaded.tasks.tasks.iter().any(|t| &t.id == active),
                    "active_task_id points at a ghost task after reload"
                );
            }
        }

        // Live invariants, checked continuously.
        assert!(app.settings_index <= 8);
        assert!((1..=20).contains(&app.task_input_estimated));
        assert!(
            app.timer.time_remaining_secs <= 120 * 60 || app.timer.status != TimerStatus::Stopped
        );
    }
    cleanup(dir);
}

#[test]
fn chaos_filter_modal_interleave_maintains_core_invariants() {
    let (mut app, dir) = make_app("chaos_modal");
    app.active_tab = ActiveTab::Tasks;
    for i in 1..=8 {
        app.tasks.add(format!("Interleave {}", i), (i % 4) + 1);
        if i % 3 == 0 {
            app.tasks.toggle_selected();
        }
    }

    let mut rng: u64 = 0xABCD_1234;
    let actions = [
        KeyCode::Char('1'),
        KeyCode::Char('2'),
        KeyCode::Char('3'),
        KeyCode::Char('j'),
        KeyCode::Char('k'),
        KeyCode::Char('a'),
        KeyCode::Tab,
        KeyCode::Esc,
        KeyCode::Enter,
        KeyCode::Char('x'),
        KeyCode::Char(' '),
        KeyCode::Char('t'),
        KeyCode::Backspace,
    ];
    for _ in 0..1500 {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let chosen = actions[(rng % actions.len() as u64) as usize];
        app.on_key_event(key(chosen));

        let visible = app.tasks.filtered_indices().len();
        if visible > 0 {
            assert!(
                app.tasks.selected_index < visible,
                "cursor escaped visible list"
            );
        } else {
            assert_eq!(app.tasks.selected_index, 0, "empty view must reset cursor");
        }
        // Modal open/closed consistency: typing went somewhere well-defined.
        if app.show_task_modal {
            assert!(!app.should_quit, "'q' inside modal typed text, never quit");
        }
    }
    cleanup(dir);
}
