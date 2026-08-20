# 🗺️ Blink — Future Updates Roadmap

> A prioritized list of planned features, improvements, and ideas for future Blink releases.

---

## 🟢 v1.2.0 — Quality of Life (Level 2 — Auto-Build)

- [ ] **Dark/Light Mode Toggle** — Add a manual toggle in Settings (currently follows system theme only)
- [ ] **Break History & Stats Dashboard** — Track how many breaks you've taken today/this week with a simple bar chart in the About tab
- [ ] **Custom Break Messages** — Let users write their own reminder text (e.g., "Stand up and stretch!" instead of the default)
- [ ] **Multi-Monitor Overlay Positioning** — Let users choose which monitor the overlay notification appears on
- [ ] **Keyboard Shortcut for Pause/Resume** — Global hotkey (e.g., `Ctrl+Shift+B`) to pause/resume without opening the tray menu
- [ ] **Tray Tooltip Preview** — Show remaining time on hover over the tray icon (not just in the right-click menu)

---

## 🟡 v1.3.0 — Personalization & Polish (Level 2 — Auto-Build)

- [ ] **Custom Sounds** — Let users pick their own `.wav` or `.mp3` chime file instead of the built-in bell
- [ ] **Break Streak Counter** — Show a motivational streak ("🔥 12 breaks in a row!") to encourage consistency
- [ ] **Pomodoro Mode** — Optional alternate mode: 25 min work / 5 min break / 15 min long break after 4 cycles
- [ ] **Scheduled Quiet Hours** — Set time ranges (e.g., 12:00–1:00 PM lunch) where Blink auto-pauses
- [ ] **Animated Overlay Transitions** — Smooth fade-in/fade-out for the screen overlay instead of instant show/hide
- [ ] **Config Import/Export** — One-click export `config.json` to share settings with friends, and import theirs

---

## 🔵 v1.4.0 — Accessibility & Health (Level 2 — Auto-Build)

- [ ] **Screen Dimming Mode** — Gradually dim the screen during breaks to naturally encourage looking away
- [ ] **Eye Exercise Prompts** — Optional guided micro-exercises during breaks (e.g., "Look left → right → up → down")
- [ ] **Hydration Reminder** — Optional secondary timer that reminds you to drink water every X minutes
- [ ] **Posture Check Reminder** — Optional nudge to check your sitting posture at configurable intervals
- [ ] **High Contrast / Large Text Mode** — Accessibility improvements for visually impaired users
- [ ] **Screen Reader Support (ARIA)** — Ensure all UI elements are navigable via Windows Narrator

---

## 🟣 v1.5.0 — Power User Features (Level 2 — Auto-Build)

- [ ] **CLI Mode** — `blink --status`, `blink --pause`, `blink --reset` commands for terminal power users
- [ ] **Multiple Timer Profiles** — Save and switch between profiles (e.g., "Coding" = 25 min, "Reading" = 15 min, "Gaming" = 30 min)
- [ ] **Weekly Summary Report** — Generate a local markdown report of your break compliance for the week
- [ ] **Auto-Update In-Place** — Download and replace the app binary automatically (not just notify, actually update)
- [ ] **Startup Delay Option** — Wait X seconds after login before starting the timer (avoids alerts during boot)
- [ ] **Windows Widget Integration** — Show a live countdown widget on the Windows 11 Widgets board

---

## 🔴 v2.0.0 — Cross-Platform (Level 1 — Auto-Build)

- [ ] **macOS Support** — Native `.dmg` installer with macOS menu bar integration and native notifications
- [ ] **Linux Support** — `.AppImage` / `.deb` packages with system tray via `libappindicator`
- [ ] **Unified Codebase** — Single Rust + Tauri codebase that compiles for Windows, macOS, and Linux from one CI pipeline
- [ ] **Platform-Native Look & Feel** — Respect each OS's design language (Fluent on Windows, macOS HIG on Mac, GTK on Linux)

---

## 💡 Ideas Backlog (Unscheduled)

| Idea | Description |
| :--- | :--- |
| **Sync Settings via GitHub Gist** | Backup/restore config across machines using a private Gist (still offline-first, opt-in only) |
| **Focus Session Mode** | Deep work mode that extends intervals and batches breaks |
| **Calendar Integration** | Auto-pause during calendar events (Google Calendar / Outlook) |
| **Wearable Companion** | Vibrate a smartwatch when it's break time |
| **Theming Engine** | Let users create and share custom CSS themes for the settings UI |
| **Localization (i18n)** | Translate UI into Spanish, Arabic, Hindi, Chinese, Japanese, etc. |
| **Plugin System** | Let community developers write Rust/JS plugins that hook into break events |
| **Ambient Sound During Breaks** | Play nature sounds (rain, birds, ocean) during the 20-second break |

---

## 📋 How to Use This Roadmap

1. Items are grouped by **target release version** following our [VERSIONING.md](VERSIONING.md) scheme.
2. Within each version, items are roughly ordered by priority (top = highest).
3. Move items between versions as priorities shift.
4. Check off items `[x]` as they are completed.
5. Community contributions are welcome for any item — see [CONTRIBUTING.md](CONTRIBUTING.md).

---

*Last updated: v1.1.0*
