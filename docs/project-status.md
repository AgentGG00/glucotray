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
- [x] Error handling (typed AppError, internet check, retry logic per error type)
- [x] Autostart (Windows + Linux via tauri-plugin-autostart)
- [x] mg/dL as internal unit, single conversion point in db.rs
- [x] AppState with unit setting
- [x] Legal document reading from bundled resources (`read_legal_document`)
- [x] Legal acceptance storage (`save_legal_acceptance`)

### Frontend
- [x] Tray icon with live value
- [x] Trend arrow display (Unicode)
- [x] Color scheme logic (zone-based, clinically correct)
- [ ] Unit toggle (mg/dL / mmol/L) in settings window

### Features
#### feat: Wizard
- [x] Step 0: legal document acceptance (privacy policy, terms of use, disclaimer), one document at a time with individual accept buttons
- [x] G6 / G7 selection screen
- [x] Prerequisites checklist
- [x] Credentials input
- [x] Live API validation
- [x] Settings screen (unit, thresholds, color scheme, autostart)
- [x] Completion screen
- [x] App restart after wizard completion
- [x] Localized plain-text error display (de/en/jp) for auth errors

#### feat: Tray
- [x] Dynamic tray icon with live value
- [x] Trend arrow (Unicode)
- [x] Color scheme logic (5 zones)
- [x] Context menu (GlucoTray / Update check / Quit / Restart)
- [x] First-start notification (pin tray icon)
- [x] Update badge prepared
- [ ] Update check logic (before first public release)

#### feat: Settings
- [x] Autostart toggle
- [x] Unit selection
- [x] Threshold configuration
- [x] Color scheme configuration
- [ ] Settings window (still to be built)

#### feat: Error Handling
- [x] Invalid credentials
- [x] No session
- [x] No readings
- [x] Timeout
- [x] Rate limit
- [x] No internet connection (TCP check before auth)

#### feat: Legal
- [x] Privacy policy (de/en/jp)
- [x] Terms of use (de/en/jp)
- [x] Medical disclaimer (de/en/jp)
- [x] Store description disclaimer (short form, de/en/jp)
- [x] Wizard Step 0 for sequential acceptance
- [ ] Re-prompt on document version change (versioned, not yet enforced at startup)

### Distribution
- [ ] GitHub Actions release workflow (tauri-build.yml missing projectPath config — untested)
- [ ] .exe installer build
- [ ] .AppImage build
- [ ] MSIX build
- [ ] Microsoft Store submission
- [ ] Flathub submission
- [ ] pling.com listing
- [ ] GitHub Pages live

### Test / Review
- [ ] Wizard flow tested end-to-end (requires real machine, dev container insufficient)
- [ ] API error cases tested
- [ ] Windows 11 tested
- [ ] Linux KDE Plasma tested