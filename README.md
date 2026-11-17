# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/solidinserts-logo-inverse.svg">
  <img src="assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

**English** · [Italiano](README.it.md) · [Español](docs/readme/es.md) · [Français](docs/readme/fr.md) · [Deutsch](docs/readme/de.md) · [Português (Portugal)](docs/readme/pt.md) · [Русский](docs/readme/ru.md) · [简体中文](docs/readme/zh-CN.md) · [தமிழ்](docs/readme/ta.md)

[User guide for 1.1.0](https://github.com/NoOneExceptEnderk/SolidInserts/blob/v1.1.0/docs/user-guide.md)

Create seats for heat-set threaded inserts in existing holes with SolidCraft Desktop.

SolidInserts uses the circular edge of each hole to locate the seat. Choose a CNC Kitchen insert series and a screw size, then cut several seats in one operation. The sketches and cuts stay editable in the SolidCraft timeline. One undo removes the whole batch.

## Try SolidInserts

1. Use [SolidCraft Desktop 0.44.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.44.1). Get the plugin ZIP from [Releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases). You can also build it with the commands below to produce `dist/solidinserts-<version>.zip`.
2. Install the ZIP from **File → Plugins…**. For manual installation, extract the entire `solidinserts` folder, including its assets, into the plugins folder for your system.

   | OS | Plugins folder |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (usually `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Open the [sample plate with Ø3 and Ø4 mm holes](examples/pilot-holes.sc). In the ribbon, open **Model → Plugins → Insert Seats**. The **Hole rims** selector is active when the dialog opens. Click the circular rims on the side where the inserts go. Choose the series and thread size, check the preview, and press **Run**. The [finished sample](examples/insert-seats.sc) shows the resulting cuts.

## Preview the cuts before applying them

Official SolidCraft Desktop 0.44.1 provides the 3D preview without a patch. SolidCraft updates it as you pick rims or change the settings. **Cancel** discards the preview. **Run** applies the cuts as one undo step. The **Type** dropdown shows an illustration for each insert series.

## Supported inserts

SolidInserts is an independent plugin with no affiliation with CNC Kitchen. The seat dimensions for the listed inserts are based on the dimension sheet published by CNC Kitchen.

| CNC Kitchen series | Screw sizes | M3 seat example (diameter × depth) |
| --- | --- | --- |
| Standard | M2, M2.5, M3, M4, M5, M6, M8, M10 | Ø4 × 6.7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4.4 × 5 mm |

Automatic sizing matches the hole diameter to a nominal thread diameter within ±0.05 mm. For clearance holes or tap drills, choose the screw size manually. For example, choose M3 for a Ø3.2 mm M3 clearance hole. One insert series applies to the batch. Automatic sizing can choose a different thread size for each hole.

SolidInserts accepts cylindrical holes with a circular entrance on a planar face. It checks the surrounding material and depth, and preserves a section of the original bore in through holes. Seat depth equals insert length plus 1 mm. The diameter compensation defaults to zero and changes only the diameter. Print a test piece to fine-tune the fit. [CNC Kitchen dimensions and design guidelines](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Build and test

Install Rust with Cargo, Bash, Python 3 and [wasm-tools](https://github.com/bytecodealliance/wasm-tools). From the repository root, run the commands below. The build creates the plugin ZIP and its SHA-256 checksum.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

Use the SolidCraft 0.44.1 CLI for the integration tests. For CLI or MCP scripts, call `applica` through `plugin.run` with `solo_quote: true` to get the dimensions without editing the model. SolidInserts uses the [SolidCraft plugin runtime](https://github.com/tangency-labs/solidcraft/blob/v0.44.1/docs/plugin-runtime.md).

## License and support

SolidInserts is an independent plugin distributed under the [MIT License](LICENSE). [Third-party credits](NOTICE.md) · [Support development on Ko-fi](https://ko-fi.com/enderk).
