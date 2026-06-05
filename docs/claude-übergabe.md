# Übergabe – GlucoTray

## Projekt
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **Lizenz:** MIT
- **Status:** Wizard vollständig (Step 1–5), nächstes Ziel: Tray-Icon mit Live-Wert

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

## Wizard – aktueller Stand

### Step 1 – Sensor & Region
- G6 / G7 Auswahl
- Region: USA / Außerhalb der USA / Japan
- Voraussetzungs-Checklist
- Sprache per Flag-Button wechselbar (de/en/jp)

### Step 2 – Credentials
- Login-Typ wählbar: E-Mail oder Telefonnummer
- E-Mail: Regex-Validierung + Levenshtein-Tippfehler-Erkennung für gängige Domains
- Telefon: libphonenumber-js, nationale 0 → internationale Vorwahl, OS-Locale für Land
- Passwort: custom •••-Darstellung, letztes Zeichen 3s sichtbar, Eye-Toggle, Paste-Support, Passwort-Manager-kompatibel
- `externalError` Prop für Fehler aus Step 3

### Step 3 – Auth Loading
- Spinner + i18n-Text
- `onMount` → `invoke("validate_credentials")`
- Bei Erfolg: Passwort wird direkt im OS Keychain gespeichert (`save_credentials`)
- Success → Step 4
- Fail → Step 2 mit Fehlermeldung

### Step 4 – Settings
- Einheit: mg/dL oder mmol/L (Radio-Buttons)
- Dropdowns ausgegraut bis Einheit gewählt
- Min-Grenzwert: 2.8–4.5 mmol/L (feste Schritte), Anzeige je nach Einheit umgerechnet
- Max-Grenzwert: 8.0–13.0 mmol/L, nur Werte > Min verfügbar
- Farbschema: 5 Zonen (Sehr Hoch / Hoch / Normal / Niedrig / Kritisch Niedrig), nativer Farbpicker, Defaults nach klinischem Standard
- Autostart-Checkbox: „Soll GlucoTray mit Windows starten?"
- Intern wird immer in mmol/L gespeichert

### Step 5 – Completion
- `onMount` → `invoke("save_wizard_data")` mit allen Wizard-Daten
- Passwort wird nicht übergeben (liegt bereits im Keychain aus Step 3)
- Threshold-Werte werden als Integer übergeben (mgdl: direkt, mmol: × 10)
- Zusammenfassung: Sensor, Region, Username, Einheit, Min/Max, Autostart, Farbswatches
- Fehleranzeige bei `save_wizard_data`-Fehler
- Finish-Button → `handleFinish()` in `+page.svelte` (noch leer, Tray-Widget folgt)

## i18n
- System-Locale via `tauri-plugin-os` → Fallback `getLocaleFromNavigator()`
- Sprachen: de, en, jp
- `setupI18n()` ist async, wird in `+layout.svelte` per `{#await}` abgewartet
- Land (für Telefon-Formatter) ist von Sprache entkoppelt – kommt separat aus OS-Locale

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
| `src-tauri/src/lib.rs` | Tauri Commands |
| `src-tauri/src/dexcom.rs` | Dexcom Share API |
| `src-tauri/src/db.rs` | SQLite Init + Queries |
| `src-tauri/src/worker.rs` | Polling Worker |
| `src-tauri/src/keychain.rs` | OS Keychain Integration |
| `src-tauri/src/error.rs` | Logger Init |

## Tauri Commands
| Command | Zweck |
|---|---|
| `validate_credentials` | Dexcom Auth + Passwort in Keychain speichern |
| `save_wizard_data` | Alle Settings in SQLite schreiben |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR auf main gemergt |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published |
| `review.yml` | PR auf main |
| `create-issue.yml` | Push mit Änderung an `docs/issues.md` |

## Offene Punkte
- `handleFinish()` in `+page.svelte` implementieren (Fenster schließen / Tray aktiv)
- Tray-Icon mit Live-Wert implementieren
- Trend-Pfeil
- Farbschema-Logik im Tray anwenden
- Einheit-Toggle mg/dL / mmol/L im Tray
- Autostart-Implementierung (Windows + Linux)
- Fehlerbehandlung im Worker (No readings, Timeout, Rate limit)
- Wizard flow end-to-end auf echter Maschine testen