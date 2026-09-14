# sblade-explorer（剑星存档收集度分析）

分析 Steam 版《剑星》(Stellar Blade) 存档中的可收集物进度：纳米战衣、罐子、记录、密码、
营地、发型/眼镜/耳饰/无人机外观/同伴服装、设计图案、鱼类等，重点告诉你：

- **还缺什么**
- **在哪里拿**
- **是否需要二周目/三周目（NG+/NG++）**
- **是否 DLC 限定**

工具为**只读**分析，不会修改存档。

## 安装

需要 [uv](https://docs.astral.sh/uv/)（Python 3.10+，uv 会自动管理环境）。

```powershell
uv sync
uv run sbsave --help
```

## 用法

```powershell
# 列出检测到的存档
uv run sbsave saves

# 自动探测最新存档并输出中文报告
uv run sbsave report

# 指定存档 / 槽位
uv run sbsave report --save "C:\Users\you\AppData\Local\SB\Saved\SaveGames\7656...\StellarBladeSave00.sav"
uv run sbsave report --slot 0

# 只分析某些分类（分类键或中文名均可）
uv run sbsave report --category nano_suits,cans
uv run sbsave report --category 纳米战衣

# 文本语言：默认中文（攻略描述已翻译）；en 为英文原文，both 为中英对照
uv run sbsave report --lang en
uv run sbsave report --lang both

# 同时列出已收集的物品
uv run sbsave report --all

# 导出 JSON / Markdown 报告
uv run sbsave report --json report.json --markdown report.md

# 查看目录库 / 校验
uv run sbsave catalog list
uv run sbsave catalog check

# 解析存档结构（调试用）
uv run sbsave dump
uv run sbsave dump --obtained
uv run sbsave dump --json dump_summary.json
```

## 存档位置

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本/试玩版 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版 |

`report` 会自动探测并优先选择最新写入的主槽位；`Backup` 目录会被忽略。

## 报告说明

- **周目**：读取存档中的 `NewGamePlusPlayCount`，据此判断哪些物品需要 NG+/NG++。
- **NG+ 变体**：多数纳米战衣/耳饰/无人机外观在 NG+、NG++ 有换色版本，标注为
  `需要二周目(NG+)` / `需要三周目(NG++)`。
- **多周目获取一览**：含周目/DLC 变体的分类（纳米战衣、耳饰、眼镜/面饰、无人机外观、
  亚当/莉莉服装）按章节（Eidos 7、Xion、Wasteland 等）分表，行=获取地点，
  列=首周目/二周目(NG+)/三周目(NG++)/DLC 特典；同一地点不同周目给出的物品并列展示。
  状态用 emoji 标记：✅ 已获得、❌ 未获得、🔒 需更高周目、🎁 DLC/特典、➖ 默认外观。
  这些分类不再输出逐条缺失列表，其余分类（如设计图案、罐子）不受影响。
- **映射待确认**：目录库中仍有少量条目是推定映射（个别记录版本变体、部分营地）。
  报告会显示该标记，便于核对与修正。
- **语言**：名称来自游戏本地化（简中/英文）；区域、地点与获取描述来自英文攻略，
  其简体中文由 `data/raw/api/i18n/` 手工维护，默认输出中文（缺失时回退英文）。
  `--lang en` 输出全英文原文，`--lang both` 以「中文（English）」形式并排显示；
  导出的 JSON 报告始终同时包含 `area`/`area_zh`、`location`/`location_zh`、
  `obtain`/`obtain_zh` 两套字段。

## 自定义目录库

如发现条目缺失或映射错误，可创建用户覆盖文件（不需要改代码）：

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

也可以用 `--catalog 路径.json` 临时附加。需要覆盖中文攻略文案时，在条目中补充
`area_zh`/`location_zh`/`obtain_zh` 字段即可（优先级高于内置翻译）。

## 目录数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品名称（英文）、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 游戏本体数据表 + `Game.locres`（zh-Hans/en）—— 内部别名到官方名称的精确映射
  （由 `tools/mine_game_names.py` 提取到 `data/raw/game/name_map.json`）
- `data/raw/api/i18n/` —— stellarbladeguide.com 区域/地点/获取描述的手工简体中文翻译
- 原始数据保存在 `data/raw/`，目录库由 `tools/build_catalog.py` 生成到 `src/sbsave/data/catalog.json`

如需更新目录库：

```powershell
uv run python tools/build_catalog.py
uv run sbsave catalog check
```

游戏更新后如需刷新名称映射（需要本机游戏与本目录以外下载的 cue4parse/repak）：

```powershell
uv run python tools/mine_game_names.py --game "H:\SteamLibrary\steamapps\common\StellarBlade" --tools path\to\tools
uv run python tools/build_catalog.py
```

## 存档格式说明

正式版存档为 `EVAS`(8 字节) 包裹的 UE4.26 GVAS（试玩版为纯 GVAS），游戏类
`/Script/SB.SBSaveGame`。解析器（`src/sbsave/gvas.py`）只读实现了该存档的
tagged property 布局，并针对本作的特殊字段顺序做了适配。收集状态主要来自：

- `SBSaveGameData_ItemObject → ItemOtaineSet`：已获得物品别名集合
- `SBSaveGameData_Achievement → AchievementList`：成就/事件进度（营地、部分记录）
- `DataMap_int32`：周目数、难度、游玩时间等

## 已知限制

- 工具只读，不支持修改存档。
- 记录（文档/记忆棒/密码）、罐子、设计图案与外观名称来自游戏数据，均为精确映射；
  剩余少量推定项（记录版本变体、未能与营地表关联的营地）仍会标记「映射待确认」。
- 装备词条（Gear）与技能（PT）等非收集类数据未纳入 v1。
- 游戏更新可能改变存档结构；解析失败会给出明确错误，请不要用旧版本目录强行解析新版本。

## 开发

```powershell
uv run pytest
uv run ruff check src tools tests
```

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
