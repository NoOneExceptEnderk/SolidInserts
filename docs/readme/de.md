# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · [Español](es.md) · [Français](fr.md) · **Deutsch** · [Português (Portugal)](pt.md) · [Русский](ru.md) · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Erstelle in SolidCraft Desktop Aufnahmen für Gewindeeinsätze zum Einschmelzen, ausgehend von vorhandenen Bohrungen.

SolidInserts positioniert die Aufnahme an der kreisförmigen Bohrungskante. Wähle eine CNC-Kitchen-Serie und eine Schraubengröße, um mehrere Aufnahmen in einem Arbeitsgang zu erstellen. Skizzen und Schnitte bleiben im Modellverlauf von SolidCraft bearbeitbar. Einmal **Rückgängig** entfernt alle Schnitte des Arbeitsgangs.

## SolidInserts ausprobieren

1. Verwende [SolidCraft Desktop 0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0). Lade das Plugin-ZIP aus den [Releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases) herunter, sobald es verfügbar ist. Du kannst das Plugin auch mit den Befehlen unten erstellen. Dabei entsteht `dist/solidinserts-<Version>.zip`.
2. Entpacke das ZIP in den Plugin-Ordner deines Systems. Lass `plugin.json` und `plugin.wasm` zusammen im Unterordner `solidinserts`.

   | System | Plugin-Ordner |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (normalerweise `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Öffne die [Musterplatte mit Bohrungen von Ø3 und Ø4 mm](../../examples/pilot-holes.sc). Wähle die kreisförmigen Kanten auf der Einsetzseite. Öffne **Plugin → Sedi per inserti**, wähle Serie und Gewinde, klicke auf **Run** und bestätige das Paket, wenn SolidCraft danach fragt. Die [fertige Musterplatte](../../examples/insert-seats.sc) zeigt das Ergebnis.

## Schnitte in der Vorschau prüfen

Mit dem optionalen Desktop-Patch kannst du Bohrungen im Bedienfeld auswählen, das abzutragende Material sehen und Durchmesser und Tiefe vor dem Ausführen ablesen. Das Bedienfeld folgt den 36 Sprachoptionen von SolidCraft 0.41.0. [Anleitung zum Erstellen und Verwenden der angepassten Desktop-Version](../../docs/desktop-preview.md).

![Bedienfeld mit Vorschau, Durchmesser und Tiefe der Aufnahme](../../assets/screenshots/insert-seat-preview.png)

*Vorschau in der Desktop-Version mit optionalem Patch.*

## Unterstützte Gewindeeinsätze

| CNC-Kitchen-Serie | Schraubengrößen | Beispiel M3 (Durchmesser × Tiefe) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

Im Automatikmodus wird ein Gewinde gewählt, dessen Nenndurchmesser höchstens 0,05 mm vom Bohrungsdurchmesser abweicht. Wähle bei Durchgangsbohrungen mit Schraubenspiel und Kernlöchern die Schraubengröße von Hand. Wähle etwa M3 für eine Bohrung mit Ø3,2 mm und einer M3-Schraube. Die Serie gilt für die ganze Auswahl. Die Automatik kann jeder Bohrung eine andere Gewindegröße zuordnen.

SolidInserts unterstützt zylindrische Bohrungen mit kreisförmigem Eintritt auf einer ebenen Fläche. Es prüft das umgebende Material und die verfügbare Tiefe. Bei durchgehenden Bohrungen bleibt ein Teil der ursprünglichen Bohrung erhalten. Die Aufnahmetiefe entspricht der Einsatzlänge plus 1 mm. Die Durchmesserkorrektur startet bei null und verändert nur den Durchmesser. Drucke ein Muster, um die Passung abzustimmen. [Maße und Konstruktionshinweise von CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Erstellen und testen

Installiere Rust mit Cargo, Bash, Python 3 und [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Führe die Befehle im Stammverzeichnis des Repositorys aus. Der Build erzeugt das Plugin-ZIP und seine SHA-256-Prüfsumme. Mit `./package-preview-kit.sh` erstellst du anschließend ein Paket mit Desktop-Patch, Abbildungen und Anleitung.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

Der Integrationstest benötigt die CLI von SolidCraft 0.41.0. In CLI- oder MCP-Skripten liefert `applica` über `plugin.run` mit `solo_quote: true` die Maße, ohne das Modell zu ändern. SolidInserts nutzt die [Plugin-Laufzeit von SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.41.0/docs/plugin-runtime.md).

## Lizenz und Unterstützung

SolidInserts ist ein unabhängiges Plugin unter der [MIT-Lizenz](../../LICENSE). [Danksagung und Quellen](../../NOTICE.md) · [Entwicklung auf Ko-fi unterstützen](https://ko-fi.com/enderk).
