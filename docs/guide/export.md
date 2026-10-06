# 导出报告

## 桌面应用

右上角提供两个导出按钮：

| 按钮 | 内容 |
| --- | --- |
| **导出 JSON** | 完整分析数据，始终包含中英双语字段 |
| **导出 Markdown** | 可读报告，按当前语言模式生成 |

点击后选择保存位置（默认文件名 `sblade-report.json` / `sblade-report.md`），
导出成功后界面会提示「已导出 <路径>」。导出过程只读取存档，不会修改任何文件。

语言有两个独立设置：

- **内容语言**（右上角「内容」按钮）：在 中文 / English / 双语 之间切换，影响界面中的物品文案与
  Markdown 报告内容；JSON 始终包含中英双语字段。
- **界面语言**（设置 → 界面语言）：菜单、按钮、提示与导出的 Markdown 标题/表头；首次启动按系统语言推断。

Markdown 报告包含：

- 存档信息：SteamID、周目、难度、游玩时间、目录/图鉴进度；
- 分类汇总与图鉴汇总表格；
- 周目矩阵分类的对照表；
- 未收集清单：矩阵分类以对照表呈现，其余分类逐条列出地点、获取方式、
  未收集原因与标记；
- 未映射别名清单（如有）。

## CLI

`sbsave report` 输出控制台报告，也可以导出同样的 JSON / Markdown：

```powershell
cargo run -p sbsave-cli -- report --json report.json --markdown report.md
cargo run -p sbsave-cli -- report --ui-lang en --lang en    # 英文界面 + 英文物品文本
```

`--lang zh|en|both` 控制报告物品文本（JSON 始终双语），`--ui-lang zh|en` 控制帮助、表头与错误等
界面文字（默认跟随系统语言）；`--all` 可列出已收集物品。完整命令与参数见 [CLI 文档](../cli/index.md)。
