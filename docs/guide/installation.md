# 下载与安装

## 下载

桌面应用和命令行工具均发布在 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases)：

| 发布文件 | 说明 |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | 桌面应用便携版：解压后直接运行 `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | 命令行工具：解压后运行 `sbsave.exe`，用法见 [CLI 文档](../cli/index.md) |

最新版本下载地址：<https://github.com/perillaroc/sblade-explorer/releases/latest>

## 系统要求

- Windows 10 或 Windows 11；
- 已安装 **WebView2 运行时**（通常已随 Microsoft Edge 安装）。如果程序提示缺少该组件，请从微软官网下载并安装 Evergreen Runtime。

## 便携版说明

- 无需安装，不会创建开始菜单快捷方式或卸载项；
- 不会写入注册表，也不会修改存档；
- 升级时，用新版本的 `sblade-explorer.exe` 替换旧文件即可。

## 从源码运行

需要安装 Rust 和 Node.js，常用命令见[开发文档](../development/index.md)。
