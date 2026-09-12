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
- **纳米战衣获取一览**：按章节（Eidos 7、Xion、Wasteland 等）分表，行=获取地点，
  列=首周目/二周目(NG+)/三周目(NG++)/DLC 特典；同一地点在不同周目给出的战衣并列展示。
  状态用 emoji 标记：✅ 已获得、❌ 未获得、🔒 需更高周目、🎁 DLC/特典、➖ 默认外观。
- **映射待确认**：目录库中部分条目是根据内部 ID 推导/顺序推定的（如罐子编号顺序、
  记录/营地/图案）。报告会显示该标记，便于核对与修正。

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

也可以用 `--catalog 路径.json` 临时附加。

## 目录数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品名称（英文）、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 原始数据保存在 `data/raw/`，目录库由 `tools/build_catalog.py` 生成到 `src/sbsave/data/catalog.json`

如需更新目录库：

```powershell
uv run python tools/build_catalog.py
uv run sbsave catalog check
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
- 记录（文档/记忆棒/密码）的逐条名称由内部 ID 推导（区域+编号），精确名称请对照攻略
  同区域列表；罐子编号顺序为推定映射。
- 装备词条（Gear）与技能（PT）等非收集类数据未纳入 v1。
- 游戏更新可能改变存档结构；解析失败会给出明确错误，请不要用旧版本目录强行解析新版本。

## 开发

```powershell
uv run pytest
uv run ruff check src tools tests
```

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
