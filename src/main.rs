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
                // Accept press AND auto-repeat events: filtering only Press
                // previously disabled key-repeat entirely, forcing one physical
                // press per navigation step when holding j/k/h/l.
                // Release events remain filtered (they double-fire on some platforms).
                if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                    // Dispatch key to application handler
                    app.on_key_event(key);
                }
            }
        }

        // Reconcile elapsed wall-clock time against scheduled tick boundaries.
        // Advancing last_tick BY the interval (instead of resetting it to now)
        // preserves sub-interval remainders, so recurring stalls no longer
        // make the countdown visibly lag behind real time.
        if last_tick.elapsed() >= TICK_RATE {
            // How many whole tick intervals have elapsed since the deadline
            let missed = (last_tick.elapsed().as_millis() / TICK_RATE.as_millis()).max(1);
            // Schedule the next boundary after ALL missed intervals
            last_tick += TICK_RATE * missed as u32;
            // Fire one application tick per missed interval, capped for safety
            for _ in 0..missed.min(MAX_CATCH_UP_TICKS) {
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
