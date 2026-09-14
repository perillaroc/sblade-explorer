# 原始数据说明

本目录保存构建目录库所需的原始资料与人工对照表，运行时不会联网。

## 目录结构

| 路径 | 内容 |
| --- | --- |
| `api/*.json` | stellarbladeguide.com API 响应（物品名称、位置描述、周目标签） |
| `api/i18n/levels.json`、`locations.json` | 攻略区域/地点的简体中文翻译（英文原文 -> 中文） |
| `api/i18n/obtain_*.json` | 攻略获取描述的简体中文翻译（站点条目 id -> 中文） |
| `universe/aliases.json` | 从参考存档与本地存档提取的物品别名全集（不含未解锁 ID） |
| `game/name_map.json` | 游戏数据表 + `Game.locres` 提取的内部别名→官方名称映射（生成物） |
| `reference/Sources_TrainerCore_*.swift` | 中文名称对照（来自 stellar-blade-macos-save-editor） |
| `reference/*.sav` | 参考存档（仅本地校验用，已在 .gitignore 中忽略） |

## 数据来源

- <https://stellarbladeguide.com> —— 英文名称、位置、描述、Base/NG+/NG++/DLC 标签
- `api/i18n/` —— 上述英文攻略文案的手工简体中文翻译（名称仍以游戏本地化为准）
- <https://github.com/wuxiao00j/stellar-blade-macos-save-editor> —— 简体中文名称
- <https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file> —— 别名全集校验
- 游戏本体数据表（`ItemTable`、`ZoneCampTable`）与 `Game.locres`（zh-Hans/en）——
  内部别名到官方名称的精确映射，经 `tools/mine_game_names.py` 生成 `game/name_map.json`

## 重新生成目录库

```powershell
uv run python tools/build_catalog.py
uv run sbsave catalog check
```

人工别名映射在 `tools/crosswalk_data.py`；攻略文案翻译在 `data/raw/api/i18n/`
（新增站点条目后需补齐 `obtain_*.json`，`tools/build_catalog.py` 会按英文原文合并，
`tests/test_catalog.py` 会校验覆盖率）。如只需修正个别条目，优先使用
`%LOCALAPPDATA%\sbsave\catalog.user.json` 覆盖文件。

## 刷新游戏名称映射（需要本机安装游戏）

需要社区工具 `cue4parse.exe`、`repak.exe` 与社区 `.usmap`。把它们放在同一目录后执行：

```powershell
uv run python tools/mine_game_names.py `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

`--game` 会把数据表导出到 `data/raw/game/dump`（已在 .gitignore 中忽略）并解包
`Game.{zh-Hans,en}.locres`，然后写出 `data/raw/game/name_map.json`。也可以先用
`retoc`/`cue4parse` 自行导出，再用 `--dump 目录` 离线运行。

## 刷新 API 数据（需要可访问外网）

```powershell
curl.exe -x "http://127.0.0.1:50802" -A "Mozilla/5.0" `
  -o data/raw/api/collectibles__cans.json `
  "https://stellarbladeguide.com/api/collectibles/cans"
```

其余端点：`collectibles/{cans,documents,memorysticks,passcodes,camps}`、
`cosmetics/{nano-suits,hairstyles,glasses,earrings,drone,lily,adam}`。
