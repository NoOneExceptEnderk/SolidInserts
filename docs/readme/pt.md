# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · **Português (Portugal)** · [Русский](ru.md) · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Cria alojamentos para insertos roscados de instalação a quente em furos existentes no SolidCraft Desktop.

O SolidInserts posiciona cada alojamento a partir da aresta circular do furo. Escolhe uma série CNC Kitchen e uma medida de parafuso para fazer vários cortes numa só operação. Os esboços e cortes ficam editáveis no histórico do SolidCraft. Basta usar **Anular** uma vez para retirar todos os cortes da operação.

## Experimentar o SolidInserts

1. Usa o [SolidCraft Desktop 0.41.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.41.0). Compila o módulo com os comandos abaixo para obter `dist/solidinserts-<versão>.zip`.
2. Extrai o ZIP para a pasta de módulos do teu sistema. Mantém `plugin.json` e `plugin.wasm` juntos na subpasta `solidinserts`.

   | Sistema | Pasta de módulos |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (normalmente `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Abre o [provete com furos de Ø3 e Ø4 mm](../../examples/pilot-holes.sc). Seleciona as arestas circulares do lado por onde vão entrar os insertos. Abre **Plugin → Sedi per inserti**, escolhe a série e a rosca, carrega em **Run** e aprova o pacote quando o SolidCraft o pedir. O [provete final](../../examples/insert-seats.sc) mostra o resultado.

## Pré-visualizar os cortes

A alteração opcional à aplicação de computador permite selecionar os furos a partir do painel, ver o material a remover e confirmar o diâmetro e a profundidade antes de aplicar os cortes. O painel segue as 36 opções de idioma do SolidCraft 0.41.0. [Guia para compilar e usar a aplicação com a alteração](../../docs/desktop-preview.md).

![Painel com pré-visualização e cotas do alojamento](../../assets/screenshots/insert-seat-preview.png)

*Pré-visualização na aplicação de computador com a alteração opcional.*

## Insertos compatíveis

| Série CNC Kitchen | Medidas do parafuso | Exemplo M3 (diâmetro × profundidade) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

O modo automático procura uma rosca cujo diâmetro nominal esteja a ±0,05 mm do diâmetro do furo. Para furos com folga ou preparados para roscar, escolhe a medida do parafuso manualmente. Por exemplo, escolhe M3 para um furo de folga de Ø3,2 mm para um parafuso M3. A série aplica-se a toda a seleção. O modo automático pode escolher uma medida diferente em cada furo.

O SolidInserts aceita furos cilíndricos com entrada circular numa face plana. Verifica o material à volta do alojamento e a profundidade disponível. Nos furos passantes conserva uma parte do furo original. A profundidade do alojamento corresponde ao comprimento do inserto mais 1 mm. A compensação começa em zero e só altera o diâmetro. Imprime um provete para afinar o encaixe. [Dimensões e orientações de projeto da CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Compilar e testar

Instala o Rust com Cargo, Bash, Python 3 e [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Executa os comandos na raiz do repositório. A compilação gera o ZIP e a respetiva soma SHA-256. Executa depois `./package-preview-kit.sh` para criar também o pacote com a alteração à aplicação, as ilustrações e o guia.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

O teste de integração usa a CLI do SolidCraft 0.41.0. Em scripts CLI ou MCP, chama `applica` através de `plugin.run` com `solo_quote: true` para obter as cotas sem editar o modelo. O SolidInserts usa o [ambiente de execução de módulos do SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.41.0/docs/plugin-runtime.md).

## Licença e apoio

O SolidInserts é um módulo independente distribuído com a [licença MIT](../../LICENSE). [Créditos](../../NOTICE.md) · [Apoia o desenvolvimento no Ko-fi](https://ko-fi.com/enderk).
