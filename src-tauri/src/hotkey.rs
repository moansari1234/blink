use crate::config::ConfigManager;
use crate::timer::TimerEngine;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

pub struct HotkeyManager;

impl HotkeyManager {
    #[cfg(target_os = "windows")]
    pub fn start(
        timer: Arc<TimerEngine>,
        config_mgr: Arc<ConfigManager>,
        app_handle: AppHandle,
    ) {
        std::thread::spawn(move || {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::Input::KeyboardAndMouse::{
                RegisterHotKey, HOT_KEY_MODIFIERS, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT,
            };
            use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

            const HOTKEY_PAUSE_RESUME_ID: i32 = 1; // Ctrl + Shift + B
            const HOTKEY_SKIP_BREAK_ID: i32 = 2;   // Ctrl + Shift + N
            const VK_B_CODE: u32 = 0x42;           // 'B' key
            const VK_N_CODE: u32 = 0x4E;           // 'N' key

            let modifiers: HOT_KEY_MODIFIERS = MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT;

            unsafe {
                // Register Ctrl+Shift+B
                let reg_b = RegisterHotKey(
                    HWND::default(),
                    HOTKEY_PAUSE_RESUME_ID,
                    modifiers,
                    VK_B_CODE,
                );
                if let Err(e) = reg_b {
                    eprintln!(
                        "[Blink Hotkey] Warning: Could not register Ctrl+Shift+B (may be in use by another app): {}",
                        e
                    );
                }

                // Register Ctrl+Shift+N
                let reg_n = RegisterHotKey(
                    HWND::default(),
                    HOTKEY_SKIP_BREAK_ID,
                    modifiers,
                    VK_N_CODE,
                );
                if let Err(e) = reg_n {
                    eprintln!(
                        "[Blink Hotkey] Warning: Could not register Ctrl+Shift+N (may be in use by another app): {}",
                        e
                    );
                }

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
                    if msg.message == WM_HOTKEY {
                        let hotkey_id = msg.wParam.0 as i32;
                        let config = config_mgr.get_config();

                        if !config.hotkeys_enabled {
                            continue;
                        }

                        match hotkey_id {
                            HOTKEY_PAUSE_RESUME_ID => {
                                let info = timer.get_info();
                                if info.is_paused {
                                    timer.resume();
                                } else {
                                    timer.pause();
                                }
                            }
                            HOTKEY_SKIP_BREAK_ID => {
                                timer.reset(config.work_duration_minutes);
                                if let Some(snooze_win) = app_handle.get_webview_window("snooze") {
                                    let _ = snooze_win.hide();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    pub fn start(
        _timer: Arc<TimerEngine>,
        _config_mgr: Arc<ConfigManager>,
        _app_handle: AppHandle,
    ) {
        // Hotkey integration for non-Windows platforms planned for v2.0
    }
}
