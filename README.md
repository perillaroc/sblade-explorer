# sblade-explorer（剑星存档收集度分析）

读取 Steam《剑星》(Stellar Blade) 存档并报告缺失的收集物，支持 Windows 桌面应用与 `sbsave` CLI：

- **13 类收集物**：纳米战衣、罐子、记录（文档/记忆棒）、密码、营地、发型、眼镜/面饰、耳饰、
  无人机外观、亚当服装、莉莉服装、设计图案、鱼类。
- **只读**：绝不修改存档；解析、统计与界面展示全部在内存中完成，导出是唯一的写文件操作。
- **离线**：目录库与全部数据内置于程序，运行时不联网。

> 本项目为 AI 大模型开发项目：全部代码、目录库数据管线与文档均由 AI 大模型生成，
> 人类负责提出需求、提供数据与最终审阅。

## 功能概述

### 桌面应用（Tauri 2 + Vue 3）

- **侧边栏多页面导航**：汇总页 + 每个收集分类的独立页面；分类页头部显示收集进度条。
- **存档选择**：自动探测默认目录下的存档并选中最新写入的主槽位，可手动切换、刷新。
- **汇总页**：SteamID、周目/难度/游玩时间、总进度条、分类汇总表（点击进入分类页）。
- **筛选与搜索**：每页可按已收集/未收集/全部筛选，按名称/地点/ID 搜索；列表视图点击任意条目
  弹窗查看完整详情（获取方式、未收集原因、周目/DLC/可错过标记、记录类型、备注、别名、
  数据来源与映射置信度），覆盖全部 13 类收集物。
- **周目矩阵视图**：纳米战衣、设计图案、耳饰、眼镜/面饰、无人机外观、亚当/莉莉服装
  支持「周目矩阵 / 列表」切换；以 区域 × 地点 × 周目 矩阵展示（首周目 / 二周目(NG+) /
  三周目(NG++) / DLC/特典），同一行为同一获取点（高周目会替换该点的物品），点击行弹窗
  查看完整获取方式、周目变体、ID 与多周目/DLC/可错过标记；区域与地点分组均可折叠。
- **记录页**：按游戏内记录类型分标签页（记忆棒与文档 9 个子类）；记忆棒再按游戏区域
  （埃多斯7号/废土/矩阵11/大沙漠/埃多斯9号/尖塔4/希雍）分组，并按游戏内数据库选单顺序排列，
  区域分组可折叠；文档与密码显示攻略区域/地点（与其它收集物同一套地点模型），全部记录显示
  攻略获取方式（默认中文，可切换英文/双语）。
- **矩阵状态图例**：已获得 / 未获得 / 需更高周目 / DLC 特典 / 默认外观。
- **语言切换**：物品文案支持 中文 / English / 双语（中文（English））；JSON 报告始终双语。
- **导出**：一键导出 JSON 或 Markdown 报告（经系统保存对话框选择路径）。
- **关于**：左下角「关于」按钮打开对话框，列明外部数据来源、版权与免责声明。

### CLI（`sbsave`）

- `saves`：列出自动探测到的存档（槽位、SteamID、大小、修改时间、路径）。
- `report`：分析并输出中文控制台报告，可导出 JSON/Markdown；支持指定存档/槽位/分类/语言，
  并可列出已收集物品。
- `dump`：解析存档并输出结构信息（调试用），可导出摘要 JSON、完整解析树 JSON、已获得别名。
- `catalog list|check`：查看/校验目录库（分类数量、重复别名等）。

### 报告与统计口径

- 只要物品的任一 `aliases` 出现在存档物品集合或从成就记录派生的别名中，即视为「已获得」。
- 读取存档中的 `NewGamePlusPlayCount`，标注 `需要二周目(NG+)` / `需要三周目(NG++)`。
- 少量记录版本变体与未关联营地保留低置信度，报告中标记「映射待确认」（共 69 条）。
- 分类数量（回归哨兵）：共 810 条 / 13 分类，纳米战衣 126、罐子 49、营地 89、
  记录 310（记忆棒 187、文档 123）、设计图案 87、鱼类 35 等。

## 使用方法

### 桌面应用（Windows）

安装包由 `pnpm tauri build` 生成，位于：

- `target/release/bundle/nsis/sblade-explorer_0.1.0_x64-setup.exe`（NSIS）
- `target/release/bundle/msi/sblade-explorer_0.1.0_x64_en-US.msi`（MSI）

系统要求 Windows 10/11；WebView2 由安装包按 Tauri 默认引导方式处理。首次打开自动探测最新
存档并分析；未找到存档时可将存档放入下方默认目录后点击刷新。

### CLI

```powershell
cargo run -p sbsave-cli -- saves
cargo run -p sbsave-cli -- report
cargo run -p sbsave-cli -- report --save "C:\...\StellarBladeSave00.sav" --slot 0
cargo run -p sbsave-cli -- report --category 纳米战衣 --lang both
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
cargo run -p sbsave-cli -- dump --obtained
cargo run -p sbsave-cli -- catalog list
cargo run -p sbsave-cli -- catalog check
```

参数：

| 命令 | 参数 |
| --- | --- |
| `report` | `--save/-s`、`--slot`、`--category/-c`（分类键或中文名，逗号分隔）、`--lang`（`zh` 默认 / `en` / `both`）、`--all`（列出已收集）、`--json`、`--markdown`、`--catalog` |
| `dump` | `--save/-s`、`--slot`、`--json`（摘要）、`--tree`（完整解析树）、`--obtained`（别名） |
| `catalog` | `list` / `check`，均支持 `--catalog` 附加覆盖文件 |

### 存档位置（自动探测）

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本/试玩版 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版 |

`Backup` 目录会被忽略；CLI `report` 与桌面应用自动选择最新写入的主槽位。

### 自定义目录库

如发现条目缺失或映射错误，可创建用户覆盖文件（无需改代码、无需重新生成）：

`%LOCALAPPDATA%\sbsave\catalog.user.json`

```json
{
  "items": [
    {
      "id": "Can_007",
      "name": "狄俄尼索斯C",
      "category": "cans",
      "aliases": ["Can_007"],
      "area": "希雍",
      "location": "公告板附近地面",
      "obtain": "前往公告板路上捡取",
      "confidence": "high"
    }
  ]
}
```

CLI 亦可用 `--catalog 路径.json` 临时附加；条目内 `area_zh`/`location_zh`/`obtain_zh`
优先级高于内置翻译。

## 开发

### 环境要求

- Rust（stable 工具链）与 Cargo
- Node.js + pnpm（前端 / 桌面）
- Windows 打包需 WebView2（Windows 10/11 通常已安装）

### 常用命令

```powershell
# Rust（仓库根目录）
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo run -p sbsave-cli -- --version

# 前端 / 桌面（apps/desktop）
pnpm install
pnpm dev            # 仅前端
pnpm build          # vue-tsc --noEmit + vite build
pnpm tauri dev      # 桌面应用开发
pnpm tauri build    # Windows 安装包（NSIS + MSI）
```

### 项目结构

```text
crates/sbsave-core/    # Rust 核心库：GVAS/EVAS 解析、存档提取、目录库、差集分析、报告渲染
crates/sbsave-cli/     # clap CLI（二进制 sbsave），命令与输出沿用原 Python 版约定
crates/sbsave-tools/   # 构建期数据管线（二进制 sbsave-tools）：catalog build / mine-names
apps/desktop/          # Tauri 2 桌面应用：前端 Vue 3 + Vite + TS + Tailwind CSS v4（工程根）
apps/desktop/src-tauri # Tauri shell：list_saves / analyze_save / export_report 三个 command
data/raw/              # 已提交的数据快照（含手工维护的 crosswalk.json）；运行时绝不联网
data/catalog.json      # 由 sbsave-tools catalog build 生成的目录库；禁止手改
```

其中 `crates/sbsave-core` 的模块划分：`gvas`（GVAS/EVAS 解析）、`savegame`（存档探测与提取）、
`catalog`（目录库）、`analyze`（差集与 NG+/DLC 逻辑）、`report`（JSON 契约/Markdown/控制台）。

### 数据管线

```powershell
# 重新生成 data/catalog.json
cargo run -p sbsave-tools -- catalog build
cargo run -p sbsave-cli -- catalog check

# 集成测试：生成结果与已提交文件逐字节比对
cargo test -p sbsave-tools
```

修改目录库数据源后的顺序：先 `catalog build`，再 `catalog check`。人工别名映射在
`data/raw/crosswalk.json`（唯一事实来源）；攻略文案翻译在 `data/raw/api/i18n/`。

刷新游戏名称映射（需要本机安装游戏与 `cue4parse.exe`/`repak.exe`，详见 `data/raw/README.md`）：

```powershell
cargo run -p sbsave-tools -- mine-names `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

### 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品名称（英文）、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 游戏本体数据表 + `Game.locres`（zh-Hans/en）—— 内部别名到官方名称的精确映射
  （`sbsave-tools mine-names` → `data/raw/game/name_map.json`）
- `data/raw/api/i18n/` —— stellabladeguide.com 区域/地点/获取描述的手工简体中文翻译
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与
  [游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml) —— 记忆棒游戏内选单顺序

### 回归哨兵与验收

- 目录库 810 条 / 13 分类；纳米战衣 126、罐子 49、营地 89、低置信度 69。
- 记录 310 条全部带游戏内类型（记忆棒 187、文档 123）与攻略获取方式；记忆棒 186 条带游戏内选单顺序
  （版本变体继承基础条目）；文档与密码带攻略区域/地点，密码 23 条全部带攻略获取方式。
- 迁移验收（2026-09-14）：与原 Python 版 `report --json` 逐节点一致、Markdown 归一化后逐字一致；
  Python 参考实现已于 2026-09-15 删除，代码历史保留在 repo git（提交 `33cb981`）。
- 无 CI；`cargo fmt/clippy/test` + 前端 `pnpm build` 是关卡。GVAS 解析器保持只读与防御性：
  无法解析的 struct/array/map/set 回退为 `RawValue`，reader 始终重同步到 `tag_start + size`；
  测试使用逐字节构造的合成档案，绝不修改真实存档。

## 已知限制

- 工具只读，不支持修改存档。
- 装备词条（Gear）与技能（PT）等非收集类数据未纳入 v1。
- 游戏更新可能改变存档结构；解析失败会给出明确错误，请不要用旧版本目录强行解析新版本。
- 存档位置探测仅覆盖上述三处 Windows 路径，不跨平台。

## 协议

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
