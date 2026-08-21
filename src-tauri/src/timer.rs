use crate::config::{BlinkConfig, TimerMode};
use crate::idle::IdleDetector;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerStatus {
    Running,
    PausedIdle,
    PausedManual,
    PausedQuietHours,
    OnBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerInfo {
    pub remaining_seconds: u32,
    pub break_remaining_seconds: u32,
    pub formatted_time: String,
    pub state: String,
    pub is_paused: bool,
    pub timer_mode: String,
    pub current_cycle: u32,
    pub is_long_break: bool,
}

#[derive(Debug)]
struct InternalState {
    status: TimerStatus,
    remaining_seconds: u32,
    break_remaining_seconds: u32,
    current_cycle: u32,
    is_long_break: bool,
    last_tick_time: Instant,
}

pub struct TimerEngine {
    state: Arc<RwLock<InternalState>>,
}

impl TimerEngine {
    pub fn new(work_duration_minutes: u32) -> Self {
        let initial_seconds = work_duration_minutes * 60;
        Self {
            state: Arc::new(RwLock::new(InternalState {
                status: TimerStatus::Running,
                remaining_seconds: initial_seconds,
                break_remaining_seconds: 0,
                current_cycle: 1,
                is_long_break: false,
                last_tick_time: Instant::now(),
            })),
        }
    }

    pub fn get_info(&self) -> TimerInfo {
        if let Ok(state) = self.state.read() {
            let (formatted, state_str, is_paused) = match state.status {
                TimerStatus::Running => (Self::format_time(state.remaining_seconds), "Running", false),
                TimerStatus::PausedIdle => (Self::format_time(state.remaining_seconds), "PausedIdle", true),
                TimerStatus::PausedManual => (Self::format_time(state.remaining_seconds), "PausedManual", true),
                TimerStatus::PausedQuietHours => (Self::format_time(state.remaining_seconds), "PausedQuietHours", true),
                TimerStatus::OnBreak => (Self::format_time(state.break_remaining_seconds), "OnBreak", false),
            };

            let mode_str = if state.is_long_break {
                "PomodoroLongBreak".to_string()
            } else {
                "Standard".to_string()
            };

            TimerInfo {
                remaining_seconds: state.remaining_seconds,
                break_remaining_seconds: state.break_remaining_seconds,
                formatted_time: formatted,
                state: state_str.to_string(),
                is_paused,
                timer_mode: mode_str,
                current_cycle: state.current_cycle,
                is_long_break: state.is_long_break,
            }
        } else {
            TimerInfo {
                remaining_seconds: 0,
                break_remaining_seconds: 0,
                formatted_time: "--:--".to_string(),
                state: "Unknown".to_string(),
                is_paused: false,
                timer_mode: "Standard".to_string(),
                current_cycle: 1,
                is_long_break: false,
            }
        }
    }

    pub fn pause(&self) {
        if let Ok(mut state) = self.state.write() {
            if state.status == TimerStatus::Running {
                state.status = TimerStatus::PausedManual;
            }
        }
    }

    pub fn resume(&self) {
        if let Ok(mut state) = self.state.write() {
            if state.status == TimerStatus::PausedManual
                || state.status == TimerStatus::PausedIdle
                || state.status == TimerStatus::PausedQuietHours
            {
                state.status = TimerStatus::Running;
                state.last_tick_time = Instant::now();
            }
        }
    }

    pub fn reset(&self, work_duration_minutes: u32) {
        if let Ok(mut state) = self.state.write() {
            state.remaining_seconds = work_duration_minutes * 60;
            state.break_remaining_seconds = 0;
            state.status = TimerStatus::Running;
            state.last_tick_time = Instant::now();
        }
    }

    pub fn snooze(&self, snooze_duration_minutes: u32) {
        if let Ok(mut state) = self.state.write() {
            state.remaining_seconds = snooze_duration_minutes * 60;
            state.break_remaining_seconds = 0;
            state.status = TimerStatus::Running;
            state.last_tick_time = Instant::now();
        }
    }

    pub fn adjust_work_duration(&self, old_duration_minutes: u32, new_duration_minutes: u32) {
        if let Ok(mut state) = self.state.write() {
            if state.status != TimerStatus::OnBreak {
                let old_total_seconds = old_duration_minutes * 60;
                let new_total_seconds = new_duration_minutes * 60;

                let elapsed_seconds = old_total_seconds.saturating_sub(state.remaining_seconds);
                if new_total_seconds > elapsed_seconds {
                    state.remaining_seconds = new_total_seconds - elapsed_seconds;
                } else {
                    state.remaining_seconds = 1;
                }
            }
        }
    }

    pub fn tick(&self, config: &BlinkConfig) -> bool {
        let mut state = match self.state.write() {
            Ok(s) => s,
            Err(_) => return false,
        };

        // System Sleep / Wakeup compensation
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_tick_time).as_secs();
        state.last_tick_time = now;

        if elapsed > 10 {
            println!("[Blink Timer] System resumed from sleep/hibernate ({}s elapsed). Resetting interval.", elapsed);
            let total_work = if config.timer_mode == TimerMode::Pomodoro {
                config.pomodoro_work_minutes * 60
            } else {
                config.work_duration_minutes * 60
            };
            state.remaining_seconds = total_work;
            state.status = TimerStatus::Running;
            return false;
        }

        // 1. Handle Active Break Countdown
        if state.status == TimerStatus::OnBreak {
            if state.break_remaining_seconds > 0 {
                state.break_remaining_seconds -= 1;
            }
            if state.break_remaining_seconds == 0 {
                // Break completed!
                if config.timer_mode == TimerMode::Pomodoro {
                    if state.is_long_break {
                        state.current_cycle = 1;
                        state.is_long_break = false;
                    } else {
                        state.current_cycle += 1;
                    }
                    state.remaining_seconds = config.pomodoro_work_minutes * 60;
                } else {
                    state.remaining_seconds = config.work_duration_minutes * 60;
                }
                state.status = TimerStatus::Running;
            }
            return false;
        }

        // 2. Check Scheduled Quiet Hours
        if IdleDetector::is_quiet_hours_active(config) {
            if state.status != TimerStatus::PausedQuietHours {
                state.status = TimerStatus::PausedQuietHours;
            }
            return false;
        } else if state.status == TimerStatus::PausedQuietHours {
            state.status = TimerStatus::Running;
        }

        // 3. Check Smart Idle Detection
        if config.idle_detection_enabled {
            let idle_seconds = IdleDetector::get_idle_seconds();
            if idle_seconds >= config.idle_threshold_seconds as u64 {
                if state.status == TimerStatus::Running {
                    state.status = TimerStatus::PausedIdle;
                }
                return false;
            } else if state.status == TimerStatus::PausedIdle {
                state.status = TimerStatus::Running;
            }
        }

        // 4. Do not tick if manually paused
        if state.status == TimerStatus::PausedManual {
            return false;
        }

        // 5. Decrement remaining work timer
        if state.remaining_seconds > 0 {
            state.remaining_seconds -= 1;
        }

        // 6. Check for Break Trigger
        if state.remaining_seconds == 0 {
            if config.timer_mode == TimerMode::Pomodoro {
                let is_long = state.current_cycle >= config.pomodoro_cycles_before_long_break;
                state.is_long_break = is_long;
                state.break_remaining_seconds = if is_long {
                    config.pomodoro_long_break_minutes * 60
                } else {
                    config.pomodoro_short_break_minutes * 60
                };
            } else {
                state.is_long_break = false;
                state.break_remaining_seconds = config.break_duration_seconds;
            }
            state.status = TimerStatus::OnBreak;
            return true;
        }

        false
    }

    pub fn format_time(seconds: u32) -> String {
        let m = seconds / 60;
        let s = seconds % 60;
        format!("{:02}:{:02}", m, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(TimerEngine::format_time(1200), "20:00");
        assert_eq!(TimerEngine::format_time(45), "00:45");
        assert_eq!(TimerEngine::format_time(0), "00:00");
        assert_eq!(TimerEngine::format_time(60), "01:00");
    }

    #[test]
    fn test_timer_countdown_and_trigger() {
        let engine = TimerEngine::new(1);
        let cfg = BlinkConfig {
            work_duration_minutes: 1,
            break_duration_seconds: 20,
            idle_detection_enabled: false,
            quiet_hours_enabled: false,
            ..BlinkConfig::default()
        };

        if let Ok(mut s) = engine.state.write() {
            s.remaining_seconds = 2;
        }

        assert!(!engine.tick(&cfg));
        assert_eq!(engine.get_info().remaining_seconds, 1);

        assert!(engine.tick(&cfg));
        assert_eq!(engine.get_info().state, "OnBreak");
        assert_eq!(engine.get_info().break_remaining_seconds, 20);
    }

    #[test]
    fn test_pomodoro_cycle_progression() {
        let engine = TimerEngine::new(25);
        let cfg = BlinkConfig {
            timer_mode: TimerMode::Pomodoro,
            pomodoro_work_minutes: 25,
            pomodoro_short_break_minutes: 5,
            pomodoro_long_break_minutes: 15,
            pomodoro_cycles_before_long_break: 2, // 2 cycles for test
            idle_detection_enabled: false,
            quiet_hours_enabled: false,
            ..BlinkConfig::default()
        };

        // Cycle 1 Trigger -> Short Break (5 min = 300s)
        if let Ok(mut s) = engine.state.write() {
            s.remaining_seconds = 1;
            s.current_cycle = 1;
        }
        assert!(engine.tick(&cfg));
        assert_eq!(engine.get_info().state, "OnBreak");
        assert!(!engine.get_info().is_long_break);
        assert_eq!(engine.get_info().break_remaining_seconds, 300);

        // Finish Short Break -> Moves to Cycle 2
        if let Ok(mut s) = engine.state.write() {
            s.break_remaining_seconds = 1;
        }
        assert!(!engine.tick(&cfg));
        assert_eq!(engine.get_info().state, "Running");
        assert_eq!(engine.get_info().current_cycle, 2);

        // Cycle 2 Trigger -> Long Break (15 min = 900s)
        if let Ok(mut s) = engine.state.write() {
            s.remaining_seconds = 1;
        }
        assert!(engine.tick(&cfg));
        assert_eq!(engine.get_info().state, "OnBreak");
        assert!(engine.get_info().is_long_break);
        assert_eq!(engine.get_info().break_remaining_seconds, 900);

        // Finish Long Break -> Resets to Cycle 1
        if let Ok(mut s) = engine.state.write() {
            s.break_remaining_seconds = 1;
        }
        assert!(!engine.tick(&cfg));
        assert_eq!(engine.get_info().state, "Running");
        assert_eq!(engine.get_info().current_cycle, 1);
    }
}
