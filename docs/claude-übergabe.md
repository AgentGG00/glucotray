# Übergabe – GlucoTray

## Projekt
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **Lizenz:** MIT
- **Status:** Wizard + Tray vollständig, Autostart implementiert – nächstes Ziel: Fehlerbehandlung Worker + Settings-Fenster

## Stack
- **Frontend:** Svelte 5 + TypeScript + Tailwind CSS
- **Backend:** Rust (Tauri 2)
- **Datenbank:** SQLite (lokal)
- **Credentials:** OS Keychain (Windows Credential Manager / Linux Secret Service)
- **API:** Dexcom Share API, 150s Polling

## Branch-Strategie
- `main` – stable, nur Releases
- `dev` – aktive Entwicklung
- Feature-Branches von `dev`: `feature/name`, `fix/name`

## Daten & Einheiten
- API liefert immer mg/dL als arabische Ziffern
- Einzige Umrechnungsstelle: `db.rs` → `insert_reading` schreibt `value_mgdl` + `value_mmol`
- Schwellwerte in DB immer als mg/dL Integer (`threshold_low`, `threshold_high`)
- On-the-fly Umrechnung nur für Anzeige im Frontend
- Feste klinische Grenzen: Very High = 250 mg/dL, Critical Low = 54 mg/dL
- API-String `"Low"` triggert ebenfalls Critical Low Zone

## AppState
- `unit: String` – wird beim App-Start aus DB geladen
- Bei Einheiten-Änderung in Settings: App-Neustart erforderlich
- `TrayState` – enthält `update_available: bool` für Update-Badge

## Tray-Icon
- Dynamisch gerendert via `imageproc` + `ab_glyph` + NotoSans-Bold.ttf
- Zeigt Wert in mg/dL oder mmol/L je nach `AppState.unit`
- Trendpfeil als Unicode: ⇈ ↑ ↗ → ↘ ↓ ⇊
- 5 Farbzonen: Critical Low / Low / Normal / High / Very High
- N/A bei `is_valid = false` oder `value = 0`
- Update-Badge: roter Punkt oben rechts wenn `update_available = true`
- Tooltip zeigt beide Einheiten: 97 mg/dL / 5.4 mmol/L ↑
- Kontextmenü: GlucoTray (disabled) / Update check / Quit / Restart
- Linksklick öffnet Settings-Fenster (noch zu bauen)

## App-Flow

### Erster Start
App startet → DB leer → Fenster sichtbar → Wizard → Wizard abgeschlossen → save_wizard_data → restart_app → Neustart → normaler Start

### Normaler Start
DB init → unit + autostart aus DB → AppState befüllen → autostart enable/disable → Worker starten → Tray aktiv → erster Start: Toast-Notification (einmalig, tray_hint_shown in DB)

## Autostart
- `tauri-plugin-autostart` – Windows Registry + Linux .desktop
- Wird bei jedem App-Start aus DB gelesen und gesetzt
- Bei Änderung in Settings: Neustart erforderlich damit Änderung greift

## Wizard – aktueller Stand

### Step 1 – Sensor & Region
- G6 / G7 Auswahl, Region: USA / OUS / Japan
- Voraussetzungs-Checklist, Sprache per Flag-Button (de/en/jp)

### Step 2 – Credentials
- Login-Typ: E-Mail oder Telefonnummer
- E-Mail: Regex + Levenshtein-Tippfehler-Erkennung
- Telefon: libphonenumber-js, OS-Locale für Land
- Passwort: custom Darstellung, Eye-Toggle, Paste-Support
- `externalError` Prop für Fehler aus Step 3

### Step 3 – Auth Loading
- `onMount` → `invoke("validate_credentials")`
- Erfolg → Step 4, Fehler → Step 2 mit Fehlermeldung

### Step 4 – Settings
- Einheit: mg/dL oder mmol/L (Radio-Buttons)
- Dropdowns zeigen Werte in gewählter Einheit an
- Intern immer mmol-Werte, `handleNext` rechnet nach mg/dL um
- Min: 2.8–4.5 mmol/L, Max: 8.0–13.0 mmol/L
- 5 Farbzonen mit nativem Farbpicker
- Autostart-Checkbox

### Step 5 – Completion
- `onMount` → `invoke("save_wizard_data")` mit mg/dL Schwellwerten
- Zusammenfassung, Fehleranzeige bei save-Fehler
- Finish → `invoke("restart_app")`

## Wichtige Dateien
| Datei | Zweck |
|---|---|
| `src/routes/+page.svelte` | Wizard-Steuerung, State-Management |
| `src/routes/+layout.svelte` | i18n Init, Theme Init |
| `src/lib/components/WizardStep1.svelte` | Step 1 |
| `src/lib/components/WizardStep2.svelte` | Step 2 |
| `src/lib/components/WizardStep3.svelte` | Step 3 |
| `src/lib/components/WizardStep4.svelte` | Step 4 |
| `src/lib/components/WizardStep5.svelte` | Step 5 |
| `src/lib/styles/wizard.css` | Wizard-Styles |
| `src/lib/styles/app.css` | Globale Styles + CSS-Variablen |
| `src/lib/i18n/index.ts` | i18n Setup mit OS-Locale |
| `src/lib/i18n/de.json` | Deutsche Übersetzungen |
| `src/lib/i18n/en.json` | Englische Übersetzungen |
| `src/lib/i18n/jp.json` | Japanische Übersetzungen |
| `src-tauri/src/lib.rs` | Tauri Commands, App-Setup |
| `src-tauri/src/state.rs` | AppState Struct |
| `src-tauri/src/tray.rs` | Tray-Icon, Farblogik, Menü |
| `src-tauri/src/dexcom.rs` | Dexcom Share API |
| `src-tauri/src/db.rs` | SQLite Init, Queries, Umrechnung |
| `src-tauri/src/worker.rs` | Polling Worker, Tray-Update |
| `src-tauri/src/keychain.rs` | OS Keychain Integration |
| `src-tauri/src/error.rs` | Logger Init |
| `src-tauri/assets/fonts/NotoSans-Bold.ttf` | Font für Tray-Rendering |

## Tauri Commands
| Command | Zweck |
|---|---|
| `validate_credentials` | Dexcom Auth + Passwort in Keychain speichern |
| `save_wizard_data` | Alle Settings in SQLite schreiben |
| `restart_app` | App-Neustart |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR auf main gemergt |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published |
| `review.yml` | PR auf main |
| `create-issue.yml` | Push mit Änderung an `docs/issues.md` |

## Offene Punkte
- Fehlerbehandlung Worker (No readings, Timeout, Rate limit)
- Settings-Fenster (Einheit, Schwellwerte, Farben, Autostart, Credentials ändern, Nutzungsbedingungen)
- Einheit-Änderung → Neustart-Hinweis im Settings-Fenster
- Wizard flow end-to-end auf echter Maschine testen
- Vor erstem Public Release: Azure AD + Store Submission, Flathub Bot, In-App Updater

## Hinweise für nächste Session
- Settings-Fenster ist ein neues Tauri-Fenster (Linksklick auf Tray öffnet es)
- Einheiten-Änderung erfordert App-Neustart wegen AppState
- `get_latest_reading` in db.rs noch unused – wird im Settings-Fenster gebraucht
- `delete_credentials` in keychain.rs – wird beim Credentials-Ändern gebraucht
- `MMOL_TO_MGDL` Konstante in lib.rs noch unused – prüfen ob noch nötig