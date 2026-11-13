<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/solidinserts-logo-inverse.svg">
  <img src="assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

# SolidInserts

[Italiano](README.it.md)

SolidInserts is a [SolidCraft](https://github.com/tangency-labs/solidcraft) desktop plugin that turns existing holes into seats for heat-set threaded inserts. Select the circular rims, choose a catalog series and thread size, and apply all the cuts at once. The resulting sketches and cuts stay editable in the SolidCraft timeline, and one undo removes the batch.

![SolidInserts previews a 4 mm diameter, 6.7 mm deep insert seat in SolidCraft](assets/screenshots/insert-seat-preview.png)

*Live preview in SolidCraft 0.41.0 with the [optional desktop patch](docs/desktop-preview.md).*

## Install and try it

1. For [SolidCraft desktop 0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0), get the plugin ZIP from [Releases](../../releases) when available, or [build it locally](#build-and-test).
2. Extract the ZIP into the SolidCraft plugins folder. Keep `plugin.json` and `plugin.wasm` together inside `plugins/solidinserts/`.

   | OS | Plugins folder |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (usually `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Start SolidCraft and open the [sample with Ø3 and Ø4 mm holes](examples/provino-fori-pilota.sc).
4. In stock SolidCraft, select the circular rims on the side where the inserts will go, then open **Plugin → Sedi per inserti**, choose the options and press **Run**. SolidCraft asks you to approve the package. The [finished sample](examples/provino-sedi-inserti.sc) shows how the cuts appear in the timeline.

The stock plugin workflow was checked on Linux. The preview kit adds **Select the holes → Select** above the insert series, a live cut preview that follows the theme, and a panel in all 36 SolidCraft languages. See the [desktop build instructions](docs/desktop-preview.md).

## Insert sizes and fit

| CNC Kitchen series | Supported metric threads | Example M3 seat (diameter × depth) |
| --- | --- | --- |
| Standard | M2, M2.5, M3, M4, M5, M6, M8, M10 | Ø4 × 6.7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4.4 × 5 mm |

**Automatic (nominal Ø)** matches the rim diameter to a nominal thread diameter within ±0.05 mm. A Ø3 mm hole matches M3. Choose the thread manually for clearance or tap drill holes: for a Ø3.2 mm clearance hole for an M3 screw, choose M3. One series applies to the whole selection; automatic sizing can pick a different thread for each hole.

Seat depth is the insert length plus 1 mm. Diameter compensation starts at zero and changes only the seat diameter. SolidInserts works on cylindrical holes opening on a planar face. It checks the surrounding wall and available depth, and preserves part of the original bore on through holes. Print a test piece to tune the fit for your material, printer and print orientation. See the [CNC Kitchen dimensions](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf) and [installation guide](https://cnckitchen.com/blog/tips-and-tricks-for-heat-set-inserts).

## Build and test

Use Rust with Cargo, Bash, Python 3 and [wasm-tools](https://github.com/bytecodealliance/wasm-tools).

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

The build validates the WebAssembly component and writes `dist/solidinserts/`, a versioned ZIP and its SHA-256 checksum. Run `./package-preview-kit.sh` after `./build.sh` to bundle the plugin ZIP with the desktop patch, illustrations and local build instructions. The integration tests require the SolidCraft 0.41.0 CLI. They exercise sizing, the cuts and undo against the actual plugin host.

The plugin uses SolidCraft’s experimental [`craft:plugin@0.1.0` runtime](https://github.com/tangency-labs/solidcraft/blob/v0.41.0/docs/plugin-runtime.md) with `commands@0.1`. For CLI or MCP scripts, call entry `applica` via `plugin.run` with `solo_quote: true` to return dimensions without changing the model.

SolidInserts is an independent plugin. See [third-party credits](NOTICE.md).

## License and support

[MIT](LICENSE). If SolidInserts is useful to you, you can [support its development on Ko-fi](https://ko-fi.com/enderk).
