# SolidInserts 1.1.0 desktop preview on SolidCraft 0.42.0

The full SolidInserts panel runs on a locally patched SolidCraft 0.42.0 desktop build. Open it from **Model → Plugins → Insert Seats** in the ribbon or from the **Plugins** menu. The panel includes **Select the holes → Select** above Type, a help icon, an insert illustration and a live 3D preview with diameter and depth labels. The preview follows the theme’s cut colour and opacity and the graphics preference for removed material.

## Build for local use

1. Obtain the [SolidCraft v0.42.0 source](https://github.com/tangency-labs/solidcraft/releases/tag/v0.42.0) and follow its build prerequisites in `CONTRIBUTING.md`.
2. From the root of the unmodified SolidCraft source tree, run the commands below. Set `KIT` to the absolute path to the extracted SolidInserts preview kit (or the SolidInserts repository at tag `v1.1.0`).

   ```sh
   KIT=/absolute/path/to/solidinserts-kit
   git apply --check "$KIT/solidcraft-v0.42.0-select-holes.patch"
   git apply "$KIT/solidcraft-v0.42.0-select-holes.patch"
   mkdir -p assets/solidinserts
   cp "$KIT"/assets/solidinserts/{standard,short,voron,help}.png assets/solidinserts/
   cargo build --locked --release -p solidcraft
   ```

3. Install the plugin ZIP included in the kit, or build it with `./build.sh` from this repository. Follow the [README](https://github.com/NoOneExceptEnderk/SolidInserts/blob/v1.1.0/README.md) to install it. Launch the app you just built from `target/release/` (on Windows, `target\release\solidcraft.exe`).
4. Open the sample `examples/pilot-holes.sc`. From **Model → Plugins → Insert Seats**, press **Select** beside **Select the holes** and click a circular hole rim. Use **Ctrl+click** to add or remove holes. Choose the series and thread, check the preview and press **Run**.

In stock SolidCraft, select the rims before opening the plugin. Some technical messages from the WebAssembly module use English when a translation is unavailable.

The preview kit contains source changes and original SolidInserts illustrations. See the [SolidCraft Brand and Interface Licence](https://github.com/tangency-labs/solidcraft/blob/v0.42.0/LICENSE-BRAND) for the terms on distributing a modified SolidCraft binary.

## Istruzioni in italiano

La patch per SolidCraft 0.42.0 apre lo stesso pannello compatto dalla ribbon e dal menu Plugin. Include il pulsante **Seleziona**, l’illustrazione dell’inserto e l’anteprima 3D del materiale da asportare con le quote della sede.

1. Scarica i [sorgenti di SolidCraft v0.42.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.42.0) e segui i prerequisiti di compilazione in `CONTRIBUTING.md`.
2. Dalla radice dei sorgenti esegui i comandi sopra, impostando `KIT` sul percorso assoluto del kit estratto o della copia di SolidInserts.
3. Installa lo ZIP del plugin seguendo il [README italiano](https://github.com/NoOneExceptEnderk/SolidInserts/blob/v1.1.0/README.it.md), poi esegui l’app appena compilata in `target/release/`.
4. Apri il [provino](../examples/pilot-holes.sc) e vai in **Modella → Plugin → Insert Seats**. Premi **Seleziona** e fai clic sull’imbocco. Usa **Ctrl+clic** per aggiungere o rimuovere fori. Con **Fine selezione** conservi l’anteprima. Con **Esegui** applichi i tagli.

Nell’app standard seleziona gli imbocchi prima di aprire il plugin.

Su Linux, esegui `./build.sh` e poi `./try-hole-selection.sh /percorso/al/binario/solidcraft` per aprire il provino con un profilo temporaneo.
