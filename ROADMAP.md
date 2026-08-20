# 🗺️ Blink — Future Updates Roadmap

> A detailed, versioned roadmap for the Blink desktop app.
> Each release follows the [VERSIONING.md](VERSIONING.md) scheme:
> - **Level 1** (`vX.0.0`) — Major releases with auto-build
> - **Level 2** (`vX.Y.0`) — Feature releases with auto-build
> - **Level 3** (`vX.Y.Z`) — Patch releases, manual build only

---

## 🟢 v1.2.x — Quality of Life

### v1.2.0 ⚡ Auto-Build
> **Theme**: Dark/Light Mode & Break Tracking Foundation

- [ ] **Dark/Light Mode Toggle** — Add a manual toggle switch in Settings → Behavior tab. Currently Blink only follows the Windows system theme; this lets users override it. Store preference in `config.json` as `"theme": "system" | "dark" | "light"`.
- [ ] **Break History Tracking (Backend)** — Record every completed break as a timestamped entry in a local `%APPDATA%\Blink\history.json` file. Each entry stores: `timestamp`, `break_duration_seconds`, `was_snoozed`, `was_dismissed`. Cap file at 90 days of history with automatic pruning on startup.
- [ ] **Break Stats Dashboard (Frontend)** — New "Stats" tab in Settings showing: breaks taken today, breaks taken this week, average breaks per day, longest streak, and a simple 7-day bar chart rendered with HTML5 `<canvas>`. No external chart library needed.

### v1.2.1
> **Theme**: Custom Break Messages

- [ ] **Custom Break Message** — Add a text field in Settings → Notifications tab: "Break Message". Users can type their own reminder text (e.g., "Stand up and stretch!", "Look out the window 🌳"). Store in `config.json` as `"break_message": "string"`. Default: `"Time for a 20-second break! Look at something 20 feet away."`. Display this message in toast notifications, tray balloons, and the overlay window.
- [ ] **Message Rotation** — Allow users to enter multiple messages separated by `|` (pipe). Blink randomly picks one each break. Example: `"Stretch your legs|Look outside|Blink 20 times"`.

### v1.2.2
> **Theme**: Keyboard Shortcuts

- [ ] **Global Hotkey: Pause/Resume** — Register a system-wide hotkey (default: `Ctrl+Shift+B`) using the Win32 `RegisterHotKey` API. Pressing it toggles pause/resume without opening the tray menu or settings window. Configurable in Settings → Behavior tab.
- [ ] **Global Hotkey: Skip Break** — Register a secondary hotkey (default: `Ctrl+Shift+N`) to dismiss the current break notification instantly.
- [ ] **Hotkey Conflict Detection** — If the chosen hotkey is already registered by another app, show a warning toast and fall back to the default.

### v1.2.3
> **Theme**: Tray & Overlay Polish

- [ ] **Tray Tooltip on Hover** — Show remaining time as a native Windows tooltip when hovering over the tray icon (e.g., `"Blink — 14:32 remaining"`). Uses `NOTIFYICONDATA.szTip` Win32 field, updated every 15 seconds.
- [ ] **Multi-Monitor Overlay Positioning** — Add a dropdown in Settings → Notifications: "Show overlay on: Primary Monitor / Monitor 2 / Monitor 3 / All Monitors". Use Tauri's `available_monitors()` API to enumerate displays and position the overlay window accordingly.
- [ ] **Overlay Animation** — Smooth 300ms fade-in and fade-out CSS transition on the overlay window instead of instant show/hide. Add `opacity` and `transform: translateY` animation.

---

## 🟡 v1.3.x — Personalization & Motivation

### v1.3.0 ⚡ Auto-Build
> **Theme**: Custom Sounds & Break Streaks

- [ ] **Custom Sound File** — Add a "Browse..." button in Settings → Notifications tab that lets users pick a custom `.wav` or `.mp3` file from their filesystem. Store the absolute path in `config.json` as `"custom_sound_path": "string | null"`. If set, `rodio` loads and plays this file instead of the embedded bell chime. Validate file exists on each app start; fall back to built-in chime if missing.
- [ ] **Sound Preview** — The existing "Test Sound" button plays the custom sound if one is set, or the default chime otherwise.
- [ ] **Break Streak Counter** — Track consecutive breaks taken without dismissing. Display in the tray menu: `"🔥 Streak: 12 breaks"`. Reset streak to 0 if a break is dismissed or snoozed past the follow-up window. Store current streak and best streak in `history.json`.
- [ ] **Streak Milestone Toasts** — Show a celebratory toast at streak milestones: 5, 10, 25, 50, 100. Example: `"🎉 Amazing! 25 breaks in a row — your eyes thank you!"`.

### v1.3.1
> **Theme**: Pomodoro Mode

- [ ] **Pomodoro Timer Mode** — Add a toggle in Settings → Timer tab: "Timer Mode: 20-20-20 / Pomodoro". When Pomodoro is active: 25 min work → 5 min short break → repeat 4 times → 15 min long break. All durations are configurable. The tray icon cycle count shows `"Cycle 3/4"` in the menu.
- [ ] **Pomodoro Cycle Counter** — Display current cycle number in the tray menu and overlay. After 4 cycles, show a distinct "Long Break" notification with a different color overlay (blue instead of red).

### v1.3.2
> **Theme**: Quiet Hours & Scheduling

- [ ] **Scheduled Quiet Hours** — Add a "Quiet Hours" section in Settings → Behavior tab with start time and end time pickers (e.g., 12:00 PM – 1:00 PM). During quiet hours, Blink auto-pauses the timer and sets the tray icon to yellow. Store as `"quiet_hours": { "enabled": true, "start": "12:00", "end": "13:00" }` in `config.json`.
- [ ] **Multiple Quiet Periods** — Support up to 3 quiet hour ranges (e.g., lunch break, evening wind-down, meeting block).
- [ ] **Day-of-Week Filter** — Let users specify which days quiet hours apply (e.g., weekdays only, weekends only, or specific days).

### v1.3.3
> **Theme**: Config Sharing

- [ ] **Config Export** — "Export Settings" button in Settings → About tab. Saves a copy of `config.json` to a user-chosen location via a native Save File dialog. Strips any machine-specific paths (like custom sound paths) and replaces with placeholders.
- [ ] **Config Import** — "Import Settings" button that opens a native File Open dialog. Validates JSON structure, sanitizes values using existing `BlinkConfig::sanitize()`, applies immediately, and shows a success toast.
- [ ] **Config Reset Confirmation** — The existing "Defaults" button now shows a confirmation dialog before resetting all settings.

---

## 🔵 v1.4.x — Accessibility & Wellness

### v1.4.0 ⚡ Auto-Build
> **Theme**: Screen Dimming & Eye Exercises

- [ ] **Screen Dimming During Breaks** — Create a fullscreen transparent overlay window that gradually increases opacity from 0% to 60% over the 20-second break duration, naturally encouraging users to look away from the screen. Configurable max dimness (30%, 50%, 70%, 90%) in Settings → Notifications tab.
- [ ] **Eye Exercise Prompts** — During breaks, the overlay shows guided micro-exercises with animated text prompts cycling every 5 seconds: "Look far left ← ... Look far right → ... Look up ↑ ... Look down ↓ ... Blink slowly 5 times". Toggle on/off in Settings → Notifications.
- [ ] **Exercise Variety** — Rotate between 3 exercise sets across breaks so users don't see the same sequence every time.

### v1.4.1
> **Theme**: Hydration & Posture Reminders

- [ ] **Hydration Reminder** — Optional secondary timer (default: every 45 minutes) that shows a gentle toast: "💧 Time to drink some water!". Independent of the 20-20-20 timer. Configurable interval and toggle in Settings → Behavior tab. Store as `"hydration_reminder": { "enabled": false, "interval_minutes": 45 }`.
- [ ] **Posture Check Reminder** — Optional tertiary timer (default: every 30 minutes) with a toast: "🪑 Check your posture — sit up straight!". Same configurable pattern as hydration. Both reminders respect Focus Assist and quiet hours.
- [ ] **Combined Reminders** — If a hydration/posture reminder coincides with a break, combine them into a single notification to avoid notification fatigue.

### v1.4.2
> **Theme**: Accessibility Compliance

- [ ] **High Contrast Mode** — Detect Windows High Contrast setting and switch to a high-contrast color palette with bold borders, larger text, and maximum contrast ratios (WCAG AAA). Add manual toggle in Settings → Behavior.
- [ ] **Large Text Mode** — Scale all UI text by 125% or 150% via a dropdown. Useful for users with low vision or high-DPI displays where default text feels small.
- [ ] **Screen Reader Support (ARIA)** — Add `role`, `aria-label`, `aria-live`, and `tabindex` attributes to all interactive elements in `index.html` and `snooze.html`. Ensure full keyboard navigation (Tab, Enter, Escape) works throughout the settings UI. Test with Windows Narrator.
- [ ] **Reduced Motion** — Respect `prefers-reduced-motion` CSS media query. Disable overlay fade animations and any future motion effects for users who have enabled "Show animations in Windows" → Off.

---

## 🟣 v1.5.x — Power User Features

### v1.5.0 ⚡ Auto-Build
> **Theme**: CLI & Timer Profiles

- [ ] **CLI Mode** — Support command-line arguments for headless control:
  - `blink --status` → Print current timer state, remaining time, and config to stdout as JSON
  - `blink --pause` / `blink --resume` → Send pause/resume commands via a named pipe or localhost socket to the running Blink instance
  - `blink --reset` → Reset the timer
  - `blink --config` → Print the current config.json path
  - Uses Tauri's single-instance plugin to communicate with the running app.
- [ ] **Multiple Timer Profiles** — Create and save named profiles (e.g., "Coding" = 25 min, "Reading" = 15 min, "Gaming" = 45 min). Quick-switch via tray menu: `Profiles → Coding ✓ / Reading / Gaming`. Store profiles in `%APPDATA%\Blink\profiles\` as separate JSON files. Each profile overrides work duration, break duration, notification style, and sound settings.
- [ ] **Profile Hotkeys** — Assign global hotkeys to switch profiles instantly (e.g., `Ctrl+Shift+1` for Profile 1).

### v1.5.1
> **Theme**: Reports & Analytics

- [ ] **Weekly Summary Report** — Every Sunday at midnight (or configurable day/time), generate a local markdown file at `%APPDATA%\Blink\reports\week-YYYY-WW.md` containing: total breaks taken, breaks dismissed, average streak, most productive day, total break time, and a daily breakdown table.
- [ ] **Monthly Summary** — Same format, generated on the 1st of each month. Includes week-over-week trend comparison.
- [ ] **"View Reports" Button** — In Settings → About tab, a button that opens the reports folder in Windows Explorer.

### v1.5.2
> **Theme**: Auto-Update & Startup

- [ ] **Auto-Update In-Place** — When "Check for Updates" finds a newer version, add a "Download & Install" button that: downloads the `.msi` to a temp folder, launches the installer silently with `msiexec /i ... /passive`, and exits the current app. The MSI installer handles replacing the old binary. Show a progress bar during download.
- [ ] **Startup Delay Option** — New setting: "Delay timer start after login by X seconds" (default: 0, range: 0–300). Prevents the first break notification from firing during the boot-up period when users are getting settled. Store as `"startup_delay_seconds": 0` in `config.json`.
- [ ] **First-Run Welcome Screen** — On first launch (no `config.json` exists), show a friendly onboarding overlay explaining the 20-20-20 rule and letting users configure the 3 most important settings (interval, notification style, sound on/off) before starting.

### v1.5.3
> **Theme**: Windows Integration

- [ ] **Windows 11 Widget** — Create a Widgets Board widget showing a live countdown circle, current streak, and a quick pause button. Uses the Windows Widget Provider API (requires Win32 adaptive card registration).
- [ ] **Windows Action Center Integration** — Break notifications appear as actionable cards in Action Center with inline "Snooze 5 min" and "Dismiss" buttons (already partially supported via toast, but needs explicit action registration).
- [ ] **Taskbar Progress Bar** — Show the timer progress as a subtle green progress bar on the Blink taskbar icon using `ITaskbarList3::SetProgressValue`. Fills from 0% to 100% over the work interval.

---

## 🔴 v2.0.0 — Cross-Platform (Level 1 — Auto-Build)

### v2.0.0 ⚡ Auto-Build
> **Theme**: macOS & Linux Support

- [ ] **macOS Support** — Compile for `aarch64-apple-darwin` and `x86_64-apple-darwin`. Generate `.dmg` installer via Tauri's built-in macOS bundler. Implement macOS menu bar integration using Tauri's tray API. Replace Win32 idle detection with `CGEventSourceSecondsSinceLastEventType` (CoreGraphics). Replace Win32 Focus Assist with macOS "Do Not Disturb" detection via `NSDoNotDisturbEnabled` user defaults key.
- [ ] **Linux Support** — Compile for `x86_64-unknown-linux-gnu`. Generate `.AppImage` and `.deb` packages. System tray via `libappindicator3` or `StatusNotifierItem` (SNI) D-Bus protocol. Idle detection via `XScreenSaverQueryInfo` (X11) or `org.gnome.Mutter.IdleMonitor` (Wayland). DND detection via `org.freedesktop.Notifications` D-Bus hints.
- [ ] **Unified CI Pipeline** — Single GitHub Actions workflow with a build matrix (`windows-latest`, `macos-latest`, `ubuntu-latest`) that produces platform-specific installers and uploads all 3 to the same GitHub Release page.
- [ ] **Platform-Native Look & Feel** — Fluent Design on Windows, macOS HIG styling (SF Pro font, vibrancy) on Mac, GTK/Adwaita styling on Linux. Achieved via platform-specific CSS files loaded at runtime based on `std::env::consts::OS`.

---

## 💡 Ideas Backlog (Unscheduled)

| Idea | Description | Complexity |
| :--- | :--- | :---: |
| **Sync via GitHub Gist** | Backup/restore config across machines using a private Gist (opt-in, still offline-first) | Medium |
| **Focus Session Mode** | Deep work mode: extends work intervals to 45 min and batches 3 breaks into one 2-minute stretch | Low |
| **Calendar Integration** | Auto-pause during Google Calendar / Outlook events via OAuth or local `.ics` file parsing | High |
| **Wearable Companion** | Send BLE vibration to a smartwatch when break starts (requires platform-specific BLE stack) | High |
| **Theming Engine** | Let users create and share custom CSS themes (`.blink-theme` files) for the settings UI | Medium |
| **Localization (i18n)** | Translate all UI strings into Spanish, Arabic, Hindi, Chinese, Japanese, French, German. Use a `locales/` folder with JSON translation files and a language picker in Settings | Medium |
| **Plugin System** | Community Rust/JS plugins that hook into `on_break_start`, `on_break_end`, `on_snooze` events | High |
| **Ambient Break Sounds** | Play calming nature sounds (rain, birds, ocean waves) during the 20-second break. Ship 3 built-in clips (~50KB each) and let users add custom ambient files | Low |
| **Team Mode** | Shared break sync for small teams — everyone's break fires simultaneously via a lightweight WebSocket relay | High |
| **Gamification** | Earn badges and achievements for milestones (100 breaks, 7-day streak, etc.) displayed in the About tab | Low |

---

## 📋 How to Use This Roadmap

1. Items are grouped by **target release version** following our [VERSIONING.md](VERSIONING.md) scheme.
2. Level 2 releases (`vX.Y.0`) marked with ⚡ trigger automatic GitHub Actions builds.
3. Level 3 releases (`vX.Y.Z`) are patch updates — tested automatically, built manually if needed.
4. Check off items `[x]` as they are completed and commit the updated roadmap.
5. Community contributions welcome for any item — see [CONTRIBUTING.md](CONTRIBUTING.md).

---

*Last updated: v1.1.0*
