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
        // Calculate initial total seconds for work phase.
        // saturating_mul defends against absurd config values (a u32 minute
        // count above ~35.7 million would otherwise overflow and wrap to a
        // garbage duration); config sanitization already clamps inputs, this
        // is defense in depth.
        let total = config.work_duration_mins.saturating_mul(60);
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
        // Match on the requested phase; saturating_mul prevents overflow wrap
        match phase {
            // Work duration in minutes converted to seconds
            PomodoroPhase::Work => config.work_duration_mins.saturating_mul(60),
            // Short break duration in minutes converted to seconds
            PomodoroPhase::ShortBreak => config.short_break_mins.saturating_mul(60),
            // Long break duration in minutes converted to seconds
            PomodoroPhase::LongBreak => config.long_break_mins.saturating_mul(60),
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
        let elapsed = self
            .total_duration_secs
            .saturating_sub(self.time_remaining_secs);
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

        // Update timer running status according to auto-start setting.
        // A zero-length phase must never auto-start: doing so previously
        // produced a per-second completion churn (each tick instantly
        // "finished" the phase again, flooding stats, notifications, and disk
        // writes). Config sanitization makes this unreachable via data.json,
        // but direct struct construction can still produce it.
        self.status = if should_auto_start && total > 0 {
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

        // Total 0 edge case
        timer.total_duration_secs = 0;
        assert_eq!(timer.progress_ratio(), 0.0);
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

    #[test]
    fn test_formatted_time() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.time_remaining_secs = 1500;
        assert_eq!(timer.formatted_time(), (25, 0));

        timer.time_remaining_secs = 65;
        assert_eq!(timer.formatted_time(), (1, 5));

        timer.time_remaining_secs = 0;
        assert_eq!(timer.formatted_time(), (0, 0));
    }

    #[test]
    fn test_tick_when_running_and_completion_event() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.status = TimerStatus::Running;
        timer.time_remaining_secs = 2;

        // First tick: 2 -> 1, no event
        let event1 = timer.tick(&config);
        assert_eq!(event1, None);
        assert_eq!(timer.time_remaining_secs, 1);

        // Second tick: 1 -> 0, triggers PhaseCompleted event
        let event2 = timer.tick(&config);
        assert_eq!(
            event2,
            Some(TimerEvent::PhaseCompleted {
                finished_phase: PomodoroPhase::Work,
                next_phase: PomodoroPhase::ShortBreak,
            })
        );
        // Should have transitioned to ShortBreak (5 minutes = 300s)
        assert_eq!(timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(timer.time_remaining_secs, 5 * 60);
        // Default auto_start_breaks is false, so status is Stopped
        assert_eq!(timer.status, TimerStatus::Stopped);
    }

    #[test]
    fn test_tick_when_paused_or_stopped_does_nothing() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.status = TimerStatus::Paused;
        timer.time_remaining_secs = 100;
        assert_eq!(timer.tick(&config), None);
        assert_eq!(timer.time_remaining_secs, 100);

        timer.status = TimerStatus::Stopped;
        assert_eq!(timer.tick(&config), None);
        assert_eq!(timer.time_remaining_secs, 100);
    }

    #[test]
    fn test_auto_start_settings_on_transition() {
        let config = Config {
            auto_start_breaks: true,
            auto_start_work: false,
            ..Default::default()
        };

        let mut timer = PomodoroTimer::new(&config);
        // Work -> ShortBreak: auto_start_breaks is true, so should be Running
        timer.advance_phase(&config);
        assert_eq!(timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(timer.status, TimerStatus::Running);

        // ShortBreak -> Work: auto_start_work is false, so should be Stopped
        timer.advance_phase(&config);
        assert_eq!(timer.phase, PomodoroPhase::Work);
        assert_eq!(timer.status, TimerStatus::Stopped);
    }

    #[test]
    fn test_skip_to_next() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        let next = timer.skip_to_next(&config);
        assert_eq!(next, PomodoroPhase::ShortBreak);
        assert_eq!(timer.phase, PomodoroPhase::ShortBreak);
        assert_eq!(timer.time_remaining_secs, 5 * 60);
    }

    #[test]
    fn test_phase_titles_and_emojis() {
        assert_eq!(PomodoroPhase::Work.title(), "FOCUS SESSION");
        assert_eq!(PomodoroPhase::Work.emoji(), "🍅");
        assert_eq!(PomodoroPhase::ShortBreak.title(), "SHORT BREAK");
        assert_eq!(PomodoroPhase::ShortBreak.emoji(), "☕");
        assert_eq!(PomodoroPhase::LongBreak.title(), "LONG BREAK");
        assert_eq!(PomodoroPhase::LongBreak.emoji(), "🌴");
    }

    #[test]
    fn test_target_duration_secs_all_phases() {
        let config = Config {
            work_duration_mins: 40,
            short_break_mins: 8,
            long_break_mins: 20,
            ..Config::default()
        };
        let timer = PomodoroTimer::new(&config);
        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::Work, &config),
            40 * 60
        );
        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::ShortBreak, &config),
            8 * 60
        );
        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::LongBreak, &config),
            20 * 60
        );
    }

    #[test]
    fn test_timer_toggle_transitions() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);

        // Stopped -> Running
        assert_eq!(timer.status, TimerStatus::Stopped);
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Running);

        // Running -> Paused
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Paused);

        // Paused -> Running
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Running);

        // Running -> Paused via pause()
        timer.pause();
        assert_eq!(timer.status, TimerStatus::Paused);
        // Calling pause() again while already paused keeps it paused
        timer.pause();
        assert_eq!(timer.status, TimerStatus::Paused);
    }

    #[test]
    fn test_twenty_four_cycle_advancement_and_long_break_trigger() {
        let config = Config {
            long_break_interval: 24,
            ..Config::default()
        };
        let mut timer = PomodoroTimer::new(&config);

        for cycle in 1..24 {
            assert_eq!(timer.current_cycle, cycle);
            assert_eq!(timer.phase, PomodoroPhase::Work);
            // Work -> ShortBreak
            let next = timer.advance_phase(&config);
            assert_eq!(next, PomodoroPhase::ShortBreak);
            assert_eq!(timer.current_cycle, cycle + 1);

            // ShortBreak -> Work
            let next = timer.advance_phase(&config);
            assert_eq!(next, PomodoroPhase::Work);
        }

        // At cycle 24 Work
        assert_eq!(timer.current_cycle, 24);
        assert_eq!(timer.phase, PomodoroPhase::Work);
        // Advancing from 24th Work phase triggers LongBreak and resets cycle to 1
        let next = timer.advance_phase(&config);
        assert_eq!(next, PomodoroPhase::LongBreak);
        assert_eq!(timer.current_cycle, 1);
        assert_eq!(timer.completed_pomodoros, 24);

        // Advancing from LongBreak goes back to Work
        let next = timer.advance_phase(&config);
        assert_eq!(next, PomodoroPhase::Work);
        assert_eq!(timer.current_cycle, 1);
    }

    #[test]
    fn test_formatted_time_large_values() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        // 120 minutes
        timer.time_remaining_secs = 120 * 60;
        assert_eq!(timer.formatted_time(), (120, 0));

        // 90 minutes 45 seconds
        timer.time_remaining_secs = 90 * 60 + 45;
        assert_eq!(timer.formatted_time(), (90, 45));

        // 1 second
        timer.time_remaining_secs = 1;
        assert_eq!(timer.formatted_time(), (0, 1));
    }

    #[test]
    fn test_timer_zero_total_duration_progress_ratio() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.total_duration_secs = 0;
        timer.time_remaining_secs = 0;
        assert_eq!(timer.progress_ratio(), 0.0);
    }

    #[test]
    fn test_timer_multiple_consecutive_skips() {
        let config = Config {
            long_break_interval: 4,
            ..Default::default()
        };
        let mut timer = PomodoroTimer::new(&config);

        // Skip 50 times in a row
        for _ in 0..50 {
            timer.skip_to_next(&config);
            assert!(timer.current_cycle >= 1 && timer.current_cycle <= 4);
            assert!(timer.time_remaining_secs > 0);
            assert_eq!(timer.time_remaining_secs, timer.total_duration_secs);
        }
    }

    #[test]
    fn test_timer_reset_across_all_phases() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);

        // Work reset
        timer.status = TimerStatus::Running;
        timer.time_remaining_secs = 10;
        timer.reset(&config);
        assert_eq!(timer.status, TimerStatus::Stopped);
        assert_eq!(timer.time_remaining_secs, config.work_duration_mins * 60);

        // ShortBreak reset
        timer.phase = PomodoroPhase::ShortBreak;
        timer.status = TimerStatus::Paused;
        timer.time_remaining_secs = 5;
        timer.reset(&config);
        assert_eq!(timer.status, TimerStatus::Stopped);
        assert_eq!(timer.time_remaining_secs, config.short_break_mins * 60);

        // LongBreak reset
        timer.phase = PomodoroPhase::LongBreak;
        timer.status = TimerStatus::Running;
        timer.time_remaining_secs = 20;
        timer.reset(&config);
        assert_eq!(timer.status, TimerStatus::Stopped);
        assert_eq!(timer.time_remaining_secs, config.long_break_mins * 60);
    }

    #[test]
    fn test_timer_serde_roundtrip() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.completed_pomodoros = 7;
        timer.current_cycle = 3;
        timer.status = TimerStatus::Paused;

        let json = serde_json::to_string(&timer).unwrap();
        let restored: PomodoroTimer = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.completed_pomodoros, 7);
        assert_eq!(restored.current_cycle, 3);
        assert_eq!(restored.status, TimerStatus::Paused);
    }

    #[test]
    fn test_timer_rapid_status_flipping_under_tick_loop() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        let start_time = timer.time_remaining_secs;

        // 1. Start and tick 5 times
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Running);
        for _ in 0..5 {
            timer.tick(&config);
        }
        assert_eq!(timer.time_remaining_secs, start_time - 5);

        // 2. Pause and tick 5 times (time remaining must NOT change)
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Paused);
        for _ in 0..5 {
            timer.tick(&config);
        }
        assert_eq!(timer.time_remaining_secs, start_time - 5);

        // 3. Resume and tick 3 times
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Running);
        for _ in 0..3 {
            timer.tick(&config);
        }
        assert_eq!(timer.time_remaining_secs, start_time - 8);
    }

    #[test]
    fn test_timer_exact_phase_transition_cycle_counting() {
        for interval in [2, 3, 6, 8] {
            let config = Config {
                long_break_interval: interval,
                auto_start_breaks: true,
                auto_start_work: true,
                ..Default::default()
            };
            let mut timer = PomodoroTimer::new(&config);

            for c in 1..=interval {
                assert_eq!(timer.current_cycle, c);
                assert_eq!(timer.phase, PomodoroPhase::Work);
                timer.time_remaining_secs = 1;
                timer.status = TimerStatus::Running;
                timer.tick(&config);

                if c < interval {
                    assert_eq!(timer.phase, PomodoroPhase::ShortBreak);
                    assert_eq!(timer.current_cycle, c + 1);
                    timer.time_remaining_secs = 1;
                    timer.status = TimerStatus::Running;
                    timer.tick(&config);
                    assert_eq!(timer.phase, PomodoroPhase::Work);
                } else {
                    assert_eq!(timer.phase, PomodoroPhase::LongBreak);
                    assert_eq!(timer.current_cycle, 1);
                }
            }
        }
    }

    #[test]
    fn test_timer_time_remaining_never_underflows_sub_second_ticks() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);
        timer.time_remaining_secs = 0;
        timer.status = TimerStatus::Paused;

        // Ticking when paused at 0 must not underflow u32
        timer.tick(&config);
        assert_eq!(timer.time_remaining_secs, 0);
    }

    #[test]
    fn test_timer_formatted_time_zero_and_single_digits() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);

        timer.time_remaining_secs = 0;
        assert_eq!(timer.formatted_time(), (0, 0));

        timer.time_remaining_secs = 7;
        assert_eq!(timer.formatted_time(), (0, 7));

        timer.time_remaining_secs = 65;
        assert_eq!(timer.formatted_time(), (1, 5));
    }

    #[test]
    fn test_timer_status_transitions_and_predicates() {
        let config = Config::default();
        let mut timer = PomodoroTimer::new(&config);

        assert_eq!(timer.status, TimerStatus::Stopped);
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Running);
        timer.toggle();
        assert_eq!(timer.status, TimerStatus::Paused);
        timer.reset(&config);
        assert_eq!(timer.status, TimerStatus::Stopped);
    }

    #[test]
    fn test_timer_target_duration_all_phases_with_custom_config() {
        let config = Config {
            work_duration_mins: 90,
            short_break_mins: 20,
            long_break_mins: 45,
            long_break_interval: 12,
            ..Default::default()
        };
        let timer = PomodoroTimer::new(&config);

        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::Work, &config),
            90 * 60
        );
        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::ShortBreak, &config),
            20 * 60
        );
        assert_eq!(
            timer.target_duration_secs(PomodoroPhase::LongBreak, &config),
            45 * 60
        );
    }

    #[test]
    fn test_timer_progress_ratio_bounds_and_rounding() {
        let config = Config {
            work_duration_mins: 10,
            ..Default::default()
        };
        let mut timer = PomodoroTimer::new(&config);
        // Total is 600s
        assert_eq!(timer.total_duration_secs, 600);

        timer.time_remaining_secs = 600;
        assert!((timer.progress_ratio() - 0.0).abs() < f64::EPSILON);

        timer.time_remaining_secs = 300;
        assert!((timer.progress_ratio() - 0.5).abs() < f64::EPSILON);

        timer.time_remaining_secs = 0;
        assert!((timer.progress_ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_timer_long_break_to_work_cycle_reset() {
        let config = Config {
            long_break_interval: 3,
            auto_start_work: false,
            ..Default::default()
        };
        let mut timer = PomodoroTimer::new(&config);
        timer.phase = PomodoroPhase::LongBreak;
        timer.current_cycle = 1; // Cycle was reset to 1 when entering LongBreak
        timer.time_remaining_secs = 1;
        timer.status = TimerStatus::Running;

        let event = timer.tick(&config);
        assert_eq!(
            event,
            Some(TimerEvent::PhaseCompleted {
                finished_phase: PomodoroPhase::LongBreak,
                next_phase: PomodoroPhase::Work,
            })
        );
        assert_eq!(timer.phase, PomodoroPhase::Work);
        assert_eq!(timer.current_cycle, 1);
        assert_eq!(timer.status, TimerStatus::Stopped);
    }

    #[test]
    fn test_timer_phase_title_and_emoji_completeness() {
        assert_eq!(PomodoroPhase::Work.title(), "FOCUS SESSION");
        assert_eq!(PomodoroPhase::ShortBreak.title(), "SHORT BREAK");
        assert_eq!(PomodoroPhase::LongBreak.title(), "LONG BREAK");

        assert_eq!(PomodoroPhase::Work.emoji(), "🍅");
        assert_eq!(PomodoroPhase::ShortBreak.emoji(), "☕");
        assert_eq!(PomodoroPhase::LongBreak.emoji(), "🌴");
    }
}
