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
- [ ] Tauri project initialized
- [ ] Svelte frontend initialized
- [ ] GitHub Actions workflow created

### Backend
- [ ] Dexcom Share API connection
- [ ] OS Keychain integration (Windows + Linux)
- [ ] SQLite database setup
- [ ] Polling logic (150s interval)
- [ ] Error handling

### Frontend
- [ ] Tray icon with live value
- [ ] Trend arrow display
- [ ] Color scheme logic
- [ ] Unit toggle (mg/dL / mmol/L)

### Features
#### feat: Wizard
- [ ] G6 / G7 selection screen
- [ ] Prerequisites checklist
- [ ] Credentials input
- [ ] Live API validation
- [ ] Settings screen (unit, thresholds, color scheme, autostart)
- [ ] Completion screen

#### feat: Settings
- [ ] Autostart toggle
- [ ] Unit selection
- [ ] Threshold configuration
- [ ] Color scheme configuration

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
- [ ] Linux KDE Plasma testedw