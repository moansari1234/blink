use crate::config::BlinkConfig;

#[derive(Debug, Clone, Copy, Default)]
pub struct IdleMetrics {
    pub idle_seconds: u64,
    pub is_suppressed: bool,
    pub in_quiet_hours: bool,
}

pub struct IdleDetector;

impl IdleDetector {
    #[cfg(target_os = "windows")]
    pub fn get_idle_seconds() -> u64 {
        use windows::Win32::System::SystemInformation::GetTickCount64;
        use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

        unsafe {
            let mut lii = LASTINPUTINFO {
                cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
                dwTime: 0,
            };

            if GetLastInputInfo(&mut lii).as_bool() {
                let current_tick = GetTickCount64();
                let last_input_tick = lii.dwTime as u64;
                if current_tick >= last_input_tick {
                    return (current_tick - last_input_tick) / 1000;
                }
            }
        }
        0
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_idle_seconds() -> u64 {
        0
    }

    #[cfg(target_os = "windows")]
    pub fn is_focus_assist_or_dnd_active() -> bool {
        use windows::Win32::UI::Shell::{
            SHQueryUserNotificationState, QUNS_ACCEPTS_NOTIFICATIONS,
        };

        unsafe {
            if let Ok(state) = SHQueryUserNotificationState() {
                // If it does NOT equal QUNS_ACCEPTS_NOTIFICATIONS, Focus Assist, Fullscreen D3D, or Presentation Mode is active
                return state != QUNS_ACCEPTS_NOTIFICATIONS;
            }
        }
        false
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_focus_assist_or_dnd_active() -> bool {
        false
    }

    #[cfg(target_os = "windows")]
    pub fn get_local_time_and_day() -> (u32, u32, u8) {
        use windows::Win32::System::SystemInformation::GetLocalTime;
        unsafe {
            let st = GetLocalTime();
            // st.wHour, st.wMinute, st.wDayOfWeek (0 = Sunday, 1 = Monday, ..., 6 = Saturday)
            (st.wHour as u32, st.wMinute as u32, st.wDayOfWeek as u8)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_local_time_and_day() -> (u32, u32, u8) {
        (12, 0, 1)
    }

    pub fn is_quiet_hours_active(config: &BlinkConfig) -> bool {
        if !config.quiet_hours_enabled {
            return false;
        }

        let (hour, minute, day) = Self::get_local_time_and_day();
        if !config.quiet_hours_days.contains(&day) {
            return false;
        }

        let start_mins = parse_time_to_minutes(&config.quiet_hours_start).unwrap_or(720);
        let end_mins = parse_time_to_minutes(&config.quiet_hours_end).unwrap_or(780);
        let current_mins = hour * 60 + minute;

        is_in_minute_range(current_mins, start_mins, end_mins)
    }

    pub fn inspect(config: &BlinkConfig) -> IdleMetrics {
        IdleMetrics {
            idle_seconds: Self::get_idle_seconds(),
            is_suppressed: Self::is_focus_assist_or_dnd_active(),
            in_quiet_hours: Self::is_quiet_hours_active(config),
        }
    }
}

pub fn parse_time_to_minutes(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(h), Ok(m)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            if h < 24 && m < 60 {
                return Some(h * 60 + m);
            }
        }
    }
    None
}

pub fn is_in_minute_range(current: u32, start: u32, end: u32) -> bool {
    if start <= end {
        current >= start && current < end
    } else {
        // Overnight range (e.g. 22:00 to 06:00)
        current >= start || current < end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_range_same_day() {
        // 12:00 to 13:00 (720 to 780)
        assert!(is_in_minute_range(720, 720, 780)); // 12:00
        assert!(is_in_minute_range(750, 720, 780)); // 12:30
        assert!(!is_in_minute_range(780, 720, 780)); // 13:00 (end is exclusive)
        assert!(!is_in_minute_range(719, 720, 780)); // 11:59
    }

    #[test]
    fn test_time_range_overnight() {
        // 22:00 to 06:00 (1320 to 360)
        assert!(is_in_minute_range(1320, 1320, 360)); // 22:00
        assert!(is_in_minute_range(1400, 1320, 360)); // 23:20
        assert!(is_in_minute_range(100, 1320, 360));  // 01:40
        assert!(!is_in_minute_range(360, 1320, 360)); // 06:00 (end is exclusive)
        assert!(!is_in_minute_range(720, 1320, 360)); // 12:00
    }
}
