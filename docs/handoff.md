# Handoff – GlucoTray

## Project
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **License:** MIT
- **Status:** Wizard + tray complete, autostart implemented, error handling (backend + frontend) complete, legal document acceptance (Step 0) implemented – next goal: settings window

## Stack
- **Frontend:** Svelte 5 + TypeScript + Tailwind CSS
- **Backend:** Rust (Tauri 2)
- **Database:** SQLite (local)
- **Credentials:** OS keychain (Windows Credential Manager / Linux Secret Service)
- **API:** Dexcom Share API, 150s polling

## Branch Strategy
- `main` – stable, releases only
- `dev` – active development
- Feature branches from `dev`: `feature/name`, `fix/name`

## Data & Units
- API always returns mg/dL as Arabic numerals
- Single conversion point: `db.rs` → `insert_reading` writes `value_mgdl` + `value_mmol`
- Thresholds in DB always stored as mg/dL integers (`threshold_low`, `threshold_high`)
- On-the-fly conversion only for frontend display
- Fixed clinical limits: Very High = 250 mg/dL, Critical Low = 54 mg/dL
- API string `"Low"` also triggers Critical Low zone

## AppState
- `unit: String` – loaded from DB at app start
- Unit change in settings requires app restart
- `TrayState` – contains `update_available: bool` for update badge

## Tray Icon
- Dynamically rendered via `imageproc` + `ab_glyph` + NotoSans-Bold.ttf
- Shows value in mg/dL or mmol/L depending on `AppState.unit`
- Trend arrow as Unicode: ⇈ ↑ ↗ → ↘ ↓ ⇊
- 5 color zones: Critical Low / Low / Normal / High / Very High
- N/A when `is_valid = false` or `value = 0`
- Update badge: red dot top-right when `update_available = true`
- Tooltip shows both units: 97 mg/dL / 5.4 mmol/L ↑
- Context menu: GlucoTray (disabled) / Update check / Quit / Restart
- Left click opens settings window (still to be built)

## App Flow

### First Start
App starts → DB empty → window visible → wizard (Step 0 legal, then Steps 1–5) → wizard completed → save_wizard_data → restart_app → restart → normal start

### Normal Start
DB init → unit + autostart from DB → AppState populated → autostart enable/disable → worker starts → tray active → first start: toast notification (one-time, tray_hint_shown in DB)

## Autostart
- `tauri-plugin-autostart` – Windows Registry + Linux .desktop
- Read from DB and applied on every app start
- Settings change requires app restart to take effect

## Error Handling (Backend + Frontend)

### `error.rs`
- `AppError` enum: `InvalidCredentials`, `NoSession`, `NoReadings`, `Timeout`, `RateLimit`, `SessionExpired`, `NoInternetConnection`, `KeychainError`, `DbError`, `NetworkError`, `Unknown`
- `Display` trait provides English plain text (for logs)
- `log()` method with severity mapping (error/warn per variant) via `tracing`
- `From<sqlx::Error>`, `From<keyring::Error>`, `From<reqwest::Error>` (reqwest conversion distinguishes timeout/connect/other network errors)

### `dexcom.rs`
- `check_internet_connection()`: TCP connect to `8.8.8.8:53` (DNS port) with 3s timeout, runs as first step in `authenticate()`
- All methods (`authenticate`, `fetch_session`, `get_readings`) return `Result<_, AppError>` instead of `String`
- HTTP status codes handled distinctly: 401/403 → `InvalidCredentials`, 429 → `RateLimit`, 500 on readings → `SessionExpired` (session auto-renewed)
- Empty readings list → `NoReadings`

### `worker.rs`
- Match logic on `AppError` variants, `e.log()` on every error
- `InvalidCredentials`/`NoSession` during initial login or in the polling loop → worker stops completely (no retry loop, since the problem won't resolve itself – user must use the settings window, then restart the app)
- `RateLimit` → waits out the normal poll interval, does not increment the failure counter
- `SessionExpired` → does not count as a failure, session was already auto-renewed in `get_readings()`
- All other errors (`Timeout`, `NetworkError`, `NoInternetConnection`, `NoReadings`, `Unknown`) go through the existing `failure_count` mechanism, N/A after `MAX_FAILURES` (8)

### `lib.rs`
- `validate_credentials` returns a stable string code on error (e.g. `"InvalidCredentials"`), no longer the raw Rust error text
- New helper function `error_code(&AppError) -> String` maps enum variant to code

### Frontend (i18n)
- New `errors` block in `de.json`, `en.json`, `jp.json` under `wizard.errors.*` with plain text + suggested fix per error code
- `WizardStep2.svelte`: `externalError` (error code from Step 3) is resolved via `$_(\`wizard.errors.${externalError}\`)`, shows localized text instead of raw code
- `WizardStep3.svelte` unchanged – just passes `onFail(e as string)` through, translation happens in Step 2

## Legal Documents & Wizard Step 0

### Documents
- Located in `docs/legal/` (workspace root, outside `GlucoTray/`): `privacy-policy.{de,en,jp}.md`, `terms-of-use.{de,en,jp}.md`, `disclaimer.{de,en,jp}.md`
- `store-disclaimer.md` (short form for Microsoft Store listing) intentionally NOT in `docs/legal/` to avoid being bundled into the app
- All based on German law (controller: Niklas Rühl, named explicitly per GDPR since no impressum obligation applies but identifiability is still required), CGM-vendor-neutral wording (not Dexcom-specific) to allow future integrations (LibreLink, Accu-Chek, etc.)
- Minimum age 16 per Art. 8 GDPR (health data, Art. 9 GDPR)

### Bundling
- `tauri.conf.json` → `bundle.resources`: `"../../docs/legal/*": "legal/"` (path relative to `src-tauri/`, since `docs/` sits one level above the `GlucoTray/` project root)
- Accessible at runtime via `BaseDirectory::Resource`

### Backend (`lib.rs`)
- `read_legal_document(document, lang) -> Result<String, String>` – reads `legal/{document}.{lang}.md` from the resource directory
- `save_legal_acceptance(legal_version) -> Result<(), String>` – writes six settings keys: `privacy_accepted`, `privacy_version`, `terms_accepted`, `terms_version`, `disclaimer_accepted`, `disclaimer_version` (single shared version string across all three documents, e.g. `"2026-06"`)
- Both registered in `invoke_handler!`

### Frontend (`WizardStep0.svelte`)
- Single component with internal sub-step state (`legalStep` 0–2) cycling through privacy policy → terms of use → disclaimer
- Loads current document via `read_legal_document`, renders Markdown via the `marked` library, displays via `{@html}`
- One individual "Accept" button per document; only after accepting all three does `save_legal_acceptance` fire once, then `onNext()` advances to Step 1
- `$effect` reloads the document whenever `legalStep` changes (replaces the previous `onMount`, which became redundant)
- New i18n block `wizard.legal.*` in `de.json`, `en.json`, `jp.json` (titles per document, button labels, saving state)

### `+page.svelte`
- `step` now starts at `0` instead of `1`
- New `handleStep0()` simply advances to `step = 1`
- `handleCancel()` still resets to `step = 1`, not `0` — legal acceptance is not re-shown once Step 1 has been reached once

## Wizard – Current State

### Step 0 – Legal Acceptance
- See "Legal Documents & Wizard Step 0" above

### Step 1 – Sensor & Region
- G6 / G7 selection, region: US / OUS / Japan
- Prerequisites checklist, language via flag button (de/en/jp)

### Step 2 – Credentials
- Login type: email or phone number
- Email: regex + Levenshtein typo detection
- Phone: libphonenumber-js, OS locale for country
- Password: custom display, eye toggle, paste support
- `externalError` prop for errors from Step 3, shown as localized plain text via i18n

### Step 3 – Auth Loading
- `onMount` → `invoke("validate_credentials")`
- Success → Step 4, failure → Step 2 with error code (translated there)

### Step 4 – Settings
- Unit: mg/dL or mmol/L (radio buttons)
- Dropdowns show values in selected unit
- Internally always mmol values, `handleNext` converts to mg/dL
- Min: 2.8–4.5 mmol/L, Max: 8.0–13.0 mmol/L
- 5 color zones with native color picker
- Autostart checkbox

### Step 5 – Completion
- `onMount` → `invoke("save_wizard_data")` with mg/dL thresholds
- Summary, error display on save failure
- Finish → `invoke("restart_app")`

## Key Files
| File | Purpose |
|---|---|
| `src/routes/+page.svelte` | Wizard control, state management |
| `src/routes/+layout.svelte` | i18n init, theme init |
| `src/lib/components/WizardStep0.svelte` | Step 0 – legal acceptance |
| `src/lib/components/WizardStep1.svelte` | Step 1 |
| `src/lib/components/WizardStep2.svelte` | Step 2, incl. localized error display |
| `src/lib/components/WizardStep3.svelte` | Step 3 |
| `src/lib/components/WizardStep4.svelte` | Step 4 |
| `src/lib/components/WizardStep5.svelte` | Step 5 |
| `src/lib/styles/wizard.css` | Wizard styles, incl. legal document display |
| `src/lib/styles/app.css` | Global styles + CSS variables |
| `src/lib/i18n/index.ts` | i18n setup with OS locale |
| `src/lib/i18n/de.json` | German translations, incl. `wizard.errors.*` and `wizard.legal.*` |
| `src/lib/i18n/en.json` | English translations, incl. `wizard.errors.*` and `wizard.legal.*` |
| `src/lib/i18n/jp.json` | Japanese translations, incl. `wizard.errors.*` and `wizard.legal.*` |
| `src-tauri/src/lib.rs` | Tauri commands, app setup, `error_code()` helper |
| `src-tauri/src/state.rs` | AppState struct |
| `src-tauri/src/tray.rs` | Tray icon, color logic, menu |
| `src-tauri/src/dexcom.rs` | Dexcom Share API, typed errors, internet check |
| `src-tauri/src/db.rs` | SQLite init, queries, conversion |
| `src-tauri/src/worker.rs` | Polling worker, tray update, AppError matching |
| `src-tauri/src/keychain.rs` | OS keychain integration |
| `src-tauri/src/error.rs` | AppError enum, logger init, From conversions |
| `src-tauri/assets/fonts/NotoSans-Bold.ttf` | Font for tray rendering |
| `docs/legal/*.{de,en,jp}.md` | Privacy policy, terms of use, disclaimer (bundled as Tauri resources) |
| `docs/legal/store-disclaimer.md` | Short-form disclaimer for store listings (NOT bundled into the app) |

## Tauri Commands
| Command | Purpose |
|---|---|
| `validate_credentials` | Dexcom auth + save password to keychain, returns error code instead of raw text |
| `save_wizard_data` | Write all settings to SQLite |
| `restart_app` | App restart |
| `read_legal_document` | Read a legal document (privacy-policy/terms-of-use/disclaimer) for a given language from bundled resources |
| `save_legal_acceptance` | Write acceptance + version for all three legal documents to SQLite |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR merged to main |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published — **likely misconfigured**: `tauri-apps/tauri-action@v0` has no `projectPath` set, even though the Tauri project lives in `GlucoTray/`, not the workspace root. Only ran once so far (failed, Tauri wasn't installed); never successfully tested. |
| `review.yml` | PR to main |
| `create-issue.yml` | Push with changes to `docs/issues.md` |

## Installer Architecture Decision (June 2026)

- **Windows (.exe via Tauri/NSIS bundler):** planned to eventually move the entire wizard flow (including Step 0) into the NSIS installer itself, likely via an external helper binary for network/DB access since NSIS's scripting language can't do this natively. Not yet implemented, decision only.
- **Linux:** stays on Flatpak with the in-app wizard for now, since Flatpak's sandboxing model doesn't support interactive install-time hooks (unlike `.deb` postinst or AUR PKGBUILD). An additional `.deb`/AUR path with an installer-side wizard was considered but deferred, to avoid losing Flatpak's reach (Fedora/openSUSE/KDE Neon) — can be revisited later.
- **Microsoft Store:** keeps the in-app wizard in all cases, since MSIX does not allow interactive installer dialogs.

## Open Items
- Settings window (unit, thresholds, colors, autostart, change credentials, view legal documents)
- Unit change → restart notice in settings window
- Wizard flow tested end-to-end on real hardware
- Targeted API error-case testing (invalid credentials, no session, no readings, timeout, rate limit, no internet)
- `tauri-build.yml` projectPath fix before first real release
- Before first public release: Azure AD + store submission, Flathub bot, in-app updater

## Notes for Next Session
- Settings window is a new Tauri window (left-click on tray opens it)
- Unit change requires app restart due to AppState
- Credentials change in settings window: same flow as wizard – call `validate_credentials`, then save, then `restart_app` (no periodic reload needed in the worker)
- `get_latest_reading` in db.rs still unused – needed in the settings window
- `delete_credentials` in keychain.rs – needed when changing credentials
- `MMOL_TO_MGDL` constant in lib.rs still unused – check if still needed
- Legal document version is currently set once at acceptance time (`LEGAL_VERSION = "2026-06"` in `WizardStep0.svelte`) and never re-checked at startup — if documents are updated later, there is no mechanism yet to detect the version mismatch and re-prompt the user