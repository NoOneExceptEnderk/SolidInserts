# SolidInserts desktop preview

The full SolidInserts panel runs on a locally patched SolidCraft 0.41.0 desktop build. It adds **Select the holes → Select** above the CNC Kitchen series, an illustration of the chosen insert, all 36 SolidCraft language options, and a live cut preview that uses the theme’s extrusion-cut colour and opacity. The graphics preference for removed material also applies to this preview.

## Build for local use

1. Obtain the [SolidCraft v0.41.0 source](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0) and follow its build prerequisites in `CONTRIBUTING.md`.
2. From the root of the unmodified SolidCraft source tree, run the commands below. Set `KIT` to the absolute path to the extracted SolidInserts preview kit (or the SolidInserts repository).

   ```sh
   KIT=/absolute/path/to/solidinserts-kit
   git apply --check "$KIT/solidcraft-v0.41.0-select-holes.patch"
   git apply "$KIT/solidcraft-v0.41.0-select-holes.patch"
   mkdir -p assets/solidinserts
   cp "$KIT"/assets/solidinserts/{standard,short,voron}.png assets/solidinserts/
   cargo build --locked --release -p solidcraft
   ```

3. Install the plugin ZIP included in the kit, or build it with `./build.sh` from this repository. Follow the [README](../README.md) to install it. Launch the app you just built from `target/release/` (on Windows, `target\release\solidcraft.exe`).
4. Open the sample `examples/pilot-holes.sc`. In **Plugin → Insert seats**, press **Select** beside **Select the holes** and click a circular hole rim. Use **Ctrl+click** to add or remove holes. Choose the series and thread, check the preview and press **Run**.

The standalone plugin ZIP also works in stock SolidCraft 0.41.0. Select the rims before opening the plugin there. The themed preview and 36-language panel are provided by the desktop patch. Some technical messages from the WebAssembly module use English when a translation is unavailable.

The preview kit contains source changes and original SolidInserts illustrations. The [SolidCraft Brand and Interface Licence](https://raw.githubusercontent.com/tangency-labs/solidcraft/v0.41.0/LICENSE-BRAND) permits local development and testing. See section 3 before distributing a modified SolidCraft binary.

## Istruzioni in italiano

Lo ZIP del plugin funziona con SolidCraft 0.41.0 standard. La patch aggiunge il pulsante **Seleziona** nel pannello, l’anteprima del volume da asportare e il pannello in tutte le 36 lingue di SolidCraft.

1. Scarica i [sorgenti di SolidCraft v0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0) e segui i prerequisiti di compilazione in `CONTRIBUTING.md`.
2. Dalla radice dei sorgenti esegui i comandi sopra, impostando `KIT` sul percorso assoluto del kit estratto o della copia di SolidInserts.
3. Installa lo ZIP del plugin seguendo il [README italiano](../README.it.md), poi esegui l’app appena compilata in `target/release/`.
4. Apri il [provino](../examples/pilot-holes.sc) e vai in **Plugin → Sedi per inserti**. Premi **Seleziona**, poi fai clic sull’imbocco; **Ctrl+clic** aggiunge o rimuove fori. Con **Fine selezione** conservi l’anteprima; con **Esegui** applichi i tagli.

Su Linux, per usare un profilo temporaneo, copia il binario compilato in `dist/anteprima-linux/solidcraft-select-run` nella copia di SolidInserts, poi esegui `./build.sh` e `./prova-select-holes.sh` da lì.
