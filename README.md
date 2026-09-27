# sblade-explorer（剑星存档收集度分析）

[![CI](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml)
![100% Vibe Coded](https://img.shields.io/badge/100%25-Vibe_Coded-blueviolet)

读取 Steam《剑星》(Stellar Blade) 存档，报告尚未收集的收集物。**Windows 桌面应用是主要发布形式**，
另提供 `sbsave` 命令行工具。只读、离线：绝不修改存档，解析与统计全部在内存中完成，运行时不联网。

> [!IMPORTANT]
> 本项目采用 Vibe Coding 方式开发，主要由 AI Agent 完成代码实现、测试与文档编写。

## 下载

桌面应用与 CLI 均发布在 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases)：

| 发布物 | 说明 |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | 桌面应用便携版：解压后直接运行 `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | 命令行工具：解压后运行 `sbsave.exe` |

- 系统要求 Windows 10/11，需要系统已安装 **WebView2 运行时**（通常随 Edge 自带）。
- 便携版不含安装程序：没有开始菜单快捷方式与卸载项，程序本体只读、不写注册表；升级即替换 exe。

## 主要功能

- **进度汇总**：显示存档信息（SteamID、周目/难度/游玩时间）与 13 类收集物的分类进度，每类有独立页面；
- **图鉴**：孽奇拔与角色图鉴单列展示，不计入收集进度；
- **浏览与筛选**：支持按收集状态筛选、按名称/地点搜索，条目详情提供获取方式、攻略链接与未收集原因等信息；
- **周目与记录视图**：部分类别支持按「区域 × 地点 × 周目」对照同一获取点的物品变化；记录页按游戏内类型与区域整理；
- **语言与导出**：物品文案支持中文 / English / 双语，可一键导出 JSON 或 Markdown 报告；
- **本地设置**：外观主题、打开攻略链接的浏览器与搜索引擎等偏好仅保存在本机。

## 快速开始

1. 解压下载的 zip，运行 `sblade-explorer.exe`；
2. 首次打开自动探测存档并选中最新写入的主槽位，也可手动切换或刷新；
3. 在「汇总」查看总进度，点击左侧分类查看未收集内容与获取方式；
4. 右上角一键导出 JSON / Markdown 报告。

更详细的步骤见[快速开始](https://sblade-explorer.perillaroc.wang/guide/getting-started)。

## 文档

完整使用文档：

**https://sblade-explorer.perillaroc.wang/**

- [用户指南](https://sblade-explorer.perillaroc.wang/guide/getting-started)：下载与安装、存档位置、收集度分析、导出报告与 FAQ
- [数据说明](https://sblade-explorer.perillaroc.wang/reference/data)：数据来源、目录库与别名模型、自定义覆盖

## CLI

命令行工具 `sbsave` 与桌面应用共用同一核心，功能一致但不含图形界面，也随
[GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) 发布。
完整命令与参数见 [CLI 文档](https://sblade-explorer.perillaroc.wang/cli/)。

## 开发

开发环境、项目结构、数据管线、测试与发布见[开发文档](https://sblade-explorer.perillaroc.wang/development/)；
桌面应用自身的结构与命令见 [apps/desktop/README.md](apps/desktop/README.md)。

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品英文名称、位置描述、周目标签
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) —— 简体中文物品名称与别名映射
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) —— 别名全集与数据校验
- 游戏本体数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）—— 内部别名到官方名称的精确映射
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与 [游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml) —— 记忆棒的游戏内数据库选单顺序
- [游民星空](https://www.gamersky.com/handbook/202404/1738505.shtml) 与 [哔哩哔哩](https://www.bilibili.com/video/BV1Wfj1ztEuj) —— 内置中文攻略链接（仅由系统浏览器打开）

完整来源清单与原始数据刷新方式见 [data/raw/README.md](data/raw/README.md)，
目录库与别名模型见[在线文档](https://sblade-explorer.perillaroc.wang/reference/data)。

## 协议

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

本项目与 Shift Up / Sony Interactive Entertainment 无关，仅供个人存档分析使用。
