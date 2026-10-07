# AGENTS.md

读取 Steam《剑星》存档并报告缺失收集物的工具，已完成 **Python CLI → Rust + Tauri 桌面应用** 迁移：
主实现为 Rust workspace + Tauri 2 桌面应用；原 Python 参考实现已删除，代码历史保留在 git 中。
只读：绝不写入存档。默认输出为中文，桌面 UI 与 CLI 支持中英切换（见「多语言」）；代码标识符/注释用英文。

## 仓库与目录结构

- `crates/sbsave-core/` - Rust 核心库：`gvas`（GVAS/EVAS 解析）、`savegame`（存档探测与提取）、`catalog`（目录库）、`analyze`（差集与 NG+/DLC 逻辑）、`report`（JSON 契约/Markdown/控制台）。
- `crates/sbsave-cli/` - clap CLI（二进制 `sbsave`），命令与输出沿用原 Python 版约定。
- `crates/sbsave-tools/` - 构建期数据管线（二进制 `sbsave-tools`）：`catalog build` 由 `data/raw` 生成 `data/catalog.json`；`mine-names` 从游戏数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）+ `Game.locres` 提取内部别名→官方名称（未直接关联的营地经 `camp_alias_rows.json` 对照行号），生成 `data/raw/game/name_map.json`（需本机游戏与 cue4parse/repak，dump 目录不入库）。
- `apps/desktop/` - Tauri 2 桌面应用：前端 Vue 3 + Vite + TypeScript + Tailwind CSS v4 在工程根，Rust 在 `src-tauri/`。
- `data/raw/` - 已提交的数据快照（`api/`、`api/i18n/` 手工中文翻译、`crosswalk.json` 手工别名映射、`universe/aliases.json`、`game/name_map.json`、`reference/`）。运行时绝不联网。
- `data/catalog.json` - 由 `sbsave-tools catalog build` 生成的目录库；禁止手改。
- 根 `README.md` - 面向用户的总说明（精简版：下载与使用、功能、数据来源、限制）；英文版 `README.en.md` 与之同步，两份顶部语言互链，英文版文档链接指向 `/en/` 页面。
- `docs/cli/index.md`、`docs/reference/data.md`、`docs/development/index.md` - CLI、数据与目录库、开发与发布的详细说明；`apps/desktop/README.md` - 桌面应用开发说明。

## 环境与命令

```powershell
# Rust（仓库根目录）
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo run -p sbsave-cli -- --version

# 前端 / 桌面（apps/desktop）
pnpm install
pnpm dev                 # 仅前端
pnpm build               # vue-tsc --noEmit + vite build
pnpm test                # vitest：语言包 key 对齐与插值
pnpm tauri dev           # 桌面应用开发
pnpm tauri build         # Windows 打包（输出 target/release/bundle/{nsis,msi}）

# 数据管线（仓库根目录）
cargo run -p sbsave-tools -- catalog build   # 重新生成 data/catalog.json
cargo test -p sbsave-tools                   # 生成结果与已提交文件逐字节比对
```

修改目录库数据源后的顺序：先 `cargo run -p sbsave-tools -- catalog build`，再用
`cargo run -p sbsave-cli -- catalog check` 校验生成结果。
工具体经 `CARGO_MANIFEST_DIR` 定位仓库根，可在任意目录运行；输出按平台换行（Windows CRLF）以复刻原 Python 工具。

迁移验收结论（2026-09-14）：与原 Python 版 `report --json` 逐节点一致、Markdown 归一化后逐字一致；
Python 参考实现已于 2026-09-15 删除，代码历史保留在 git 中。

## CI 与发布（GitHub Actions）

- `.github/workflows/ci.yml`：push `main` / PR / 手动 —— Rust 作业跑在 **windows-latest**（必须：`sbsave-tools`
  的集成测试对 `data/catalog.json` 逐字节比对，生成结果按平台换行），执行 fmt / clippy `-D warnings` /
  `cargo test` / CLI 冒烟；前端作业与文档作业跑在 ubuntu，各自执行 `pnpm install --frozen-lockfile` +
  `pnpm build`（文档只校验构建，部署由 Netlify 负责）。
  声明了 `workflow_call` 供发布复用。
- `.github/workflows/release.yml`：push tag `v*` 或手动 —— 先复用 `ci.yml` 把关，再校验 tag 与
  `tauri.conf.json` 版本一致（不一致直接失败），然后 `pnpm tauri build --no-bundle` 出便携版 exe、
  `cargo build --release -p sbsave-cli` 出 CLI，两者各打包 zip，用 `gh release create --draft`
  创建 **draft** Release（未使用 tauri-action，也未接入自动升级）。
- 发版：`cd apps/desktop && pnpm bump <x.y.z>`（同步 tauri.conf.json / package.json / Cargo.toml，并刷新
  Cargo.lock）→ 提交 → `git tag v<x.y.z>` → `git push origin main v<x.y.z>` → 复核 draft 后 Publish。
  标签沿用轻量标签（与 v0.1.0 / v0.2.0 一致）；`--follow-tags` 只推附注标签，轻量标签必须显式推送。
- 仓库公开（`perillaroc/sblade-explorer`），Actions 免费；发布不需要额外 secret，只用 `GITHUB_TOKEN`。
- **布局不变量**：`crates/` 必须在仓库根下（`sbsave-tools::repo_root()` 上溯两级）；`data/catalog.json`
  必须在 `crates/sbsave-core` 上溯三级（`include_str!` 编译期嵌入）；`apps/desktop/src-tauri` 必须列在
  workspace `members` 中（tauri-action 依此解析 workspace 的 `target/`）。改动目录结构前先同步这些约束。

## 目录库 / 别名模型

- 只要物品的任一 `aliases` 出现在存档物品集合、从成就记录派生的别名或成就别名（图鉴）中，即视为"已获得"。
- `data/raw/crosswalk.json`（site id -> 别名 -> 中文名，手工维护）是收集品的事实来源；`data/catalog.json` 为生成物；图鉴两类（`naytiba` 孽奇拔 67、`characters` 角色 55 页）由 `AlbumTable` + `Game.locres` 挖掘生成，别名是 `Ach_Album_Unlock_*` 成就别名。只想修正个别条目时，用 `%LOCALAPPDATA%\sbsave\catalog.user.json`（合并覆盖内置目录库）或 CLI 的 `--catalog path.json`，无需重新生成。
- 目录库分两段：`section=collection`（13 类收集品，计入目录进度）与 `section=album`（图鉴，**不计入目录进度**，报告/UI 单列）；角色按页展示（`艾德姆（资料 3/5）`），图鉴条目带官方 `desc_zh`/`desc_en` 说明。
- 记录/密码/罐子/设计图案/外观名称由 `data/raw/game/name_map.json`（游戏数据挖掘）覆盖为官方简中名称并置 `confidence="high"`；仅埃多斯9号 6 座营地的别名→行号仍为顺序推定，保留低置信度并在报告中标记 映射待确认。期望数量（126 纳米战衣、49 罐子、89 营地、67 孽奇拔、55 角色页）是回归哨兵，迁移时应保留断言。
- 分类参数（CLI `--category` 与 UI 筛选）同时接受分类键（`nano_suits`）和中文名（`纳米战衣`）。
- 桌面端图鉴展示约定：孽奇拔按五种图鉴类型分组（小兵/战士/精锐/阿尔法/上古，可折叠 + 组内进度），角色按页平铺（`艾德姆（资料 3/5）`）；图鉴官方说明在详情弹窗「图鉴说明」中显示（`zh|en|both`）。
- 攻略文案翻译在 `data/raw/api/i18n/`；`sbsave-tools catalog build` 合并为 `area_zh`/`location_zh`/`obtain_zh`；语言模式 `zh|en|both`（默认 zh，缺翻译回退英文），JSON 始终双语。
- 分类英文名与记录类型英文名由 `crates/sbsave-tools/src/catalog_build.rs` 的 `CATEGORIES` / `RECORD_TYPES` 维护，写入 `data/catalog.json` 的 `categories[].name_en` 与条目的 `record_type_en`；analysis JSON 同步输出 `name_en`/`record_type_en`（附加字段）。

## 多语言

- 界面语言 `uiLocale`（`zh`/`en`）与内容语言 `contentLang`（`zh`/`en`/`both`）分离：前者控制菜单、弹窗、错误与窗口标题，后者控制收集物名称/位置/说明（顶部语言按钮）。
- 文案统一用「界面语言 / 内容语言」二分：顶部控件显示「内容」（`contentShort`）+ 完整 tooltip（`contentHint`），设置内为「界面语言」并在 `interfaceHint` 中说明顶部按钮；两处 hint 必须互相指向，避免出现第二个「语言」控件时产生歧义。
- 前端用 vue-i18n：消息在 `apps/desktop/src/locales/zh.ts`（`MessageSchema` 事实来源）与 `en.ts`（`satisfies MessageSchema`，缺 key 时 `pnpm build` 失败）；`src/lib/i18n.ts` 负责初始化、`<html lang>` 与窗口标题同步；组件内一律用 `t("...")`，禁止硬编码用户可见文案；lib 模块用 `translate(...)`。`settings.uiLocale`/`settings.contentLang` 持久化在 localStorage（`sbsave.settings.v1`），首次按系统语言推断界面语言。
- Rust 侧文案集中在 `crates/sbsave-core/src/i18n.rs`（`Locale` + `Messages`），供 CLI、报告与 Tauri 命令共用；报告 chrome 用 `Locale`、物品文本用 `lang`，两参数保持分离。
- CLI 用全局 `--ui-lang zh|en` 控制 chrome（默认读系统语言，兜底 zh），`--lang` 仍是内容语言；clap 帮助在解析前按 locale 替换，clap 内建报错保持英文。Tauri 命令新增可选 `locale` 参数（前端传 `settings.uiLocale`）。
- 新增界面文案必须同时补 `zh.ts` 与 `en.ts`；改动用户可见的 CLI/报告文案时同步更新 `sbsave-core` 测试与 `crates/sbsave-cli/tests/test_cli.rs`。
- Web 文档（VitePress）同样双语：中文在 `docs/` 根，英文在 `docs/en/` 对应路径；站点配置在 `docs/.vitepress/config.ts`，英文导航/侧栏/页脚放在**顶层 `locales.en.themeConfig`**（VitePress 实际读取 `locales[lang].themeConfig`，不是 `themeConfig.locales`）。改中文页必须同步 `docs/en/` 对应页面，英文页内部链接使用 `/en/...` 或同目录相对路径，界面元素名称与 `apps/desktop/src/locales/en.ts` 保持一致；`documentation-inventory.md` 为内部文档，不翻译也不发布。

## Git 提交规范

提交信息用中文：`:gitmoji:` + 一行标题；多行列表与收尾说明均为可选，列表用于补充细节（简单改动可只写标题）。

- gitmoji 用文本短码（如 `:wrench:`、`:bug:`、`:sparkles:`），不要粘贴 emoji 字符；标题与列表项（如有）均保持单行，不按宽度折行。
- **不要自动提交**：Agent 完成后停在「工作区已改动 + 本地关卡通过」，由用户检查后再决定是否提交；除非明确要求，不执行 `git commit` / `git push`。
- 示例：

  ```text
  :wrench: 桌面开发服务器改用 127.0.0.1:51430

  - vite.config.ts：server.port 1420 → 51430，server.host 默认绑定 127.0.0.1，HMR ws 端口同步改为 51431
  - tauri.conf.json：devUrl 改为 http://127.0.0.1:51430，与 Vite 的 host/port 一致
  ```

## 约定与陷阱

- 无 pre-commit、无 `opencode.json`；关卡 = 本地 `cargo fmt/clippy/test` + 前端 `pnpm build`，CI 同款见
  `.github/workflows/ci.yml`（Rust 作业固定 Windows，原因见「CI 与发布」）。
- GVAS 解析器保持只读与防御性：无法解析的 struct/array/map/set 回退为 `RawValue`，reader 始终重同步到 `tag_start + size`。
- JSON 契约冻结为原 Python 版 `report.py::analysis_to_dict`（`save`/`summary`/`categories`/`unmapped_obtained`，条目含中英两套字段），供 CLI、UI 与导出共用。
- CLI/报告默认输出中文（CLI 用 `--ui-lang`、报告物品文本用 `--lang` 切换）且被测试断言 - 保持消息稳定，或同步更新测试。
- 绝不修改存档文件；需要测试解析时使用合成档案（逐字节构造，沿用原 Python 测试用例）。
- `crates/sbsave-tools` 的集成测试断言 `cargo run -p sbsave-tools -- catalog build` 的结果与已提交 `data/catalog.json` 逐字节一致；改动数据源或生成逻辑后必须重新生成并让该测试通过。
- 行为约定以迁移时的决策为准；原 Python 实现可从 git 历史（`33cb981`）查回。
- `SBSAVE_SAVE_DIRS`（分号分隔）仅供测试：设置后替换默认存档探测目录，便于验证「未找到存档」界面；见 `docs/development/index.md`。
- 文档与数据来源说明在 `README.md`、`docs/`（cli/reference/development）与 `data/raw/README.md`。
