// Import serde traits for serializing and deserializing timer state
use serde::{Deserialize, Serialize};
// Import the Config struct for reading user-defined phase durations
use crate::config::Config;

// Enum representing the current interval phase of the Pomodoro technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PomodoroPhase {
    // Standard work/focus session
    Work,
    // Short break between focus sessions
    ShortBreak,
    // Long restorative break after completing a full cycle
    LongBreak,
}

impl PomodoroPhase {
    // Returns user-facing title string for the phase
    pub fn title(&self) -> &'static str {
        // Match phase variant
        match self {
            // Title for Work
            PomodoroPhase::Work => "FOCUS SESSION",
            // Title for ShortBreak
            PomodoroPhase::ShortBreak => "SHORT BREAK",
            // Title for LongBreak
            PomodoroPhase::LongBreak => "LONG BREAK",
        }
    }

    // Returns emoji icon representing the phase
    pub fn emoji(&self) -> &'static str {
        // Match phase variant
        match self {
            // Tomato emoji for work
            PomodoroPhase::Work => "🍅",
            // Coffee cup for short break
            PomodoroPhase::ShortBreak => "☕",
            // Palm tree for long break
            PomodoroPhase::LongBreak => "🌴",
        }
    }
}

// Enum representing the operational execution state of the countdown timer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerStatus {
    // Timer is reset / idle
    Stopped,
    // Timer is actively counting down
    Running,
    // Timer is paused mid-countdown
    Paused,
}

// Event triggered when a countdown phase completes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerEvent {
    // Event fired when phase reaches 00:00
    PhaseCompleted {
        // The phase that just concluded
        finished_phase: PomodoroPhase,
        // The upcoming phase scheduled next
        next_phase: PomodoroPhase,
    },
}

// Structure encapsulating the entire state and logic of the Pomodoro timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroTimer {
    // Active Pomodoro phase (Work, ShortBreak, LongBreak)
    pub phase: PomodoroPhase,
    // Current running status (Stopped, Running, Paused)
    pub status: TimerStatus,
    // Number of seconds remaining in the current countdown
    pub time_remaining_secs: u32,
    // Total duration in seconds for the current phase
    pub total_duration_secs: u32,
    // Total number of completed work pomodoros in current session
    pub completed_pomodoros: u32,
    // Current cycle index (1..=long_break_interval)
    pub current_cycle: u32,
}

impl PomodoroTimer {
    // Constructs a new PomodoroTimer configured according to user settings
    pub fn new(config: &Config) -> Self {
        // Calculate initial total seconds for work phase
        let total = config.work_duration_mins * 60;
        // Build and return new instance
        Self {
            // Initial phase is Work
            phase: PomodoroPhase::Work,
            // Initial status is Stopped
            status: TimerStatus::Stopped,
            // Initial remaining seconds matches work duration
            time_remaining_secs: total,
            // Total seconds for progress calculation
            total_duration_secs: total,
            // 0 completed pomodoros at start
            completed_pomodoros: 0,
            // Start at cycle 1
            current_cycle: 1,
        }
    }

    // Computes target duration in seconds for a specific phase based on config
    pub fn target_duration_secs(&self, phase: PomodoroPhase, config: &Config) -> u32 {
        // Match on the requested phase
        match phase {
            // Work duration in minutes converted to seconds
            PomodoroPhase::Work => config.work_duration_mins * 60,
            // Short break duration in minutes converted to seconds
            PomodoroPhase::ShortBreak => config.short_break_mins * 60,
            // Long break duration in minutes converted to seconds
            PomodoroPhase::LongBreak => config.long_break_mins * 60,
        }
    }

    // Toggles timer state between Running and Paused
    pub fn toggle(&mut self) {
        // Check current running status
        match self.status {
            // If running, switch to paused
            TimerStatus::Running => self.status = TimerStatus::Paused,
            // If paused or stopped, switch to running
            TimerStatus::Paused | TimerStatus::Stopped => self.status = TimerStatus::Running,
        }
    }

    // Pauses the timer if currently running
    #[allow(dead_code)]
    pub fn pause(&mut self) {
        // If timer is running, set to paused
        if self.status == TimerStatus::Running {
            // Set status to Paused
            self.status = TimerStatus::Paused;
        }
    }

    // Resets current phase timer back to its initial full duration
    pub fn reset(&mut self, config: &Config) {
        // Stop timer execution
        self.status = TimerStatus::Stopped;
        // Get target duration for current phase
        let total = self.target_duration_secs(self.phase, config);
        // Reset total duration
        self.total_duration_secs = total;
        // Reset remaining seconds
        self.time_remaining_secs = total;
    }

    // Calculates completion ratio from 0.0 (just started) to 1.0 (completed)
    pub fn progress_ratio(&self) -> f64 {
        // Guard against division by zero
        if self.total_duration_secs == 0 {
            // Return 0.0
            return 0.0;
        }
        // Calculate number of seconds elapsed
        let elapsed = self.total_duration_secs.saturating_sub(self.time_remaining_secs);
        // Return ratio of elapsed over total
        (elapsed as f64) / (self.total_duration_secs as f64)
    }

    // Returns a tuple of (minutes, seconds) representing remaining time
    pub fn formatted_time(&self) -> (u32, u32) {
        // Calculate whole minutes remaining
        let mins = self.time_remaining_secs / 60;
        // Calculate remaining seconds modulo 60
        let secs = self.time_remaining_secs % 60;
        // Return minutes and seconds tuple
        (mins, secs)
    }

    // Advances timer by one second and emits event if phase reached zero
    pub fn tick(&mut self, config: &Config) -> Option<TimerEvent> {
        // Only tick if timer is actively running
        if self.status != TimerStatus::Running {
            // Return None if paused or stopped
            return None;
        }

        // Decrement remaining seconds if greater than zero
        if self.time_remaining_secs > 0 {
            // Subtract 1 second
            self.time_remaining_secs -= 1;
        }

        // Check if countdown has reached zero
        if self.time_remaining_secs == 0 {
            // Save finished phase reference
            let finished_phase = self.phase;
            // Advance phase state machine to next step
            let next_phase = self.advance_phase(config);
            // Return phase completion event
            Some(TimerEvent::PhaseCompleted {
                // Completed phase
                finished_phase,
                // Upcoming phase
                next_phase,
            })
        } else {
            // No phase change yet
            None
        }
    }

    // Transition state machine to the next Pomodoro phase based on interval cycles
    pub fn advance_phase(&mut self, config: &Config) -> PomodoroPhase {
        // Calculate the next phase
        let next = match self.phase {
            // If currently in Work phase
            PomodoroPhase::Work => {
                // Increment completed pomodoros count
                self.completed_pomodoros += 1;
                // If cycle reached interval threshold, take a long break
                if self.current_cycle >= config.long_break_interval {
                    // Reset cycle counter back to 1
                    self.current_cycle = 1;
                    // Transition to LongBreak
                    PomodoroPhase::LongBreak
                } else {
                    // Increment cycle counter
                    self.current_cycle += 1;
                    // Transition to ShortBreak
                    PomodoroPhase::ShortBreak
                }
            }
            // If currently in any break phase, transition back to Work
            PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => PomodoroPhase::Work,
        };

        // Update active phase
        self.phase = next;
        // Calculate total seconds for the new phase
        let total = self.target_duration_secs(next, config);
        // Set total duration
        self.total_duration_secs = total;
        // Set time remaining
        self.time_remaining_secs = total;

        // Check auto-start settings for the new phase
        let should_auto_start = match next {
            // Check auto_start_work setting
            PomodoroPhase::Work => config.auto_start_work,
            // Check auto_start_breaks setting
            PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => config.auto_start_breaks,
        };

        // Update timer running status according to auto-start setting
        self.status = if should_auto_start {
            // Start countdown immediately
            TimerStatus::Running
        } else {
            // Wait for user manual start
            TimerStatus::Stopped
        };

        // Return the new phase
        next
    }

    // Skips current phase and transitions directly to the next phase
    pub fn skip_to_next(&mut self, config: &Config) -> PomodoroPhase {
        // Advance to next phase
        self.advance_phase(config)
    }
}

#[cfg(test)]
mod tests {
    // Import super symbols for testing
    use super::*;

    // Test initializing PomodoroTimer with default configuration
    #[test]
    fn test_timer_initialization() {
        // Create default config
        let config = Config::default();
        // Initialize timer
        let timer = PomodoroTimer::new(&config);
        // Verify initial phase is Work
        assert_eq!(timer.phase, PomodoroPhase::Work);
        // Verify initial status is Stopped
        assert_eq!(timer.status, TimerStatus::Stopped);
        // Verify remaining seconds match 25 minutes (1500s)
        assert_eq!(timer.time_remaining_secs, 25 * 60);
    }

    // Test phase transitions and cycle counter
    #[test]
    fn test_phase_advancement() {
        // Create default config with 4 cycles per long break
        let config = Config::default();
        // Initialize timer
        let mut timer = PomodoroTimer::new(&config);

        // Advance 1: Work -> ShortBreak (cycle 1 -> 2)
        let phase = timer.advance_phase(&config);
        // Verify phase is ShortBreak
        assert_eq!(phase, PomodoroPhase::ShortBreak);
        // Verify cycle is 2
        assert_eq!(timer.current_cycle, 2);

        // Advance 2: ShortBreak -> Work
        let phase = timer.advance_phase(&config);
        // Verify phase is Work
        assert_eq!(phase, PomodoroPhase::Work);

        // Advance 3: Work -> ShortBreak (cycle 2 -> 3)
        let _ = timer.advance_phase(&config);
        // Advance 4: ShortBreak -> Work
        let _ = timer.advance_phase(&config);

        // Advance 5: Work -> ShortBreak (cycle 3 -> 4)
        let _ = timer.advance_phase(&config);
        // Advance 6: ShortBreak -> Work
        let _ = timer.advance_phase(&config);

        // Advance 7: Work (at cycle 4) -> LongBreak (resets to cycle 1)
        let phase = timer.advance_phase(&config);
        // Verify phase is LongBreak
        assert_eq!(phase, PomodoroPhase::LongBreak);
        // Verify cycle reset to 1
        assert_eq!(timer.current_cycle, 1);
    }

    // Test progress ratio calculation
    #[test]
    fn test_progress_ratio() {
        // Create default config
        let config = Config::default();
        // Initialize timer
        let mut timer = PomodoroTimer::new(&config);
        // Halfway elapsed
        timer.time_remaining_secs = 750;
        // Total is 1500
        timer.total_duration_secs = 1500;
        // Ratio should be 0.5
        assert!((timer.progress_ratio() - 0.5).abs() < f64::EPSILON);
    }

    // Test pause and reset functions
    #[test]
    fn test_pause_and_reset() {
        // Create default config
        let config = Config::default();
        // Initialize timer
        let mut timer = PomodoroTimer::new(&config);
        // Toggle to running
        timer.toggle();
        // Verify running
        assert_eq!(timer.status, TimerStatus::Running);
        // Pause timer
        timer.pause();
        // Verify paused
        assert_eq!(timer.status, TimerStatus::Paused);
        // Reset timer
        timer.reset(&config);
        // Verify stopped
        assert_eq!(timer.status, TimerStatus::Stopped);
    }
}


