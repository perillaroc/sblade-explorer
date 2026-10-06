---
layout: home

hero:
  name: sblade-explorer
  text: 剑星存档收集度分析工具
  tagline: 读取 Stellar Blade PC 存档，快速发现尚未收集的服装、记录、罐子、营地等内容。
  image:
    src: /screenshots/overview.webp
    alt: sblade-explorer 汇总页：目录进度、图鉴进度与分类汇总
  actions:
    - theme: brand
      text: 下载最新版
      link: https://github.com/perillaroc/sblade-explorer/releases/latest
    - theme: alt
      text: 开始使用
      link: /guide/getting-started
    - theme: alt
      text: GitHub
      link: https://github.com/perillaroc/sblade-explorer

features:
  - icon: 🔒
    title: 只读安全
    details: 不修改存档文件
  - icon: ✈️
    title: 完全离线
    details: 解析和统计均在本机完成
  - icon: 🔍
    title: 自动发现
    details: 自动寻找 Steam 游戏存档
  - icon: 📦
    title: 收集分析
    details: 覆盖主要收集物和游戏图鉴
  - icon: 🔁
    title: 周目矩阵
    details: 支持 NG+ / NG++ 内容分析
  - icon: 🌐
    title: 双语数据
    details: 界面与物品名称支持中文 / English
---

## 下载

从 [GitHub Releases](https://github.com/perillaroc/sblade-explorer/releases/latest) 获取桌面应用便携版与
`sbsave` 命令行工具（Windows 10/11，需要 WebView2 运行时）；安装与升级说明见
[下载与安装](./guide/installation.md)。

## 文档

- [用户指南](./guide/getting-started.md)：安装、存档位置、收集度分析、导出报告与 FAQ
- [CLI](./cli/index.md)：`sbsave` 命令与参数
- [数据说明](./reference/data.md)：数据来源、目录库与别名模型
- [开发](./development/index.md)：开发环境、数据管线、CI 与发布
- [English documentation](/en/)：the same documentation in English

项目源码、问题反馈与全部版本见 [GitHub 仓库](https://github.com/perillaroc/sblade-explorer)。
