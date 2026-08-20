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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OverlayMonitor {
    #[default]
    Primary,
    Cursor,
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
}

fn default_work_duration() -> u32 {
    20
}
fn default_break_duration() -> u32 {
    20
}
fn default_idle_threshold() -> u32 {
    120
}
fn default_snooze_duration() -> u32 {
    5
}
fn default_volume() -> f32 {
    0.5
}
fn default_true() -> bool {
    true
}
fn default_break_message() -> String {
    "Time for a 20-second break! Look at something 20 feet away.".to_string()
}

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
        }
    }
}

impl BlinkConfig {
    pub fn sanitize(&mut self) {
        if self.work_duration_minutes < 1 {
            self.work_duration_minutes = 1;
        }
        if self.break_duration_seconds < 5 {
            self.break_duration_seconds = 5;
        }
        if self.idle_threshold_seconds < 30 {
            self.idle_threshold_seconds = 30;
        }
        if self.snooze_duration_minutes < 1 {
            self.snooze_duration_minutes = 1;
        }
        self.sound_volume = self.sound_volume.clamp(0.0, 1.0);
        if self.break_message.trim().is_empty() {
            self.break_message = default_break_message();
        }
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

        // Default config creation
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
        assert!(cfg.idle_detection_enabled);
        assert_eq!(cfg.idle_threshold_seconds, 120);
        assert!(cfg.auto_start);
        assert!(cfg.sound_enabled);
        assert_eq!(cfg.sound_volume, 0.5);
        assert_eq!(cfg.snooze_duration_minutes, 5);
        assert!(cfg.respect_focus_assist);
        assert_eq!(cfg.theme, Theme::System);
        assert_eq!(
            cfg.break_message,
            "Time for a 20-second break! Look at something 20 feet away."
        );
        assert!(cfg.hotkeys_enabled);
        assert_eq!(cfg.overlay_monitor, OverlayMonitor::Primary);
    }

    #[test]
    fn test_sanitization_bounds() {
        let mut cfg = BlinkConfig {
            work_duration_minutes: 0,
            break_duration_seconds: 1,
            notification_style: NotificationStyle::Overlay,
            idle_detection_enabled: true,
            idle_threshold_seconds: 10,
            auto_start: false,
            sound_enabled: true,
            sound_volume: 1.5,
            snooze_duration_minutes: 0,
            respect_focus_assist: false,
            theme: Theme::Dark,
            break_message: "   ".to_string(),
            hotkeys_enabled: false,
            overlay_monitor: OverlayMonitor::Cursor,
        };
        cfg.sanitize();
        assert_eq!(cfg.work_duration_minutes, 1);
        assert_eq!(cfg.break_duration_seconds, 5);
        assert_eq!(cfg.idle_threshold_seconds, 30);
        assert_eq!(cfg.snooze_duration_minutes, 1);
        assert_eq!(cfg.sound_volume, 1.0);
        assert_eq!(
            cfg.break_message,
            "Time for a 20-second break! Look at something 20 feet away."
        );
    }

    #[test]
    fn test_json_roundtrip() {
        let cfg = BlinkConfig::default();
        let json = serde_json::to_string(&cfg).expect("serialize");
        let parsed: BlinkConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, parsed);
    }

    #[test]
    fn test_backward_compatibility() {
        // v1.1.0 JSON format without new v1.2.0 fields
        let v1_1_json = r#"{
            "work_duration_minutes": 25,
            "break_duration_seconds": 30,
            "notification_style": "overlay",
            "idle_detection_enabled": true,
            "idle_threshold_seconds": 180,
            "auto_start": true,
            "sound_enabled": false,
            "sound_volume": 0.8,
            "snooze_duration_minutes": 10,
            "respect_focus_assist": false
        }"#;

        let parsed: BlinkConfig = serde_json::from_str(v1_1_json).expect("deserialize legacy json");
        assert_eq!(parsed.work_duration_minutes, 25);
        assert_eq!(parsed.break_duration_seconds, 30);
        assert_eq!(parsed.notification_style, NotificationStyle::Overlay);
        assert_eq!(parsed.theme, Theme::System);
        assert_eq!(parsed.break_message, "Time for a 20-second break! Look at something 20 feet away.");
        assert!(parsed.hotkeys_enabled);
        assert_eq!(parsed.overlay_monitor, OverlayMonitor::Primary);
    }
}
