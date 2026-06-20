# Privacy Policy

**Last updated: June 2026**

## 1. Controller

The party responsible for data processing in connection with GlucoTray is:

Niklas Rühl
Contact: via the contact form on the GlucoTray project website

## 2. Principle: Local Data Processing

GlucoTray processes your health data (glucose values) and credentials exclusively **locally on your own device**. None of this data is transmitted to the developer or any third party, with the exception of the direct connection to the Dexcom Share API (see Section 4).

## 3. What Data Is Processed

### 3.1 Health Data
GlucoTray stores glucose values, timestamps, and trend direction locally in an SQLite database on your device. Under Art. 9 GDPR, this data is classified as a special category of personal data. It remains exclusively on your device and is never transmitted to the developer.

### 3.2 Credentials
Your username (email or phone number) and your CGM account password are stored in your operating system's encrypted keychain (Windows Credential Manager or Linux Secret Service). This data leaves your device only for authentication with the Dexcom Share API.

### 3.3 Settings
Your configuration (thresholds, color scheme, unit of measurement, sensor type, region, autostart setting) is stored locally in the SQLite database on your device.

### 3.4 Log Data
GlucoTray creates local log files for error diagnosis (e.g. connection errors, authentication errors). These logs remain exclusively on your device and are not automatically transmitted to the developer.

## 4. Data Transmission to Dexcom (Dexcom Share API)

GlucoTray connects directly from your device to the Dexcom Share API to retrieve glucose values. The following data is transmitted to Dexcom in this process:
- your credentials, for authentication,
- your IP address (a technical necessity of any internet connection).

This connection is established directly between your device and Dexcom's servers; the developer of GlucoTray has no access to or insight into this data. Dexcom Inc.'s own privacy policy additionally applies to their services.

## 5. Internet Connectivity Check

Before each login attempt, GlucoTray performs a technical check to determine whether an internet connection is available (a connection attempt to a public DNS server). No personal data is transmitted during this check; it is purely a technical connectivity test.

## 6. Contact Form on the Project Website

If you use the contact form on the GlucoTray project website, the data you provide (name, email address, message) is processed in order to respond to your inquiry.

### 6.1 Processor
Sending is handled via the email service provider Brevo (Sendinblue SAS, France, EU), engaged as a processor within the meaning of Art. 28 GDPR. Brevo processes the form data solely according to instructions and forwards it as an email to info@glucotray.framenode.net.

### 6.2 Receipt and Handling
The incoming email is handled and answered in the developer's email inbox. Your data is not passed on to any further third parties, nor stored outside of this email inbox.

### 6.3 Retention Period
Your inquiry is retained for up to 24 months from the date of receipt. Closed inquiries are archived within the email inbox after 3 days; this does not affect the 24-month retention period, after which the data is deleted.

### 6.4 Legal Basis
Processing is based on your consent (Art. 6(1)(a) GDPR), given by submitting the form.

## 7. No Disclosure to Other Third Parties

Except for the cases described in Section 4 (Dexcom) and Section 6 (Brevo), your data is not disclosed to any third parties.

## 8. Your Rights

Under the GDPR, you have in particular the following rights:
- the right to access the data processed about you (Art. 15 GDPR),
- the right to rectification of inaccurate data (Art. 16 GDPR),
- the right to erasure (Art. 17 GDPR),
- the right to restriction of processing (Art. 18 GDPR),
- the right to data portability (Art. 20 GDPR),
- the right to object to processing (Art. 21 GDPR),
- the right to withdraw consent already given, with effect for the future (Art. 7(3) GDPR).

Since health data and credentials within GlucoTray are stored exclusively locally on your own device, you can view, change, or delete this data yourself at any time (e.g. by uninstalling the app or deleting the local database).

For inquiries regarding the data processed via the contact form, you can reach the controller using the contact details listed in Section 1.

## 9. Right to Lodge a Complaint

You have the right to lodge a complaint with a data protection supervisory authority regarding the processing of your personal data.

## 10. Changes to This Privacy Policy

This Privacy Policy may be updated, particularly when new features are introduced (e.g. integration of additional CGM providers, optional error reporting). The current version is available in the repository and on the project website.