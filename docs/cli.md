# `sbsave` CLI

命令行工具 `sbsave` 与桌面应用共用 `sbsave-core`，功能一致但不含图形界面。
从 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) 下载
`sbsave-<tag>-x86_64-pc-windows-msvc.zip`，解压后运行 `sbsave.exe`；也可以从源码运行：

```powershell
cargo run -p sbsave-cli -- <命令>
```

## 命令

### `saves`

列出自动探测到的存档（槽位、SteamID、大小、修改时间、路径）。

```powershell
cargo run -p sbsave-cli -- saves
```

### `report`

分析并输出中文控制台报告，可导出 JSON/Markdown；支持指定存档/槽位/分类/语言，并可列出已收集物品。

```powershell
cargo run -p sbsave-cli -- report
cargo run -p sbsave-cli -- report --save "C:\...\StellarBladeSave00.sav" --slot 0
cargo run -p sbsave-cli -- report --category 纳米战衣 --lang both
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
```

### `dump`

解析存档并输出结构信息（调试用），可导出摘要 JSON、完整解析树 JSON、已获得别名。

```powershell
cargo run -p sbsave-cli -- dump --obtained
```

### `catalog`

查看/校验目录库（分类数量、重复别名等）。

```powershell
cargo run -p sbsave-cli -- catalog list
cargo run -p sbsave-cli -- catalog check
```

## 参数

| 命令 | 参数 |
| --- | --- |
| `report` | `--save/-s`、`--slot`、`--category/-c`（分类键或中文名，逗号分隔）、`--lang`（`zh` 默认 / `en` / `both`）、`--all`（列出已收集）、`--json`、`--markdown`、`--catalog` |
| `dump` | `--save/-s`、`--slot`、`--json`（摘要）、`--tree`（完整解析树）、`--obtained`（别名） |
| `catalog` | `list` / `check`，均支持 `--catalog` 附加覆盖文件 |

`--catalog` 用于临时附加用户覆盖文件，格式说明见 [data.md](data.md#自定义目录库)。
报告中的「已获得」判定、NG+/DLC 标注与低置信度条目说明见 [data.md](data.md)。
