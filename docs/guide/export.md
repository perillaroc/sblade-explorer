# 导出报告

## 桌面应用

右上角提供两种导出格式：

| 按钮 | 内容 |
| --- | --- |
| **导出 JSON** | 完整的分析数据，始终包含中英双语字段 |
| **导出 Markdown** | 便于阅读的报告，按当前内容语言生成 |

点击按钮并选择保存位置即可，默认文件名为 `sblade-report.json` 或 `sblade-report.md`。
导出成功后，界面会提示「已导出 <路径>」。导出过程只读取存档，不会修改任何文件。

语言设置分为两项，互不影响：

- **内容语言**（右上角「内容」按钮）：可选择中文、English 或双语，影响界面中的物品信息和 Markdown 报告内容；JSON 始终包含中英双语字段。
- **界面语言**（设置 → 界面语言）：控制菜单、按钮、提示以及导出的 Markdown 标题和表头；首次启动时根据系统语言自动选择。

Markdown 报告包含：

- 存档信息：SteamID、周目、难度、游玩时间和目录/图鉴进度；
- 分类汇总表和图鉴汇总表；
- 支持周目矩阵的分类对照表；
- 未收集清单：矩阵分类以对照表呈现，其他分类逐条列出地点、获取方式、未收集原因和标记；
- 未映射别名清单（如有）。

## CLI

`sbsave report` 会在控制台输出报告，也可以导出相同的 JSON 或 Markdown 文件：

```powershell
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
cargo run -p sbsave-cli -- report --ui-lang en --lang en    # 英文界面 + 英文物品文本
```

`--lang zh|en|both` 控制报告中的物品文本（JSON 始终双语），`--ui-lang zh|en` 控制帮助、表头和错误等界面文字（默认跟随系统语言）。
使用 `--all` 还可以列出已收集物品。完整命令和参数见 [CLI 文档](../cli/index.md)。
