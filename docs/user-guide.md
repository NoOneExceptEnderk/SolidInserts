# SolidInserts 1.1.0 user guide

[Italiano](user-guide.it.md)

SolidInserts enlarges the opening of an existing hole to make a seat for a heat-set threaded insert. It cuts inward from the rim you select and adds an editable sketch and extrusion cut for each seat to the SolidCraft timeline.

## Install on SolidCraft 0.42.0

1. Get `solidinserts-1.1.0.zip` from the [SolidInserts releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases). To build the ZIP yourself, follow the [README](../README.md#build-and-test).
2. Extract the ZIP into the plugins folder for your OS. The ZIP already contains the `solidinserts` folder. Place it directly under `plugins` so that `plugins/solidinserts/plugin.json` and `plugins/solidinserts/plugin.wasm` sit together.

   | OS | Plugins folder |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` or `~/.local/share/solidcraft/plugins/` if `XDG_DATA_HOME` is unset |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Start SolidCraft desktop 0.42.0 and open a design. Use **Preferences → Plugins** to check that SolidInserts is listed and enabled.

The plugin ZIP works with the standard desktop app. For in-panel hole selection and a live 3D preview, install the same ZIP and run a locally patched SolidCraft build. The preview kit contains the patch and assets for building it. [Desktop preview build instructions](desktop-preview.md)

## Prepare the model

Close any active sketch or tool and move the timeline marker to the end. Use a cylindrical hole that opens directly onto a flat face, with no chamfer. For a through hole, select the rim on the side where the insert will enter. The wider part of the cut starts at that rim and leaves part of the original through bore below it.

## Choose the series, thread and compensation

**Type** selects the CNC Kitchen insert series. The table gives example seat dimensions at zero compensation.

| Series | Supported screw threads | M3 seat (diameter × depth) |
| --- | --- | --- |
| Standard | M2, M2.5, M3, M4, M5, M6, M8, M10 | Ø4 × 6.7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4.4 × 5 mm |

**Thread** sets the screw thread. **Automatic (nominal Ø)** compares each selected hole diameter with the nominal thread diameter within ±0.05 mm. A Ø3 mm hole gives M3, and a Ø4 mm hole gives M4. For clearance and tap-drill holes, choose the intended screw thread explicitly. For example, choose **M3 × 0,50** for a Ø3.2 mm M3 clearance hole.

One run uses the same series and compensation for every selected hole. Automatic mode can resolve different thread sizes in that batch if the series supports all of them. A manual thread choice applies to the whole batch. Use separate runs for different series, different compensation values, or different manually assigned threads. Select up to 24 rims per run.

**Diameter ±** adds a correction to the catalog seat diameter, from −1 to +1 mm in 0.01 mm steps. Start at **0 mm** for the catalog size. For a Standard M3 insert, **+0.10 mm** gives a Ø4.10 mm seat. Depth stays at 6.7 mm because it is set by the insert length plus 1 mm. To tune the fit, print a small test piece in the same material and orientation as the final part.

In standard SolidCraft, the fields are **Type**, **Screw thread** and **Diameter compensation**. The automatic option appears as **Automatic (nominal Ø)**.

## Use standard SolidCraft

1. In the 3D view, click the first hole rim. Hold **Ctrl** (**Cmd** on macOS) while clicking additional rims to add or remove them from the selection.
2. Open **Plugin → Insert Seats** and set the series, thread and compensation.
3. Click **Run**. Approve the package if SolidCraft prompts you. The standard dialog uses the selection from the 3D view and applies the cuts directly. Use **Cancel** if you want to go back and change the selection before applying.

## Use the local build with the desktop patch

1. Open **Model → Plugins → Insert Seats** from the ribbon or use the **Plugins** menu. Click **Select** next to **Select the holes**.
2. Click a circular hole rim. A regular click replaces the selection. **Ctrl+click** adds or removes rims (**Cmd+click** also works on macOS).
3. Set **Type**, **Thread** and **Diameter ±**. The viewport updates the preview as you change the selection or settings. Read the **Ø diameter × depth** labels and check that the highlighted material is on the correct side. The fill follows the theme and the removed-material graphics setting. If the fill is off, the outlines remain visible.
4. Click **Done selecting** to stop picking and keep the preview. Click **Select** again if you want to adjust the rims. While picking, **Esc** clears the selection and preview and leaves the panel open. **Cancel** closes the panel and discards the preview.
5. Wait for **Seats ready** and click **Run** to apply the prepared cuts. If the panel shows an error, adjust the selection or settings and wait for the preview to update.

## Check and undo the cuts

After a successful run, inspect the sketches and `SolidInserts · …` cuts in the timeline. Edit the sketch dimension to change the seat diameter or the extrusion distance to change its depth. One **Undo** immediately after the run removes the whole batch.

The sample designs are in `examples/` in the SolidInserts source and preview kit. To try a two-hole batch, open [pilot-holes.sc](../examples/pilot-holes.sc), select the Ø3 mm and Ø4 mm rims on the top face and run **Standard** with **Automatic (nominal Ø)** and **0 mm** compensation. The result is an M3 seat of **Ø4 × 6.7 mm** and an M4 seat of **Ø5.7 × 9.1 mm**. Open [insert-seats.sc](../examples/insert-seats.sc) to see the finished geometry and timeline.

## Troubleshooting

| Symptom | What to check |
| --- | --- |
| The plugin command is missing | Check **Preferences → Plugins** and confirm that the folder is named `solidinserts` and contains both plugin files. |
| **Select the holes** or the live preview is missing | Launch the locally compiled SolidCraft binary to use the patched panel. In standard SolidCraft, select the rims in the viewport before opening the plugin. |
| A rim is ignored, or the plugin reports an invalid opening | Rotate the model to face the intended entrance and click the circular edge where the cylindrical wall meets the flat face. Choose an unchamfered opening and clear any unrelated items from the standard viewport selection. |
| **Run** is disabled in the patched panel | Finish the current tool or sketch, move to the end of the timeline and select at least one valid rim. Wait for **Seats ready**. If the model has changed, select the rims again. |
| Automatic sizing fails, or a thread is unavailable | Select the screw thread explicitly for a clearance or tap-drill hole. If the intended size is unavailable in the chosen series, use a compatible series and matching insert. |
| The seat is narrower than the hole or reaches an edge, cavity or another seat | Check the original hole size and compensation. If the selected insert needs more space, revise the hole spacing or surrounding material. |
| There is too little depth | Check the body thickness and any internal cavities. Use a shorter compatible insert or revise the part to leave enough material under the seat. |
| The hole already has a SolidInserts seat | Edit the existing sketch and cut in the timeline. |
