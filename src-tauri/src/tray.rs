use crate::config::{BlinkConfig, TimerMode};
use crate::timer::TimerEngine;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIcon;
use tauri::{AppHandle, Manager};

pub struct TrayManager {
    tray: TrayIcon,
    green_icon: Image<'static>,
    yellow_icon: Image<'static>,
    red_icon: Image<'static>,
}

impl TrayManager {
    pub fn setup(app: &AppHandle) -> Result<Self, String> {
        let pause_resume_item = MenuItem::with_id(app, "pause_resume", "Pause / Resume Timer", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let reset_item = MenuItem::with_id(app, "reset", "Reset Timer", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let snooze_item = MenuItem::with_id(app, "snooze", "Snooze Break (5 min)", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let quit_item = MenuItem::with_id(app, "quit", "Quit Blink", true, None::<&str>)
            .map_err(|e| e.to_string())?;

        let menu = Menu::with_items(
            app,
            &[
                &pause_resume_item,
                &reset_item,
                &snooze_item,
                &settings_item,
                &quit_item,
            ],
        )
        .map_err(|e| e.to_string())?;

        let green_bytes = include_bytes!("../icons/icon-green.png");
        let yellow_bytes = include_bytes!("../icons/icon-yellow.png");
        let red_bytes = include_bytes!("../icons/icon-red.png");

        let green_icon = Image::from_bytes(green_bytes).map_err(|e| e.to_string())?;
        let yellow_icon = Image::from_bytes(yellow_bytes).map_err(|e| e.to_string())?;
        let red_icon = Image::from_bytes(red_bytes).map_err(|e| e.to_string())?;

        let tray = tauri::tray::TrayIconBuilder::new()
            .icon(green_icon.clone())
            .menu(&menu)
            .tooltip("Blink — 20-20-20 Break Reminder")
            .on_tray_icon_event(|tray, event| {
                if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                    let app = tray.app_handle();
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                }
            })
            .build(app)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            tray,
            green_icon,
            yellow_icon,
            red_icon,
        })
    }

    pub fn update(&self, timer: &TimerEngine, config: &BlinkConfig, streak: u32) {
        let info = timer.get_info();

        // Mode and cycle segment for tooltip
        let mode_desc = if config.timer_mode == TimerMode::Pomodoro {
            format!("🍅 Pomodoro ({}/{})", info.current_cycle, config.pomodoro_cycles_before_long_break)
        } else {
            "👁 20-20-20".to_string()
        };

        // Update Tray Icon & Tooltip
        let tooltip = match info.state.as_str() {
            "Running" => {
                let _ = self.tray.set_icon(Some(self.green_icon.clone()));
                format!("Blink — ⏱ {} ({}) | 🔥 Streak: {}", info.formatted_time, mode_desc, streak)
            }
            "PausedIdle" => {
                let _ = self.tray.set_icon(Some(self.yellow_icon.clone()));
                format!("Blink — 💤 Away / Idle (Paused: {})", info.formatted_time)
            }
            "PausedQuietHours" => {
                let _ = self.tray.set_icon(Some(self.yellow_icon.clone()));
                "Blink — 🌙 Quiet Hours (Paused)".to_string()
            }
            "PausedManual" => {
                let _ = self.tray.set_icon(Some(self.yellow_icon.clone()));
                format!("Blink — ⏸ Paused ({})", info.formatted_time)
            }
            "OnBreak" => {
                let _ = self.tray.set_icon(Some(self.red_icon.clone()));
                if info.is_long_break {
                    format!("Blink — 🍅 Long Break! ({} remaining)", info.formatted_time)
                } else {
                    format!("Blink — 👁 Break Time! ({} remaining)", info.formatted_time)
                }
            }
            _ => "Blink — Break Reminder".to_string(),
        };

        let _ = self.tray.set_tooltip(Some(tooltip));
    }
}
