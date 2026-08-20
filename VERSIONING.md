# 🏷️ Blink Versioning & Release Guide

This document defines the versioning scheme, automation rules, and release workflow for the Blink project.

---

## 📌 Three-Tier Versioning Scheme

Blink follows Semantic Versioning (`MAJOR.MINOR.PATCH` formatted as `vX.Y.Z`).

The **Git tag** and the **App Version** (in `Cargo.toml`, `tauri.conf.json`, `index.html`, and `main.js`) **must always be identical**.

| Level | Name | Version Format | Change Description | CI/CD Auto-Build? | Example |
| :---: | :---: | :---: | :--- | :---: | :--- |
| **Level 3** | **Patch** | `vX.Y.(Z+1)` | Small tweaks, typos, internal refactoring, non-breaking bug fixes | ❌ **No Auto-Build** *(Fast CI tests only; manual build optional)* | `v1.1.0` ➔ `v1.1.1` |
| **Level 2** | **Minor** | `vX.(Y+1).0` | Moderate updates, new features (e.g. updater UI, settings), non-breaking UI overhauls | ⚡ **AUTO-BUILD** *(Creates `.msi` & `.exe` releases on GitHub)* | `v1.1.0` ➔ `v1.2.0` |
| **Level 1** | **Major** | `v(X+1).0.0` | Large architectural shifts, cross-platform porting (macOS/Linux), breaking format changes | ⚡ **AUTO-BUILD** *(Creates `.msi` & `.exe` releases on GitHub)* | `v1.2.0` ➔ `v2.0.0` |

---

## ⚙️ How Automation Works

GitHub Actions uses the tag pattern `'v*.*.0'` to distinguish between auto-build releases and patch-only tags:

- **Level 1 & Level 2 releases always end in `.0`** (`v1.2.0`, `v2.0.0`).
  - When pushed to GitHub, `.github/workflows/release.yml` immediately triggers.
  - Generates `Blink_X.Y.Z_x64_en-US.msi` and `Blink_X.Y.Z_x64-setup.exe`.
  - Publishes them directly to [GitHub Releases](https://github.com/moansari1234/blink/releases).
  - The [README.md](README.md) dynamic badge and download links automatically update to point to the newest release.
- **Level 3 patches end in a non-zero number** (`v1.1.1`, `v1.1.2`).
  - When pushed to GitHub, only `.github/workflows/ci.yml` runs fast unit tests (~30–45s).
  - No installer build is triggered, saving build minutes.
  - If you ever want to build installers for a patch release, you can click **"Run workflow"** manually in GitHub Actions UI.

---

## 🚀 How to Ship a Release

### Step 1: Update App Version
Ensure the version string is updated to match across all 4 project files:
1. `src-tauri/Cargo.toml` (`version = "X.Y.Z"`)
2. `src-tauri/tauri.conf.json` (`"version": "X.Y.Z"`)
3. `src/index.html` (`Version X.Y.Z`)
4. `src/main.js` (`const CURRENT_VERSION = 'X.Y.Z'`)

### Step 2: Commit & Push Code
```bash
git add .
git commit -m "feat: description of changes (v1.2.0)"
git push origin main
```

### Step 3: Push the Tag

#### For Level 1 or Level 2 (Auto-Build):
```bash
git tag v1.2.0
git push origin v1.2.0
```
> GitHub Actions will build both `.msi` and `.exe` in ~2–3 minutes and publish the release!

#### For Level 3 (Patch / No-Build):
```bash
git tag v1.1.1
git push origin v1.1.1
```
> Fast CI tests will run. No installers will be built unless manually triggered.
