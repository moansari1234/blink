use crate::config::BlinkConfig;
use crate::timer::TimerEngine;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Wry};

const ICON_GREEN_BYTES: &[u8] = include_bytes!("../icons/icon-green.png");
const ICON_YELLOW_BYTES: &[u8] = include_bytes!("../icons/icon-yellow.png");
const ICON_RED_BYTES: &[u8] = include_bytes!("../icons/icon-red.png");

pub struct TrayManager {
    tray_handle: TrayIcon,
    countdown_item: MenuItem<Wry>,
    pause_resume_item: MenuItem<Wry>,
    snooze_item: MenuItem<Wry>,
    img_green: Image<'static>,
    img_yellow: Image<'static>,
    img_red: Image<'static>,
}

impl TrayManager {
    pub fn setup(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let img_green = Image::from_bytes(ICON_GREEN_BYTES)?;
        let img_yellow = Image::from_bytes(ICON_YELLOW_BYTES)?;
        let img_red = Image::from_bytes(ICON_RED_BYTES)?;

        // Build Menu Items
        let countdown_item = MenuItem::with_id(app, "countdown", "⏱ 20:00 remaining", false, None::<&str>)?;
        let pause_resume_item = MenuItem::with_id(app, "pause_resume", "⏸ Pause Timer", true, None::<&str>)?;
        let reset_item = MenuItem::with_id(app, "reset", "🔄 Reset Timer", true, None::<&str>)?;
        let snooze_item = MenuItem::with_id(app, "snooze", "💤 Snooze (5m)", true, None::<&str>)?;
        let settings_item = MenuItem::with_id(app, "settings", "⚙ Settings", true, None::<&str>)?;
        let quit_item = MenuItem::with_id(app, "quit", "❌ Quit", true, None::<&str>)?;
        let sep1 = PredefinedMenuItem::separator(app)?;
        let sep2 = PredefinedMenuItem::separator(app)?;

        let menu = Menu::with_items(
            app,
            &[
                &countdown_item,
                &sep1,
                &pause_resume_item,
                &reset_item,
                &snooze_item,
                &sep2,
                &settings_item,
                &quit_item,
            ],
        )?;

        let tray = TrayIconBuilder::with_id("blink-tray")
            .icon(img_green.clone())
            .menu(&menu)
            .tooltip("Blink — 20-20-20 Break Reminder")
            .show_menu_on_left_click(false)
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click { button, .. } = event {
                    if button == tauri::tray::MouseButton::Left {
                        if let Some(win) = tray.app_handle().get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                }
            })
            .build(app)?;

        Ok(Self {
            tray_handle: tray,
            countdown_item,
            pause_resume_item,
            snooze_item,
            img_green,
            img_yellow,
            img_red,
        })
    }

    pub fn update(&self, timer: &TimerEngine, config: &BlinkConfig) {
        let info = timer.get_info();

        // 1. Update Tooltip
        let tooltip_text = format!("Blink — ⏱ {} remaining ({})", info.formatted_time, info.state);
        let _ = self.tray_handle.set_tooltip(Some(&tooltip_text));

        // 2. Update Countdown Menu Item
        let countdown_text = format!("⏱ {} remaining", info.formatted_time);
        let _ = self.countdown_item.set_text(countdown_text);

        // 3. Update Pause/Resume Text
        if info.is_paused {
            let _ = self.pause_resume_item.set_text("▶ Resume Timer");
        } else {
            let _ = self.pause_resume_item.set_text("⏸ Pause Timer");
        }

        // 4. Update Snooze text with configured snooze minutes
        let snooze_text = format!("💤 Snooze ({}m)", config.snooze_duration_minutes);
        let _ = self.snooze_item.set_text(snooze_text);

        // 5. Update Tray Icon Color
        if info.state == "Running" {
            let _ = self.tray_handle.set_icon(Some(self.img_green.clone()));
        } else if info.state == "PausedIdle" || info.state == "PausedManual" {
            let _ = self.tray_handle.set_icon(Some(self.img_yellow.clone()));
        } else if info.state == "OnBreak" {
            let _ = self.tray_handle.set_icon(Some(self.img_red.clone()));
        }
    }
}
