# sblade-explorer（剑星存档收集度分析）

读取 Steam《剑星》(Stellar Blade) 存档并报告缺失的收集物：纳米战衣、罐子、记录、密码、
营地、发型/眼镜/耳饰/无人机外观/同伴服装、设计图案、鱼类等。桌面版（Tauri + Vue）提供
总览进度、分类汇总、NG+/DLC 矩阵与缺失清单搜索；同时保留 `sbsave` CLI。

工具为**只读**分析，不会修改存档。

> 本项目为 AI 大模型开发项目：全部代码、目录库数据管线与文档均由 AI 大模型生成，
> 人类负责提出需求、提供数据与最终审阅。

## 安装与运行

### 桌面应用（Windows）

- 安装包由 `pnpm tauri build` 生成：
  `target/release/bundle/nsis/sblade-explorer_0.1.0_x64-setup.exe`（NSIS）或
  `target/release/bundle/msi/sblade-explorer_0.1.0_x64_en-US.msi`（MSI）。
- 系统要求：Windows 10/11；WebView2 由安装包按默认引导方式处理。
- 首次打开自动探测最新存档并分析。

界面功能：存档切换、总览进度、分类汇总（点击筛选）、NG+/DLC 矩阵（章节 × 地点 × 周目）、
缺失清单（名称/地点/ID 搜索）、中文/英文/双语切换、导出 JSON/Markdown。

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

## 存档位置（自动探测）

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本/试玩版 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版 |

`Backup` 目录会被忽略；CLI `report` 与桌面应用自动选择最新写入的主槽位。

## 报告说明

- **周目**：读取存档中的 `NewGamePlusPlayCount`，标注 `需要二周目(NG+)` / `需要三周目(NG++)`。
- **多周目获取一览**：纳米战衣、耳饰、眼镜/面饰、无人机外观、亚当/莉莉服装按章节 × 地点 ×
  周目矩阵展示；状态：✅ 已获得、❌ 未获得、🔒 需更高周目、🎁 DLC/特典、➖ 默认外观。
- **语言**：`zh`（默认）/ `en` / `both`（中文（English））；JSON 报告始终双语。
- **映射待确认**：少量记录版本变体与未关联的营地仍会标记（共 69 条）。
- **只读**：不修改存档；导出是唯一写文件操作。

## 自定义目录库

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

## 项目结构

- `crates/sbsave-core/` - Rust 核心库：GVAS 解析、存档提取、目录库、差集分析、报告与 JSON 契约。
- `crates/sbsave-cli/` - `sbsave` CLI（clap），命令与输出对齐归档 Python 版。
- `apps/desktop/` - Tauri 2 桌面应用：前端 Vue 3 + Vite + TypeScript + Tailwind v4（工程根），
  Rust 在 `src-tauri/`。
- `tools/` - 构建期 Python 数据管线（`build_catalog.py`、`crosswalk_data.py`、`mine_game_names.py`）。
- `data/raw/` - 已提交的数据快照；`data/catalog.json` 为生成物（禁止手改）。

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品名称（英文）、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 游戏本体数据表 + `Game.locres`（zh-Hans/en）—— 内部别名到官方名称的精确映射
  （`tools/mine_game_names.py` → `data/raw/game/name_map.json`）
- `data/raw/api/i18n/` —— stellabladeguide.com 区域/地点/获取描述的手工简体中文翻译

## 开发

```powershell
# Rust（仓库根目录）
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test

# 前端 / 桌面（apps/desktop）
pnpm install
pnpm dev            # 仅前端
pnpm build          # vue-tsc --noEmit + vite build
pnpm tauri dev      # 桌面应用开发
pnpm tauri build    # Windows 安装包（NSIS + MSI）

# Python 数据管线（仓库根目录，首次 uv sync）
uv run python tools/build_catalog.py
uv run ruff check tools
```

回归哨兵：目录库 810 条 / 13 分类；纳米战衣 126、罐子 49、营地 89、低置信度 69。
差分基准：Python 参考实现归档于仓库外 `archived/sblade-explorer-python/`，迁移期间
两端 `report --json` 逐节点一致、Markdown 逐字一致。

## 已知限制

- 工具只读，不支持修改存档。
- 装备词条（Gear）与技能（PT）等非收集类数据未纳入 v1。
- 游戏更新可能改变存档结构；解析失败会给出明确错误，请不要用旧版本目录强行解析新版本。

## 许可证

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
