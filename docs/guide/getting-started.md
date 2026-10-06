# 快速开始

本页带新用户走完最短路径：**下载 → 解压 → 启动 → 找到存档 → 查看收集度 → 查看未收集 → 导出结果**。
程序只读、离线：解析与统计全部在本机完成，绝不修改存档。

## 1. 下载

桌面应用发布在 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases)，
下载 `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip`（最新版见
[releases/latest](https://github.com/perillaroc/sblade-explorer/releases/latest)）。
发布物清单、CLI 与系统要求见 [下载与安装](./installation.md)。

## 2. 解压并启动

解压 zip 后直接运行 `sblade-explorer.exe`。便携版无需安装；要求 Windows 10/11，
并需要系统已安装 WebView2 运行时（通常随 Edge 自带）。

## 3. 找到存档

首次打开时程序会自动探测默认目录下的存档，并选中最新写入的主槽位；
顶部「存档」下拉可切换槽位，右侧显示文件大小与修改时间，点「刷新」可重新扫描。
程序会记住手动选择的存档，也可用「选择存档文件…」打开任意位置的存档。
若提示未找到存档，界面上会列出实际扫描的目录与操作入口，详见 [存档位置](./save-location.md)。

## 4. 查看收集度

左侧「汇总」页显示：

- 存档信息：SteamID、周目（含 NG+ 次数）、难度、游玩时间；
- **目录进度**：13 类收集品的总进度与未收集数量；
- **图鉴进度**：孽奇拔与角色图鉴单独显示，不计入目录进度。

点击左侧任意分类进入独立页面。分类页支持「已收集 / 未收集 / 全部」筛选与
名称、地点、ID 搜索；点击条目查看完整详情。详见 [收集度分析](./collections.md)。

## 5. 查看未收集内容

在分类页点击「未收集」筛选，或直接搜索名称；点击条目打开详情弹窗，
可查看获取方式、中文攻略按钮与未收集原因（如「需要二周目(NG+)」）。

## 6. 导出结果

右上角提供「导出 JSON」与「导出 Markdown」按钮；右上角「内容」按钮可在 中文 / English / 双语
之间切换物品文案，界面语言在 设置 → 界面语言 中单独切换。详见 [导出报告](./export.md)。

## 下一步

- [下载与安装](./installation.md)
- [存档位置](./save-location.md)
- [收集度分析](./collections.md)
- [导出报告](./export.md)
- [FAQ](./faq.md)
