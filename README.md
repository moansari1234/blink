# 👁 Blink

> An ultra-lightweight, privacy-first, open-source Windows 11 desktop app that reminds you to take breaks following the 20-20-20 rule.

[![GitHub Release](https://img.shields.io/github/v/release/moansari1234/blink?color=green&logo=github)](https://github.com/moansari1234/blink/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-0078D6.svg)](https://microsoft.com/windows)
[![Built with: Rust](https://img.shields.io/badge/Built%20with-Rust%20%2B%20Tauri%20v2-dea584.svg)](https://tauri.app)

---

## 📥 Quick Download (v1.0.1)

| Package | Format | Direct Download |
| :--- | :--- | :--- |
| **Windows Installer (Recommended)** | `.msi` | [⬇️ Download `Blink_1.0.1_x64_en-US.msi`](https://github.com/moansari1234/blink/releases/download/v1.0.1/Blink_1.0.1_x64_en-US.msi) |
| **NSIS Setup Wizard** | `.exe` | [⬇️ Download `Blink_1.0.1_x64-setup.exe`](https://github.com/moansari1234/blink/releases/download/v1.0.1/Blink_1.0.1_x64-setup.exe) |

*All release assets and changelogs are available on the [Releases Page](https://github.com/moansari1234/blink/releases).*

---

## 📖 The 20-20-20 Rule

Digital eye strain (Computer Vision Syndrome) affects millions of people who look at screens for prolonged periods. The **20-20-20 rule** is an ophthalmologist-recommended practice:

> **Every 20 minutes**, look at something at least **20 feet away** for at least **20 seconds**.

This allows your eye focusing muscles (ciliary muscles) to completely relax and prevents headaches, dry eyes, and fatigue.

---

## ✨ Features

- **⚡ Ultra-Lightweight**: Uses `< 38 MB` RAM and `< 0.1%` CPU. Runs quietly in your system tray.
- **🔒 100% Offline & Private**: Zero telemetry, zero tracking, zero network connections.
- **🎨 Windows 11 Fluent Design**: Modern mica-inspired settings interface styled with Windows 11 design tokens.
- **⏱ Live Tray Countdown**: Right-click the system tray icon anytime to see your live remaining time (`⏱ 14:32 remaining`).
- **🟢🟡🔴 Dynamic Status Icons**:
  - 🟢 **Green**: Timer actively running
  - 🟡 **Yellow**: Paused (idle detected or manually paused)
  - 🔴 **Red**: Break time!
- **💤 Smart Idle Detection**: Pauses automatically when you are away from keyboard/mouse or when your screen is locked, and resumes exactly where you left off.
- **🔔 Configurable Notifications**:
  - **Native Windows Toast**: Clean OS notifications.
  - **System Tray Balloon**: Lightweight tray notification.
  - **Interactive Overlay**: Gentle screen corner prompt with direct **Snooze** & **Dismiss** buttons.
- **🔕 Windows Focus Assist Aware**: Respects "Do Not Disturb" / Focus Assist modes so it won't interrupt high-stakes presentations or full-screen gaming sessions.
- **🔔 Gentle Bell Chime**: Calming, soft harmonic chime that alerts you without jarring loud beeps. Volume adjustable or muteable.
- **🔄 Instant Hot-Reload**: Settings changes take effect immediately without restarting the app. You can also edit `%APPDATA%\Blink\config.json` directly!
- **🚀 Auto-Start on Login**: Optionally launches minimized to tray when your PC boots.

---

## ⚙️ Configuration

Blink stores its settings in standard JSON format at:
```text
%APPDATA%\Blink\config.json
```

### Example `config.json`:
```json
{
  "work_duration_minutes": 20,
  "break_duration_seconds": 20,
  "notification_style": "toast",
  "idle_detection_enabled": true,
  "idle_threshold_seconds": 120,
  "auto_start": true,
  "sound_enabled": true,
  "sound_volume": 0.5,
  "snooze_duration_minutes": 5,
  "respect_focus_assist": true
}
```

---

## 🛠️ Tech Stack

- **Backend**: Rust 2021 Edition
- **App Framework**: Tauri v2 (`tray-icon`, `tauri-plugin-notification`, `tauri-plugin-autostart`, `tauri-plugin-single-instance`)
- **Idle Detection**: Windows Win32 API (`GetLastInputInfo`, `SHQueryUserNotificationState`)
- **Audio Engine**: `rodio` with embedded Vorbis/WAV audio synthesis
- **Frontend**: Vanilla HTML5, CSS3 (Fluent Design Tokens), JavaScript (No bulky node_modules runtime)

---

## 🤝 Contributing

Contributions are welcome! Please check out [CONTRIBUTING.md](CONTRIBUTING.md) to set up your development environment.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
