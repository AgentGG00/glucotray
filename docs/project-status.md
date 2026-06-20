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
- [x] mg/dL as internal unit, single conversion point in db.rs (frontend still partially mmol/L-based internally — see issue #12)
- [x] AppState with unit setting
- [x] Legal document reading from bundled resources (`read_legal_document`)
- [x] Legal acceptance storage (`save_legal_acceptance`)
- [x] Settings read/write commands (`get_settings`, `save_settings`)
- [x] Wizard status check command (`get_wizard_status`)
- [x] App stays running in background when window is closed (`CloseRequested` intercepted, window hidden not destroyed)

### Frontend
- [x] Tray icon with live value
- [x] Trend arrow display (Unicode)
- [x] Color scheme logic (zone-based, clinically correct)
- [x] Unit toggle (mg/dL / mmol/L) in settings window

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
- [x] WizardStep2/WizardStep3 reused inside settings window for credential changes (routed via +page.svelte mode state)

#### feat: Tray
- [x] Dynamic tray icon with live value
- [x] Trend arrow (Unicode)
- [x] Color scheme logic (5 zones)
- [x] Context menu (GlucoTray / Update check / Quit / Restart) — "GlucoTray" entry now active (opens window), was previously disabled
- [x] Left-click tray icon handler removed (unreliable on Windows/Linux, only used for icon dragging there); window opening now only via context menu
- [x] First-start notification (pin tray icon)
- [x] Update badge prepared
- [ ] Update check logic (placeholder button wired in settings window, no actual logic yet)

#### feat: Settings
- [x] Settings window built (same Tauri window as wizard, routed via mode state in +page.svelte)
- [x] Autostart toggle (applies on global save, not immediately)
- [x] Unit selection (radio buttons, restart notice shown after save if changed)
- [x] Threshold configuration (same dropdown UI as WizardStep4, pre-filled from DB, closest-match selection if no exact dropdown value)
- [x] Color scheme configuration (same native color picker as WizardStep4, pre-filled from DB)
- [x] Credentials view (username + masked password, both read-only) with "change credentials" flow reusing WizardStep2/WizardStep3
- [x] Legal document viewers (PrivacyPolicy.svelte, TermsOfUse.svelte, Disclaimer.svelte — read-only, no accept button, opened from settings footer)
- [x] Single global "save" button for unit/thresholds/colors/autostart
- [x] Footer links: legal documents (in-app), contact (external), repo (external)
- [ ] Update check button wired but non-functional (placeholder)

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
- [x] Read-only legal document viewers accessible from settings window
- [ ] Re-prompt on document version change (versioned, not yet enforced at startup)

### Known Issues (tracked on GitHub)
- [ ] #11 – Dexcom LOW/HIGH string values not handled (value range mapping + edge case handling needed in dexcom.rs/tray.rs)
- [ ] #12 – Refactor frontend/backend to use mg/dL internally everywhere, convert to mmol/L only for display (currently WizardStep4.svelte/Settings.svelte hold mmol/L as primary state)

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
- [ ] Settings window tested end-to-end (requires real machine)
- [ ] API error cases tested
- [ ] Windows 11 tested
- [ ] Linux KDE Plasma tested