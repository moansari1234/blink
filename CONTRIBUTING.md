# Contributing to Blink 👁

Thank you for your interest in contributing to Blink! We aim to keep Blink lightweight, fast, rock-solid, and completely offline.

---

## 🛠️ Prerequisites

To build Blink on Windows:

1. **Visual Studio 2022 Build Tools** (or Community/Pro/Enterprise)
   - Ensure the **"Desktop development with C++"** workload is installed.
2. **Rust (MSVC toolchain)**
   - Install via [rustup.rs](https://rustup.rs/):
     ```cmd
     winget install Rustlang.Rustup
     rustup default stable-x86_64-pc-windows-msvc
     ```
3. **Node.js** (v18+) & Tauri CLI:
   - Install Tauri CLI:
     ```cmd
     cargo install tauri-cli --version "^2.0"
     ```

---

## 🚀 Development Workflow

### 1. Clone the repository
```bash
git clone https://github.com/moansari1234/blink.git
cd blink
```

### 2. Run in Development Mode
To run the app with live hot-reloading:
```bash
cargo tauri dev
```
Blink will compile and start minimized to your Windows system tray. Look for the eye icon near the system clock.

### 3. Run Unit Tests
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### 4. Code Formatting & Linting
```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

---

## 📦 Building a Release Installer

To build the standalone `.exe` and the WiX `.msi` installer:
```bash
cargo tauri build
```
The compiled output will be placed in:
`src-tauri/target/release/bundle/msi/`

For our release lifecycle and version numbering standards, see [VERSIONING.md](VERSIONING.md).

---

## 📐 Project Architecture

```text
blink/
├── src/                       # Frontend Settings & Notification UI
│   ├── index.html             # Settings window UI (HTML5)
│   ├── style.css              # Fluent Design CSS variables & tokens
│   ├── main.js                # IPC communication bridge with Rust
│   ├── snooze.html            # Overlay notification popup UI
│   ├── snooze.css             # Overlay styling
│   └── snooze.js              # Overlay button interaction & auto-dismiss
├── src-tauri/                 # Rust Backend
│   ├── src/
│   │   ├── main.rs            # Application bootstrap & event loop
│   │   ├── timer.rs           # Core 20-20-20 countdown engine
│   │   ├── idle.rs            # Win32 idle & Focus Assist detection
│   │   ├── notification.rs    # Toast, balloon & overlay dispatcher
│   │   ├── audio.rs           # Soft bell chime playback engine
│   │   ├── config.rs          # Config validation, save/load & hot-reload
│   │   ├── tray.rs            # System tray with live countdown & icons
│   │   └── commands.rs        # Tauri IPC commands
│   ├── icons/                 # State icons (green, yellow, red)
│   ├── sounds/                # Embedded audio chime
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri v2 bundle configuration
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

---

## 💡 Guidelines for Pull Requests

1. **Keep it lightweight**: Avoid heavy dependencies or large frontend frameworks (React/Vue/Angular are not needed for a settings dialog).
2. **Privacy First**: Blink must remain 100% offline. No network requests or analytics.
3. **Write Tests**: Any changes to interval math, config serialization, or timer states must have unit test coverage.
4. **Follow Windows 11 Human Interface Guidelines**: Keep UI elements consistent with Fluent Design.
