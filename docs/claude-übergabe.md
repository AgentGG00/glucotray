# Übergabe – GlucoTray

## Projekt
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **Lizenz:** MIT
- **Status:** Wizard Step 1–3 abgeschlossen, Step 4 (Settings) als nächstes

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
- Success → Step 4
- Fail → Step 2 mit Fehlermeldung

### Step 4 – Settings (TODO)
Laut Projektplan:
- Einheit: mg/dL oder mmol/L
- Grenzwerte: Low / High
- Farbschema
- Autostart

### Step 5 – Completion (TODO)
- Zusammenfassung
- `invoke("save_wizard_data")` mit allen gesammelten Daten
- Tray-Widget aktiv

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
| `src/lib/styles/wizard.css` | Wizard-Styles |
| `src/lib/i18n/index.ts` | i18n Setup mit OS-Locale |
| `src/lib/i18n/de.json` | Deutsche Übersetzungen |
| `src/lib/i18n/en.json` | Englische Übersetzungen |
| `src/lib/i18n/jp.json` | Japanische Übersetzungen |
| `src-tauri/src/lib.rs` | Tauri Commands |
| `src-tauri/src/dexcom.rs` | Dexcom Share API |

## Workflows
| Workflow | Trigger |
|---|---|
| `release.yml` | PR auf main gemergt |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published |
| `review.yml` | PR auf main |
| `create-issue.yml` | Push mit Änderung an `docs/issues.md` |

## Offene Punkte
- Step 4: Settings-Screen bauen
- Step 5: Completion-Screen + `save_wizard_data` aufrufen
- Tray-Icon mit Live-Wert implementieren
- Trend-Pfeil
- Farbschema-Logik
- Einheit-Toggle mg/dL / mmol/L
- Autostart-Implementierung (Windows + Linux)
- Fehlerbehandlung im Worker (No readings, Timeout, Rate limit)