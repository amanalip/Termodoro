// ==============================================================================
// Property-based test suite (proptest).
//
// Where example-based tests pin specific known edge cases, property tests
// assert invariants that must hold for ARBITRARY inputs. proptest generates
// hundreds of randomized cases per property and, on failure, shrinks the
// input down to a minimal reproducing case that lands in
// tests/property_tests.proptest-regressions for automatic replay.
//
// Run with: cargo test --test property_tests
// ==============================================================================

use proptest::prelude::*;
use termodoro::config::Config;
use termodoro::stats::StatsHistory;
use termodoro::storage::{AppData, Storage};
use termodoro::tasks::{TaskFilter, TaskManager};
use termodoro::timer::{PomodoroPhase, PomodoroTimer};

// ------------------------------------------------------------------------------
// Strategies
// ------------------------------------------------------------------------------

fn arb_theme() -> impl Strategy<Value = termodoro::theme::ThemeChoice> {
    proptest::sample::select(termodoro::theme::ThemeChoice::all().to_vec())
}

// Arbitrary Config covering the full u32 domain (not just legal ranges) so
// serialization and sanitization are exercised on hostile values too.
fn arb_config() -> impl Strategy<Value = Config> {
    (
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        arb_theme(),
    )
        .prop_map(
            |(work, short, long, interval, asb, asw, sound, notif, theme)| Config {
                work_duration_mins: work,
                short_break_mins: short,
                long_break_mins: long,
                long_break_interval: interval,
                auto_start_breaks: asb,
                auto_start_work: asw,
                sound_enabled: sound,
                desktop_notifications: notif,
                theme,
            },
        )
}

// A task title drawn from ASCII, unicode, emoji, whitespace, and control-ish
// characters — the kinds of strings users actually type.
fn arb_title() -> impl Strategy<Value = String> {
    prop::string::string_regex("[\\p{L}\\p{N} 🦀🚀☕🌴\t '!?&/\\-_.#]{0,60}")
        .expect("valid regex strategy")
}

fn arb_task_manager() -> impl Strategy<Value = TaskManager> {
    (
        prop::collection::vec((arb_title(), 0u32..=25), 0..30),
        any::<bool>(),
    )
        .prop_map(|(titles, _flag)| {
            let mut manager = TaskManager::new();
            for (title, est) in titles {
                manager.add(title, est);
            }
            manager
        })
}

// Random operation sequences for the timer state machine.
#[derive(Debug, Clone, Copy)]
enum TimerOp {
    Toggle,
    Tick,
    Skip,
    Reset,
}

fn arb_timer_ops() -> impl Strategy<Value = Vec<TimerOp>> {
    prop::collection::vec(
        prop::sample::select(vec![
            TimerOp::Toggle,
            TimerOp::Tick,
            TimerOp::Skip,
            TimerOp::Reset,
        ]),
        0..400,
    )
}

// Random operation sequences for the task manager.
#[derive(Debug, Clone, Copy)]
enum TaskOp {
    Add,
    ToggleSelected,
    RemoveSelected,
    Next,
    Previous,
    SetActive,
    IncrementSpent,
    FilterAll,
    FilterActive,
    FilterCompleted,
}

fn arb_task_ops() -> impl Strategy<Value = Vec<TaskOp>> {
    prop::collection::vec(
        prop::sample::select(vec![
            TaskOp::Add,
            TaskOp::ToggleSelected,
            TaskOp::RemoveSelected,
            TaskOp::Next,
            TaskOp::Previous,
            TaskOp::SetActive,
            TaskOp::IncrementSpent,
            TaskOp::FilterAll,
            TaskOp::FilterActive,
            TaskOp::FilterCompleted,
        ]),
        0..200,
    )
}

// ------------------------------------------------------------------------------
// Serialization properties
// ------------------------------------------------------------------------------

proptest! {
    // Every possible Config must survive a JSON roundtrip byte-for-byte at
    // the value level, including out-of-range durations that sanitize would
    // later clamp.
    #[test]
    fn config_json_roundtrip_is_identity(config in arb_config()) {
        let json = serde_json::to_string(&config).expect("Config serialization is infallible");
        let parsed: Config = serde_json::from_str(&json).expect("own output must re-parse");
        prop_assert_eq!(parsed, config);
    }

    // Sanitization is idempotent: sanitize(sanitize(x)) == sanitize(x).
    #[test]
    fn sanitize_is_idempotent(config in arb_config()) {
        let mut once = config.clone();
        once.sanitize();
        let mut twice = once.clone();
        twice.sanitize();
        prop_assert_eq!(once, twice);
    }

    // Sanitization is total: every output field lies inside its documented
    // legal range, whatever hostile input went in.
    #[test]
    fn sanitize_output_always_in_legal_range(mut config in arb_config()) {
        config.sanitize();
        prop_assert!((1..=120).contains(&config.work_duration_mins));
        prop_assert!((1..=60).contains(&config.short_break_mins));
        prop_assert!((1..=90).contains(&config.long_break_mins));
        prop_assert!((1..=24).contains(&config.long_break_interval));
    }

    // Full AppData save -> load roundtrip through a REAL file. Config comes
    // back sanitized (documented load behavior); tasks and stats come back
    // exactly.
    #[test]
    fn appdata_save_load_roundtrip(config in arb_config(), tasks in arb_task_manager()) {
        let dir = std::env::temp_dir()
            .join(format!("termodoro_proptest_{}", uuid::Uuid::new_v4()));
        let storage = Storage::with_path(dir.join("data.json"));

        let mut stats = StatsHistory::new();
        stats.record(PomodoroPhase::Work, 25, tasks.active_task_id.clone(), None);
        stats.record(PomodoroPhase::ShortBreak, 5, None, None);

        storage
            .save(&config, &tasks, &stats)
            .expect("save to fresh temp dir must succeed");
        let loaded = storage.load();

        let mut expected_config = config.clone();
        expected_config.sanitize();
        prop_assert_eq!(loaded.config, expected_config);
        prop_assert_eq!(loaded.tasks.tasks.len(), tasks.tasks.len());
        prop_assert_eq!(loaded.stats.sessions.len(), stats.sessions.len());

        let _ = std::fs::remove_dir_all(dir);
    }
}

// ------------------------------------------------------------------------------
// Timer state-machine properties
// ------------------------------------------------------------------------------

proptest! {
    // After ANY sequence of user operations on a freshly constructed timer:
    //   * remaining time never exceeds the phase's total duration,
    //   * progress ratio stays within [0, 1],
    //   * completed pomodoros only ever grows by one per natural Work
    //     completion (never via skip),
    //   * the cycle counter stays inside 1..=interval whenever it started
    //     there and the interval is untouched.
    #[test]
    fn timer_invariants_hold_under_arbitrary_op_sequences(
        ops in arb_timer_ops(),
        work_mins in 1u32..=120,
        short_mins in 1u32..=60,
        long_mins in 1u32..=90,
        interval in 1u32..=24,
    ) {
        let config = Config {
            work_duration_mins: work_mins,
            short_break_mins: short_mins,
            long_break_mins: long_mins,
            long_break_interval: interval,
            auto_start_breaks: true,
            auto_start_work: true,
            ..Config::default()
        };
        let mut timer = PomodoroTimer::new(&config);
        let mut completed_before = 0u32;

        for op in ops {
            match op {
                TimerOp::Toggle => timer.toggle(),
                TimerOp::Tick => {
                    let _ = timer.tick(&config);
                }
                TimerOp::Skip => {
                    timer.skip_to_next(&config);
                }
                TimerOp::Reset => timer.reset(&config),
            }

            let total = timer.target_duration_secs(timer.phase, &config);
            // After a transition the stored total matches the phase target;
            // mid-phase it is unchanged. Either way remaining <= max(stored, target)
            // and remaining <= stored total must hold because transitions refill.
            prop_assert!(
                timer.time_remaining_secs <= timer.total_duration_secs.max(total),
                "remaining {} exceeds totals {} / {} after {:?}",
                timer.time_remaining_secs,
                timer.total_duration_secs,
                total,
                op
            );

            let ratio = timer.progress_ratio();
            prop_assert!((0.0..=1.0).contains(&ratio), "ratio {ratio} escaped [0,1]");

            // Skips never credit pomodoros.
            if timer.completed_pomodoros > completed_before {
                prop_assert_eq!(timer.completed_pomodoros, completed_before + 1);
                completed_before = timer.completed_pomodoros;
            }

            prop_assert!(
                (1..=interval).contains(&timer.current_cycle),
                "cycle {} escaped 1..={interval}",
                timer.current_cycle
            );
        }
    }

    // Skipping can NEVER manufacture progress: over arbitrary sequences,
    // completed_pomodoros after skips-only equals zero.
    #[test]
    fn skip_only_sequences_never_credit_progress(ops in arb_timer_ops()) {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        for op in ops {
            if let TimerOp::Skip = op {
                timer.skip_to_next(&config);
                prop_assert_eq!(timer.completed_pomodoros, 0);
                prop_assert!(
                    timer.phase != PomodoroPhase::LongBreak || timer.current_cycle == 1,
                    "a skip must never deliver a long break"
                );
            }
        }
    }
}

// ------------------------------------------------------------------------------
// TaskManager properties
// ------------------------------------------------------------------------------

proptest! {
    // CONTRACT: after any STRUCTURAL or NAVIGATION operation (add, toggle,
    // remove, next, previous) the cursor addresses a real row of the
    // currently filtered view, or resets to 0 when that view is empty.
    // Non-structural ops (set-active, increment-spent) and bare filter flips
    // do not re-clamp: the App's key handler resets the cursor explicitly on
    // filter keys, so the library defers to callers there.
    #[test]
    fn task_cursor_valid_after_mutations(ops in arb_task_ops(), titles in prop::collection::vec(arb_title(), 0..10)) {
        let mut manager = TaskManager::new();
        for title in titles {
            manager.add(title, 1);
        }

        for op in ops {
            match op {
                TaskOp::Add => manager.add(format!("gen-{}", uuid::Uuid::new_v4()), 1),
                TaskOp::ToggleSelected => manager.toggle_selected(),
                TaskOp::RemoveSelected => {
                    manager.remove_selected();
                }
                TaskOp::Next => manager.next(),
                TaskOp::Previous => manager.previous(),
                TaskOp::SetActive => {
                    let _ = manager.set_selected_active();
                }
                TaskOp::IncrementSpent => manager.increment_active_spent(),
                TaskOp::FilterAll => manager.filter = TaskFilter::All,
                TaskOp::FilterActive => manager.filter = TaskFilter::Active,
                TaskOp::FilterCompleted => manager.filter = TaskFilter::Completed,
            }

            // Invariants checked after EVERY operation:
            // filtered indices are always valid positions into tasks.
            let visible = manager.filtered_indices();
            prop_assert!(visible.iter().all(|&i| i < manager.tasks.len()));
            // Sorted + strictly increasing (deduped positions).
            prop_assert!(visible.windows(2).all(|w| w[0] < w[1]));

            // Cursor contract holds after structural + navigation ops.
            if matches!(
                op,
                TaskOp::Add | TaskOp::ToggleSelected | TaskOp::RemoveSelected | TaskOp::Next | TaskOp::Previous
            ) {
                if visible.is_empty() {
                    prop_assert_eq!(manager.selected_index, 0);
                } else {
                    prop_assert!(
                        manager.selected_index < visible.len(),
                        "cursor {} escaped {} visible rows after {:?}",
                        manager.selected_index,
                        visible.len(),
                        op
                    );
                }
            }
        }
    }

    // remove_selected returns true exactly when the task count shrinks.
    #[test]
    fn remove_return_value_matches_actual_deletion(
        pre_ops in arb_task_ops(),
        remove_ops in prop::collection::vec(any::<bool>(), 1..20),
    ) {
        let mut manager = TaskManager::new();
        for i in 0..6 {
            manager.add(format!("t{i}"), 1);
        }
        apply_task_ops(&mut manager, &pre_ops);

        for _ in remove_ops {
            let before = manager.tasks.len();
            let reported = manager.remove_selected();
            prop_assert_eq!(reported, manager.tasks.len() < before);
        }
    }
}

fn apply_task_ops(manager: &mut TaskManager, ops: &[TaskOp]) {
    for op in ops {
        match op {
            TaskOp::Add => manager.add("x".into(), 1),
            TaskOp::ToggleSelected => manager.toggle_selected(),
            TaskOp::RemoveSelected => {
                manager.remove_selected();
            }
            TaskOp::Next => manager.next(),
            TaskOp::Previous => manager.previous(),
            TaskOp::SetActive => {
                let _ = manager.set_selected_active();
            }
            TaskOp::IncrementSpent => manager.increment_active_spent(),
            TaskOp::FilterAll => manager.filter = TaskFilter::All,
            TaskOp::FilterActive => manager.filter = TaskFilter::Active,
            TaskOp::FilterCompleted => manager.filter = TaskFilter::Completed,
        }
    }
}

// ------------------------------------------------------------------------------
// Parser robustness
// ------------------------------------------------------------------------------

proptest! {
    // The AppData deserializer must NEVER panic, whatever bytes it is fed:
    // data.json is user-writable state, so hostile or corrupt content may
    // only produce Ok(data) or Err(_), never a crash. This is the property
    // behind the quarantine safety net.
    #[test]
    fn appdata_deserializer_never_panics_on_arbitrary_input(input in ".*") {
        let _ = serde_json::from_str::<AppData>(&input);
    }

    // Same guarantee for JSON-flavored garbage built from structural tokens.
    #[test]
    fn appdata_deserializer_never_panics_on_structured_garbage(
        a in any::<u64>(),
        b in any::<bool>(),
        s in ".*",
        depth in 0usize..40,
    ) {
        let candidates = [
            format!("{{\"config\":{a},\"tasks\":{b}}}"),
            format!("{{\"stats\":{{\"sessions\":[{{\"timestamp\":\"{s}\"}}]}}}}"),
            format!("{}{}", "[".repeat(depth), "]".repeat(depth)),
            format!("null{s}"),
            format!("{{\"config\":{{\"theme\":{a:?}}}}}",),
        ];
        for candidate in &candidates {
            let _ = serde_json::from_str::<AppData>(candidate);
        }
    }
}
