# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · [Português (Portugal)](pt.md) · **Русский** · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Создавайте посадочные места для термовставок с резьбой в существующих отверстиях SolidCraft Desktop.

SolidInserts привязывает каждое посадочное место к круглой кромке отверстия. Выберите серию вставок CNC Kitchen и размер винта, чтобы обработать несколько отверстий за одну операцию. Эскизы и операции вырезания остаются редактируемыми в истории SolidCraft. Одно действие **Отменить** убирает все вырезы операции.

## Как попробовать SolidInserts

1. Используйте [SolidCraft Desktop 0.44.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.44.1). Скачайте ZIP плагина из раздела [Releases](https://github.com/NoOneExceptEnderk/SolidInserts/releases). Плагин также можно собрать командами ниже. Результат появится в `dist/solidinserts-<version>.zip`.
2. Установите ZIP через **Файл → Plugins…**. Для ручной установки распакуйте папку `solidinserts` целиком, вместе с ресурсами, в каталог плагинов своей системы.

   | Система | Каталог плагинов |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (обычно `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Откройте [образец с отверстиями Ø3 и Ø4 мм](../../examples/pilot-holes.sc). Откройте **Model → Plugins → Insert Seats**. В поле **Hole rims** выберите круглые кромки со стороны установки вставок. Выберите серию и размер резьбы, проверьте предварительный просмотр и нажмите **Run**. Результат можно посмотреть в [готовом образце](../../examples/insert-seats.sc).

## Предварительный просмотр вырезов

В официальной версии SolidCraft 0.44.1 трёхмерный предварительный просмотр работает без патча. Он обновляется при выборе кромок и изменении настроек. **Cancel** закрывает предварительный просмотр.

## Поддерживаемые вставки

SolidInserts — независимый плагин, не связанный с CNC Kitchen. Размеры посадочных мест для указанных вставок основаны на таблице размеров, опубликованной CNC Kitchen.

| Серия CNC Kitchen | Размеры винтов | Пример M3 (диаметр × глубина) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 мм |
| Short Version | M3, M4, M5 | Ø4 × 4 мм |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 мм |

Автоматический режим подбирает резьбу по диаметру отверстия с допуском ±0,05 мм относительно номинального диаметра резьбы. Для отверстий с зазором или под нарезание резьбы задайте размер винта вручную. Например, для отверстия Ø3,2 мм под винт M3 выберите M3. Серия распространяется на всю выборку, а размер в автоматическом режиме может отличаться для каждого отверстия.

SolidInserts работает с цилиндрическими отверстиями с круглым входом на плоской грани. Плагин проверяет окружающий материал и доступную глубину. В сквозном отверстии сохраняется участок исходного канала. Глубина посадочного места равна длине вставки плюс 1 мм. Поправка по умолчанию равна нулю и меняет только диаметр. Напечатайте пробный образец, чтобы настроить посадку. [Размеры и рекомендации CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Сборка и проверка

Установите Rust с Cargo, Bash, Python 3 и [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Выполните команды из корня репозитория. Сборка создаёт ZIP и контрольную сумму SHA-256.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

Интеграционный тест использует CLI SolidCraft 0.44.1. Из скриптов CLI или MCP вызовите `applica` через `plugin.run` с параметром `solo_quote: true`, чтобы получить размеры без изменения модели. SolidInserts использует [среду выполнения плагинов SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.44.1/docs/plugin-runtime.md).

## Лицензия и поддержка

SolidInserts — независимый плагин под [лицензией MIT](../../LICENSE). [Авторство и источники](../../NOTICE.md) · [Поддержать разработку на Ko-fi](https://ko-fi.com/enderk).
