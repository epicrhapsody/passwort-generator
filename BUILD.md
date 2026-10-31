# Builds · 1.0.1

Herausgeber: Karsten vom Wege

Die Systemvoraussetzungen stehen in der README. Alle Befehle im Projektordner ausführen.

| System | Befehle nach `npm ci` |
| --- | --- |
| Windows x64 | `npm run tauri build -- --ci --bundles nsis,msi -- --locked` |
| macOS Apple Silicon | `APPLE_SIGNING_IDENTITY=- npm run tauri build -- --ci --bundles app,dmg -- --locked` |
| macOS Intel | Auf einem Intel-Mac denselben macOS-Befehl ausführen. |
| Ubuntu x64 | `npm run tauri build -- --ci --bundles deb,appimage -- --locked` |

Installer: `target/release/bundle/`. Signierung mit Entwicklerzertifikat und macOS-Notarisierung sind nicht konfiguriert. Die macOS-Ad-hoc-Signatur bestätigt keine Herausgeberidentität.

Alternativ baut `.github/workflows/build.yml` alle vier Ziele, sobald der Quellcode als eigenes GitHub-Repository vorliegt. Zielrepository: https://github.com/epicrhapsody/passwort-generator. Alle vier Plattform-Builds sind erfolgreich; manuelle Installationstests auf Windows und macOS stehen aus.

[Installer-Downloads und Prüfsummen](https://github.com/epicrhapsody/passwort-generator/actions/runs/41245374394)
