# 文档现状盘点（在线文档建设 D01）

> 本文件是「sblade-explorer 在线文档建设任务方案」**D01（现状盘点）**的产出物，只记录盘点结果，
> 不代表任何迁移或建设已经完成。
>
> 盘点基线：`main` 分支提交 `872c25b`；范围为仓库内全部 Markdown（`git ls-files "*.md"`），共 9 个文件。

## 1. 盘点方法

- 逐个阅读仓库内全部 Markdown 文件，记录面向对象、主要章节与内容要点；
- 核对文件之间的相互引用与内容重叠；
- 确认当前是否已存在在线文档基础设施（VitePress / Netlify / 站点目录）。

当前基础设施状态（均为实测，供 D02 起参考）：

- 不存在 `docs/.vitepress/`、`docs/package.json`、`docs/index.md`；
- 不存在 `docs/guide/`、`docs/reference/`、`docs/development/` 目录；
- 不存在 `netlify.toml`，仓库中没有任何 `vitepress` / `netlify` 配置或引用；
- `.github/workflows/` 只有 `ci.yml`、`release.yml`（Rust + 前端 + 发布），没有文档构建作业。

## 2. Markdown 清单

| 当前文档 | 行数 | 面向对象 | 当前内容 | 未来位置 |
| --- | --- | --- | --- | --- |
| `README.md` | 83 | 用户（GitHub 访客 / 下载用户） | 项目定位、下载与使用（发布物表、系统要求、存档位置）、主要功能、统计口径、数据来源、已知限制、文档索引、协议与免责声明 | 保留在仓库根 README，D05 精简为 Landing Page；用户详解迁 D04 `guide/` |
| `AGENTS.md` | 98 | AI Agent / 贡献者 | 仓库与目录结构、环境与命令、CI 与发布、目录库与别名模型、Git 提交规范、约定与陷阱 | 保留仓库根，**不进入文档站**（协作规范，非用户/开发手册） |
| `docs/cli.md` | 62 | CLI 用户 | `sbsave` 获取方式、`saves` / `report` / `dump` / `catalog` 命令与示例、参数表、JSON 契约说明 | `docs/cli/index.md`（D03，平移不大改写） |
| `docs/data.md` | 102 | 开发者（兼高级用户） | 数据来源、目录库与别名模型、图鉴（AlbumTable）、自定义目录库、中文攻略链接、回归哨兵 | `docs/reference/data.md`（D03） |
| `docs/development.md` | 107 | 开发者 | 环境要求、常用命令、项目结构、数据管线、测试与验收、CI 与发布、布局不变量 | `docs/development/index.md`（D03） |
| `apps/desktop/README.md` | 33 | 桌面应用开发者 | Tauri 2 + Vue 3 结构（`src/`、`src-tauri/`）、常用命令、相关文档链接 | 保留原位（桌面开发者入口）；D03 在站点 `development/` 中体现并互链 |
| `data/raw/README.md` | 91 | 数据维护者 / 开发者 | 原始数据目录结构与逐项说明、数据来源、`guides.json` 规则、重新生成目录库、刷新 name_map、刷新 API 数据 | 保留原位（数据来源与刷新说明的事实层）；站点 `reference/` 按需引用或摘要 |
| `data/raw/reference/README.md` | 65 | 第三方资料（快照） | stellar-blade-macos-save-editor 项目 README 快照：功能、下载、存档位置、构建、安全设计 | 保留原位，**不入站、不改写**（第三方参考数据来源） |
| `data/raw/reference/使用说明.md` | 83 | 第三方资料（快照） | 同上项目 v0.8 使用说明快照：使用方法、如何找到存档、安全机制 | 保留原位，**不入站、不改写**（第三方参考数据来源） |

另有本文件 `docs/documentation-inventory.md`（D01 新增）：盘点产物，D03 之后信息即过时，
可移入 `docs/development/` 归档或在 D13「最终整理」时删除。

## 3. 验收问题回答

### 3.1 有哪些 Markdown

共 9 个（不含本文件），分四类：

- **站点候选（6）**：`README.md`、`docs/cli.md`、`docs/data.md`、`docs/development.md`、
  `apps/desktop/README.md`、`data/raw/README.md`；
- **协作规范（1）**：`AGENTS.md`；
- **第三方资料快照（2）**：`data/raw/reference/README.md`、`data/raw/reference/使用说明.md`；
- 没有 `CHANGELOG.md`、`CONTRIBUTING.md`、`LICENSE.md` 等其它 Markdown（License 为纯文本 `LICENSE`）。

### 3.2 是否内容重复

存在多处重复，但属于「同一事实、不同颗粒度」的自然重叠，没有相互矛盾的表述：

| 重复主题 | 出现位置 | 说明与建议 |
| --- | --- | --- |
| 数据来源清单 | `README.md`、`docs/data.md`、`data/raw/README.md` | 三处均列来源：README 最简（外链）、`data.md` 中等（带 `data/raw` 路径）、`raw/README.md` 最全（逐文件+刷新）。README 应缩减为摘要并指向站点 |
| 目录库数量 / 统计口径 | `README.md`（统计口径段）、`docs/data.md`（回归哨兵）、`AGENTS.md`（哨兵数量） | 932 条 / 15 分类、图鉴 122、纳米战衣 126 等数字多处出现。站点版只保留 `reference/` 一处 |
| 环境与常用命令 | `docs/development.md`、`AGENTS.md`、`apps/desktop/README.md` | 三处命令块高度相似（cargo / pnpm）。站点以 `development/` 为准 |
| 项目结构 | `docs/development.md`、`AGENTS.md` | 目录树与一句话说明接近完全重复 |
| CI 与发布 | `docs/development.md`、`AGENTS.md` | 工作流、发版流程、Windows 作业原因近乎一致 |
| 数据管线 | `docs/development.md`、`data/raw/README.md`、`AGENTS.md` | `catalog build` / `mine-names` 步骤在 3 处说明 |
| 中文攻略链接规则 | `docs/data.md`（概述+桌面端行为）、`data/raw/README.md`（完整规则） | 后者更细，属「摘要 vs 明细」 |
| 自定义目录库 | `docs/data.md`（含 JSON 示例）、`AGENTS.md`（一句）、`data/raw/README.md`（一句） | `data.md` 为主，其余为提示 |
| CLI 获取方式 | `README.md`（发布物表）、`docs/cli.md`（开头） | 小范围重复，可接受 |

### 3.3 哪些属于用户文档

现状**没有独立的用户指南页面**，用户文档全部集中在 README：

- `README.md`：下载与使用、存档位置、主要功能、已知限制（当前唯一的用户文档主体）；
- `docs/cli.md`：面向 CLI 用户（兼具用户文档与参考手册属性）；
- `docs/data.md` 的「自定义目录库」「中文攻略链接」：偏高级用户；
- 尚不存在 `guide/getting-started`、`installation`、`save-location`、`collections`、`export`、
  `faq` 等页面（D04 创建；素材应来自 README 与现有 docs，不得编造 UI 行为）。

### 3.4 哪些属于开发者文档

- `docs/development.md`：开发环境、项目结构、数据管线、测试、CI 与发布、布局不变量；
- `apps/desktop/README.md`：桌面应用结构与开发命令；
- `data/raw/README.md`：数据维护与刷新（数据来源的事实层说明）；
- `docs/data.md`：目录库 / 别名模型 / 图鉴 / 回归哨兵（开发参考）；
- `AGENTS.md`：面向 AI Agent 与贡献者的协作规范（不是站点内容）。

### 3.5 哪些内容应该继续保留在 README

按方案 3.3「README 与在线文档职责分离」，README 应保留：

- 项目名称与 Badges、Vibe Coding 声明；
- 一句话简介（读取存档、报告未收集、只读离线）；
- 核心特性摘要（短列表）；
- Release 下载入口、发布物表（桌面 / CLI）与基本系统要求（Windows 10/11、WebView2）；
- 最基本使用/安装方法（解压运行，CLI 只给指引链接）；
- 存档位置表（快速开始的一部分，详细版日后放 `guide/save-location`）；
- 数据来源摘要（几行，指向站点 `reference/`）；
- 已知限制（用户决策需要）；
- 在线文档入口 `https://sblade-explorer.perillaroc.wang/`（D05 接入）；
- 开发入口（站点 `development/` 与仓库文档）；
- License 与免责声明。

应迁移或缩减到站点（D04 / D05 处理）：

- 主要功能的逐条长描述 → `guide/`；
- 完整 CLI 参数 → `cli/`；
- 完整数据模型、图鉴、自定义目录库、回归哨兵 → `reference/`；
- 完整开发流程、CI 与发布细节 → `development/`；
- 原始数据刷新方式 → 站点引用 `data/raw/README.md` 或 `reference/`。

## 4. 与后续任务的对应关系

| 现有文档 | 目标（D03 / D04 / D05） | 迁移原则 |
| --- | --- | --- |
| `docs/cli.md` | `docs/cli/index.md` | 平移优先，只调整标题、内部链接、导航 |
| `docs/data.md` | `docs/reference/data.md` | 平移优先，保留数据结构与示例 |
| `docs/development.md` | `docs/development/index.md` | 平移优先，保留命令与 CI 说明 |
| `README.md` | 仓库根 README（精简版） | D04 先产出 `guide/`，D05 再缩减 README |
| `apps/desktop/README.md` | 原位保留 + 站点 `development/` 引用 | 桌面开发细节留在仓库 |
| `data/raw/README.md` | 原位保留 + 站点 `reference/` 引用 | 刷新步骤依赖本机游戏/工具，适合留在仓库 |
| `AGENTS.md` | 原位保留，不入站 | Agent 协作规范 |
| `data/raw/reference/*.md` | 原位保留，不入站 | 第三方项目资料快照 |
| `docs/documentation-inventory.md` | 本文件，D03 后过时 | 归档至 `development/` 或 D13 清理 |

## 5. 盘点结论

- 现有文档数量少、结构扁平：所有内容集中在 README + `docs/` 三篇，无导航、无搜索、无在线入口，
  符合建设在线文档站的立项前提。
- 内容总体准确、无相互矛盾；主要问题是**职责混用**（README 同时承担 Landing Page、用户手册、
  数据参考与开发入口）与**跨文件重复**。
- 用户指南（`guide/`）是当前最大的内容缺口，但其素材已存在于 README、`docs/cli.md` 与
  `docs/data.md`，D04 无需编造功能即可编写。
- D03 的三篇迁移对象（`docs/cli.md`、`docs/data.md`、`docs/development.md`）内容自洽、
  内部链接简单，适合按「平移优先、不大规模改写」执行。
