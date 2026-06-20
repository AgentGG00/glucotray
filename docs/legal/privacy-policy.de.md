# Datenschutzerklärung

**Zuletzt aktualisiert: Juni 2026**

## 1. Verantwortlicher

Verantwortlich für die Datenverarbeitung im Zusammenhang mit GlucoTray ist:

Niklas Rühl
Kontakt: über das Kontaktformular auf der GlucoTray-Projektseite

## 2. Grundsatz: Lokale Datenverarbeitung

GlucoTray verarbeitet deine Gesundheitsdaten (Glukosewerte) und Zugangsdaten ausschließlich **lokal auf deinem eigenen Gerät**. Es findet keine Übertragung dieser Daten an den Entwickler oder an Dritte statt, mit Ausnahme der direkten Verbindung zur Dexcom Share API (siehe Abschnitt 4).

## 3. Welche Daten verarbeitet werden

### 3.1 Gesundheitsdaten
GlucoTray speichert Glukosewerte, Zeitstempel und Trendrichtung lokal in einer SQLite-Datenbank auf deinem Gerät. Diese Daten gelten nach Art. 9 DSGVO als besondere Kategorie personenbezogener Daten. Sie verbleiben ausschließlich auf deinem Gerät und werden zu keinem Zeitpunkt an den Entwickler übertragen.

### 3.2 Zugangsdaten
Dein Benutzername (E-Mail oder Telefonnummer) und dein Passwort für deinen CGM-Account werden im verschlüsselten Schlüsselbund deines Betriebssystems gespeichert (Windows Credential Manager bzw. Linux Secret Service). Diese Daten verlassen dein Gerät ausschließlich zur Authentifizierung gegenüber der Dexcom Share API.

### 3.3 Einstellungen
Deine Konfiguration (Schwellwerte, Farbschema, Maßeinheit, Sensor-Typ, Region, Autostart-Einstellung) wird lokal in der SQLite-Datenbank auf deinem Gerät gespeichert.

### 3.4 Protokolldaten (Logs)
GlucoTray erstellt lokale Protokolldateien zur Fehlerdiagnose (z. B. Verbindungsfehler, Authentifizierungsfehler). Diese Logs verbleiben ausschließlich auf deinem Gerät und werden nicht automatisch an den Entwickler übermittelt.

## 4. Datenübertragung an Dexcom (Dexcom Share API)

GlucoTray verbindet sich direkt von deinem Gerät aus mit der Dexcom Share API, um Glukosewerte abzurufen. Dabei werden folgende Daten an Dexcom übertragen:
- deine Zugangsdaten zur Authentifizierung,
- deine IP-Adresse (technisch bedingt bei jeder Internetverbindung).

Diese Verbindung erfolgt direkt zwischen deinem Gerät und den Servern von Dexcom, ohne dass der Entwickler von GlucoTray Einsicht in diese Daten hat oder sie verarbeitet. Es gilt zusätzlich die Datenschutzerklärung von Dexcom Inc. für deren Dienste.

## 5. Internet-Verbindungsprüfung

GlucoTray führt vor jedem Anmeldeversuch eine technische Prüfung durch, ob eine Internetverbindung besteht (Verbindungsversuch zu einem öffentlichen DNS-Server). Hierbei werden keine personenbezogenen Daten übertragen, lediglich ein technischer Verbindungstest durchgeführt.

## 6. Kontaktformular auf der Projektseite

Wenn du das Kontaktformular auf der GlucoTray-Projektseite nutzt, werden die von dir angegebenen Daten (Name, E-Mail-Adresse, Nachricht) verarbeitet, um deine Anfrage zu beantworten.

### 6.1 Auftragsverarbeiter
Der Versand erfolgt über den E-Mail-Dienstleister Brevo (Sendinblue SAS, Frankreich, EU), der als Auftragsverarbeiter im Sinne von Art. 28 DSGVO eingesetzt wird. Brevo verarbeitet die Formulardaten ausschließlich nach Anweisung und leitet sie als E-Mail an die Adresse info@glucotray.framenode.net weiter.

### 6.2 Empfang und Bearbeitung
Die eingehende E-Mail wird im E-Mail-Postfach des Entwicklers bearbeitet und beantwortet. Eine Weitergabe deiner Daten an weitere Dritte oder eine Speicherung außerhalb dieses E-Mail-Postfachs findet nicht statt.

### 6.3 Aufbewahrungsdauer
Deine Anfrage wird ab Empfang für bis zu 24 Monate aufbewahrt. Abgeschlossene Anfragen werden nach 3 Tagen im E-Mail-Postfach archiviert; dies ändert nichts an der Aufbewahrungsdauer von 24 Monaten, nach deren Ablauf die Daten gelöscht werden.

### 6.4 Rechtsgrundlage
Die Verarbeitung erfolgt auf Grundlage deiner Einwilligung (Art. 6 Abs. 1 lit. a DSGVO) durch das Absenden des Formulars.

## 7. Keine Weitergabe an sonstige Dritte

Mit Ausnahme der in Abschnitt 4 (Dexcom) und Abschnitt 6 (Brevo) genannten Fälle erfolgt keine Weitergabe deiner Daten an Dritte.

## 8. Deine Rechte

Du hast nach der DSGVO insbesondere folgende Rechte:
- Recht auf Auskunft über die zu deiner Person verarbeiteten Daten (Art. 15 DSGVO),
- Recht auf Berichtigung unrichtiger Daten (Art. 16 DSGVO),
- Recht auf Löschung (Art. 17 DSGVO),
- Recht auf Einschränkung der Verarbeitung (Art. 18 DSGVO),
- Recht auf Datenübertragbarkeit (Art. 20 DSGVO),
- Recht auf Widerspruch gegen die Verarbeitung (Art. 21 DSGVO),
- Recht auf Widerruf einer erteilten Einwilligung mit Wirkung für die Zukunft (Art. 7 Abs. 3 DSGVO).

Da Gesundheits- und Zugangsdaten innerhalb von GlucoTray ausschließlich lokal auf deinem eigenen Gerät gespeichert werden, kannst du diese Daten jederzeit selbst einsehen, ändern oder löschen (z. B. durch Deinstallation der App oder Löschung der lokalen Datenbank).

Für Anfragen zu den über das Kontaktformular verarbeiteten Daten kannst du dich über die in Abschnitt 1 genannten Kontaktwege an den Verantwortlichen wenden.

## 9. Beschwerderecht

Du hast das Recht, dich bei einer Datenschutz-Aufsichtsbehörde über die Verarbeitung deiner personenbezogenen Daten zu beschweren.

## 10. Änderungen dieser Datenschutzerklärung

Diese Datenschutzerklärung kann angepasst werden, insbesondere bei der Einführung neuer Funktionen (z. B. Anbindung weiterer CGM-Anbieter, optionale Fehlerberichte). Die jeweils aktuelle Version ist im Repository sowie auf der Projektseite einsehbar.