// Import App struct and ui module from library crate
use termodoro::{app::App, ui};

// Import crossterm event poll and read functions
use crossterm::{
    // Event types and polling
    event::{self, Event, KeyEventKind},
    // Terminal execution helper
    execute,
    // Terminal mode and screen operations
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// Import Ratatui Terminal backend
use ratatui::{backend::CrosstermBackend, Terminal};
// Import standard error and I/O utilities
use std::{
    error::Error,
    io::stdout,
    panic,
    time::{Duration, Instant},
};

// RAII guard that restores the terminal to a sane state when dropped.
//
// Restoring inside main() alone is not enough: if terminal setup fails partway
// (for example Terminal::new errors after raw mode was already enabled), the
// `?` operator would return early and skip every restore call, leaving the
// user's shell in raw mode with a hidden cursor and no echo. Because this
// struct implements Drop, restoration runs on EVERY exit path: normal return,
// early error return, and panics alike.
struct TerminalGuard;

impl TerminalGuard {
    // Enters raw mode + alternate screen and returns a guard that undoes it
    fn acquire() -> Result<Self, Box<dyn Error>> {
        // Enable terminal raw mode (captures keystrokes immediately without waiting for enter)
        enable_raw_mode()?;
        // Switch to alternate screen and hide terminal cursor
        execute!(stdout(), EnterAlternateScreen, crossterm::cursor::Hide)?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort restore: each step is independent so one failure does
        // not prevent the others from running
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
    }
}

// Sets up a panic hook to guarantee terminal state is restored if the program encounters a panic
fn setup_panic_hook() {
    // Save original default panic hook
    let original_hook = panic::take_hook();
    // Register custom hook
    panic::set_hook(Box::new(move |panic_info| {
        // Disable terminal raw mode
        let _ = disable_raw_mode();
        // Leave alternate screen and show cursor
        let _ = execute!(stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
        // Invoke original hook to print stack trace
        original_hook(panic_info);
    }));
}

// Pure decision logic for reconciling missed tick intervals after a stall.
//
// Returns `(intervals_to_advance, ticks_to_fire)`:
//   * `intervals_to_advance` — how many TICK_RATE multiples to jump
//     `last_tick` forward so sub-interval remainders are preserved.
//   * `ticks_to_fire` — how many application ticks to run now, capped by
//     `max_catch_up` so a long stall cannot burst hundreds of notifications.
//
// Extracted from the event loop so the arithmetic is unit-testable: the
// previous inline version performed a lossy `u128 as u32` cast and could
// panic on `Duration` multiplication overflow for absurd stalls (>34 years).
fn reconcile_missed_ticks(
    elapsed: Duration,
    tick_rate: Duration,
    max_catch_up: u128,
) -> (u32, u128) {
    // Not yet due: nothing to advance, nothing to fire.
    if elapsed < tick_rate {
        return (0, 0);
    }
    // How many whole tick intervals elapsed since the deadline. At least 1:
    // the caller only invokes this when `elapsed >= tick_rate`, but rounding
    // of sub-millisecond remainders must never produce zero work.
    let missed = (elapsed.as_millis() / tick_rate.as_millis()).max(1);
    // Saturate the deadline jump at u32::MAX instead of wrapping/panicking:
    // a stall longer than ~2.7 years of intervals is pathological, but must
    // degrade to "jump far forward" rather than abort the event loop.
    let advance = missed.min(u32::MAX as u128) as u32;
    // Fire at most the cap; the remaining time debt is absorbed by the jump.
    let fire = missed.min(max_catch_up);
    (advance, fire)
}

// Filter deciding whether a crossterm key event should reach the app.
//
// Press AND auto-repeat are accepted: filtering only Press previously
// disabled key-repeat entirely, forcing one physical press per navigation
// step when holding j/k/h/l. Release events stay filtered because they
// double-fire on some platforms.
fn is_dispatchable_key(kind: KeyEventKind) -> bool {
    matches!(kind, KeyEventKind::Press | KeyEventKind::Repeat)
}

// Application entry point function
fn main() -> Result<(), Box<dyn Error>> {
    // Install panic hook for terminal safety
    setup_panic_hook();

    // Acquire the terminal through the RAII guard so ANY early failure or
    // panic path still restores raw mode, the main screen, and the cursor
    let _guard = TerminalGuard::acquire()?;

    // Construct Crossterm backend for Ratatui
    let backend = CrosstermBackend::new(stdout());
    // Initialize Ratatui terminal instance
    let mut terminal = Terminal::new(backend)?;

    // Instantiate core application state
    let mut app = App::new();

    // Run main application event loop
    let res = run_app(&mut terminal, &mut app);

    // Ensure state is saved on exit (failures are surfaced inside save_state)
    app.save_state();

    // Check if error occurred during main loop
    if let Err(err) = res {
        // Print error message
        eprintln!("Application Error: {:?}", err);
    }

    // _guard drops here (or earlier, on any `?`), restoring the terminal
    Ok(())
}

// Main event loop handling rendering, tick intervals, and keyboard input
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    // Define tick rate interval (250 milliseconds for responsive UI and status updates)
    const TICK_RATE: Duration = Duration::from_millis(250);
    // Upper bound on how many missed intervals are reconciled in one pass.
    // Without a cap, a single long stall (network hiccup on a slow SSH link,
    // blocked disk write) would fire hundreds of ticks at once, bursting
    // notifications and status expirations; the cap trades perfect catch-up
    // for bounded, predictable behavior in pathological cases.
    const MAX_CATCH_UP_TICKS: u128 = 10;
    // Track timestamp of the next scheduled tick boundary
    let mut last_tick = Instant::now();

    // Continuous event loop
    loop {
        // Render current application UI frame
        terminal.draw(|f| ui::render(f, app))?;

        // Calculate dynamic timeout until next tick boundary
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Check for incoming terminal events with dynamic timeout
        if event::poll(timeout)? {
            // Read terminal event
            if let Event::Key(key) = event::read()? {
                // Accept press AND auto-repeat events; Release is filtered.
                if is_dispatchable_key(key.kind) {
                    // Dispatch key to application handler
                    app.on_key_event(key);
                }
            }
        }

        // Reconcile elapsed wall-clock time against scheduled tick boundaries.
        // Advancing last_tick BY the interval (instead of resetting it to now)
        // preserves sub-interval remainders, so recurring stalls no longer
        // make the countdown visibly lag behind real time.
        let elapsed = last_tick.elapsed();
        if elapsed >= TICK_RATE {
            // Jump the deadline past ALL missed intervals and fire a bounded
            // number of application ticks (see reconcile_missed_ticks).
            let (advance, fire) = reconcile_missed_ticks(elapsed, TICK_RATE, MAX_CATCH_UP_TICKS);
            last_tick += TICK_RATE * advance;
            for _ in 0..fire {
                app.on_tick();
            }
        }

        // Check if user requested to quit
        if app.should_quit {
            // Break from event loop
            break;
        }
    }

    // Return Ok on normal exit
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

    const TICK: Duration = Duration::from_millis(250);

    // Below one full interval nothing is due yet.
    #[test]
    fn reconcile_no_catch_up_before_interval_elapses() {
        assert_eq!(
            reconcile_missed_ticks(Duration::from_millis(0), TICK, 10),
            (0, 0)
        );
        assert_eq!(
            reconcile_missed_ticks(Duration::from_millis(249), TICK, 10),
            (0, 0)
        );
    }

    // Exactly one elapsed interval fires exactly one tick.
    #[test]
    fn reconcile_exactly_one_interval() {
        assert_eq!(reconcile_missed_ticks(TICK, TICK, 10), (1, 1));
        assert_eq!(
            reconcile_missed_ticks(Duration::from_millis(251), TICK, 10),
            (1, 1)
        );
    }

    // Sub-interval remainders round DOWN to whole intervals.
    #[test]
    fn reconcile_rounds_down_to_whole_intervals() {
        // 1.5 intervals -> 1 missed
        assert_eq!(
            reconcile_missed_ticks(Duration::from_millis(375), TICK, 10),
            (1, 1)
        );
        // 4.9 intervals -> 4 missed
        assert_eq!(
            reconcile_missed_ticks(Duration::from_millis(1225), TICK, 10),
            (4, 4)
        );
    }

    // A long stall advances the deadline past every missed interval but only
    // fires the capped number of application ticks.
    #[test]
    fn reconcile_caps_fired_ticks_but_advances_full_deadline() {
        let (advance, fire) = reconcile_missed_ticks(Duration::from_secs(250), TICK, 10);
        assert_eq!(advance, 1000, "deadline must jump past all 1000 intervals");
        assert_eq!(fire, 10, "fired ticks must be capped at MAX_CATCH_UP_TICKS");
    }

    // Pathological stall (>34 years of intervals): must saturate instead of
    // wrapping the u32 cast or panicking on Duration multiplication overflow.
    #[test]
    fn reconcile_saturates_on_absurd_stall_instead_of_panicking() {
        // ~2^63 milliseconds of elapsed time: far beyond u32 interval counts.
        let huge = Duration::from_millis(u64::from(u32::MAX) * 250 * 1000);
        let (advance, fire) = reconcile_missed_ticks(huge, TICK, 10);
        assert_eq!(advance, u32::MAX, "deadline jump saturates at u32::MAX");
        assert_eq!(fire, 10);
    }

    // The cap parameter is honored for arbitrary values, not just 10.
    #[test]
    fn reconcile_honors_custom_cap() {
        let (_, fire) = reconcile_missed_ticks(Duration::from_secs(100), TICK, 3);
        assert_eq!(fire, 3);
        let (_, fire) = reconcile_missed_ticks(Duration::from_secs(100), TICK, 400);
        assert_eq!(fire, 400);
    }

    // Key-kind filter contract: Press and Repeat reach the app, Release and
    // any other kind never do.
    #[test]
    fn key_filter_accepts_press_and_repeat_rejects_release() {
        assert!(is_dispatchable_key(KeyEventKind::Press));
        assert!(is_dispatchable_key(KeyEventKind::Repeat));
        assert!(!is_dispatchable_key(KeyEventKind::Release));

        // End-to-end through a real KeyEvent struct for documentation value.
        let press = KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        };
        assert!(is_dispatchable_key(press.kind));
        let release = KeyEvent {
            kind: KeyEventKind::Release,
            ..press
        };
        assert!(!is_dispatchable_key(release.kind));
    }
}
