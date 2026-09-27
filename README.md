# sblade-explorer（剑星存档收集度分析）

[![CI](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml)
![100% Vibe Coded](https://img.shields.io/badge/100%25-Vibe_Coded-blueviolet)

读取 Steam《剑星》(Stellar Blade) 存档，报告尚未收集的收集物。**Windows 桌面应用是主要发布形式**，
另提供 `sbsave` 命令行工具。只读、离线：绝不修改存档，解析与统计全部在内存中完成，运行时不联网。

> [!IMPORTANT]
> 本项目采用 Vibe Coding 方式开发，主要由 AI Agent 完成代码实现、测试与文档编写。

## 下载与使用（桌面应用）

桌面应用发布在 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases)：

| 发布物 | 说明 |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | 桌面应用便携版：解压后直接运行 `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | 命令行工具：解压后运行 `sbsave.exe`，用法见 [docs/cli.md](docs/cli.md) |

- 系统要求 Windows 10/11，需要系统已安装 **WebView2 运行时**（通常随 Edge 自带；若提示缺失，
  从微软官网安装 Evergreen Runtime 即可）。
- 便携版不含安装程序：没有开始菜单快捷方式与卸载项，升级即替换 exe（程序本体只读、不写注册表）。
- 首次打开自动探测默认目录下的存档并选中最新写入的主槽位，可手动切换、刷新。

存档位置（自动探测，`Backup` 目录会被忽略）：

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本/试玩版 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版 |

## 主要功能

- **进度汇总**：显示存档信息（SteamID、周目/难度/游玩时间）、总进度条，以及 13 类收集物的分类
  进度汇总，每类有独立页面。
- **图鉴**：孽奇拔与角色图鉴单列展示，不计入收集进度。
- **浏览与筛选**：支持按收集状态筛选、按名称/地点搜索，条目详情提供获取方式、攻略链接与
  未收集原因等信息。
- **周目与记录视图**：部分类别支持按「区域 × 地点 × 周目」对照同一获取点的物品变化；
  记录页按游戏内类型与区域整理。
- **语言与导出**：物品文案支持中文 / English / 双语，可一键导出 JSON 或 Markdown 报告。
- **本地设置**：外观主题、打开攻略链接的浏览器与搜索引擎等偏好仅保存在本机。

统计口径：物品任一别名出现在存档物品集合或成就派生别名中即视为已收集，图鉴条目以
`Ach_Album_Unlock_*` 成就判定是否解锁。目录库共 932 条 / 15 分类，其中图鉴 122 条单列、
不计入总进度，详见 [docs/data.md](docs/data.md)。

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品英文名称、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 游戏本体数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）—— 内部别名到官方名称的精确映射
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与 [游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml) —— 记忆棒的游戏内数据库选单顺序
- [游民星空](https://www.gamersky.com/handbook/202404/1738505.shtml) 与 [哔哩哔哩](https://www.bilibili.com/video/BV1Wfj1ztEuj) —— 内置中文攻略链接（仅由系统浏览器打开）

完整来源清单、原始数据快照与刷新方式见 [data/raw/README.md](data/raw/README.md)，
目录库与别名模型见 [docs/data.md](docs/data.md)。

## 已知限制

- 工具只读，不支持修改存档。
- 装备词条（Gear）与技能（PT）等非收集类数据未纳入 v1。
- 游戏更新可能改变存档结构；解析失败会给出明确错误，请不要用旧版本目录强行解析新版本。
- 存档位置探测仅覆盖上述三处 Windows 路径，不跨平台。

## 文档

| 内容 | 位置 |
| --- | --- |
| CLI 命令与参数 | [docs/cli.md](docs/cli.md) |
| 数据来源、目录库与别名、自定义覆盖、回归哨兵 | [docs/data.md](docs/data.md) |
| 开发环境、项目结构、数据管线、CI 与发布 | [docs/development.md](docs/development.md) |
| 原始数据说明与刷新 | [data/raw/README.md](data/raw/README.md) |
| 桌面应用开发 | [apps/desktop/README.md](apps/desktop/README.md) |

## 协议

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
