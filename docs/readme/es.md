# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · **Español** · [Français](fr.md) · [Deutsch](de.md) · [Português (Portugal)](pt.md) · [Русский](ru.md) · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Crea alojamientos para insertos roscados de instalación térmica en orificios existentes de SolidCraft Desktop.

SolidInserts sitúa cada alojamiento a partir del borde circular del orificio. Elige una serie de CNC Kitchen y una medida de tornillo. Puedes hacer varios cortes en una sola operación. Los bocetos y los cortes siguen siendo editables en el historial de SolidCraft y se pueden deshacer juntos.

## Prueba SolidInserts

1. Usa [SolidCraft Desktop 0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0). Descarga el ZIP del complemento desde [Releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases) cuando esté disponible. También puedes compilarlo con los comandos de abajo para generar `dist/solidinserts-<versión>.zip`.
2. Descomprime el ZIP en la carpeta de complementos de tu sistema. Mantén `plugin.json` y `plugin.wasm` juntos dentro de la subcarpeta `solidinserts`.

   | Sistema | Carpeta de complementos |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (normalmente `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Abre la [probeta con orificios de Ø3 y Ø4 mm](../../examples/pilot-holes.sc). Selecciona los bordes circulares por el lado de entrada de los insertos. Abre **Plugin → Sedi per inserti**, elige la serie y la rosca, pulsa **Run** y aprueba el paquete cuando SolidCraft te lo pida. La [probeta terminada](../../examples/insert-seats.sc) muestra el resultado.

## Vista previa del corte

El parche de escritorio opcional permite seleccionar orificios desde el panel, ver el material que se eliminará y consultar el diámetro y la profundidad antes de aplicar los cortes. El panel sigue las 36 opciones de idioma de SolidCraft 0.41.0. [Guía para compilar y usar el parche](../../docs/desktop-preview.md).

![Panel con la vista previa y las cotas del alojamiento](../../assets/screenshots/insert-seat-preview.png)

*Vista previa en la aplicación de escritorio con el parche opcional.*

## Insertos compatibles

| Serie de CNC Kitchen | Medidas de tornillo | Ejemplo M3 (diámetro × profundidad) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

El modo automático busca una rosca cuyo diámetro nominal difiera como máximo 0,05 mm del orificio. En orificios de paso con holgura o previos al roscado, elige la medida manualmente. Por ejemplo, selecciona M3 para un orificio de Ø3,2 mm destinado a un tornillo M3. La serie se aplica a toda la selección y el modo automático puede asignar una rosca distinta a cada orificio.

SolidInserts admite orificios cilíndricos con entrada circular en una cara plana. Comprueba el material circundante y la profundidad disponible. En orificios pasantes conserva un tramo del orificio original. La profundidad es la longitud del inserto más 1 mm. La compensación parte de cero y solo modifica el diámetro. Imprime una probeta para ajustar el encaje. [Cotas y pautas de diseño de CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Compilación y pruebas

Instala Rust con Cargo, Bash, Python 3 y [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Ejecuta los comandos desde la raíz del repositorio. La compilación genera el ZIP y su suma SHA-256. Ejecuta después `./package-preview-kit.sh` para reunir el parche de escritorio, las ilustraciones y la guía.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

La prueba de integración usa la CLI de SolidCraft 0.41.0. Desde scripts de CLI o MCP, llama a `applica` mediante `plugin.run` con `solo_quote: true` para obtener las cotas sin modificar el modelo. SolidInserts usa el [entorno de ejecución de complementos de SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.41.0/docs/plugin-runtime.md).

## Licencia y apoyo

SolidInserts es un complemento independiente distribuido con la [licencia MIT](../../LICENSE). [Créditos](../../NOTICE.md) · [Apoya el desarrollo en Ko-fi](https://ko-fi.com/enderk).
