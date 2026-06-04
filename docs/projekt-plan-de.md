# Projekt-Plan – GlucoTray

## Projektbeschreibung

GlucoTray ist eine leichtgewichtige System-Tray-App für Windows 11 und Linux KDE Plasma.
Sie zeigt Live-Blutzuckerwerte von Dexcom CGM-Sensoren direkt in der Taskleiste an.

---

## Ziele

- Live-Anzeige des aktuellen Blutzuckerwerts im System Tray
- Trendpfeil und konfigurierbares Farbschema
- Geführter Setup-Wizard für nicht-technische Nutzer
- Plattformunterstützung: Windows 11 und Linux KDE Plasma
- Open Source, MIT-lizenziert

---

## Tech-Stack

| Bereich | Technologie | Begründung |
|---|---|---|
| Frontend | Svelte + TypeScript + Tailwind CSS | Bekannter Stack, schnell, leichtgewichtig |
| Backend | Rust (Tauri 2) | Nativer Tray-Support, niedriger RAM-Verbrauch (~30MB), MSIX-Build out of the box |
| Datenbank | SQLite (lokal) | Verlaufsdaten und Settings, keine Cloud-Abhängigkeit |
| Credentials | OS Keychain | Windows Credential Manager / Linux Secret Service, verschlüsselt |
| API | Dexcom Share API | Einzige Option für Live-Daten, community-erprobt via pydexcom |
| CI/CD | GitHub Actions | Automatischer Build von .exe, .AppImage, .msix bei Release-Tag |

---

## Features – MVP

- [ ] Tray-Icon mit Live-Blutzuckerwert
- [ ] Trendpfeil (steigend, fallend, stabil)
- [ ] Farbschema konfigurierbar (grün/gelb/rot nach Grenzwerten)
- [ ] Einheit wählbar: mg/dL oder mmol/L
- [ ] Polling-Intervall: 150 Sekunden
- [ ] Autostart bei Login (Windows + Linux)
- [ ] Setup-Wizard mit Dexcom-Initialisierung
- [ ] Fehlerbehandlung mit klaren Meldungen
- [ ] Einstellungsfenster

---

## Setup-Wizard Flow

1. G6 oder G7 Auswahl
2. Voraussetzungs-Checklist (App installiert, Share aktiv, Follower eingetragen)
3. Credentials-Eingabe → Speicherung im OS Keychain
4. Live API-Validierung
5. Einstellungen (Einheit, Grenzwerte, Farbschema, Autostart)
6. Abschluss → Tray-Widget aktiv

---

## Fehlerbehandlung

| Fehler | Ursache | Meldung |
|---|---|---|
| Invalid credentials | Falsches Passwort / falsche Region | "Benutzername oder Passwort falsch. Bist du außerhalb der USA? Stelle sicher dass du 'Outside US' ausgewählt hast." |
| No session | Share nicht aktiviert | "Keine aktive Share-Session gefunden. Share in der Dexcom App aktivieren und mindestens einen Follower eintragen." |
| No readings | Sensor läuft nicht | "Keine aktuellen Messwerte. Läuft dein Sensor gerade?" |
| Timeout | Keine Internetverbindung | "Verbindung zur Dexcom API fehlgeschlagen. Internetverbindung prüfen." |
| Rate limit | Zu viele Anfragen | "Zu viele Anfragen. Bitte kurz warten." |

---

## Distribution

| Plattform | Format | Kanal |
|---|---|---|
| Windows | `.exe` Installer | GitHub Releases |
| Windows | MSIX | Microsoft Store |
| Linux | `.AppImage` | GitHub Releases |
| Linux | Flatpak | Flathub (nach Review) |
| Listing | Link | pling.com |

---

## Projektseite

GitHub Pages über `glucotray.github.io`

---

## Rahmenbedingungen

- Entwicklung in GitHub Codespaces
- Branch-Strategie: `main` (stable) + `dev` (aktiv)
- Publisher: AgentGG
- Lizenz: MIT
- Microsoft Store Name reserviert: GlucoTray