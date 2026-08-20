use crate::audio::AudioPlayer;
use crate::config::{BlinkConfig, ConfigManager};
use crate::notification::NotificationManager;
use crate::timer::{TimerEngine, TimerInfo};
use std::sync::Arc;
use tauri::{AppHandle, State};

pub struct AppState {
    pub config_mgr: Arc<ConfigManager>,
    pub timer: Arc<TimerEngine>,
    pub notifications: Arc<NotificationManager>,
    pub audio: Arc<AudioPlayer>,
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> BlinkConfig {
    state.config_mgr.get_config()
}

#[tauri::command]
pub fn save_config(state: State<'_, AppState>, config: BlinkConfig) -> Result<(), String> {
    state.config_mgr.update_config(config.clone())?;
    // Reset/update timer interval immediately
    state.timer.reset(config.work_duration_minutes);
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
    state.timer.reset(cfg.work_duration_minutes);
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
pub fn test_sound(state: State<'_, AppState>, volume: f32) {
    state.audio.play_chime(volume);
}
