# `sbsave` CLI

命令行工具 `sbsave` 与桌面应用共用 `sbsave-core`，提供相同的分析功能，但不包含图形界面。
从 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) 下载
`sbsave-<tag>-x86_64-pc-windows-msvc.zip`，解压后运行 `sbsave.exe`。也可以从源码运行：

```powershell
cargo run -p sbsave-cli -- <命令>
```

## 命令

### `saves`

列出自动找到的存档，包括槽位、SteamID、大小、修改时间和路径。

```powershell
cargo run -p sbsave-cli -- saves
```

### `report`

分析存档并在控制台输出报告，也可以导出 JSON 或 Markdown。
支持指定存档、槽位、分类和语言，以及列出已收集物品。报告默认使用中文界面，可通过 `--ui-lang` 切换。

报告分为两部分：**收集品**（13 类，计入目录进度）和**图鉴**（孽奇拔 67 条、角色 55 页，单列且不计入目录进度）。
JSON 的 `summary` 同时包含 `catalog_*`（收集品）和 `album_*`（图鉴）字段，每个分类都带有
`section`（`collection` 或 `album`）；图鉴条目还包含官方说明 `desc_zh` 和 `desc_en`。

```powershell
cargo run -p sbsave-cli -- report
cargo run -p sbsave-cli -- report --save "C:\...\StellarBladeSave00.sav" --slot 0
cargo run -p sbsave-cli -- report --category 纳米战衣 --lang both
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
```

### `dump`

解析存档并输出结构信息，主要用于调试。可导出摘要 JSON、完整解析树 JSON 或已获得别名列表。

```powershell
cargo run -p sbsave-cli -- dump --obtained
```

### `catalog`

查看或校验目录库，包括分类数量和重复别名等。

```powershell
cargo run -p sbsave-cli -- catalog list
cargo run -p sbsave-cli -- catalog check
```

## 参数

| 命令 | 参数 |
| --- | --- |
| `report` | `--save/-s`、`--slot`、`--category/-c`（分类键、中文名或英文名，多个分类用逗号分隔）、`--lang`（物品文本：`zh`（默认）/ `en` / `both`）、`--all`（列出已收集物品）、`--json`、`--markdown`、`--catalog`、`--dlc`（已拥有的 DLC，逗号分隔：`nier,nikke`） |
| `dump` | `--save/-s`、`--slot`、`--json`（摘要）、`--tree`（完整解析树）、`--obtained`（别名） |
| `catalog` | `list` / `check`，均支持 `--catalog` 加载覆盖文件 |

所有命令都接受全局参数 `--ui-lang zh|en` 与 `--dlc nier,nikke`，前者控制帮助、表头和错误等界面文字，
后者声明已拥有的 DLC（存档中没有可靠的授权字段，仅影响 DLC 限定判定）。
未指定语言时使用系统语言，无法识别时回退到中文。`--lang` 只影响物品文本和导出报告的内容。

`--catalog` 用于临时加载用户覆盖文件，格式说明见 [data.md](../reference/data.md#自定义目录库)。
报告中「已获得」的判定方式、NG+/DLC 标注和低置信度条目说明见 [data.md](../reference/data.md)。
