# Issues

<!-- Format: ## Titel, labels: (optional), body: Beschreibung, --- als Trenner -->

## Beispiel Issue
labels: bug
body: Kurze Beschreibung was das Problem ist und wo es auftritt.

---

## Dexcom LOW/HIGH string values not handled
labels: bug
body: Dexcom's Share API can return the string values "LOW" or "HIGH" instead of a numeric value for `Value` when glucose is below or above the sensor's measurable range. `GlucoseReading.value` in `dexcom.rs` is typed as `u32`, so deserialization of a non-numeric string likely fails or produces unexpected behavior. `resolve_color` in `tray.rs` currently only checks `trend == "Low"` for coloring purposes and does not adjust the displayed numeric value itself; there is no handling for a `"High"` trend/value string at all. Needs a defined value range mapping (example target ranges in mmol/L: 2.2–3.1 urgent low, 3.2–4.0 low, 4.1–8.9 normal, 9.0–13.9 high, 14.0–16.6/22.2 very high depending on trend graph setting, plus LOW/HIGH as their own distinct states) and corresponding handling in `dexcom.rs` and `tray.rs` so both the displayed value and the tray color are correct for these edge cases.

  ---

  Die Dexcom Share API kann statt eines numerischen Werts die Strings "LOW" oder "HIGH" für `Value` zurückgeben, wenn der Glukosewert außerhalb des messbaren Bereichs des Sensors liegt. `GlucoseReading.value` in `dexcom.rs` ist als `u32` typisiert, wodurch die Deserialisierung eines nicht-numerischen Strings wahrscheinlich fehlschlägt oder zu unerwartetem Verhalten führt. `resolve_color` in `tray.rs` prüft aktuell nur `trend == "Low"` für die Farblogik und passt den angezeigten Zahlenwert selbst nicht an; für einen `"High"`-Trend/Wert-String gibt es überhaupt keine Behandlung. Es wird eine definierte Wertebereich-Zuordnung benötigt (Beispiel-Zielbereiche in mmol/L: 2,2–3,1 dringend niedrig, 3,2–4,0 niedrig, 4,1–8,9 normal, 9,0–13,9 hoch, 14,0–16,6/22,2 sehr hoch je nach Trenddiagramm-Einstellung, plus LOW/HIGH als eigene, separate Zustände) sowie eine entsprechende Behandlung in `dexcom.rs` und `tray.rs`, damit sowohl der angezeigte Wert als auch die Tray-Farbe für diese Grenzfälle korrekt sind.

---

## Refactor frontend/backend to use mg/dL internally everywhere, convert to mmol/L only for display
labels: refactor
body: The Dexcom Share API always returns values in mg/dL, and db.rs is meant to be the single conversion point (per claude-übergabe.md). However, several frontend components (WizardStep4.svelte, Settings.svelte) currently hold their primary state in mmol/L (e.g. minMmol/maxMmol, fixed MIN_VALUES_MMOL/MAX_VALUES_MMOL lists) and convert to mg/dL only at the end via mmolToMgdl(). This works but contradicts the stated architecture principle and adds unnecessary round-trip conversions (mg/dL from DB → mmol/L for state → mg/dL again for saving). Should be refactored so all internal state (frontend and backend) is mg/dL-based, with mmol/L conversion happening only at the point of display (e.g. dropdown labels), not as the underlying state itself. Not structurally urgent, can be addressed later.

  Die Dexcom Share API liefert Werte immer in mg/dL, und db.rs soll laut claude-übergabe.md der einzige Umrechnungspunkt sein. Aktuell halten jedoch mehrere Frontend-Komponenten (WizardStep4.svelte, Settings.svelte) ihren primären State in mmol/L (z.B. minMmol/maxMmol, feste Listen MIN_VALUES_MMOL/MAX_VALUES_MMOL) und rechnen erst am Ende über mmolToMgdl() nach mg/dL um. Das funktioniert, widerspricht aber dem eigentlich festgelegten Architekturprinzip und führt zu unnötigen Hin- und Rückrechnungen (mg/dL aus DB → mmol/L für State → wieder mg/dL zum Speichern). Sollte so umgebaut werden, dass der gesamte interne State (Frontend und Backend) auf mg/dL basiert, und die Umrechnung nach mmol/L nur an der Anzeigestelle (z.B. Dropdown-Labels) passiert, nicht als zugrunde liegender State selbst. Strukturell nicht dringend, kann später angegangen werden.

---