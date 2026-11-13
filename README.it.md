<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/solidinserts-logo-inverse.svg">
  <img src="assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

# SolidInserts

[English](README.md)

SolidInserts è un plugin per [SolidCraft](https://github.com/tangency-labs/solidcraft) desktop che ricava sedi per inserti filettati a caldo partendo da fori esistenti. Seleziona i bordi circolari, scegli serie e filettatura, poi applica tutti i tagli insieme. Schizzi e tagli restano modificabili nella cronologia di SolidCraft e un solo **Annulla** ritira l’intera operazione.

![Anteprima di una sede Ø4 mm profonda 6,7 mm in SolidCraft](assets/screenshots/insert-seat-preview.png)

*Anteprima nella versione 0.41.0 di SolidCraft con la [patch desktop facoltativa](docs/desktop-preview.md).*

## Installazione e primo provino

1. Per [SolidCraft desktop 0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0), scarica lo ZIP del plugin dalle [release](../../releases) quando disponibile, oppure [compilalo in locale](#compilazione-e-test).
2. Estrai lo ZIP nella cartella dei plugin di SolidCraft. I file `plugin.json` e `plugin.wasm` devono trovarsi insieme in `plugins/solidinserts/`.

   | Sistema | Cartella dei plugin |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (di solito `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Avvia SolidCraft e apri il [provino con fori Ø3 e Ø4 mm](examples/provino-fori-pilota.sc).
4. In SolidCraft standard seleziona i bordi circolari dal lato d’ingresso degli inserti, poi apri **Plugin → Sedi per inserti**, imposta i parametri e premi **Run**. SolidCraft chiede di approvare il pacchetto. Il [provino finito](examples/provino-sedi-inserti.sc) mostra i tagli nella cronologia.

Il flusso con SolidCraft standard è stato verificato su Linux. Il kit per l’anteprima aggiunge **Seleziona i fori → Seleziona** sopra la serie dell’inserto, un’anteprima di taglio che segue il tema e il pannello in tutte le 36 lingue di SolidCraft. Vedi le [istruzioni per la build desktop](docs/desktop-preview.md).

## Catalogo e misure

| Serie CNC Kitchen | Filettature disponibili | Esempio di sede M3 (diametro × profondità) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

**Automatica (Ø nominale)** cerca una filettatura con diametro nominale entro ±0,05 mm da quello del bordo. Un foro Ø3 mm corrisponde a M3. Per i fori di gioco o di premaschiatura scegli la misura manualmente: per un foro di gioco Ø3,2 mm per una vite M3, scegli M3. La serie vale per tutta la selezione; in modalità automatica la filettatura può variare da foro a foro.

La sede è profonda quanto la lunghezza dell’inserto più 1 mm. La compensazione parte da zero e modifica solo il diametro. Il plugin lavora su fori cilindrici con imbocco su una faccia piana, controlla la parete attorno alla sede e la profondità disponibile e sui passanti conserva un tratto del foro originale. Stampa un provino per tarare la sede su materiale, stampante e orientamento. Vedi la [scheda dimensionale](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf) e le [indicazioni di montaggio](https://cnckitchen.com/blog/tips-and-tricks-for-heat-set-inserts) di CNC Kitchen.

## Compilazione e test

Servono Rust con Cargo, Bash, Python 3 e [wasm-tools](https://github.com/bytecodealliance/wasm-tools).

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /percorso/di/solidcraft-cli
```

La build valida il componente WebAssembly e scrive in `dist/` il pacchetto, lo ZIP con versione e il suo checksum SHA-256. Dopo `./build.sh`, esegui `./package-preview-kit.sh` per raggruppare lo ZIP del plugin con la patch desktop, le illustrazioni e le istruzioni di compilazione locale. I test di integrazione usano la CLI di SolidCraft 0.41.0 per verificare quote, tagli e annullamento con il runtime effettivo.

Il plugin usa il [runtime sperimentale `craft:plugin@0.1.0`](https://github.com/tangency-labs/solidcraft/blob/v0.41.0/docs/plugin-runtime.md) con `commands@0.1`. Negli script CLI o MCP, esegui l’entry `applica` via `plugin.run` con `solo_quote: true` per ottenere le quote senza modificare il modello.

SolidInserts è un plugin indipendente. Vedi le [attribuzioni](NOTICE.md).

## Licenza e supporto

[MIT](LICENSE). Se SolidInserts ti è utile, puoi [sostenerne lo sviluppo su Ko-fi](https://ko-fi.com/enderk).
