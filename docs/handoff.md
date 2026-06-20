# Handoff – GlucoTray

## Project
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **License:** MIT
- **Store ID:** 9P2TR53FHBBH (Microsoft Store, already reserved in Partner Center)
- **Status:** Wizard + tray complete, settings window complete, error handling complete, self-updater with store/flatpak fallback complete, CI/CD projectPath bug fixed – next: real-hardware testing, then pub-release prep (MSIX/Flatpak packaging, store submissions)

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

## Distribution Strategy (decided June 2026 — important context for any future packaging/CI work)

- **Public release channels are Microsoft Store (Windows) and Flathub (Linux) ONLY.** No public direct `.exe` download is planned. This was a deliberate simplification after researching code-signing costs.
- **Why no direct `.exe` distribution:** unsigned `.exe` triggers Windows Defender "unknown publisher" warnings; a real code-signing certificate is expensive (traditional OV certs) or ~$10/month minimum (Azure Trusted Signing) — decided not worth it for a free hobby project. Microsoft Store and Flathub both sign/verify identity for free (Store: free individual developer registration since the fee waiver, Microsoft signs MSIX during submission; Flathub: free, just GitHub/domain ownership verification, no certificate).
- **`.exe`/`.AppImage` still get built automatically** on every GitHub Release via `tauri-build.yml` — but these are **dev-only artifacts**, living only in GitHub Releases, never linked from the public GitHub Page. Intended for Niklas's own testing and for technically comfortable users who know how to handle unsigned/repo-distributed builds.
- **MSIX (Store) and Flatpak (Flathub) packaging are deferred** to pub-release preparation. Plan: a SECOND, separate workflow (e.g. `store-build.yml`) with `workflow_dispatch` (manual trigger only, never automatic on release) will build and submit these — kept deliberately separate so dev releases never accidentally trigger store updates.
- **Azure AD is NOT needed** for now — it's only required for the Microsoft Store Submission API (automated/programmatic submission). Manual submission via the Partner Center web UI needs nothing beyond the (already free) developer account. Revisit only if automated store publishing becomes worthwhile later.
- **MSIX is not a native Tauri bundle target** (`tauri.conf.json`'s `bundle.targets` supports `msi` via WiX, not `msix`) — packaging an MSIX requires an additional step (e.g. Windows SDK `MakeAppx.exe`) on top of a built `.exe`/`.msi`. This is why MSIX packaging is its own deferred workflow, not just a config tweak.

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
- **Context menu:** `GlucoTray` (active, opens/focuses the window) / separator / `Update check` / separator / `Quit` / `Restart`
- **Left-click on tray icon does nothing app-wise** — removed `on_tray_icon_event`, unreliable for opening windows on Windows/Linux (only used there for dragging the icon). All window-opening goes through the `GlucoTray` context menu entry (`on_menu_event`, id `"open_window"`).
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
Tray context menu → `GlucoTray` → window shown/focused → `+page.svelte` calls `get_wizard_status` on mount → since `wizard_done == true`, renders `Settings.svelte` instead of the wizard steps. Closing the window again only hides it; the worker is untouched.

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
- `validate_credentials` also takes `app: tauri::AppHandle` and, on success, writes the (possibly changed) `username` into the `settings` table via `set_setting` — lets the same command serve both the wizard and the settings "change credentials" flow
- Helper `open_db(app) -> Result<SqlitePool, String>` consolidates the repeated "resolve app data dir → build db path → init_db" pattern used in almost every command
- `error_code(&AppError) -> String` maps enum variant to a stable string code returned to the frontend

### Frontend (i18n)
- `errors` block in `de.json`, `en.json`, `jp.json` under `wizard.errors.*` with plain text + suggested fix per error code
- `WizardStep2.svelte`: `externalError` (error code from Step 3) is resolved via `$_(\`wizard.errors.${externalError}\`)`, shows localized text instead of raw code
- `WizardStep3.svelte` unchanged – just passes `onFail(e as string)` through, translation happens wherever it's consumed

## Legal Documents

### Documents
- Located in `docs/legal/` (workspace root, outside `GlucoTray/`): `privacy-policy.{de,en,jp}.md`, `terms-of-use.{de,en,jp}.md`, `disclaimer.{de,en,jp}.md`
- `store-disclaimer.md` (short form for Microsoft Store listing) intentionally kept OUTSIDE `docs/legal/` to avoid being bundled into the app
- All based on German law (controller: Niklas Rühl, named explicitly per GDPR since no impressum obligation applies but identifiability is still required), CGM-vendor-neutral wording (not Dexcom-specific) to allow future integrations (LibreLink, Accu-Chek, etc.)
- Minimum age 16 per Art. 8 GDPR (health data, Art. 9 GDPR)

### Bundling
- `tauri.conf.json` → `bundle.resources`: `"../../docs/legal/*": "legal/"` (path relative to `src-tauri/`)
- Accessible at runtime via `BaseDirectory::Resource`

### Backend (`lib.rs`)
- `read_legal_document(document, lang) -> Result<String, String>` – reads `legal/{document}.{lang}.md` from the resource directory; used both by `WizardStep0.svelte` and the three settings-window legal viewers
- `save_legal_acceptance(legal_version) -> Result<(), String>` – writes six settings keys (`privacy_accepted`, `privacy_version`, `terms_accepted`, `terms_version`, `disclaimer_accepted`, `disclaimer_version`)

### Wizard Step 0 (`WizardStep0.svelte`)
- Single component with internal sub-step state (`legalStep` 0–2) cycling through privacy policy → terms of use → disclaimer
- One individual "Accept" button per document; after all three, `save_legal_acceptance` fires once, then `onNext()` advances to Step 1
- `$effect` reloads the document whenever `legalStep` changes

### Settings-Window Legal Viewers (`PrivacyPolicy.svelte`, `TermsOfUse.svelte`, `Disclaimer.svelte`)
- Three separate, near-identical components (deliberately not a shared generic component) — each loads exactly one fixed document via `read_legal_document` and renders it via `marked` + `{@html}`
- Read-only: just a "Back" button (`onBack` prop) that returns to `mode = "settings"`
- No automatic "last updated" date from git history — rejected as too complex for a local resource bundle at runtime; manually-maintained "Last updated" line inside each markdown file is the only date shown

## Wizard – Current State

### Step 0 – Legal Acceptance
See "Legal Documents" above.

### Step 1 – Sensor & Region
G6 / G7 selection, region: US / OUS / Japan. Prerequisites checklist, language via flag button (de/en/jp).

### Step 2 – Credentials
Login type: email or phone number. Email: regex + Levenshtein typo detection. Phone: libphonenumber-js, OS locale for country. Password: custom display, eye toggle, paste support. `externalError` prop for errors, shown as localized plain text via i18n. **Reused in settings mode** for credential changes.

### Step 3 – Auth Loading
`onMount` → `invoke("validate_credentials")`. Success → Step 4 (wizard mode) or back to Settings (settings mode), failure → Step 2 with error code. **Reused in settings mode.**

### Step 4 – Settings (Wizard)
Unit: mg/dL or mmol/L (radio buttons). Dropdowns show values in selected unit, internal state in mmol/L (see issue #12). Min: 2.8–4.5 mmol/L, Max: 8.0–13.0 mmol/L. 5 color zones with native color picker. Autostart checkbox.

### Step 5 – Completion
`onMount` → `invoke("save_wizard_data")` with mg/dL thresholds. Summary, error display on save failure. Finish → `invoke("restart_app")`.

## Settings Window

- **Not a separate Tauri window** — same `main` window as the wizard, content switched via routing in `+page.svelte`
- Opened via tray context menu `GlucoTray` entry; closing it only hides it
- `+page.svelte` has a `mode` state (`"loading" | "wizard" | "settings" | "settings-edit-credentials" | "settings-validate-credentials" | "settings-privacy" | "settings-terms" | "settings-disclaimer"`), determined on mount via `get_wizard_status`
- **Layout:** single scrollable page — Credentials (read-only + "change credentials") → Unit → Threshold range → Color picker → Autostart checkbox → action row (Restart / Update check / global Save) → footer (legal links, contact, repo)
- **Loading:** `get_settings` fetches current values on mount; thresholds converted to mmol/L for dropdowns, snapped to closest fixed value if not exact (helper `closestValue`)
- **Saving:** single global "Save" button (`save_settings`) for unit/thresholds/colors/autostart; `hasChanges` flag gates the button
- **Unit change:** after save, if unit differs from `originalUnit`, a restart notice + "Restart now" button appears
- **Credentials section:** username + fixed `**********` placeholder (never fetched from keychain, purely cosmetic). "Change credentials" → `mode = "settings-edit-credentials"` (WizardStep2) → success → `"settings-validate-credentials"` (WizardStep3) → auth success → back to `"settings"`; auth failure → back to `"settings-edit-credentials"` with error shown
- **Legal document access:** three footer buttons set `mode` to the respective `settings-privacy/terms/disclaimer` value
- Styling lives in `wizard.css` (deliberate choice, despite the filename) — `.footer-links`, `.footer-link`, `.footer-separator`, `.flatpak-command-row`, `.flatpak-command`

## Self-Updater

### Architecture decision
- Built using `tauri-plugin-updater`, gated behind a Cargo feature flag `self-updater` (NOT a runtime check) so the updater code is physically absent from MSIX/Flatpak builds, present only in `.exe`/`.AppImage` builds
- Rationale: Store and Flatpak installations are sandboxed and manage their own updates; a self-updater inside them would be both pointless and potentially blocked by the sandbox. The `.exe`/`.AppImage` builds (dev artifacts, see "Distribution Strategy") are the only ones that benefit from self-updating, since they have no platform-managed update mechanism.

### `Cargo.toml`
```toml
[features]
default = []
self-updater = ["tauri-plugin-updater"]

[dependencies]
tauri-plugin-updater = { version = "2", optional = true }
```

### Setup completed
- Signer keypair generated via `cargo tauri signer generate -w ~/.tauri/glucotray.key`
- Private key content + password stored as GitHub repo secrets: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (repo-level, not org-level — only this repo needs them)
- Public key embedded in `tauri.conf.json` → `plugins.updater.pubkey`
- `tauri.conf.json` → `bundle.createUpdaterArtifacts: true` (required for new apps, generates `.sig` files + `latest.json`)
- `tauri.conf.json` → `plugins.updater.endpoints`: GitHub Releases `latest.json` (`https://github.com/AgentGG00/glucotray/releases/latest/download/latest.json`), no separate hosting needed since `tauri-action` generates and uploads this automatically
- `tauri.conf.json` → `plugins.updater.windows.installMode: "passive"` (background install, no NSIS UI dialogs during update)
- `capabilities/default.json` → `updater:default` permission added (covers check/download/install/download-and-install)
- `tauri-build.yml` matrix args now include `--features self-updater` for both Windows and Linux dev builds

### Backend (`lib.rs`)
- Two `check_for_update` implementations selected via `#[cfg(feature = "self-updater")]` / `#[cfg(not(...))]` — only one compiles into any given binary
  - **With feature:** uses `tauri_plugin_updater::UpdaterExt`, calls `updater.check()`, on `Some(update)` calls `update.download_and_install(...)`, returns `"updated"` or `"up_to_date"`
  - **Without feature:** returns a platform-specific hint string — `"store_hint"` on Windows, `"flatpak_hint"` on Linux, `"unsupported"` otherwise. No actual action taken in Rust; the frontend interprets the string.
- `run()` builds the `tauri::Builder` into a `let mut builder` variable so the updater plugin can be conditionally appended via `#[cfg(feature = "self-updater")]` before the rest of the builder chain (`.manage(...)`, `.invoke_handler(...)`, etc.)

### Frontend (`Settings.svelte`)
- `handleUpdateCheck()` calls `check_for_update`, branches on the returned string:
  - `"updated"` → shows install-success message + a "Restart now" button (does not auto-restart)
  - `"up_to_date"` → shows a simple "already up to date" message
  - `"store_hint"` → opens `ms-windows-store://pdp/?productid=9P2TR53FHBBH` in a new window/tab (Store deep link, Store ID already reserved in Partner Center)
  - `"flatpak_hint"` → shows the command `flatpak update io.github.AgentGG00.GlucoTray` in a monospace box with a "Copy" button (`navigator.clipboard.writeText`), no auto-execution
  - `"unsupported"` → generic fallback message
- New i18n keys under `wizard.settings.*`: `update_installed`, `update_up_to_date`, `update_store_opened`, `update_flatpak_hint`, `update_unsupported`, `copy`, `copied` (de/en/jp)
- Flatpak app ID used throughout: `io.github.AgentGG00.GlucoTray` (standard Flathub naming scheme, not yet verified/reserved on Flathub itself — just the convention assumed for now)

### Not yet done
- No end-to-end test with an actual signed release yet — the whole chain (signing in CI, `latest.json` generation, real update download/install) is untested
- MSIX/Flatpak packaging itself (separate from the updater) is still deferred to pub-release prep

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
| `src/lib/components/Settings.svelte` | Settings window content (unit/thresholds/colors/autostart/credentials/update check/footer) |
| `src/lib/components/PrivacyPolicy.svelte` | Read-only privacy policy viewer (settings footer) |
| `src/lib/components/TermsOfUse.svelte` | Read-only terms of use viewer (settings footer) |
| `src/lib/components/Disclaimer.svelte` | Read-only disclaimer viewer (settings footer) |
| `src/lib/styles/wizard.css` | Styles for wizard AND settings (legal document rendering, footer links, flatpak command box) |
| `src/lib/styles/app.css` | Global styles + CSS variables |
| `src/lib/i18n/index.ts` | i18n setup with OS locale |
| `src/lib/i18n/de.json` | German translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` |
| `src/lib/i18n/en.json` | English translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` |
| `src/lib/i18n/jp.json` | Japanese translations, incl. `wizard.errors.*`, `wizard.legal.*`, `wizard.settings.*` |
| `src-tauri/src/lib.rs` | Tauri commands, app setup, window-close interception, self-updater registration, `error_code()`/`open_db()` helpers |
| `src-tauri/src/state.rs` | AppState struct |
| `src-tauri/src/tray.rs` | Tray icon, color logic, menu (left-click handler removed, `open_window` menu entry) |
| `src-tauri/src/dexcom.rs` | Dexcom Share API, typed errors, internet check |
| `src-tauri/src/db.rs` | SQLite init, queries, conversion |
| `src-tauri/src/worker.rs` | Polling worker, tray update, AppError matching |
| `src-tauri/src/keychain.rs` | OS keychain integration |
| `src-tauri/src/error.rs` | AppError enum, logger init, From conversions |
| `src-tauri/Cargo.toml` | Dependencies, `self-updater` feature flag |
| `src-tauri/capabilities/default.json` | Permissions, incl. `updater:default` |
| `src-tauri/tauri.conf.json` | Bundle resources, updater config (pubkey, endpoints, install mode) |
| `src-tauri/assets/fonts/NotoSans-Bold.ttf` | Font for tray rendering |
| `docs/legal/*.{de,en,jp}.md` | Privacy policy, terms of use, disclaimer (bundled as Tauri resources) |
| `docs/legal/store-disclaimer.md` | Short-form disclaimer for store listings (NOT bundled into the app) |
| `.github/workflows/tauri-build.yml` | Dev build workflow — fixed `projectPath`, write permissions, `self-updater` feature args |

## Tauri Commands
| Command | Purpose |
|---|---|
| `validate_credentials` | Dexcom auth + save password to keychain + update `username` setting; returns error code; used by wizard Step 3 AND settings credential change |
| `save_wizard_data` | Write all wizard settings to SQLite, mark `wizard_done` |
| `restart_app` | App restart |
| `read_legal_document` | Read a legal document for a given language from bundled resources |
| `save_legal_acceptance` | Write acceptance + version for all three legal documents to SQLite |
| `get_wizard_status` | Returns `wizard_done` bool, used by `+page.svelte` for routing |
| `get_settings` | Returns current settings for the settings window |
| `save_settings` | Writes unit/thresholds/colors/autostart (NOT credentials) |
| `check_for_update` | Self-update flow (if `self-updater` feature enabled) or store/flatpak hint string otherwise |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR merged to main |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published — builds `.exe` (NSIS) and `.AppImage`/`.deb` with `--features self-updater`, `projectPath: GlucoTray` fixed, `permissions: contents: write` at workflow root, repo "Read and write permissions" enabled (had to be unlocked at the GitHub **organization** level first — the repo-level restrictive default was inherited from the org policy) |
| `review.yml` | PR to main |
| `create-issue.yml` | Push with changes to `docs/issues.md` — confirmed working (issues #11 and #12 created this way) |
| *(planned, not yet created)* `store-build.yml` | Manual `workflow_dispatch` only — will build + submit MSIX (Store) and Flatpak (Flathub), deliberately separate from automatic dev releases |

## Installer Architecture Decision (June 2026)
- **Windows (.exe via Tauri/NSIS bundler):** planned to eventually move the entire wizard flow (including Step 0) into the NSIS installer itself, via an external helper binary for network/DB access. Not yet implemented, decision only.
- **Linux:** stays on Flatpak with the in-app wizard, since Flatpak's sandboxing model doesn't support interactive install-time hooks. An additional `.deb`/AUR path with an installer-side wizard was considered, deferred to avoid losing Flatpak's reach.
- **Microsoft Store:** keeps the in-app wizard in all cases, MSIX doesn't allow interactive installer dialogs.

## Known Open GitHub Issues
- **#11** – Dexcom LOW/HIGH string values not handled. `GlucoseReading.value` is `u32`, can't deserialize literal `"LOW"`/`"HIGH"` strings. `resolve_color` only checks `trend == "Low"` for color, no `"High"` handling at all. Not urgent.
- **#12** – Refactor frontend/backend to use mg/dL internally everywhere. `WizardStep4.svelte`/`Settings.svelte` hold mmol/L as primary dropdown state. Not urgent, consistency cleanup only.

## Open Items
- End-to-end test of self-updater with a real signed GitHub Release (untested chain: CI signing → `latest.json` → download → install)
- Wizard flow tested end-to-end on real hardware
- Settings window tested end-to-end on real hardware
- Targeted API error-case testing (invalid credentials, no session, no readings, timeout, rate limit, no internet)
- Issue #11 (LOW/HIGH value handling) and #12 (mg/dL refactor) — both deferred, not blocking
- `store-build.yml` workflow (MSIX + Flatpak, manual trigger) — to be built during pub-release prep, not now
- Flathub app ID `io.github.AgentGG00.GlucoTray` assumed by convention, not yet actually reserved/verified on Flathub
- Before first public release: Microsoft Store submission (manual, Store ID 9P2TR53FHBBH already reserved), Flathub submission, GitHub Pages with Store link + Flatpak install instructions (no more direct `.exe` link per the updated distribution strategy)

## Notes for Next Session
- Settings window uses the SAME Tauri window as the wizard — no second `WebviewWindow`. This was a deliberate mid-session correction; don't reintroduce a separate window unless explicitly requested again.
- `get_latest_reading` in `db.rs` still unused — explicitly decided NOT to use it for a settings-window live preview; no current planned use
- `delete_credentials` in `keychain.rs` — still not used; credential changes just overwrite via `save_credentials`, never explicitly delete old entries first. Worth checking for orphaned keychain entries on username change — not yet flagged as an issue, just noting it
- `MMOL_TO_MGDL` constant in `lib.rs` still unused — check if still needed
- Legal document version is set once at acceptance time (`LEGAL_VERSION = "2026-06"` in `WizardStep0.svelte`) and never re-checked at startup — no re-prompt mechanism yet if documents are updated later
- GitHub issues are readable via `web_fetch` on public issue URLs (e.g. `https://github.com/AgentGG00/glucotray/issues/<number>`) — confirmed working for #11 and #12, full body and metadata visible, no GitHub connector/MCP needed for this public repo
- Distribution strategy was significantly simplified mid-session (see "Distribution Strategy" section above) — if reviewing older context/summaries that still mention a public `.exe` download path or Azure AD as a near-term task, that information is **outdated**; the current plan is Store + Flathub only, Azure AD deferred indefinitely
- The org-level GitHub Actions policy initially blocked "Read and write permissions" at the repo level — this needed to be loosened in the **organization** settings first (not just the repo settings) before the repo-level toggle became available