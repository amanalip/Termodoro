// Declare application state module
mod app;
// Declare audio chime playback module
pub mod audio;
// Declare configuration module
mod config;
// Declare analytics and stats module
mod stats;
// Declare storage persistence module
mod storage;
// Declare task management module
mod tasks;
// Declare visual color themes module
mod theme;
// Declare Pomodoro timer engine module
mod timer;
// Declare terminal UI rendering module
mod ui;

// Import App struct from app module
use app::App;
// Import crossterm event poll and read functions
use crossterm::{
    // Event types and polling
    event::{self, Event},
    // Terminal execution helper
    execute,
    // Terminal mode and screen operations
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// Import Ratatui Terminal backend
use ratatui::{backend::CrosstermBackend, Terminal};
// Import standard error and I/O utilities
use std::{error::Error, io::stdout, panic, time::Duration};

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

    // Enable terminal raw mode (captures keystrokes immediately without waiting for enter)
    enable_raw_mode()?;
    // Standard output handle
    let mut stdout_handle = stdout();
    // Switch to alternate screen and hide terminal cursor
    execute!(stdout_handle, EnterAlternateScreen, crossterm::cursor::Hide)?;

    // Construct Crossterm backend for Ratatui
    let backend = CrosstermBackend::new(stdout_handle);
    // Initialize Ratatui terminal instance
    let mut terminal = Terminal::new(backend)?;

    // Instantiate core application state
    let mut app = App::new();

    // Run main application event loop
    let res = run_app(&mut terminal, &mut app);

    // Ensure state is saved on exit
    app.save_state();

    // Restore terminal: disable raw mode
    disable_raw_mode()?;
    // Leave alternate screen and restore cursor visibility
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    // Restore clear terminal screen
    terminal.show_cursor()?;

    // Check if error occurred during main loop
    if let Err(err) = res {
        // Print error message
        eprintln!("Application Error: {:?}", err);
    }

    // Return success
    Ok(())
}

// Main event loop handling rendering, tick intervals, and keyboard input
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    // Define tick rate interval (250 milliseconds for smooth countdown updates)
    let tick_rate = Duration::from_millis(250);

    // Continuous event loop
    loop {
        // Render current application UI frame
        terminal.draw(|f| ui::render(f, app))?;

        // Check for incoming terminal events with timeout matching tick rate
        if event::poll(tick_rate)? {
            // Read terminal event
            if let Event::Key(key) = event::read()? {
                // Ensure key press event (filters out release events on certain platforms)
                if key.kind == event::KeyEventKind::Press {
                    // Dispatch key to application handler
                    app.on_key_event(key);
                }
            }
        }

        // Invoke periodic application tick logic
        app.on_tick();

        // Check if user requested to quit
        if app.should_quit {
            // Break from event loop
            break;
        }
    }

    // Return Ok on normal exit
    Ok(())
}
