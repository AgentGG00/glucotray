# Project Plan – GlucoTray

## Description

GlucoTray is a lightweight system tray app for Windows 11 and Linux KDE Plasma.
It displays live blood glucose readings from Dexcom CGM sensors directly in the taskbar.

---

## Goals

- Live display of current blood glucose value in the system tray
- Trend arrow and configurable color scheme
- Guided setup wizard for non-technical users
- Platform support: Windows 11 and Linux KDE Plasma
- Open source, MIT licensed

---

## Tech Stack

| Area | Technology | Reason |
|---|---|---|
| Frontend | Svelte + TypeScript + Tailwind CSS | Familiar stack, fast, lightweight |
| Backend | Rust (Tauri 2) | Native tray support, low RAM usage (~30MB), MSIX build out of the box |
| Database | SQLite (local) | History data and settings, no cloud dependency |
| Credentials | OS Keychain | Windows Credential Manager / Linux Secret Service, encrypted |
| API | Dexcom Share API | Only option for live data, community-proven via pydexcom |
| CI/CD | GitHub Actions | Automatic build of .exe, .AppImage, .msix on release tag |

---

## Features – MVP

- [ ] Tray icon with live blood glucose value
- [ ] Trend arrow (rising, falling, stable)
- [ ] Configurable color scheme (green/yellow/red based on thresholds)
- [ ] Selectable unit: mg/dL or mmol/L
- [ ] Polling interval: 150 seconds
- [ ] Autostart on login (Windows + Linux)
- [ ] Setup wizard with Dexcom initialization
- [ ] Error handling with clear messages
- [ ] Settings window

---

## Setup Wizard Flow

1. G6 or G7 selection
2. Prerequisites checklist (app installed, Share active, follower added)
3. Credentials input → stored in OS Keychain
4. Live API validation
5. Settings (unit, thresholds, color scheme, autostart)
6. Done → tray widget active

---

## Error Handling

| Error | Cause | Message |
|---|---|---|
| Invalid credentials | Wrong password or region | "Username or password incorrect. Are you outside the US? Make sure to select 'Outside US'." |
| No session | Share not activated | "No active Share session found. Enable Share in the Dexcom app and add at least one follower." |
| No readings | Sensor not running | "No current readings found. Is your sensor currently active?" |
| Timeout | No internet connection | "Connection to Dexcom API failed. Please check your internet connection." |
| Rate limit | Too many requests | "Too many requests. Please wait a moment and try again." |

---

## Distribution

| Platform | Format | Channel |
|---|---|---|
| Windows | `.exe` Installer | GitHub Releases |
| Windows | MSIX | Microsoft Store |
| Linux | `.AppImage` | GitHub Releases |
| Linux | Flatpak | Flathub (pending review) |
| Listing | Link | pling.com |

---

## Project Page

GitHub Pages via `glucotray.github.io`

---

## Framework

- Development in GitHub Codespaces
- Branch strategy: `main` (stable) + `dev` (active)
- Publisher: AgentGG
- License: MIT
- Microsoft Store name reserved: GlucoTray