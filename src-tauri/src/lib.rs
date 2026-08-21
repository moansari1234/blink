pub mod audio;
pub mod commands;
pub mod config;
pub mod history;
pub mod hotkey;
pub mod idle;
pub mod notification;
pub mod timer;
pub mod tray;

use audio::AudioPlayer;
use commands::{
    export_config, get_break_stats, get_config, get_timer_state, import_config, open_url,
    pause_timer, record_break_action, reset_timer, resume_timer, save_config, select_custom_sound,
    snooze_timer, test_notification, test_sound, AppState,
};
use config::ConfigManager;
use history::HistoryManager;
use hotkey::HotkeyManager;
use notification::NotificationManager;
use std::sync::Arc;
use std::time::Duration;
use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::ManagerExt;
use timer::TimerEngine;
use tray::TrayManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_mgr = Arc::new(ConfigManager::new());
    let initial_config = config_mgr.get_config();

    let initial_work = if initial_config.timer_mode == config::TimerMode::Pomodoro {
        initial_config.pomodoro_work_minutes
    } else {
        initial_config.work_duration_minutes
    };

    let timer = Arc::new(TimerEngine::new(initial_work));
    let notifications = Arc::new(NotificationManager::new());
    let audio = Arc::new(AudioPlayer::new());
    let history = Arc::new(HistoryManager::new());

    let app_state = AppState {
        config_mgr: Arc::clone(&config_mgr),
        timer: Arc::clone(&timer),
        notifications: Arc::clone(&notifications),
        audio: Arc::clone(&audio),
        history: Arc::clone(&history),
    };

    let timer_worker = Arc::clone(&timer);
    let config_worker = Arc::clone(&config_mgr);
    let notif_worker = Arc::clone(&notifications);
    let history_worker = Arc::clone(&history);

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .manage(app_state)
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Setup System Tray
            let tray_manager = Arc::new(
                TrayManager::setup(&app_handle)
                    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?,
            );

            // Start Global Hotkey Listener Thread
            HotkeyManager::start(
                Arc::clone(&timer_worker),
                Arc::clone(&config_worker),
                app_handle.clone(),
            );

            // Sync autostart with config setting
            let autolaunch = app.handle().autolaunch();
            let initial_cfg = config_worker.get_config();
            let currently_enabled = autolaunch.is_enabled().unwrap_or(false);
            if initial_cfg.auto_start && !currently_enabled {
                let _ = autolaunch.enable();
                println!("[Blink Autostart] Enabled auto-start on login");
            } else if !initial_cfg.auto_start && currently_enabled {
                let _ = autolaunch.disable();
                println!("[Blink Autostart] Disabled auto-start on login");
            }

            // Handle Tray Menu Action Events
            let timer_menu = Arc::clone(&timer_worker);
            let config_menu = Arc::clone(&config_worker);
            let app_handle_menu = app_handle.clone();

            app.on_menu_event(move |_app, event| {
                let id = event.id().as_ref();
                match id {
                    "pause_resume" => {
                        let info = timer_menu.get_info();
                        if info.is_paused {
                            timer_menu.resume();
                        } else {
                            timer_menu.pause();
                        }
                    }
                    "reset" => {
                        let cfg = config_menu.get_config();
                        let work = if cfg.timer_mode == config::TimerMode::Pomodoro {
                            cfg.pomodoro_work_minutes
                        } else {
                            cfg.work_duration_minutes
                        };
                        timer_menu.reset(work);
                    }
                    "snooze" => {
                        let cfg = config_menu.get_config();
                        timer_menu.snooze(cfg.snooze_duration_minutes);
                    }
                    "settings" => {
                        if let Some(win) = app_handle_menu.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => {
                        app_handle_menu.exit(0);
                    }
                    _ => {}
                }
            });

            // Start 1-second background tick loop
            let timer_loop = Arc::clone(&timer_worker);
            let config_loop = Arc::clone(&config_worker);
            let notif_loop = Arc::clone(&notif_worker);
            let history_loop = Arc::clone(&history_worker);
            let tray_loop = Arc::clone(&tray_manager);
            let app_handle_loop = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                let mut was_on_break = false;

                loop {
                    interval.tick().await;
                    let current_cfg = config_loop.get_config();
                    let should_alert = timer_loop.tick(&current_cfg);

                    if should_alert {
                        notif_loop.dispatch_break_alert(&app_handle_loop, &current_cfg);
                    }

                    // Check if break just finished (was on break -> now running)
                    let current_state = timer_loop.get_info().state;
                    if was_on_break && current_state == "Running" {
                        let break_secs = if current_cfg.timer_mode == config::TimerMode::Pomodoro {
                            current_cfg.pomodoro_short_break_minutes * 60
                        } else {
                            current_cfg.break_duration_seconds
                        };

                        let (_, milestone) = history_loop.record_break("completed", break_secs);
                        if let Some(streak_val) = milestone {
                            notif_loop.dispatch_milestone_alert(&app_handle_loop, streak_val);
                        }
                    }
                    was_on_break = current_state == "OnBreak";

                    tray_loop.update(&timer_loop, &current_cfg, history_loop.get_current_streak());
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Keep app running in system tray on window close
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_timer_state,
            pause_timer,
            resume_timer,
            reset_timer,
            snooze_timer,
            test_notification,
            test_sound,
            get_break_stats,
            record_break_action,
            select_custom_sound,
            export_config,
            import_config,
            open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Blink application");
}
