use crate::config::BlinkConfig;
use crate::idle::IdleDetector;
use serde::Serialize;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TimerStatus {
    Running,
    PausedIdle,
    PausedManual,
    OnBreak,
    Snoozed,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimerInfo {
    pub remaining_seconds: u32,
    pub formatted_time: String,
    pub state: String,
    pub is_paused: bool,
}

struct InternalState {
    status: TimerStatus,
    remaining_seconds: u32,
    break_remaining_seconds: u32,
    last_wall_time: SystemTime,
    last_tick_instant: Instant,
}

pub struct TimerEngine {
    state: Arc<RwLock<InternalState>>,
}

impl TimerEngine {
    pub fn new(initial_work_minutes: u32) -> Self {
        let total_secs = initial_work_minutes.max(1) * 60;
        Self {
            state: Arc::new(RwLock::new(InternalState {
                status: TimerStatus::Running,
                remaining_seconds: total_secs,
                break_remaining_seconds: 0,
                last_wall_time: SystemTime::now(),
                last_tick_instant: Instant::now(),
            })),
        }
    }

    pub fn get_info(&self) -> TimerInfo {
        let state = self.state.read().unwrap();
        let formatted = format_time(state.remaining_seconds);
        let state_str = match state.status {
            TimerStatus::Running => "Running",
            TimerStatus::PausedIdle => "PausedIdle",
            TimerStatus::PausedManual => "PausedManual",
            TimerStatus::OnBreak => "OnBreak",
            TimerStatus::Snoozed => "Snoozed",
        };
        let is_paused = state.status == TimerStatus::PausedManual || state.status == TimerStatus::PausedIdle;

        TimerInfo {
            remaining_seconds: state.remaining_seconds,
            formatted_time: formatted,
            state: state_str.to_string(),
            is_paused,
        }
    }

    pub fn pause(&self) {
        if let Ok(mut state) = self.state.write() {
            if state.status == TimerStatus::Running || state.status == TimerStatus::Snoozed {
                state.status = TimerStatus::PausedManual;
            }
        }
    }

    pub fn resume(&self) {
        if let Ok(mut state) = self.state.write() {
            if state.status == TimerStatus::PausedManual || state.status == TimerStatus::PausedIdle {
                state.status = TimerStatus::Running;
                state.last_tick_instant = Instant::now();
                state.last_wall_time = SystemTime::now();
            }
        }
    }

    pub fn reset(&self, work_minutes: u32) {
        if let Ok(mut state) = self.state.write() {
            state.remaining_seconds = work_minutes.max(1) * 60;
            state.break_remaining_seconds = 0;
            state.status = TimerStatus::Running;
            state.last_tick_instant = Instant::now();
            state.last_wall_time = SystemTime::now();
        }
    }

    pub fn snooze(&self, snooze_minutes: u32) {
        if let Ok(mut state) = self.state.write() {
            state.remaining_seconds = snooze_minutes.max(1) * 60;
            state.break_remaining_seconds = 0;
            state.status = TimerStatus::Snoozed;
            state.last_tick_instant = Instant::now();
            state.last_wall_time = SystemTime::now();
        }
    }

    pub fn adjust_work_duration(&self, old_work_minutes: u32, new_work_minutes: u32) {
        if old_work_minutes == new_work_minutes {
            return;
        }
        if let Ok(mut state) = self.state.write() {
            if state.status == TimerStatus::Running
                || state.status == TimerStatus::PausedManual
                || state.status == TimerStatus::PausedIdle
            {
                let old_total_secs = old_work_minutes.max(1) * 60;
                let new_total_secs = new_work_minutes.max(1) * 60;
                let elapsed = old_total_secs.saturating_sub(state.remaining_seconds);
                state.remaining_seconds = new_total_secs.saturating_sub(elapsed).max(1);
            }
        }
    }

    /// Ticks the timer by 1 second. Returns `true` if a break notification should trigger!
    pub fn tick(&self, config: &BlinkConfig) -> bool {
        let mut state = match self.state.write() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let now_wall = SystemTime::now();
        let now_instant = Instant::now();

        // 1. Sleep/Hibernate delta check:
        // If the system woke from sleep and time jumped forward more than work duration,
        // treat break as taken and reset.
        if let Ok(elapsed_wall) = now_wall.duration_since(state.last_wall_time) {
            let work_duration_secs = (config.work_duration_minutes as u64) * 60;
            if elapsed_wall > Duration::from_secs(work_duration_secs) && elapsed_wall > Duration::from_secs(300) {
                state.remaining_seconds = config.work_duration_minutes * 60;
                state.status = TimerStatus::Running;
                state.last_wall_time = now_wall;
                state.last_tick_instant = now_instant;
                return false;
            }
        }
        state.last_wall_time = now_wall;
        state.last_tick_instant = now_instant;

        // 2. Idle Detection check
        if config.idle_detection_enabled {
            let idle_secs = IdleDetector::get_idle_seconds();
            if idle_secs >= (config.idle_threshold_seconds as u64) {
                if state.status == TimerStatus::Running || state.status == TimerStatus::Snoozed {
                    state.status = TimerStatus::PausedIdle;
                }
                return false;
            } else if state.status == TimerStatus::PausedIdle {
                // Resumed user activity!
                state.status = TimerStatus::Running;
            }
        }

        // 3. Status handling
        match state.status {
            TimerStatus::PausedManual | TimerStatus::PausedIdle => {
                // Do not tick
                false
            }
            TimerStatus::OnBreak => {
                if state.break_remaining_seconds > 0 {
                    state.break_remaining_seconds -= 1;
                }
                if state.break_remaining_seconds == 0 {
                    // Break finished! Start next work interval
                    state.remaining_seconds = config.work_duration_minutes * 60;
                    state.status = TimerStatus::Running;
                }
                false
            }
            TimerStatus::Running | TimerStatus::Snoozed => {
                if state.remaining_seconds > 0 {
                    state.remaining_seconds -= 1;
                }

                if state.remaining_seconds == 0 {
                    // Trigger break!
                    state.status = TimerStatus::OnBreak;
                    state.break_remaining_seconds = config.break_duration_seconds;
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn format_time(total_seconds: u32) -> String {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(1200), "20:00");
        assert_eq!(format_time(65), "01:05");
        assert_eq!(format_time(0), "00:00");
    }

    #[test]
    fn test_timer_countdown_and_trigger() {
        let engine = TimerEngine::new(1); // 60 seconds
        let config = BlinkConfig {
            work_duration_minutes: 1,
            break_duration_seconds: 5,
            idle_detection_enabled: false,
            ..Default::default()
        };

        // Initially 60s
        let info = engine.get_info();
        assert_eq!(info.remaining_seconds, 60);
        assert_eq!(info.state, "Running");

        // Fast forward 59 ticks
        for _ in 0..59 {
            let trigger = engine.tick(&config);
            assert!(!trigger);
        }

        let info = engine.get_info();
        assert_eq!(info.remaining_seconds, 1);

        // 60th tick triggers break
        let trigger = engine.tick(&config);
        assert!(trigger);

        let info = engine.get_info();
        assert_eq!(info.state, "OnBreak");
    }

    #[test]
    fn test_pause_and_resume() {
        let engine = TimerEngine::new(20);
        let config = BlinkConfig {
            idle_detection_enabled: false,
            ..Default::default()
        };

        engine.pause();
        assert!(engine.get_info().is_paused);

        // Tick while paused should not decrement
        let trigger = engine.tick(&config);
        assert!(!trigger);
        assert_eq!(engine.get_info().remaining_seconds, 1200);

        engine.resume();
        assert!(!engine.get_info().is_paused);
    }

    #[test]
    fn test_snooze() {
        let engine = TimerEngine::new(20);
        engine.snooze(3); // 3 minutes = 180s
        let info = engine.get_info();
        assert_eq!(info.remaining_seconds, 180);
        assert_eq!(info.state, "Snoozed");
    }

    #[test]
    fn test_adjust_work_duration() {
        let engine = TimerEngine::new(20);
        // Simulate working for 5 minutes (300s worked, 900s remaining)
        let config = BlinkConfig {
            work_duration_minutes: 20,
            idle_detection_enabled: false,
            ..Default::default()
        };
        for _ in 0..300 {
            engine.tick(&config);
        }
        assert_eq!(engine.get_info().remaining_seconds, 900);

        // Saving other settings (work duration same: 20 -> 20)
        engine.adjust_work_duration(20, 20);
        assert_eq!(engine.get_info().remaining_seconds, 900);

        // Changing work duration 20 -> 30 (300s worked, should have 1500s remaining = 25m)
        engine.adjust_work_duration(20, 30);
        assert_eq!(engine.get_info().remaining_seconds, 1500);

        // Changing work duration 30 -> 10 (300s worked, should have 300s remaining = 5m)
        engine.adjust_work_duration(30, 10);
        assert_eq!(engine.get_info().remaining_seconds, 300);
    }
}
