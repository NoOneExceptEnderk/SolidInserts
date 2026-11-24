# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · **Español** · [Français](fr.md) · [Deutsch](de.md) · [Português (Portugal)](pt.md) · [Русский](ru.md) · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Crea alojamientos para insertos roscados de instalación térmica en orificios existentes de SolidCraft Desktop.

SolidInserts sitúa cada alojamiento a partir del borde circular del orificio. Elige una serie de CNC Kitchen y una medida de tornillo. Puedes hacer varios cortes en una sola operación. Los bocetos y los cortes siguen siendo editables en el historial de SolidCraft y se pueden deshacer juntos.

**Captive Nut Pockets** crea alojamientos internos para tuercas hexagonales DIN 934 e indica las alturas de pausa. Configura las pausas en el laminador. [Instrucciones en inglés](../../README.md#captive-nuts-and-print-pauses)

## Prueba SolidInserts

SolidInserts 1.3.1 funciona sin parches con las versiones oficiales [SolidCraft Desktop 0.46.2](https://github.com/tangency-labs/solidcraft/releases/tag/v0.46.2) y [SolidCraft Desktop 0.48.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.48.1). La compatibilidad con 0.48.1 se verificó en Linux.

1. Descarga el ZIP del complemento desde [Releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases). También puedes compilarlo con los comandos de abajo para generar `dist/solidinserts-<versión>.zip`.
2. Instala el ZIP desde **Archivo → Plugins…**. Para instalarlo manualmente, extrae la carpeta `solidinserts` completa, incluidos sus recursos, en la carpeta de complementos de tu sistema.

   | Sistema | Carpeta de complementos |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (normalmente `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Abre la [probeta con orificios de Ø3 y Ø4 mm](../../examples/pilot-holes.sc). Abre **Model → Plugins → Insert Seats**. En **Hole rims**, selecciona los bordes circulares por el lado de entrada de los insertos. Elige la serie y la rosca, comprueba la vista previa y pulsa **Run**. La [probeta terminada](../../examples/insert-seats.sc) muestra el resultado.

## Vista previa del corte

SolidCraft Desktop muestra la vista previa 3D sin parche. Se actualiza al seleccionar bordes o cambiar los ajustes. **Cancel** descarta la vista previa.

## Insertos compatibles

SolidInserts es un complemento independiente sin afiliación con CNC Kitchen. Las cotas de los alojamientos para los insertos indicados se basan en la ficha dimensional publicada por CNC Kitchen.

| Serie de CNC Kitchen | Medidas de tornillo | Ejemplo M3 (diámetro × profundidad) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

El modo automático busca una rosca cuyo diámetro nominal difiera como máximo 0,05 mm del orificio. En orificios de paso con holgura o previos al roscado, elige la medida manualmente. Por ejemplo, selecciona M3 para un orificio de Ø3,2 mm destinado a un tornillo M3. La serie se aplica a toda la selección y el modo automático puede asignar una rosca distinta a cada orificio.

SolidInserts admite orificios cilíndricos con entrada circular en una cara plana. Comprueba el material circundante y la profundidad disponible. En orificios pasantes conserva un tramo del orificio original. La profundidad es la longitud del inserto más 1 mm. La compensación parte de cero y solo modifica el diámetro. Imprime una probeta para ajustar el encaje. [Cotas y pautas de diseño de CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Compilación y pruebas

Instala Rust con Cargo, Bash, Python 3 y [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Ejecuta los comandos desde la raíz del repositorio. La compilación genera el ZIP y su suma SHA-256.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

La prueba de integración pasó con las CLI de SolidCraft 0.46.2 y 0.48.1. Desde scripts de CLI o MCP, llama a `applica` mediante `plugin.run` con `solo_quote: true` para obtener las cotas sin modificar el modelo. SolidInserts usa el [entorno de ejecución de complementos de SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.46.2/docs/plugins.md).

## Licencia y apoyo

SolidInserts es un complemento independiente distribuido con la [licencia MIT](../../LICENSE). [Créditos](../../NOTICE.md) · [Apoya el desarrollo en Ko-fi](https://ko-fi.com/enderk).
