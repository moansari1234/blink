use crate::audio::AudioPlayer;
use crate::config::{BlinkConfig, ConfigManager};
use crate::history::{BreakStats, HistoryManager};
use crate::notification::NotificationManager;
use crate::timer::{TimerEngine, TimerInfo};
use std::sync::Arc;
use tauri::{AppHandle, State};

pub struct AppState {
    pub config_mgr: Arc<ConfigManager>,
    pub timer: Arc<TimerEngine>,
    pub notifications: Arc<NotificationManager>,
    pub audio: Arc<AudioPlayer>,
    pub history: Arc<HistoryManager>,
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> BlinkConfig {
    state.config_mgr.get_config()
}

#[tauri::command]
pub fn save_config(app: AppHandle, state: State<'_, AppState>, config: BlinkConfig) -> Result<(), String> {
    let old_config = state.config_mgr.get_config();
    state.config_mgr.update_config(config.clone())?;

    // If timer mode or work duration changed, adjust timer
    if old_config.timer_mode != config.timer_mode {
        let new_work = if config.timer_mode == crate::config::TimerMode::Pomodoro {
            config.pomodoro_work_minutes
        } else {
            config.work_duration_minutes
        };
        state.timer.reset(new_work);
    } else if old_config.work_duration_minutes != config.work_duration_minutes
        || old_config.pomodoro_work_minutes != config.pomodoro_work_minutes
    {
        let old_work = if config.timer_mode == crate::config::TimerMode::Pomodoro {
            old_config.pomodoro_work_minutes
        } else {
            old_config.work_duration_minutes
        };
        let new_work = if config.timer_mode == crate::config::TimerMode::Pomodoro {
            config.pomodoro_work_minutes
        } else {
            config.work_duration_minutes
        };
        state.timer.adjust_work_duration(old_work, new_work);
    }

    // Toggle autostart when setting changes
    if old_config.auto_start != config.auto_start {
        use tauri_plugin_autostart::ManagerExt;
        let autolaunch = app.autolaunch();
        if config.auto_start {
            let _ = autolaunch.enable();
        } else {
            let _ = autolaunch.disable();
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_timer_state(state: State<'_, AppState>) -> TimerInfo {
    state.timer.get_info()
}

#[tauri::command]
pub fn pause_timer(state: State<'_, AppState>) {
    state.timer.pause();
}

#[tauri::command]
pub fn resume_timer(state: State<'_, AppState>) {
    state.timer.resume();
}

#[tauri::command]
pub fn reset_timer(state: State<'_, AppState>) {
    let cfg = state.config_mgr.get_config();
    let work = if cfg.timer_mode == crate::config::TimerMode::Pomodoro {
        cfg.pomodoro_work_minutes
    } else {
        cfg.work_duration_minutes
    };
    state.timer.reset(work);
}

#[tauri::command]
pub fn snooze_timer(state: State<'_, AppState>) {
    let cfg = state.config_mgr.get_config();
    state.timer.snooze(cfg.snooze_duration_minutes);
}

#[tauri::command]
pub fn test_notification(app: AppHandle, state: State<'_, AppState>) {
    let cfg = state.config_mgr.get_config();
    state.notifications.dispatch_test_alert(&app, &cfg);
}

#[tauri::command]
pub fn test_sound(state: State<'_, AppState>, volume: f32, custom_path: Option<String>) {
    state.audio.play_chime(volume, custom_path.as_deref());
}

#[tauri::command]
pub fn get_break_stats(state: State<'_, AppState>) -> BreakStats {
    state.history.get_stats()
}

#[tauri::command]
pub fn record_break_action(
    state: State<'_, AppState>,
    action: String,
    duration_seconds: u32,
) -> Result<(), String> {
    let (res, _) = state.history.record_break(&action, duration_seconds);
    res
}

#[tauri::command]
pub fn select_custom_sound() -> Result<Option<String>, String> {
    let file = rfd::FileDialog::new()
        .set_title("Select Custom Break Chime")
        .add_filter("Audio Files", &["wav", "mp3", "ogg", "flac"])
        .pick_file();

    Ok(file.map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn export_config(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let config = state.config_mgr.get_config();
    let file = rfd::FileDialog::new()
        .set_title("Export Blink Settings")
        .set_file_name("blink-config.json")
        .add_filter("JSON Files", &["json"])
        .save_file();

    if let Some(path) = file {
        let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
        std::fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(Some(path.to_string_lossy().to_string()))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn import_config(state: State<'_, AppState>) -> Result<Option<BlinkConfig>, String> {
    let file = rfd::FileDialog::new()
        .set_title("Import Blink Settings")
        .add_filter("JSON Files", &["json"])
        .pick_file();

    if let Some(path) = file {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut parsed: BlinkConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid config format: {}", e))?;
        parsed.sanitize();
        state.config_mgr.update_config(parsed.clone())?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
