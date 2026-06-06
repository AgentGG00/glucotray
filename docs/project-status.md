# Project Status – GlucoTray

## Checklist

### Init
- [x] Repository created
- [x] Codespace configured
- [x] Project structure created
- [x] README.md
- [x] LICENSE (MIT)
- [x] .gitignore
- [x] CONTRIBUTING.md
- [x] projekt-plan.md / projekt-plan-en.md
- [x] Tauri project initialized
- [x] Svelte frontend initialized
- [x] GitHub Actions workflow created

### Backend
- [x] Dexcom Share API connection
- [x] OS Keychain integration (Windows + Linux)
- [x] DB - Worker
- [x] SQLite database setup
- [x] Polling logic (150s interval)
- [x] Error handling
- [x] Autostart (Windows + Linux via tauri-plugin-autostart)
- [x] mg/dL als interne Einheit, einmalige Umrechnung in db.rs
- [x] AppState mit unit-Setting

### Frontend
- [x] Tray icon with live value
- [x] Trend arrow display (Unicode)
- [x] Color scheme logic (zone-based, klinisch korrekt)
- [ ] Unit toggle (mg/dL / mmol/L) im Settings-Fenster

### Features
#### feat: Wizard
- [x] G6 / G7 selection screen
- [x] Prerequisites checklist
- [x] Credentials input
- [x] Live API validation
- [x] Settings screen (unit, thresholds, color scheme, autostart)
- [x] Completion screen
- [x] App-Neustart nach Wizard-Abschluss

#### feat: Tray
- [x] Dynamisches Tray-Icon mit Live-Wert
- [x] Trendpfeil (Unicode)
- [x] Farbschema-Logik (5 Zonen)
- [x] Kontextmenü (GlucoTray / Update check / Quit / Restart)
- [x] Erster-Start-Notification (Tray pinnen)
- [x] Update-Badge vorbereitet
- [ ] Update check Logik (vor erstem Public Release)

#### feat: Settings
- [x] Autostart toggle
- [x] Unit selection
- [x] Threshold configuration
- [x] Color scheme configuration
- [ ] Settings-Fenster (noch zu bauen)

#### feat: Error Handling
- [ ] Invalid credentials
- [ ] No session
- [ ] No readings
- [ ] Timeout
- [ ] Rate limit

### Distribution
- [ ] GitHub Actions release workflow
- [ ] .exe installer build
- [ ] .AppImage build
- [ ] MSIX build
- [ ] Microsoft Store submission
- [ ] Flathub submission
- [ ] pling.com listing
- [ ] GitHub Pages live

### Test / Review
- [ ] Wizard flow tested end-to-end
- [ ] API error cases tested
- [ ] Windows 11 tested
- [ ] Linux KDE Plasma tested