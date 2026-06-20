# Handoff – GlucoTray

## Project
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **License:** MIT
- **Status:** Wizard + tray complete, autostart implemented, error handling complete, legal document acceptance (Step 0) implemented, settings window complete – next goal: TBD (window-close behavior just finished; update checker, mg/dL refactor, and LOW/HIGH handling are open issues)

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
- Single conversion point in backend: `db.rs` → `insert_reading` writes `value_mgdl` + `value_mmol`
- Thresholds in DB always stored as mg/dL integers (`threshold_low`, `threshold_high`)
- **Known inconsistency (tracked as issue #12):** `WizardStep4.svelte` and `Settings.svelte` hold their primary dropdown state in mmol/L (fixed value lists `MIN_VALUES_MMOL`/`MAX_VALUES_MMOL`), converting to mg/dL only when saving. Works correctly but contradicts the "single conversion point" principle; flagged for later refactor, not urgent.
- Fixed clinical limits: Very High = 250 mg/dL, Critical Low = 54 mg/dL
- API string `"Low"` also triggers Critical Low zone — **but see issue #11**: Dexcom can also return literal `"LOW"`/`"HIGH"` strings instead of numeric values, which is not yet handled anywhere (deserialization risk in `dexcom.rs`, no color/value logic for `"High"` at all)

## AppState
- `unit: String` – loaded from DB at app start
- Unit change in settings requires app restart (handled: restart notice shown in settings window after save if unit changed)
- `TrayState` – contains `update_available: bool` for update badge

## Tray Icon & Menu
- Icon dynamically rendered via `imageproc` + `ab_glyph` + NotoSans-Bold.ttf
- Shows value in mg/dL or mmol/L depending on `AppState.unit`
- Trend arrow as Unicode: ⇈ ↑ ↗ → ↘ ↓ ⇊
- 5 color zones: Critical Low / Low / Normal / High / Very High
- N/A when `is_valid = false` or `value = 0`
- Update badge: red dot top-right when `update_available = true`
- Tooltip shows both units: 97 mg/dL / 5.4 mmol/L ↑
- **Context menu (current, post-refactor):** `GlucoTray` (now active, opens/focuses the window) / separator / `Update check` / separator / `Quit` / `Restart`
- **Left-click on tray icon does nothing app-wise anymore** — removed the previous `on_tray_icon_event` handler, since left-click is unreliable for opening windows on Windows/Linux (only used there for dragging the icon). All window-opening now goes through the `GlucoTray` context menu entry (`on_menu_event`, id `"open_window"`).
- `Quit` exits the whole app (`app.exit(0)`); `Restart` fully restarts (`app.restart()`)

## Window Behavior
- Closing the main window (X button) does **not** quit the app: `CloseRequested` event is intercepted in `lib.rs` setup (`window.on_window_event`), `api.prevent_close()` is called, and `window.hide()` is used instead. Tray and worker keep running in the background.
- Only `Quit` from the tray context menu fully terminates the app.
- `Restart` (tray menu or settings window button) calls `restart_app`, which fully restarts the process.

## App Flow

### First Start
App starts → DB empty → window visible → wizard (Step 0 legal, then Steps 1–5) → wizard completed → `save_wizard_data` → `restart_app` → restart → normal start

### Normal Start
DB init → unit + autostart from DB → AppState populated → autostart enable/disable → worker starts → tray active → first start: toast notification (one-time, `tray_hint_shown` in DB)

### Opening the Window Later (Settings)
Tray context menu → `GlucoTray` → window shown/focused → `+page.svelte` calls `get_wizard_status` on mount → since `wizard_done == true`, renders `Settings.svelte` instead of the wizard steps. Closing the window again only hides it (see "Window Behavior" above); the worker is untouched.

## Autostart
- `tauri-plugin-autostart` – Windows Registry + Linux .desktop
- Read from DB and applied on every app start
- Settings change requires app restart to take effect; saved via `save_settings`, actually applied to the OS only at next app start (not live-toggled)

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
- **Not yet handled (issue #11):** literal `"LOW"`/`"HIGH"` string values from the API instead of numeric `Value`

### `worker.rs`
- Match logic on `AppError` variants, `e.log()` on every error
- `InvalidCredentials`/`NoSession` during initial login or in the polling loop → worker stops completely (no retry loop; user must use the settings window to fix credentials, then restart the app)
- `RateLimit` → waits out the normal poll interval, does not increment the failure counter
- `SessionExpired` → does not count as a failure, session was already auto-renewed in `get_readings()`
- All other errors (`Timeout`, `NetworkError`, `NoInternetConnection`, `NoReadings`, `Unknown`) go through the existing `failure_count` mechanism, N/A after `MAX_FAILURES` (8)

### `lib.rs`
- `validate_credentials` now also takes `app: tauri::AppHandle` and, on success, writes the (possibly changed) `username` into the `settings` table via `set_setting` — this lets the same command serve both the wizard and the settings "change credentials" flow without a separate command
- New helper `open_db(app) -> Result<SqlitePool, String>` consolidates the repeated "resolve app data dir → build db path → init_db" pattern used in almost every command
- `error_code(&AppError) -> String` maps enum variant to a stable string code returned to the frontend

### Frontend (i18n)
- `errors` block in `de.json`, `en.json`, `jp.json` under `wizard.errors.*` with plain text + suggested fix per error code
- `WizardStep2.svelte`: `externalError` (error code from Step 3) is resolved via `$_(\`wizard.errors.${externalError}\`)`, shows localized text instead of raw code
- `WizardStep3.svelte` unchanged – just passes `onFail(e as string)` through, translation happens wherever it's consumed (Step 2 in wizard mode, also Step 2 reused in settings mode)

## Legal Documents

### Documents
- Located in `docs/legal/` (workspace root, outside `GlucoTray/`): `privacy-policy.{de,en,jp}.md`, `terms-of-use.{de,en,jp}.md`, `disclaimer.{de,en,jp}.md`
- `store-disclaimer.md` (short form for Microsoft Store listing) intentionally kept OUTSIDE `docs/legal/` to avoid being bundled into the app
- All based on German law (controller: Niklas Rühl, named explicitly per GDPR since no impressum obligation applies but identifiability is still required), CGM-vendor-neutral wording (not Dexcom-specific) to allow future integrations (LibreLink, Accu-Chek, etc.)
- Minimum age 16 per Art. 8 GDPR (health data, Art. 9 GDPR)

### Bundling
- `tauri.conf.json` → `bundle.resources`: `"../../docs/legal/*": "legal/"` (path relative to `src-tauri/`, since `docs/` sits one level above the `GlucoTray/` project root)
- Accessible at runtime via `BaseDirectory::Resource`

### Backend (`lib.rs`)
- `read_legal_document(document, lang) -> Result<String, String>` – reads `legal/{document}.{lang}.md` from the resource directory; used both by `WizardStep0.svelte` and the three settings-window legal viewers
- `save_legal_acceptance(legal_version) -> Result<(), String>` – writes six settings keys: `privacy_accepted`, `privacy_version`, `terms_accepted`, `terms_version`, `disclaimer_accepted`, `disclaimer_version` (single shared version string across all three documents, e.g. `"2026-06"`)

### Wizard Step 0 (`WizardStep0.svelte`)
- Single component with internal sub-step state (`legalStep` 0–2) cycling through privacy policy → terms of use → disclaimer
- One individual "Accept" button per document; only after accepting all three does `save_legal_acceptance` fire once, then `onNext()` advances to Step 1
- `$effect` reloads the document whenever `legalStep` changes

### Settings-Window Legal Viewers (`PrivacyPolicy.svelte`, `TermsOfUse.svelte`, `Disclaimer.svelte`)
- Three separate, near-identical components (deliberately not a shared generic component, per explicit request) — each loads exactly one fixed document via `read_legal_document` and renders it via `marked` + `{@html}`
- Read-only: no accept button, just a "Back" button (`onBack` prop) that returns to `mode = "settings"` in `+page.svelte`
- No automatic "last updated" date from git history — considered, rejected as too complex for a local resource bundle (no `.git` available at runtime in the built app); the manually-maintained "Last updated" line inside each markdown file is the only date shown

## Wizard – Current State

### Step 0 – Legal Acceptance
- See "Legal Documents" above

### Step 1 – Sensor & Region
- G6 / G7 selection, region: US / OUS / Japan
- Prerequisites checklist, language via flag button (de/en/jp)

### Step 2 – Credentials
- Login type: email or phone number
- Email: regex + Levenshtein typo detection
- Phone: libphonenumber-js, OS locale for country
- Password: custom display, eye toggle, paste support
- `externalError` prop for errors, shown as localized plain text via i18n
- **Reused in settings mode** for credential changes (see "Settings Window" below)

### Step 3 – Auth Loading
- `onMount` → `invoke("validate_credentials")`
- Success → Step 4 (wizard mode) or back to Settings (settings mode), failure → Step 2 with error code
- **Reused in settings mode** (see "Settings Window" below)

### Step 4 – Settings (Wizard)
- Unit: mg/dL or mmol/L (radio buttons)
- Dropdowns show values in selected unit, internal state in mmol/L (see issue #12)
- Min: 2.8–4.5 mmol/L, Max: 8.0–13.0 mmol/L
- 5 color zones with native color picker
- Autostart checkbox

### Step 5 – Completion
- `onMount` → `invoke("save_wizard_data")` with mg/dL thresholds
- Summary, error display on save failure
- Finish → `invoke("restart_app")`

## Settings Window

- **Not a separate Tauri window** — same `main` window as the wizard, content switched via routing in `+page.svelte`, not a second `WebviewWindow`
- Opened via tray context menu `GlucoTray` entry (`window.show()` + `window.set_focus()`); closing it only hides it (see "Window Behavior")
- `+page.svelte` introduces a `mode` state (`"loading" | "wizard" | "settings" | "settings-edit-credentials" | "settings-validate-credentials" | "settings-privacy" | "settings-terms" | "settings-disclaimer"`), determined on mount via `get_wizard_status`: `false` → `"wizard"` (starts at Step 0), `true` → `"settings"`
- **Layout:** single scrollable page, sections top to bottom: Credentials (read-only + "change credentials" button) → Unit → Threshold range → Color picker → Autostart checkbox → action row (Restart / Update check placeholder / global Save) → footer (legal document links, contact link, repo link)
- **Loading:** `get_settings` command fetches current values on mount (username, region, unit, thresholds in mg/dL, autostart, 5 colors); thresholds are converted to mmol/L for the dropdown and snapped to the closest fixed dropdown value if not an exact match (helper `closestValue`)
- **Saving:** single global "Save" button (`save_settings`) for unit/thresholds/colors/autostart; tracked via a `hasChanges` flag set by every input's change handler, button disabled until something changed
- **Unit change:** after a successful save, if the unit differs from the value loaded at mount (`originalUnit`), a restart notice + "Restart now" button appears (does not force a restart, just offers one)
- **Credentials section:** username and a fixed `**********` placeholder password shown, both disabled/read-only (password is never fetched from the keychain for display — purely cosmetic placeholder, no roundtrip). A "change credentials" button switches `+page.svelte`'s `mode` to `"settings-edit-credentials"`, which renders `WizardStep2.svelte` with settings-specific callbacks: on success → `mode = "settings-validate-credentials"` (renders `WizardStep3.svelte`, also settings-specific callbacks) → on auth success, back to `mode = "settings"`; on auth failure, back to `"settings-edit-credentials"` with the error shown in Step 2 (identical UX to the wizard's own Step 2/3 failure loop); "Back" from Step 2 in this mode returns directly to `"settings"`, never to wizard Step 0/1
- **Legal document access:** three footer buttons (privacy/terms/disclaimer) set `mode` to `"settings-privacy"` / `"settings-terms"` / `"settings-disclaimer"`, each rendering its respective standalone viewer component; "Back" in each returns to `"settings"`
- **Update check button:** present in the UI, calls a placeholder `handleUpdateCheck()` function that currently just logs to console — actual update logic not implemented yet
- Styling for the new footer links lives in `wizard.css` (despite the filename, by deliberate choice rather than creating a separate stylesheet) — `.footer-links`, `.footer-link`, `.footer-separator`

## Key Files
| File | Purpose |
|---|---|
| `src/routes/+page.svelte` | Central routing: wizard steps AND settings modes, all in one `mode`/`step` state machine |
| `src/routes/+layout.svelte` | i18n init, theme init |
| `src/lib/components/WizardStep0.svelte` | Step 0 – legal acceptance (wizard only) |
| `src/lib/components/WizardStep1.svelte` | Step 1 (wizard only) |
| `src/lib/components/WizardStep2.svelte` | Step 2 – credentials input; reused in settings mode |
| `src/lib/components/WizardStep3.svelte` | Step 3 – auth validation; reused in settings mode |
| `src/lib/components/WizardStep4.svelte` | Step 4 (wizard only) |
| `src/lib/components/WizardStep5.svelte` | Step 5 (wizard only) |
| `src/lib/components/Settings.svelte` | Settings window content (unit/thresholds/colors/autostart/credentials view/footer) |
| `src/lib/components/PrivacyPolicy.svelte` | Read-only privacy policy viewer (settings footer) |
| `src/lib/components/TermsOfUse.svelte` | Read-only terms of use viewer (settings footer) |
| `src/lib/components/Disclaimer.svelte` | Read-only disclaimer viewer (settings footer) |
| `src/lib/styles/wizard.css` | Styles for wizard AND settings (incl. legal document rendering, footer links) |
| `src/lib/styles/app.css` | Global styles + CSS variables |
| `src/lib/i18n/index.ts` | i18n setup with OS locale |
| `src/lib/i18n/de.json` | German translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` additions |
| `src/lib/i18n/en.json` | English translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` additions |
| `src/lib/i18n/jp.json` | Japanese translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` additions |
| `src-tauri/src/lib.rs` | Tauri commands, app setup, window-close interception, `error_code()`/`open_db()` helpers |
| `src-tauri/src/state.rs` | AppState struct |
| `src-tauri/src/tray.rs` | Tray icon, color logic, menu (left-click handler removed, `open_window` menu entry added) |
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
| `validate_credentials` | Dexcom auth + save password to keychain + update `username` setting; returns error code instead of raw text; used by wizard Step 3 AND settings credential change |
| `save_wizard_data` | Write all wizard settings to SQLite, mark `wizard_done` |
| `restart_app` | App restart |
| `read_legal_document` | Read a legal document for a given language from bundled resources |
| `save_legal_acceptance` | Write acceptance + version for all three legal documents to SQLite |
| `get_wizard_status` | Returns `wizard_done` bool, used by `+page.svelte` to decide wizard vs. settings mode |
| `get_settings` | Returns current settings (username, region, unit, thresholds in mg/dL, autostart, 5 colors) for the settings window |
| `save_settings` | Writes unit/thresholds/colors/autostart (NOT credentials, which go through `validate_credentials`) |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR merged to main |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published — **likely misconfigured**: `tauri-apps/tauri-action@v0` has no `projectPath` set, even though the Tauri project lives in `GlucoTray/`, not the workspace root. Only ran once so far (failed, Tauri wasn't installed); never successfully tested. |
| `review.yml` | PR to main |
| `create-issue.yml` | Push with changes to `docs/issues.md` — confirmed working (issues #11 and #12 created this way) |

## Installer Architecture Decision (June 2026)

- **Windows (.exe via Tauri/NSIS bundler):** planned to eventually move the entire wizard flow (including Step 0) into the NSIS installer itself, likely via an external helper binary for network/DB access since NSIS's scripting language can't do this natively. Not yet implemented, decision only.
- **Linux:** stays on Flatpak with the in-app wizard for now, since Flatpak's sandboxing model doesn't support interactive install-time hooks (unlike `.deb` postinst or AUR PKGBUILD). An additional `.deb`/AUR path with an installer-side wizard was considered but deferred, to avoid losing Flatpak's reach (Fedora/openSUSE/KDE Neon) — can be revisited later.
- **Microsoft Store:** keeps the in-app wizard in all cases, since MSIX does not allow interactive installer dialogs.

## Known Open GitHub Issues
- **#11** – Dexcom LOW/HIGH string values not handled. `GlucoseReading.value` is `u32`, can't deserialize the literal strings `"LOW"`/`"HIGH"` that Dexcom sometimes returns instead of a number. `resolve_color` in `tray.rs` only checks `trend == "Low"` for color, doesn't adjust the displayed value, and has no `"High"` handling at all. Needs a defined value range mapping and corresponding fixes in `dexcom.rs` + `tray.rs`. Not urgent.
- **#12** – Refactor frontend/backend to use mg/dL internally everywhere. `WizardStep4.svelte`/`Settings.svelte` hold mmol/L as primary dropdown state, only converting to mg/dL when saving. Contradicts the stated single-conversion-point architecture, not urgent, purely a consistency cleanup.

## Open Items
- Update checker (button exists in settings window, no logic yet)
- Wizard flow tested end-to-end on real hardware
- Settings window tested end-to-end on real hardware
- Targeted API error-case testing (invalid credentials, no session, no readings, timeout, rate limit, no internet)
- `tauri-build.yml` projectPath fix before first real release
- Issue #11 (LOW/HIGH value handling) and #12 (mg/dL refactor) — both deferred, not blocking
- Before first public release: Azure AD + store submission, Flathub bot, installer-side wizard for Windows (see "Installer Architecture Decision")

## Notes for Next Session
- Settings window uses the SAME Tauri window as the wizard — there is no second `WebviewWindow`. Don't reintroduce a separate window unless explicitly requested again; this was a deliberate correction mid-session.
- `get_latest_reading` in `db.rs` still unused — explicitly decided NOT to use it for a settings-window live preview (rejected); no current planned use, fine to leave as-is
- `delete_credentials` in `keychain.rs` — still not used; credential changes currently just overwrite via `save_credentials`, never explicitly delete old entries first. Worth checking if that's an issue (e.g. orphaned keychain entries on username change) — not yet flagged as an issue, just noting it
- `MMOL_TO_MGDL` constant in `lib.rs` still unused — check if still needed
- Legal document version is set once at acceptance time (`LEGAL_VERSION = "2026-06"` in `WizardStep0.svelte`) and never re-checked at startup — no mechanism yet to detect a version mismatch and re-prompt the user if documents are updated later
- Two GitHub issues are open and confirmed visible via `web_fetch` on the public issue URLs (#11, #12) — Claude can fetch `https://github.com/AgentGG00/glucotray/issues/<number>` directly when given the URL, including full body and any comments; no GitHub connector/MCP needed for read access on this public repo
</markdown>