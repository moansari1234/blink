use crate::audio::AudioPlayer;
use crate::config::{BlinkConfig, NotificationStyle, OverlayMonitor};
use crate::idle::IdleDetector;
use rand::seq::SliceRandom;
use tauri::{AppHandle, Emitter, Manager};
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

        // 3. Pick rotated/custom break message
        let break_msg = pick_break_message(&config.break_message);

        // 4. Visual notification based on chosen style
        match config.notification_style {
            NotificationStyle::Toast => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Time for an eye break!")
                    .body(&break_msg)
                    .show();
            }
            NotificationStyle::Overlay => {
                let _ = app.emit("break_message", &break_msg);
                if let Some(snooze_win) = app.get_webview_window("snooze") {
                    Self::position_overlay(&snooze_win, config.overlay_monitor);
                    let _ = snooze_win.show();
                    let _ = snooze_win.set_focus();
                } else {
                    // Fallback to toast if snooze window is unavailable
                    let _ = app
                        .notification()
                        .builder()
                        .title("👁 Time for an eye break!")
                        .body(&break_msg)
                        .show();
                }
            }
            NotificationStyle::Tray => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Blink — Break Reminder")
                    .body(&break_msg)
                    .show();
            }
        }
    }

    pub fn dispatch_test_alert(&self, app: &AppHandle, config: &BlinkConfig) {
        if config.sound_enabled {
            self.audio.play_chime(config.sound_volume);
        }

        let break_msg = pick_break_message(&config.break_message);

        match config.notification_style {
            NotificationStyle::Overlay => {
                let _ = app.emit("break_message", &break_msg);
                if let Some(snooze_win) = app.get_webview_window("snooze") {
                    Self::position_overlay(&snooze_win, config.overlay_monitor);
                    let _ = snooze_win.show();
                    let _ = snooze_win.set_focus();
                }
            }
            _ => {
                let _ = app
                    .notification()
                    .builder()
                    .title("👁 Blink Test Notification")
                    .body(&format!("Test reminder: {}", break_msg))
                    .show();
            }
        }
    }

    fn position_overlay(snooze_win: &tauri::WebviewWindow, monitor_mode: OverlayMonitor) {
        let target_monitor = if monitor_mode == OverlayMonitor::Cursor {
            snooze_win
                .cursor_position()
                .ok()
                .and_then(|cursor_pos| {
                    snooze_win.available_monitors().ok().and_then(|monitors| {
                        monitors.into_iter().find(|m| {
                            let pos = m.position();
                            let size = m.size();
                            cursor_pos.x >= pos.x as f64
                                && cursor_pos.x < (pos.x + size.width as i32) as f64
                                && cursor_pos.y >= pos.y as f64
                                && cursor_pos.y < (pos.y + size.height as i32) as f64
                        })
                    })
                })
                .or_else(|| snooze_win.primary_monitor().ok().flatten())
        } else {
            snooze_win.primary_monitor().ok().flatten()
        };

        if let Some(monitor) = target_monitor {
            let mon_pos = monitor.position();
            let mon_size = monitor.size();
            let scale = monitor.scale_factor();
            let win_width = (380.0 * scale) as i32;
            let win_height = (160.0 * scale) as i32;
            let margin_right = (24.0 * scale) as i32;
            let margin_bottom = (60.0 * scale) as i32;

            let x = mon_pos.x + mon_size.width as i32 - win_width - margin_right;
            let y = mon_pos.y + mon_size.height as i32 - win_height - margin_bottom;

            let _ = snooze_win.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
        }
    }
}

pub fn pick_break_message(raw: &str) -> String {
    let segments: Vec<&str> = raw
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if segments.is_empty() {
        "Time for a 20-second break! Look at something 20 feet away.".to_string()
    } else if segments.len() == 1 {
        segments[0].to_string()
    } else {
        let mut rng = rand::thread_rng();
        segments.choose(&mut rng).unwrap_or(&segments[0]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_break_message_single() {
        let msg = pick_break_message("Take a walk!");
        assert_eq!(msg, "Take a walk!");
    }

    #[test]
    fn test_pick_break_message_empty() {
        let msg = pick_break_message("   |  ");
        assert_eq!(
            msg,
            "Time for a 20-second break! Look at something 20 feet away."
        );
    }

    #[test]
    fn test_pick_break_message_rotation() {
        let raw = "Message 1 | Message 2 | Message 3";
        let mut found = std::collections::HashSet::new();
        for _ in 0..100 {
            let picked = pick_break_message(raw);
            found.insert(picked);
        }
        assert!(found.contains("Message 1") || found.contains("Message 2") || found.contains("Message 3"));
    }
}
