# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · [Español](es.md) · [Français](fr.md) · [Deutsch](de.md) · [Português (Portugal)](pt.md) · [Русский](ru.md) · **简体中文** · [தமிழ்](ta.md)

在 SolidCraft Desktop 的现有孔上创建热熔螺纹嵌件安装孔。

SolidInserts 使用孔口的圆形边缘定位安装孔。选择 CNC Kitchen 嵌件系列和螺钉规格，即可一次加工多个安装孔。草图和切除特征保留在 SolidCraft 的建模历史中，可以继续编辑。撤销一次即可撤销整批切除。

## 试用 SolidInserts

1. 使用 [SolidCraft Desktop 0.42.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.42.0)。插件 ZIP 发布后可从[发行版页面](https://github.com/NoOneExceptEnderk/SolidInserts/releases)下载。也可以使用下方命令编译插件，生成 `dist/solidinserts-<version>.zip`。
2. 将 ZIP 解压到系统对应的插件目录。`plugin.json` 和 `plugin.wasm` 应放在同一个 `solidinserts` 子目录中。

   | 系统 | 插件目录 |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (通常是 `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. 打开[带 Ø3 和 Ø4 毫米孔的示例试件](../../examples/pilot-holes.sc)。选择嵌件装入一侧的孔口圆边。打开 **Plugin → Insert Seats**，选择系列和螺纹规格，点击 **Run**，再按 SolidCraft 的提示批准运行插件包。[加工后的试件](../../examples/insert-seats.sc)展示了切除结果。

## 切除前预览

可选的桌面补丁允许从面板内选择孔，预览将要移除的材料，并在切除前查看安装孔的直径和深度。面板支持 SolidCraft 0.42.0 的 36 种语言选项。[编译和使用带补丁的桌面版](../../docs/desktop-preview.md)。

![面板显示安装孔预览和尺寸](../../assets/screenshots/insert-seat-preview.png)

*使用可选桌面补丁时的预览。*

## 支持的嵌件

SolidInserts 是一款独立插件，与 CNC Kitchen 无关联。所列嵌件的安装孔尺寸依据 CNC Kitchen 发布的尺寸表。

| CNC Kitchen 系列 | 螺钉规格 | M3 安装孔示例（直径 × 深度） |
| --- | --- | --- |
| Standard | M2、M2.5、M3、M4、M5、M6、M8、M10 | Ø4 × 6.7 mm |
| Short Version | M3、M4、M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4.4 × 5 mm |

自动模式将孔径与螺纹公称直径匹配，允许的偏差为 ±0.05 mm。对于螺钉间隙孔或攻丝底孔，请手动选择螺钉规格。例如，M3 螺钉的 Ø3.2 mm 间隙孔应选择 M3。整批孔使用同一嵌件系列，自动模式可以为各孔选择不同的螺纹规格。

SolidInserts 支持在平面上具有圆形入口的圆柱孔。插件检查周围材料和可用深度，并在通孔中保留一段原有孔道。安装孔深度为嵌件长度加 1 mm。补偿量默认为零，仅调整直径。可先打印试件，再调整配合。[CNC Kitchen 尺寸与设计指南](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf)。

## 编译与测试

安装 Rust 和 Cargo、Bash、Python 3 以及 [wasm-tools](https://github.com/bytecodealliance/wasm-tools)。在仓库根目录运行下方命令。编译会生成插件 ZIP 和对应的 SHA-256 校验和。随后可运行 `./package-preview-kit.sh`，将桌面补丁、插图和指南一起打包。

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

集成测试使用 SolidCraft 0.42.0 的 CLI。在 CLI 或 MCP 脚本中，可以通过 `plugin.run` 调用 `applica`，并设置 `solo_quote: true`，从而在不修改模型的情况下获取尺寸。SolidInserts 使用 [SolidCraft 插件运行时](https://github.com/tangency-labs/solidcraft/blob/v0.42.0/docs/plugin-runtime.md)。

## 许可证与支持

SolidInserts 是按 [MIT 许可证](../../LICENSE)发布的独立插件。[第三方署名](../../NOTICE.md) · [在 Ko-fi 支持开发](https://ko-fi.com/enderk)。
