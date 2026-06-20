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