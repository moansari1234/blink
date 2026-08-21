use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NotificationStyle {
    #[default]
    Toast,
    Tray,
    Overlay,
    #[serde(rename = "edgepulse")]
    EdgePulse,
    #[serde(rename = "floatingisland")]
    FloatingIsland,
    #[serde(rename = "focusveil")]
    FocusVeil,
    #[serde(rename = "audioonly")]
    AudioOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
    #[serde(rename = "highcontrast")]
    HighContrast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OverlayMonitor {
    #[default]
    Primary,
    Cursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TimerMode {
    #[default]
    TwentyTwentyTwenty,
    Pomodoro,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlinkConfig {
    #[serde(default = "default_work_duration")]
    pub work_duration_minutes: u32,

    #[serde(default = "default_break_duration")]
    pub break_duration_seconds: u32,

    #[serde(default)]
    pub notification_style: NotificationStyle,

    #[serde(default = "default_true")]
    pub idle_detection_enabled: bool,

    #[serde(default = "default_idle_threshold")]
    pub idle_threshold_seconds: u32,

    #[serde(default = "default_true")]
    pub auto_start: bool,

    #[serde(default = "default_true")]
    pub sound_enabled: bool,

    #[serde(default = "default_volume")]
    pub sound_volume: f32,

    #[serde(default = "default_snooze_duration")]
    pub snooze_duration_minutes: u32,

    #[serde(default = "default_true")]
    pub respect_focus_assist: bool,

    #[serde(default)]
    pub theme: Theme,

    #[serde(default = "default_break_message")]
    pub break_message: String,

    #[serde(default = "default_true")]
    pub hotkeys_enabled: bool,

    #[serde(default)]
    pub overlay_monitor: OverlayMonitor,

    // v1.3.x Additions
    #[serde(default)]
    pub custom_sound_path: Option<String>,

    #[serde(default)]
    pub timer_mode: TimerMode,

    #[serde(default = "default_pomodoro_work")]
    pub pomodoro_work_minutes: u32,

    #[serde(default = "default_pomodoro_short_break")]
    pub pomodoro_short_break_minutes: u32,

    #[serde(default = "default_pomodoro_long_break")]
    pub pomodoro_long_break_minutes: u32,

    #[serde(default = "default_pomodoro_cycles")]
    pub pomodoro_cycles_before_long_break: u32,

    #[serde(default = "default_false")]
    pub quiet_hours_enabled: bool,

    #[serde(default = "default_quiet_start")]
    pub quiet_hours_start: String,

    #[serde(default = "default_quiet_end")]
    pub quiet_hours_end: String,

    #[serde(default = "default_quiet_days")]
    pub quiet_hours_days: Vec<u8>,

    // v1.4.x Additions
    #[serde(default = "default_veil_opacity")]
    pub veil_opacity: f32,

    #[serde(default = "default_true")]
    pub eye_exercises_enabled: bool,

    #[serde(default = "default_false")]
    pub strict_mode_enabled: bool,

    #[serde(default = "default_false")]
    pub hydration_enabled: bool,

    #[serde(default = "default_hydration_interval")]
    pub hydration_interval_minutes: u32,

    #[serde(default = "default_false")]
    pub posture_enabled: bool,

    #[serde(default = "default_posture_interval")]
    pub posture_interval_minutes: u32,

    #[serde(default = "default_ui_scale")]
    pub ui_scale: String,

    #[serde(default = "default_false")]
    pub reduced_motion: bool,
}

fn default_work_duration() -> u32 { 20 }
fn default_break_duration() -> u32 { 20 }
fn default_idle_threshold() -> u32 { 120 }
fn default_snooze_duration() -> u32 { 5 }
fn default_volume() -> f32 { 0.5 }
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_break_message() -> String {
    "Time for a 20-second break! Look at something 20 feet away.".to_string()
}
fn default_pomodoro_work() -> u32 { 25 }
fn default_pomodoro_short_break() -> u32 { 5 }
fn default_pomodoro_long_break() -> u32 { 15 }
fn default_pomodoro_cycles() -> u32 { 4 }
fn default_quiet_start() -> String { "12:00".to_string() }
fn default_quiet_end() -> String { "13:00".to_string() }
fn default_quiet_days() -> Vec<u8> { vec![1, 2, 3, 4, 5] }
fn default_veil_opacity() -> f32 { 0.5 }
fn default_hydration_interval() -> u32 { 45 }
fn default_posture_interval() -> u32 { 30 }
fn default_ui_scale() -> String { "100%".to_string() }

impl Default for BlinkConfig {
    fn default() -> Self {
        Self {
            work_duration_minutes: 20,
            break_duration_seconds: 20,
            notification_style: NotificationStyle::Toast,
            idle_detection_enabled: true,
            idle_threshold_seconds: 120,
            auto_start: true,
            sound_enabled: true,
            sound_volume: 0.5,
            snooze_duration_minutes: 5,
            respect_focus_assist: true,
            theme: Theme::System,
            break_message: default_break_message(),
            hotkeys_enabled: true,
            overlay_monitor: OverlayMonitor::Primary,
            custom_sound_path: None,
            timer_mode: TimerMode::TwentyTwentyTwenty,
            pomodoro_work_minutes: 25,
            pomodoro_short_break_minutes: 5,
            pomodoro_long_break_minutes: 15,
            pomodoro_cycles_before_long_break: 4,
            quiet_hours_enabled: false,
            quiet_hours_start: "12:00".to_string(),
            quiet_hours_end: "13:00".to_string(),
            quiet_hours_days: vec![1, 2, 3, 4, 5],
            veil_opacity: 0.5,
            eye_exercises_enabled: true,
            strict_mode_enabled: false,
            hydration_enabled: false,
            hydration_interval_minutes: 45,
            posture_enabled: false,
            posture_interval_minutes: 30,
            ui_scale: "100%".to_string(),
            reduced_motion: false,
        }
    }
}

impl BlinkConfig {
    pub fn sanitize(&mut self) {
        if self.work_duration_minutes < 1 { self.work_duration_minutes = 1; }
        if self.break_duration_seconds < 5 { self.break_duration_seconds = 5; }
        if self.idle_threshold_seconds < 30 { self.idle_threshold_seconds = 30; }
        if self.snooze_duration_minutes < 1 { self.snooze_duration_minutes = 1; }
        self.sound_volume = self.sound_volume.clamp(0.0, 1.0);
        if self.break_message.trim().is_empty() {
            self.break_message = default_break_message();
        }

        // Pomodoro bounds
        if self.pomodoro_work_minutes < 1 { self.pomodoro_work_minutes = 1; }
        if self.pomodoro_short_break_minutes < 1 { self.pomodoro_short_break_minutes = 1; }
        if self.pomodoro_long_break_minutes < 1 { self.pomodoro_long_break_minutes = 1; }
        if self.pomodoro_cycles_before_long_break < 1 { self.pomodoro_cycles_before_long_break = 1; }

        // Custom sound path
        if let Some(ref path) = self.custom_sound_path {
            if path.trim().is_empty() {
                self.custom_sound_path = None;
            }
        }

        // Quiet hours validation
        if !is_valid_hhmm(&self.quiet_hours_start) {
            self.quiet_hours_start = default_quiet_start();
        }
        if !is_valid_hhmm(&self.quiet_hours_end) {
            self.quiet_hours_end = default_quiet_end();
        }
        self.quiet_hours_days.retain(|d| *d <= 6);
        if self.quiet_hours_days.is_empty() {
            self.quiet_hours_days = default_quiet_days();
        }

        // v1.4.x Sanitization
        self.veil_opacity = self.veil_opacity.clamp(0.2, 0.95);
        if self.hydration_interval_minutes < 5 { self.hydration_interval_minutes = 5; }
        if self.hydration_interval_minutes > 240 { self.hydration_interval_minutes = 240; }
        if self.posture_interval_minutes < 5 { self.posture_interval_minutes = 5; }
        if self.posture_interval_minutes > 240 { self.posture_interval_minutes = 240; }

        let valid_scales = ["100%", "125%", "150%"];
        if !valid_scales.contains(&self.ui_scale.as_str()) {
            self.ui_scale = "100%".to_string();
        }
    }
}

fn is_valid_hhmm(s: &str) -> bool {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return false;
    }
    if let (Ok(h), Ok(m)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
        h < 24 && m < 60
    } else {
        false
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    current_config: Arc<RwLock<BlinkConfig>>,
}

impl ConfigManager {
    pub fn new() -> Self {
        let path = Self::resolve_config_path();
        let config = Self::load_from_path(&path);
        Self {
            config_path: path,
            current_config: Arc::new(RwLock::new(config)),
        }
    }

    pub fn get_config(&self) -> BlinkConfig {
        self.current_config
            .read()
            .map(|c| c.clone())
            .unwrap_or_default()
    }

    pub fn get_config_arc(&self) -> Arc<RwLock<BlinkConfig>> {
        Arc::clone(&self.current_config)
    }

    pub fn get_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    pub fn update_config(&self, mut new_config: BlinkConfig) -> Result<(), String> {
        new_config.sanitize();
        self.save_to_path(&self.config_path, &new_config)?;
        if let Ok(mut current) = self.current_config.write() {
            *current = new_config;
        }
        Ok(())
    }

    pub fn reload(&self) -> Result<BlinkConfig, String> {
        let loaded = Self::load_from_path(&self.config_path);
        if let Ok(mut current) = self.current_config.write() {
            *current = loaded.clone();
        }
        Ok(loaded)
    }

    pub fn resolve_data_dir() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "Blink", "Blink") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                let _ = fs::create_dir_all(config_dir);
            }
            config_dir.to_path_buf()
        } else {
            PathBuf::from(".")
        }
    }

    pub fn resolve_config_path() -> PathBuf {
        Self::resolve_data_dir().join("config.json")
    }

    fn load_from_path(path: &PathBuf) -> BlinkConfig {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => match serde_json::from_str::<BlinkConfig>(&content) {
                    Ok(mut cfg) => {
                        cfg.sanitize();
                        return cfg;
                    }
                    Err(e) => {
                        eprintln!("[Blink Config] Error parsing config at {:?}: {}", path, e);
                    }
                },
                Err(e) => {
                    eprintln!("[Blink Config] Error reading config file at {:?}: {}", path, e);
                }
            }
        }

        let default_cfg = BlinkConfig::default();
        let _ = Self::save_to_path_internal(path, &default_cfg);
        default_cfg
    }

    fn save_to_path(&self, path: &PathBuf, config: &BlinkConfig) -> Result<(), String> {
        Self::save_to_path_internal(path, config)
    }

    fn save_to_path_internal(path: &PathBuf, config: &BlinkConfig) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }
        let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = BlinkConfig::default();
        assert_eq!(cfg.work_duration_minutes, 20);
        assert_eq!(cfg.break_duration_seconds, 20);
        assert_eq!(cfg.notification_style, NotificationStyle::Toast);
        assert_eq!(cfg.veil_opacity, 0.5);
        assert!(cfg.eye_exercises_enabled);
        assert!(!cfg.strict_mode_enabled);
        assert!(!cfg.hydration_enabled);
        assert_eq!(cfg.hydration_interval_minutes, 45);
        assert!(!cfg.posture_enabled);
        assert_eq!(cfg.posture_interval_minutes, 30);
        assert_eq!(cfg.ui_scale, "100%");
        assert!(!cfg.reduced_motion);
    }

    #[test]
    fn test_sanitization_bounds() {
        let mut cfg = BlinkConfig {
            veil_opacity: 0.05,
            hydration_interval_minutes: 2,
            posture_interval_minutes: 300,
            ui_scale: "200%".to_string(),
            ..BlinkConfig::default()
        };
        cfg.sanitize();
        assert_eq!(cfg.veil_opacity, 0.2);
        assert_eq!(cfg.hydration_interval_minutes, 5);
        assert_eq!(cfg.posture_interval_minutes, 240);
        assert_eq!(cfg.ui_scale, "100%");
    }

    #[test]
    fn test_backward_compatibility() {
        let legacy_json = r#"{
            "work_duration_minutes": 25,
            "break_duration_seconds": 30,
            "notification_style": "toast"
        }"#;

        let parsed: BlinkConfig = serde_json::from_str(legacy_json).expect("deserialize legacy json");
        assert_eq!(parsed.work_duration_minutes, 25);
        assert_eq!(parsed.notification_style, NotificationStyle::Toast);
        assert_eq!(parsed.veil_opacity, 0.5);
        assert!(parsed.eye_exercises_enabled);
        assert_eq!(parsed.ui_scale, "100%");
    }
}
