# Passwortgenerator · 1.0.1

Herausgeber: Karsten vom Wege

**[Installationsdateien für Windows, macOS und Ubuntu herunterladen](https://github.com/epicrhapsody/passwort-generator/releases/tag/v1.0.1)**

Kleine Desktop-App mit Rust und Tauri 2 für Windows, macOS und Ubuntu. Deutsche Oberfläche, 8–128 Zeichen, wählbare Zeichenarten, Ausschluss ähnlicher Zeichen und Kopierfunktion. Keine Speicherung, Telemetrie oder Netzwerkaufrufe der Anwendung.

## Entwicklung

Node.js 22.12+ und aktuelles stabiles Rust installieren. Zusätzlich die [Tauri-Systemvoraussetzungen](https://v2.tauri.app/start/prerequisites/) erfüllen: unter Windows Visual Studio C++ Build Tools und WebView2, unter macOS Xcode Command Line Tools; unter Ubuntu:

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev librsvg2-dev libayatana-appindicator3-dev
```

Im Projektordner:

```sh
npm ci
npm run tauri dev
```

`npm run dev` allein öffnet nur das Frontend; Passworterzeugung benötigt die Desktop-App.

## Prüfen und bauen

```sh
npm test
cargo test -p password-core --locked
cargo fmt --all --check
npm run build
npm run tauri build
```

Installer werden auf dem jeweiligen Zielbetriebssystem gebaut. Ausgabe: `target/release/bundle/`. Windows: NSIS/MSI; macOS: App/DMG; Ubuntu: DEB/AppImage. Für öffentliche Verteilung sind plattformspezifische Signierung und unter macOS Notarisierung separat einzurichten. Der Anzeigename und die Bundle-ID lassen sich in `src-tauri/tauri.conf.json` ändern.

## Sicherheit und Grenzen

Die Rust-Bibliothek bezieht Zufall über `getrandom` vom Betriebssystem. Rejection Sampling vermeidet Modulo-Verzerrung und stellt sicher, dass jede ausgewählte Zeichenart enthalten ist. Ungültige Optionen und Fehler des Zufallsgenerators werden abgefangen. Passwörter werden weder gespeichert noch protokolliert. Sie werden sichtbar angezeigt und bleiben bis zur nächsten Generierung oder zum Beenden im Arbeitsspeicher; eine garantierte Speicherlöschung wird nicht zugesichert.

Kopieren erfolgt nur per Klick. Die Zwischenablage wird nicht automatisch gelöscht. Betriebssystem-Clipboard-Historie oder Synchronisierung können kopierte Passwörter behalten. Die App besitzt ausschließlich Schreibrechte für die Zwischenablage.

Die Tests prüfen alle 15 Kombinationen der Zeichenarten, Längengrenzen, ähnliche Zeichen und ungültige Eingaben. Plattformübergreifende Builds ersetzen keine manuellen UI-Tests auf den Zielsystemen.

Technische Referenzen: [Tauri-Projektaufbau](https://v2.tauri.app/start/create-project/) und [Clipboard-Plugin](https://v2.tauri.app/plugin/clipboard/).

## Automatische Installer-Builds

Den Inhalt dieses Projektordners als Wurzel eines GitHub-Repositories verwenden. `.github/workflows/build.yml` baut bei Push auf `main`, bei Pull Requests oder manuell über „Actions → Desktop installers → Run workflow“ auf Ubuntu 22.04, Windows sowie macOS für Apple Silicon und Intel. Die Installer liegen danach als Workflow-Artefakte vor. Version 1.0.1 ist zusätzlich als öffentliches GitHub-Release verfügbar. macOS verwendet eine lokale Ad-hoc-Signatur; eine Apple-Developer-Signatur und Notarisierung sind damit nicht enthalten. Alle vier Plattform-Builds sind erfolgreich. [Installer und Prüfsummen](https://github.com/epicrhapsody/passwort-generator/actions/runs/41245374394).

Für ältere Ubuntu-Versionen sollten die dort erzeugten Linux-Pakete verwendet werden: Ein lokal auf einem neueren Ubuntu gebautes Binary kann neuere Systembibliotheken voraussetzen.

Quelle für den Plattform-Build: [Tauri GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/).

Frontend-Tests prüfen Kopieren auf Benutzeraktion, Fehlerbehandlung und das Verwerfen verspäteter Ergebnisse nach Einstellungsänderungen.

## Lokal geprüft (31.10.2026)

Ubuntu 26.04 x86_64: Version 1.0.1: Release-Build und Bundling als DEB und AppImage erfolgreich, DEB-Paketmetadaten geprüft. Die native Release-App lief beim Starttest acht Sekunden ohne Fehlermeldung und wurde anschließend gezielt beendet. Dies ist ein Starttest, kein vollständiger manueller Bedienungstest. Zwei Rust-Tests und fünf Frontend-Tests bestehen. Windows x64 sowie macOS Intel und Apple Silicon wurden ebenfalls erfolgreich auf GitHub gebaut. Manuelle Installationstests auf Windows und macOS stehen aus.

Plattformspezifische Build-Befehle: [BUILD.md](BUILD.md). Das finale Schlüssel-Icon liegt als SVG in `branding/icon.svg` und als PNG/ICO/ICNS in `src-tauri/icons/`.

## Lizenz und Linux-Metadaten

MIT, Copyright 2026 Karsten vom Wege; siehe LICENSE. Das DEB enthält AppStream-Metadaten mit Entwickler, Lizenz, Release und Zuordnung zum Desktop-Eintrag sowie `/usr/share/doc/passwortgenerator/copyright`.

Die AppStream-Prüfung besteht mit einem verbleibenden Hinweis auf die fehlende Homepage. Es wurde keine Website erfunden oder eingetragen. Die Standardprüfung meldet diesen Hinweis als Warnung.

Das Ubuntu App Center zeigt beim Öffnen lokaler DEB-Dateien im geprüften Quellcode weder Herausgeber noch Paket-Icon an und wertet hierfür die AppStream-Datei nicht aus. Die Drittanbieter-Warnung bleibt ebenfalls bestehen. Diese Anzeige bestätigt keinen Fehler im Maintainer-Feld; dieses enthält Karsten vom Wege. Auch die Lizenzanzeige des lokalen Installationsdialogs kann weiterhin „unknown“ lauten. Nach Installation liegt das Icon im regulären Anwendungsmenü vor.

Geprüfter App-Center-Quellcode: https://github.com/ubuntu/app-center/blob/775751f5c12cfbf1f1f9a3f8aadda945e1ef73aa/packages/app_center/lib/widgets/app_title.dart
