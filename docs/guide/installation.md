# 下载与安装

## 下载

桌面应用与命令行工具都发布在 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases)：

| 发布物 | 说明 |
| --- | --- |
| `sblade-explorer-<tag>-x86_64-pc-windows-msvc.zip` | 桌面应用便携版：解压后直接运行 `sblade-explorer.exe` |
| `sbsave-<tag>-x86_64-pc-windows-msvc.zip` | 命令行工具：解压后运行 `sbsave.exe`，用法见 [CLI](../cli/index.md) |

最新版下载页：<https://github.com/perillaroc/sblade-explorer/releases/latest>

## 系统要求

- Windows 10 / 11；
- 系统需已安装 **WebView2 运行时**（通常随 Edge 自带；若程序提示缺失，请从微软官网安装 Evergreen Runtime）。

## 便携版说明

- 不含安装程序：没有开始菜单快捷方式与卸载项；
- 程序本体只读：不写注册表，也不会修改存档；
- 升级方式：直接用新版本替换 exe。

## 从源码运行

需要 Rust 与 Node.js 环境，常用命令见 [开发文档](../development/index.md)。
