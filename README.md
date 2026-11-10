# sblade-explorer（剑星存档收集度分析）

**[简体中文](README.md) | [English](README.en.md)**

![100% Vibe Coded](https://img.shields.io/badge/100%25-Vibe_Coded-blueviolet)
[![Release](https://img.shields.io/github/v/release/perillaroc/sblade-explorer)](https://github.com/perillaroc/sblade-explorer/releases)
[![CI](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/perillaroc/sblade-explorer/actions/workflows/ci.yml)
[![Netlify Status](https://api.netlify.com/api/v1/badges/12901971-6ff6-4dfe-bc46-08d735ffa6aa/deploy-status)](https://app.netlify.com/projects/sblade-explorer/deploys)

读取 Steam 版《剑星》（Stellar Blade）存档，帮你找出尚未收集的物品。
**Windows 桌面应用是主要发布形式**，另提供 `sbsave` 命令行工具。
全程只读、离线运行：不会修改存档，解析和统计均在本机内存中完成，运行时不会联网。

> [!IMPORTANT]
> 本项目采用 Vibe Coding 方式开发，代码、测试和文档主要由 AI Agent 编写。

## 下载

桌面应用和命令行工具均可从 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) 下载：

| 发布文件 | 说明 |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | 桌面应用便携版：解压后直接运行 `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | 命令行工具：解压后运行 `sbsave.exe` |

- 支持 Windows 10/11，需要安装 **WebView2 运行时**（通常已随 Edge 安装）。
- 便携版无需安装，不会创建开始菜单快捷方式或卸载项，也不会写入注册表。升级时，用新版 `sblade-explorer.exe` 替换旧文件即可。

## 主要功能

- **进度汇总**：查看 SteamID、周目、难度、游玩时间等存档信息，以及 13 类收集物的进度；每类都有独立页面；
- **图鉴**：孽奇拔和角色图鉴单独展示，不计入收集进度；
- **浏览与筛选**：按收集状态筛选，按名称或地点搜索；条目详情提供获取方式、攻略链接和未收集原因；
- **周目与记录视图**：部分类别可按「区域 × 地点 × 周目」对照同一获取点的物品；记录按游戏内类型和区域整理；
- **语言与导出**：界面支持中文和 English，物品信息支持中文、English 或双语显示，可一键导出 JSON 或 Markdown 报告；
- **本地设置**：主题、打开攻略链接的浏览器、搜索引擎等偏好仅保存在本机。

## 快速开始

1. 解压下载的 ZIP 压缩包，运行 `sblade-explorer.exe`；
2. 首次启动时，程序会自动查找存档并打开最近写入的主槽位，也可以手动切换槽位或刷新列表；
3. 在「汇总」页查看总进度，点击左侧分类查看未收集物品及其获取方式；
4. 点击右上角的按钮，导出 JSON 或 Markdown 报告。

详细操作见[快速开始](https://sblade-explorer.perillaroc.wang/guide/getting-started)。

## 文档

完整文档提供中文和 English 两个版本：

**<https://sblade-explorer.perillaroc.wang/>**（[English](https://sblade-explorer.perillaroc.wang/en/)）

- [用户指南](https://sblade-explorer.perillaroc.wang/guide/getting-started)：下载与安装、存档位置、收集度分析、报告导出和常见问题；
- [数据说明](https://sblade-explorer.perillaroc.wang/reference/data)：数据来源、目录库、别名模型和自定义覆盖。

## CLI

命令行工具 `sbsave` 与桌面应用共用同一核心，提供相同的分析功能，但不包含图形界面，
也可从 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases) 下载。
完整命令和参数见 [CLI 文档](https://sblade-explorer.perillaroc.wang/cli/)。

## 开发

开发环境、项目结构、数据管线、测试和发布流程见[开发文档](https://sblade-explorer.perillaroc.wang/development/)；
桌面应用的结构和常用命令见 [apps/desktop/README.md](apps/desktop/README.md)。

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com)：物品英文名称、位置描述和周目标签；
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor)：简体中文物品名称和别名映射；
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file)：用于数据校验的完整别名集合；
- 游戏数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）：内部别名与官方名称的对应关系；
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与[游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml)：记忆棒在游戏内数据库中的排列顺序；
- [游民星空](https://www.gamersky.com/handbook/202404/1738505.shtml) 与[哔哩哔哩](https://www.bilibili.com/video/BV1Wfj1ztEuj)：内置中文攻略链接，仅在点击后通过系统浏览器打开。

完整来源清单和原始数据更新方法见 [data/raw/README.md](data/raw/README.md)，
目录库与别名模型见[在线文档](https://sblade-explorer.perillaroc.wang/reference/data)。

## 许可证

[Apache License 2.0](LICENSE) · Copyright 2026 perillaroc

界面和应用图标使用 [Lucide](https://lucide.dev)（ISC 许可证），图标说明见
[apps/desktop/src-tauri/icons/README.md](apps/desktop/src-tauri/icons/README.md)。

本项目与 Shift Up、Sony Interactive Entertainment 无关，仅供个人分析存档使用。
