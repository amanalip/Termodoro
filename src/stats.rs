// Import date and time utilities from chrono library
use chrono::{DateTime, Datelike, Local, NaiveDate, Utc};
// Import serde traits for JSON serialization of session history
use serde::{Deserialize, Serialize};
// Import BTreeMap for ordered date key indexing
use std::collections::BTreeMap;
// Import the PomodoroPhase enum from our timer module
use crate::timer::PomodoroPhase;

// Represents an individual completed timer session entry recorded in historical stats
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletedSession {
    // Exact UTC timestamp when the session finished
    pub timestamp: DateTime<Utc>,
    // The phase that was completed (Work, ShortBreak, LongBreak)
    pub phase: PomodoroPhase,
    // Duration in minutes of the completed session
    pub duration_mins: u32,
    // Optional ID of task associated with this session
    pub task_id: Option<String>,
    // Optional title of task associated with this session
    pub task_title: Option<String>,
}

// Storage structure for all historical session records and aggregated productivity analytics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatsHistory {
    // Ordered log of all completed sessions
    pub sessions: Vec<CompletedSession>,
}

impl StatsHistory {
    // Constructor to create an empty StatsHistory container
    pub fn new() -> Self {
        // Return new empty instance
        Self {
            // Empty session list
            sessions: Vec::new(),
        }
    }

    // Appends a new completed session entry into historical stats
    pub fn record(
        &mut self,
        phase: PomodoroPhase,
        duration_mins: u32,
        task_id: Option<String>,
        task_title: Option<String>,
    ) {
        // Create and push new record
        self.sessions.push(CompletedSession {
            // Current timestamp in UTC
            timestamp: Utc::now(),
            // Completed phase
            phase,
            // Duration in minutes
            duration_mins,
            // Associated task ID
            task_id,
            // Associated task title
            task_title,
        });
    }

    // Calculates total count of work pomodoros completed on the current calendar day
    pub fn today_work_sessions(&self) -> usize {
        // Get local calendar date for today
        let today = Local::now().date_naive();
        // Filter sessions by Work phase and matching today's local date
        self.sessions
            .iter()
            // Keep only focus work sessions
            .filter(|s| s.phase == PomodoroPhase::Work)
            // Compare local calendar date with today
            .filter(|s| s.timestamp.with_timezone(&Local).date_naive() == today)
            // Count matching items
            .count()
    }

    // Calculates total focus minutes logged on the current calendar day
    pub fn today_focus_minutes(&self) -> u32 {
        // Get local calendar date for today
        let today = Local::now().date_naive();
        // Filter sessions and sum their duration
        self.sessions
            .iter()
            // Keep only focus work sessions
            .filter(|s| s.phase == PomodoroPhase::Work)
            // Compare local calendar date with today
            .filter(|s| s.timestamp.with_timezone(&Local).date_naive() == today)
            // Extract duration in minutes
            .map(|s| s.duration_mins)
            // Sum all minutes
            .sum()
    }

    // Returns total count of work pomodoros logged across all time
    pub fn total_work_sessions(&self) -> usize {
        // Filter all sessions for Work phase and count
        self.sessions
            .iter()
            // Keep only Work sessions
            .filter(|s| s.phase == PomodoroPhase::Work)
            // Count total
            .count()
    }

    // Returns total focus minutes logged across all time
    pub fn total_focus_minutes(&self) -> u32 {
        // Filter all sessions for Work phase and sum durations
        self.sessions
            .iter()
            // Keep only Work sessions
            .filter(|s| s.phase == PomodoroPhase::Work)
            // Extract duration
            .map(|s| s.duration_mins)
            // Sum durations
            .sum()
    }

    // Helper returning a sorted, deduplicated list of calendar dates where focus work occurred
    pub fn distinct_work_dates(&self) -> Vec<NaiveDate> {
        // Collect dates of all work sessions
        let mut dates: Vec<NaiveDate> = self
            .sessions
            .iter()
            // Filter by Work phase
            .filter(|s| s.phase == PomodoroPhase::Work)
            // Convert timestamp to local naive date
            .map(|s| s.timestamp.with_timezone(&Local).date_naive())
            // Collect into vector
            .collect();
        // Sort dates chronologically
        dates.sort();
        // Remove duplicate dates
        dates.dedup();
        // Return clean date vector
        dates
    }

    // Calculates current active daily streak in consecutive calendar days
    pub fn current_streak_days(&self) -> u32 {
        // Retrieve distinct work dates
        let dates = self.distinct_work_dates();
        // If no work was ever logged, streak is 0
        if dates.is_empty() {
            // Return 0 days
            return 0;
        }

        // Get local calendar date for today
        let today = Local::now().date_naive();
        // Get local calendar date for yesterday
        let yesterday = today.pred_opt().unwrap_or(today);

        // Check the most recent work date
        let last_date = *dates.last().unwrap();
        // If the user did not work today or yesterday, streak is broken
        if last_date != today && last_date != yesterday {
            // Return 0 days
            return 0;
        }

        // Streak accumulator
        let mut streak = 0;
        // Expected preceding date to maintain streak
        let mut current_expected = last_date;

        // Iterate backwards through chronological dates
        for &date in dates.iter().rev() {
            // If date matches expected consecutive day
            if date == current_expected {
                // Increment streak by 1
                streak += 1;
                // Compute previous calendar day
                if let Some(prev) = current_expected.pred_opt() {
                    // Update expected date for next iteration
                    current_expected = prev;
                } else {
                    // Reached earliest possible date
                    break;
                }
            } else if date < current_expected {
                // Gap detected in streak
                break;
            }
        }

        // Return calculated streak
        streak
    }

    // Calculates longest historical daily streak in consecutive calendar days
    pub fn longest_streak_days(&self) -> u32 {
        // Retrieve distinct work dates
        let dates = self.distinct_work_dates();
        // If no dates, streak is 0
        if dates.is_empty() {
            // Return 0 days
            return 0;
        }

        // Track maximum streak observed
        let mut longest = 1;
        // Track current run streak
        let mut current = 1;

        // Iterate over consecutive dates
        for i in 1..dates.len() {
            // Check if current date is the immediate successor of previous date
            if dates[i] == dates[i - 1].succ_opt().unwrap_or(dates[i]) {
                // Increment current run
                current += 1;
                // Update longest if current exceeds it
                if current > longest {
                    // Update max
                    longest = current;
                }
            } else {
                // Streak broken, reset current run to 1
                current = 1;
            }
        }

        // Return longest streak
        longest
    }

    // Generates a list of (formatted_day_string, pomodoro_count) for the past `days` days
    pub fn last_days_distribution(&self, days: usize) -> Vec<(String, u64)> {
        // Get local calendar date for today
        let today = Local::now().date_naive();
        // Map to store counts for each date
        let mut counts_by_date: BTreeMap<NaiveDate, u64> = BTreeMap::new();

        // Pre-populate map with 0 counts for each date in window
        for i in (0..days).rev() {
            // Calculate past date
            if let Some(d) = today.checked_sub_signed(chrono::Duration::days(i as i64)) {
                // Insert default count of 0
                counts_by_date.insert(d, 0);
            }
        }

        // Aggregate actual work session counts into map
        for s in &self.sessions {
            // Filter by Work phase
            if s.phase == PomodoroPhase::Work {
                // Get local date of session
                let d = s.timestamp.with_timezone(&Local).date_naive();
                // If date falls in our window, increment count
                if let Some(entry) = counts_by_date.get_mut(&d) {
                    // Increment session count
                    *entry += 1;
                }
            }
        }

        // Map entries into formatted label and count pairs
        counts_by_date
            .into_iter()
            .map(|(d, count)| {
                // Format weekday abbreviation
                let day_str = match d.weekday() {
                    // Monday
                    chrono::Weekday::Mon => "Mon",
                    // Tuesday
                    chrono::Weekday::Tue => "Tue",
                    // Wednesday
                    chrono::Weekday::Wed => "Wed",
                    // Thursday
                    chrono::Weekday::Thu => "Thu",
                    // Friday
                    chrono::Weekday::Fri => "Fri",
                    // Saturday
                    chrono::Weekday::Sat => "Sat",
                    // Sunday
                    chrono::Weekday::Sun => "Sun",
                };
                // Format label as "Mon 17"
                (format!("{} {:02}", day_str, d.day()), count)
            })
            // Collect into vector for chart rendering
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Import super module items
    use super::*;

    // Test recording sessions and calculating daily totals
    #[test]
    fn test_stats_recording() {
        // Initialize empty stats history
        let mut stats = StatsHistory::new();
        // Record work session 1 (25 mins)
        stats.record(PomodoroPhase::Work, 25, Some("task-1".to_string()), Some("Task 1".to_string()));
        // Record short break (5 mins)
        stats.record(PomodoroPhase::ShortBreak, 5, None, None);
        // Record work session 2 (25 mins)
        stats.record(PomodoroPhase::Work, 25, Some("task-1".to_string()), Some("Task 1".to_string()));

        // Verify today work sessions count is 2
        assert_eq!(stats.today_work_sessions(), 2);
        // Verify today focus minutes is 50
        assert_eq!(stats.today_focus_minutes(), 50);
        // Verify total work sessions is 2
        assert_eq!(stats.total_work_sessions(), 2);
        // Verify total focus minutes is 50
        assert_eq!(stats.total_focus_minutes(), 50);
        // Verify current streak is 1 day
        assert_eq!(stats.current_streak_days(), 1);
    }

    #[test]
    fn test_empty_stats_streak() {
        let stats = StatsHistory::new();
        assert_eq!(stats.current_streak_days(), 0);
        assert_eq!(stats.longest_streak_days(), 0);
        assert_eq!(stats.today_work_sessions(), 0);
        assert_eq!(stats.today_focus_minutes(), 0);
        assert_eq!(stats.total_work_sessions(), 0);
        assert_eq!(stats.total_focus_minutes(), 0);
    }

    #[test]
    fn test_break_sessions_do_not_count_towards_work_stats() {
        let mut stats = StatsHistory::new();
        stats.record(PomodoroPhase::ShortBreak, 5, None, None);
        stats.record(PomodoroPhase::LongBreak, 15, None, None);

        assert_eq!(stats.today_work_sessions(), 0);
        assert_eq!(stats.today_focus_minutes(), 0);
        assert_eq!(stats.total_work_sessions(), 0);
        assert_eq!(stats.total_focus_minutes(), 0);
        assert_eq!(stats.current_streak_days(), 0);
        assert_eq!(stats.longest_streak_days(), 0);
    }

    #[test]
    fn test_streak_yesterday_preserved() {
        let mut stats = StatsHistory::new();
        let yesterday_utc = (Utc::now() - chrono::Duration::days(1)).date_naive().and_hms_opt(12, 0, 0).unwrap().and_utc();
        stats.sessions.push(CompletedSession {
            timestamp: yesterday_utc,
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });

        // Even though no sessions today, streak from yesterday should be 1
        assert_eq!(stats.current_streak_days(), 1);
        assert_eq!(stats.longest_streak_days(), 1);
    }

    #[test]
    fn test_streak_broken_two_days_ago() {
        let mut stats = StatsHistory::new();
        let two_days_ago_utc = (Utc::now() - chrono::Duration::days(2)).date_naive().and_hms_opt(12, 0, 0).unwrap().and_utc();
        stats.sessions.push(CompletedSession {
            timestamp: two_days_ago_utc,
            phase: PomodoroPhase::Work,
            duration_mins: 25,
            task_id: None,
            task_title: None,
        });

        assert_eq!(stats.current_streak_days(), 0);
        // But longest streak historically is still 1
        assert_eq!(stats.longest_streak_days(), 1);
    }

    #[test]
    fn test_consecutive_multi_day_streaks_and_longest() {
        let mut stats = StatsHistory::new();
        let today = Local::now().date_naive();

        // Simulate 4-day streak ending 5 days ago: today-8, today-7, today-6, today-5
        for offset in (5..=8).rev() {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(10, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        // Gap on today-4, today-3

        // Simulate 3-day current streak: today-2, today-1, today
        for offset in (0..=2).rev() {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(10, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        // Current active streak is 3 days
        assert_eq!(stats.current_streak_days(), 3);
        // Longest streak was the 4-day streak
        assert_eq!(stats.longest_streak_days(), 4);
    }

    #[test]
    fn test_same_day_multiple_sessions_dedup() {
        let mut stats = StatsHistory::new();
        // Record 5 work sessions today
        for _ in 0..5 {
            stats.record(PomodoroPhase::Work, 25, None, None);
        }

        assert_eq!(stats.today_work_sessions(), 5);
        assert_eq!(stats.today_focus_minutes(), 125);
        // Streak is still 1 day, not 5 days
        assert_eq!(stats.current_streak_days(), 1);
        assert_eq!(stats.longest_streak_days(), 1);
        assert_eq!(stats.distinct_work_dates().len(), 1);
    }

    #[test]
    fn test_last_days_distribution() {
        let mut stats = StatsHistory::new();
        // Record 2 sessions today
        stats.record(PomodoroPhase::Work, 25, None, None);
        stats.record(PomodoroPhase::Work, 25, None, None);

        let dist = stats.last_days_distribution(7);
        assert_eq!(dist.len(), 7);
        // Last item corresponds to today
        let (today_label, count) = &dist[6];
        assert_eq!(*count, 2);
        assert!(!today_label.is_empty());
    }

    #[test]
    fn test_multi_day_streak_yesterday_continuation() {
        let mut stats = StatsHistory::new();
        let today = Local::now().date_naive();

        // 4 consecutive days of work ending yesterday (offset 1, 2, 3, 4) - today has 0 sessions
        for offset in 1..=4 {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(14, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        // Streak from yesterday should be 4 days
        assert_eq!(stats.current_streak_days(), 4);
        assert_eq!(stats.longest_streak_days(), 4);
        assert_eq!(stats.today_work_sessions(), 0);
    }

    #[test]
    fn test_out_of_order_session_timestamps() {
        let mut stats = StatsHistory::new();
        let today = Local::now().date_naive();

        // Insert sessions in reverse/shuffled chronological order
        let offsets = [0, 3, 1, 2];
        for &offset in &offsets {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(9, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 30,
                task_id: None,
                task_title: None,
            });
        }

        // Distinct dates should be 4 and sorted properly
        let distinct = stats.distinct_work_dates();
        assert_eq!(distinct.len(), 4);
        assert!(distinct.windows(2).all(|w| w[0] < w[1]));

        // Continuous 4-day streak
        assert_eq!(stats.current_streak_days(), 4);
        assert_eq!(stats.longest_streak_days(), 4);
        assert_eq!(stats.total_focus_minutes(), 120);
    }

    #[test]
    fn test_distribution_variable_day_windows() {
        let mut stats = StatsHistory::new();
        stats.record(PomodoroPhase::Work, 25, None, None);

        // 0 days window
        let dist0 = stats.last_days_distribution(0);
        assert_eq!(dist0.len(), 0);

        // 1 day window
        let dist1 = stats.last_days_distribution(1);
        assert_eq!(dist1.len(), 1);
        assert_eq!(dist1[0].1, 1);

        // 14 days window
        let dist14 = stats.last_days_distribution(14);
        assert_eq!(dist14.len(), 14);
        assert_eq!(dist14[13].1, 1); // today is last index

        // 30 days window
        let dist30 = stats.last_days_distribution(30);
        assert_eq!(dist30.len(), 30);
    }

    #[test]
    fn test_complex_multi_streak_history() {
        let mut stats = StatsHistory::new();
        let today = Local::now().date_naive();

        // Streak 1: 3-day streak from today-20 to today-18
        for offset in 18..=20 {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        // Streak 2: 6-day streak from today-15 to today-10
        for offset in 10..=15 {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        // Streak 3: 2-day current streak: today-1 and today
        for offset in 0..=1 {
            let date = today - chrono::Duration::days(offset);
            let dt = date.and_hms_opt(12, 0, 0).unwrap().and_local_timezone(Local).unwrap().with_timezone(&Utc);
            stats.sessions.push(CompletedSession {
                timestamp: dt,
                phase: PomodoroPhase::Work,
                duration_mins: 25,
                task_id: None,
                task_title: None,
            });
        }

        assert_eq!(stats.current_streak_days(), 2);
        assert_eq!(stats.longest_streak_days(), 6);
        assert_eq!(stats.total_work_sessions(), 11);
    }
}



