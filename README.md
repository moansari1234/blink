# 👁 Blink

> An ultra-lightweight, privacy-first, open-source Windows 11 desktop app that reminds you to take breaks following the 20-20-20 rule.

[![Latest Release](https://img.shields.io/github/v/release/moansari1234/blink?color=green&label=Latest%20Release)](https://github.com/moansari1234/blink/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6.svg)](https://microsoft.com/windows)
[![Built with: Rust](https://img.shields.io/badge/Built%20with-Rust%20%2B%20Tauri%20v2-dea584.svg)](https://tauri.app)

---

## 📥 Download Latest Version

You can always download the latest installer files directly from the **[Releases Page](https://github.com/moansari1234/blink/releases/latest)**:

| Installer | Description | Download |
| :--- | :--- | :--- |
| **Windows Installer (Recommended)** | Official `.msi` package with clean install/uninstall | [⬇️ Download Latest `.msi`](https://github.com/moansari1234/blink/releases/latest) |
| **Setup Executable** | Standalone setup wizard (`.exe`) | [⬇️ Download Latest `.exe`](https://github.com/moansari1234/blink/releases/latest) |

---

## 📖 The 20-20-20 Rule

Digital eye strain (Computer Vision Syndrome) affects millions of people who look at screens for prolonged periods. The **20-20-20 rule** is an ophthalmologist-recommended practice:

> **Every 20 minutes**, look at something at least **20 feet away** for at least **20 seconds**.

This allows your eye focusing muscles (ciliary muscles) to completely relax and prevents headaches, dry eyes, and fatigue.

---

## ✨ Features

- **⚡ Ultra-Lightweight**: Uses `< 38 MB` RAM and `< 0.1%` CPU. Runs quietly in your system tray.
- **🔒 100% Offline & Private**: Zero telemetry, zero tracking, zero network connections.
- **🔔 7 Rich Notification Modes**:
  - **Native Windows Toast**: Clean OS notifications.
  - **Screen Overlay (Corner)**: Gentle corner prompt with countdown bar and direct Snooze & Dismiss buttons.
  - **Floating Top Island**: Sleek top-center pill banner with micro-countdown and inline actions.
  - **Full-Screen Focus Veil**: Customizable transparent screen dimmer (30% to 90%) with a large centered countdown circle and breathing guide.
  - **Ambient Screen Edge Glow**: Glowing 4px breathing neon/mica border around screen edges without stealing window clicks.
  - **Audio-Only Whisper Mode**: Zero visual popups; dual start & finish chimes so you can close your eyes completely.
  - **System Tray Balloon**: Lightweight tray notification directly from the taskbar.
- **👀 Guided Eye Exercises**: 3 rotating sets of animated eye stretches cycling every 5 seconds during breaks.
- **🔒 Strict / Enforced Break Mode**: Optional 10-second dismissal lockout to prevent reflexively closing break reminders (<kbd>Esc</kbd> key bypass).
- **💧 Hydration & 🪑 Posture Reminders**: Configurable secondary micro-alerts to drink water and fix your posture while working.
- **♿ Accessibility & WCAG AAA Compliance**:
  - **High Contrast Theme**: Pure high-contrast black/white/cyan/yellow palette.
  - **UI Text Scaling**: 100%, 125%, and 150% sizing options.
  - **Reduced Motion**: Full support for users who prefer motion disabled.
  - **Screen Reader Support**: Complete ARIA roles and keyboard navigation.
- **👁 20-20-20 Rule & 🍅 Pomodoro Mode**: Switch seamlessly between standard 20-20-20 eye breaks and full Pomodoro focus intervals (25m work, 5m short break, 15m long break after 4 cycles).
- **🎵 Custom Sound Chimes**: Pick your own `.wav`, `.mp3`, `.ogg`, or `.flac` audio files for break alerts with graceful built-in fallback.
- **🔥 Live Streak Counter & Milestones**: See your active break streak in the system tray menu and earn celebratory toasts at 5, 10, 25, 50, and 100 breaks in a row.
- **🌙 Scheduled Quiet Hours**: Configure automatic Do Not Disturb periods (e.g., lunch breaks) with day-of-week filtering to pause reminders automatically.
- **📤 Settings Backup & Restore**: Export and import your complete configuration to/from JSON via native file dialogs.
- **📊 Break Stats & History Dashboard**: 90-day local history tracking with streak counter, daily averages, and a crisp 7-day activity chart.
- **🎨 Dark, Light, & System Themes**: Seamless dark mode and light mode overrides across the settings interface and screen overlays.
- **💬 Custom Break Messages**: Set custom reminder messages with `|` pipe rotation support (e.g. `Look outside | Stretch your arms | Drink water`).
- **⌨️ Global Keyboard Shortcuts**: `Ctrl+Shift+B` to Pause/Resume and `Ctrl+Shift+N` to Skip Break without opening any menus.
- **🖥️ Multi-Monitor Overlay Positioning**: Automatically positions the corner overlay prompt on your primary display or whichever monitor contains your active cursor.
- **⏱ Live Tray Countdown & Tooltip**: Dynamic system tray hover tooltip showing live remaining time, timer state, and current streak.
- **🟢🟡🔴 Dynamic Status Icons**:
  - 🟢 **Green**: Timer actively running
  - 🟡 **Yellow**: Paused (idle detected, quiet hours, or manually paused)
  - 🔴 **Red**: Break time! (with distinct Pomodoro long break indicators)
- **💤 Smart Idle Detection**: Pauses automatically when you are away from keyboard/mouse or when your screen is locked, and resumes exactly where you left off.
- **🔕 Windows Focus Assist Aware**: Respects "Do Not Disturb" / Focus Assist modes so it won't interrupt high-stakes presentations or full-screen gaming sessions.
- **🔄 Instant Hot-Reload**: Settings changes take effect immediately without restarting the app. You can also edit `%APPDATA%\Blink\config.json` directly!
- **🚀 Auto-Start on Login**: Optionally launches minimized to tray when your PC boots.
- **🔄 Built-in Update Checker**: One-click "Check for Updates" inside Settings that checks GitHub Releases and provides instant download buttons.

---

## ⚙️ Configuration

Blink stores its configuration in standard JSON format at:
```text
%APPDATA%\Blink\config.json
```

### Example `config.json`:
```json
{
  "work_duration_minutes": 20,
  "break_duration_seconds": 20,
  "notification_style": "focusveil",
  "idle_detection_enabled": true,
  "idle_threshold_seconds": 120,
  "auto_start": true,
  "sound_enabled": true,
  "sound_volume": 0.5,
  "snooze_duration_minutes": 5,
  "respect_focus_assist": true,
  "theme": "system",
  "break_message": "Time for a 20-second break! Look at something 20 feet away.",
  "hotkeys_enabled": true,
  "overlay_monitor": "primary",
  "custom_sound_path": null,
  "timer_mode": "twentytwentytwenty",
  "pomodoro_work_minutes": 25,
  "pomodoro_short_break_minutes": 5,
  "pomodoro_long_break_minutes": 15,
  "pomodoro_cycles_before_long_break": 4,
  "quiet_hours_enabled": false,
  "quiet_hours_start": "12:00",
  "quiet_hours_end": "13:00",
  "quiet_hours_days": [1, 2, 3, 4, 5],
  "veil_opacity": 0.5,
  "eye_exercises_enabled": true,
  "strict_mode_enabled": false,
  "hydration_enabled": false,
  "hydration_interval_minutes": 45,
  "posture_enabled": false,
  "posture_interval_minutes": 30,
  "ui_scale": "100%",
  "reduced_motion": false
}
```

---

## 🛠️ Tech Stack

- **Backend**: Rust 2021 Edition
- **App Framework**: Tauri v2 (`tray-icon`, `tauri-plugin-notification`, `tauri-plugin-autostart`, `tauri-plugin-single-instance`)
- **Idle & Hotkeys**: Windows Win32 API (`GetLastInputInfo`, `SHQueryUserNotificationState`, `RegisterHotKey`)
- **Audio Engine**: `rodio` with embedded Vorbis/WAV audio synthesis
- **Frontend**: Vanilla HTML5, CSS3 (Fluent Design Tokens), JavaScript (No bulky node_modules runtime)

---

## 🤝 Contributing & Releases

Contributions are welcome! Please check out:
- [CONTRIBUTING.md](CONTRIBUTING.md) for local development and build guidelines.
- [VERSIONING.md](VERSIONING.md) for version bumping and automated release workflows.
- [ROADMAP.md](ROADMAP.md) for planned future features and updates.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
