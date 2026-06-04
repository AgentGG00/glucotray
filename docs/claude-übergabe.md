# Übergabe – GlucoTray

## Projekt
- **Name:** GlucoTray
- **Repo:** https://github.com/AgentGG00/glucotray
- **Publisher:** AgentGG
- **Lizenz:** MIT
- **Status:** Phase 2 abgeschlossen – Phase 2.2 Umsetzung beginnt

## Stack
- **Frontend:** Svelte + TypeScript + Tailwind CSS
- **Backend:** Rust (Tauri 2)
- **Datenbank:** SQLite (lokal)
- **Credentials:** OS Keychain (Windows Credential Manager / Linux Secret Service)
- **API:** Dexcom Share API, 150s Polling

## Branch-Strategie
- `main` – stable, nur Releases
- `dev` – aktive Entwicklung
- Feature-Branches von `dev`: `feature/name`, `fix/name`

## Nächster Schritt
Tauri + Svelte Projekt initialisieren:
```bash
pnpm create tauri-app
```
- App Name: `GlucoTray`
- Package Manager: `pnpm`
- Frontend: `Svelte` + `TypeScript`

## Workflows

| Workflow | Trigger |
|---|---|
| `release.yml` | PR auf main gemergt |
| `ci-build.yml` | Release published |
| `tauri-build.yml` | Release published |
| `review.yml` | PR auf main |
| `create-issue.yml` | Push mit Änderung an `docs/issues.md` |

## Projektstruktur

´´´txt
glucotray/
├── src/                  # Svelte Frontend
│   ├── lib/
│   │   ├── components/
│   │   └── services/
│   └── routes/
├── src-tauri/            # Rust Backend
├── pages/                # GitHub Pages Webapp
├── docs/                 # Projektdokumentation
├── .github/workflows/
└── .devcontainer/
´´´

## MVP Features
- Tray-Icon mit Live-Blutzuckerwert
- Trendpfeil (steigend, fallend, stabil)
- Farbschema konfigurierbar
- Einheit: mg/dL oder mmol/L
- Autostart
- Setup-Wizard mit Dexcom Share Initialisierung
- Fehlerbehandlung mit klaren Meldungen

## Reusable Workflows
Repo: `AgentGG00/workflows`

## Offene Punkte
- Tauri Projekt initialisieren
- Svelte Frontend aufsetzen
- Dexcom Share API Anbindung in Rust
- OS Keychain Integration
- SQLite Setup
- Setup-Wizard bauen