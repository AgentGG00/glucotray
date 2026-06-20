# GlucoTray

[![Build Status](https://img.shields.io/github/actions/workflow/status/AgentGG00/glucotray/tauri-build.yml?label=build)](https://github.com/AgentGG00/glucotray/actions)
[![Version](https://img.shields.io/github/v/release/AgentGG00/glucotray)](https://github.com/AgentGG00/glucotray/releases)
[![Status](https://img.shields.io/badge/status-WIP-yellow)]()
[![License](https://img.shields.io/github/license/AgentGG00/glucotray)](https://github.com/AgentGG00/glucotray/blob/main/LICENSE)

A lightweight system tray app for Windows and Linux that displays live blood glucose readings from Dexcom CGM sensors.

## Features

- Live blood glucose display in the system tray
- Trend arrows and configurable color scheme
- mg/dL and mmol/L support
- Guided setup wizard for Dexcom Share
- Autostart on login
- Windows 11 and Linux KDE Plasma support

## Installation

### Windows
Download the latest `.exe` installer from [Releases](https://github.com/AgentGG00/glucotray/releases).

### Linux
Download the latest `.AppImage` from [Releases](https://github.com/AgentGG00/glucotray/releases).

```bash
chmod +x GlucoTray.AppImage
./GlucoTray.AppImage
```

## Local Development

1. Clone the repo
2. Copy `.env.example` to `.env` and fill in real values
3. Install dependencies: `pnpm install`
4. Start dev server: `pnpm tauri dev`

## License

MIT © AgentGG00