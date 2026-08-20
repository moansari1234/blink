#[derive(Debug, Clone, Copy, Default)]
pub struct IdleMetrics {
    pub idle_seconds: u64,
    pub is_suppressed: bool,
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

    pub fn inspect() -> IdleMetrics {
        IdleMetrics {
            idle_seconds: Self::get_idle_seconds(),
            is_suppressed: Self::is_focus_assist_or_dnd_active(),
        }
    }
}
