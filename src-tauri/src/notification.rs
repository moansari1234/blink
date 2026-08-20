use crate::audio::AudioPlayer;
use crate::config::{BlinkConfig, NotificationStyle};
use crate::idle::IdleDetector;
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

pub struct NotificationManager {
    audio: AudioPlayer,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            audio: AudioPlayer::new(),
        }
    }

    pub fn dispatch_break_alert(&self, app: &AppHandle, config: &BlinkConfig) {
        // 1. Focus Assist check
        if config.respect_focus_assist && IdleDetector::is_focus_assist_or_dnd_active() {
            println!("[Blink Notification] Suppressed due to active Windows Focus Assist / Fullscreen");
            return;
        }

        // 2. Play Audio Chime
        if config.sound_enabled {
            self.audio.play_chime(config.sound_volume);
        }

        // 3. Visual notification based on chosen style
        match config.notification_style {
            NotificationStyle::Toast => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Time for an eye break!")
                    .body("Look at something 20 feet away for 20 seconds.")
                    .show();
            }
            NotificationStyle::Overlay => {
                if let Some(snooze_win) = app.get_webview_window("snooze") {
                    let _ = snooze_win.show();
                    let _ = snooze_win.set_focus();
                } else {
                    // Fallback to toast if snooze window is unavailable
                    let _ = app
                        .notification()
                        .builder()
                        .title("👁 Time for an eye break!")
                        .body("Look at something 20 feet away for 20 seconds.")
                        .show();
                }
            }
            NotificationStyle::Tray => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Blink — Break Reminder")
                    .body("Rest your eyes now (20-20-20 rule).")
                    .show();
            }
        }
    }

    pub fn dispatch_test_alert(&self, app: &AppHandle, config: &BlinkConfig) {
        if config.sound_enabled {
            self.audio.play_chime(config.sound_volume);
        }

        match config.notification_style {
            NotificationStyle::Overlay => {
                if let Some(snooze_win) = app.get_webview_window("snooze") {
                    let _ = snooze_win.show();
                    let _ = snooze_win.set_focus();
                }
            }
            _ => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Blink Test Notification")
                    .body("Notifications and sound are working perfectly!")
                    .show();
            }
        }
    }
}
