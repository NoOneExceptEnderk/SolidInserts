# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/branding/solidinserts-logo-inverse.svg">
  <img src="assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](README.md) · **Italiano** · [Español](docs/readme/es.md) · [Français](docs/readme/fr.md) · [Deutsch](docs/readme/de.md) · [Português (Portugal)](docs/readme/pt.md) · [Русский](docs/readme/ru.md) · [简体中文](docs/readme/zh-CN.md) · [தமிழ்](docs/readme/ta.md)

[Guida utente per la 1.1.0](https://github.com/NoOneExceptEnderk/SolidInserts/blob/v1.1.0/docs/user-guide.it.md)

Crea sedi per inserti filettati a caldo partendo da fori esistenti in SolidCraft Desktop.

SolidInserts usa il bordo circolare del foro per posizionare la sede. Scegli una serie CNC Kitchen e la misura della vite, poi ricava più sedi in una sola operazione. Schizzi e tagli restano modificabili nella cronologia di SolidCraft. Basta un **Annulla** per ritirare tutti i tagli.

## Prova SolidInserts

1. Usa [SolidCraft Desktop 0.44.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.44.1). Scarica lo ZIP del plugin dai [rilasci](https://github.com/NoOneExceptEnderk/SolidInserts/releases). Puoi anche compilare il plugin con i comandi riportati sotto per generare `dist/solidinserts-<versione>.zip`.
2. Installa lo ZIP da **File → Plugin…**. Per l’installazione manuale, estrai l’intera cartella `solidinserts`, comprese le risorse, nella cartella dei plugin del tuo sistema.

   | Sistema | Cartella dei plugin |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (di solito `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Apri il [provino con fori Ø3 e Ø4 mm](examples/pilot-holes.sc). Nella barra degli strumenti apri **Modella → Plugin → Insert Seats**. Il campo **Hole rims** è già attivo quando si apre il pannello. Fai clic sui bordi circolari dal lato in cui entreranno gli inserti. Imposta serie e filettatura, controlla l’anteprima e premi **Esegui**. Il [provino finito](examples/insert-seats.sc) mostra i tagli ottenuti.

## Guarda l’anteprima prima di applicare i tagli

SolidCraft Desktop 0.44.1 ufficiale offre l’anteprima 3D senza patch. L’anteprima si aggiorna quando selezioni i bordi o cambi le impostazioni. **Annulla** chiude il pannello e scarta l’anteprima. **Esegui** applica tutti i tagli in una sola operazione annullabile. La tendina **Type** mostra un’illustrazione per ogni serie.

## Inserti supportati

SolidInserts è un plugin indipendente, senza affiliazione con CNC Kitchen. Le quote delle sedi per gli inserti indicati si basano sulla scheda dimensionale pubblicata da CNC Kitchen.

| Serie CNC Kitchen | Misure della vite | Sede M3 di esempio (diametro × profondità) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

Il rilevamento automatico cerca una filettatura con diametro nominale entro ±0,05 mm dal diametro del foro. Per i fori di gioco o di premaschiatura scegli la misura manualmente. Per esempio, scegli M3 per un foro di gioco Ø3,2 mm per vite M3. La serie vale per tutta la selezione. In modalità automatica la misura può cambiare da foro a foro.

SolidInserts accetta fori cilindrici con imbocco circolare su una faccia piana. Controlla il materiale circostante e la profondità disponibile. Nei fori passanti conserva un tratto del foro originale. La sede è profonda quanto l’inserto più 1 mm. La compensazione parte da zero e modifica solo il diametro. Stampa un provino per tarare l’accoppiamento. [Quote e indicazioni di progettazione CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Compila e verifica

Servono Rust con Cargo, Bash, Python 3 e [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Esegui i comandi dalla radice del repository. La compilazione genera lo ZIP del plugin e il suo checksum SHA-256.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

Usa la CLI di SolidCraft 0.44.1 per i test di integrazione. Negli script CLI o MCP puoi chiamare `applica` tramite `plugin.run` con `solo_quote: true` per ottenere le quote senza modificare il modello. SolidInserts usa il [runtime dei plugin di SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.44.1/docs/plugin-runtime.md).

## Licenza e supporto

SolidInserts è un plugin indipendente distribuito con [licenza MIT](LICENSE). [Attribuzioni](NOTICE.md) · [Sostieni lo sviluppo su Ko-fi](https://ko-fi.com/enderk).
