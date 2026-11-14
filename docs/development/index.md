# 开发

## 环境要求

- Rust（stable 工具链）和 Cargo；
- Node.js 和 pnpm（前端与桌面应用）；
- Windows 打包需要 WebView2（Windows 10/11 通常已安装）。

## 常用命令

```powershell
# Rust（仓库根目录）
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo run -p sbsave-cli -- --version

# 前端 / 桌面应用（apps/desktop）
pnpm install
pnpm dev            # 仅前端
pnpm build          # vue-tsc --noEmit + vite build
pnpm test           # vitest：语言包 key 对齐与插值
pnpm tauri dev      # 桌面应用开发
pnpm tauri build    # Windows 安装包（NSIS + MSI）
```

## 项目结构

```text
crates/sbsave-core/    # Rust 核心库：GVAS/EVAS 解析、存档提取、目录库、差集分析、报告渲染
crates/sbsave-cli/     # clap CLI（二进制 sbsave），命令与输出沿用原 Python 版约定
crates/sbsave-tools/   # 构建期数据管线（二进制 sbsave-tools）：catalog build / mine-names
apps/desktop/          # Tauri 2 桌面应用：前端 Vue 3 + Vite + TS + Tailwind CSS v4（工程根）
apps/desktop/src-tauri # Tauri shell：list_saves / save_sources / inspect_save / open_save_dir / analyze_save / export_report / guide_links / list_browsers
data/raw/              # 已提交的数据快照（含手工维护的 crosswalk.json）；运行时绝不联网
data/catalog.json      # 由 sbsave-tools catalog build 生成的目录库；请勿手动修改
```

`crates/sbsave-core` 包含以下模块：`gvas`（GVAS/EVAS 解析）、`savegame`（存档查找与提取）、
`catalog`（目录库）、`analyze`（差集和 NG+/DLC 逻辑）、`report`（JSON 契约、Markdown 和控制台输出）。
桌面应用的结构和命令见 [apps/desktop/README.md](https://github.com/perillaroc/sblade-explorer/blob/main/apps/desktop/README.md)。

## 数据管线

```powershell
# 重新生成 data/catalog.json
cargo run -p sbsave-tools -- catalog build
cargo run -p sbsave-cli -- catalog check

# 集成测试：将生成结果与已提交文件逐字节比对
cargo test -p sbsave-tools
```

修改目录库数据源后，请先运行 `catalog build`，再运行 `catalog check`。
手工别名映射维护在 `data/raw/crosswalk.json`（收集品数据的唯一事实来源，包含营地与攻略站条目的对照）；
攻略文案翻译位于 `data/raw/api/i18n/`，鱼类文案位于 `data/raw/fish_obtain.json`。

数据健康检查（离线，不读取存档）：

```powershell
cargo run -p sbsave-tools -- doctor             # 输出 target/doctor.md 与 target/doctor.json
cargo run -p sbsave-tools -- doctor --strict    # 有新增未入库条目时退出码非 0
```

检查站点条目与目录库的双向差集、攻略链接、低置信度/重复别名/空获取文案、别名覆盖，
以及易错过规则和区域/任务标题。按设计不入库的材料与核心类由 `data/raw/doctor_allowlist.json`
和代码中的文件排除列表处理；CI 以非阻塞方式运行 `--strict`，便于先发现站点新增条目。

更新游戏名称映射前，需要在本机安装游戏，并准备 `cue4parse.exe` 和 `repak.exe`，详见 `data/raw/README.md`：

```powershell
cargo run -p sbsave-tools -- mine-names `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

## 测试与验收

- 测试使用逐字节构造的合成存档，绝不修改真实存档。
- GVAS 解析器采用只读、防御性设计：无法解析的 struct、array、map 或 set 回退为 `RawValue`，
  reader 始终重新同步到 `tag_start + size`。
- 迁移验收（2026-09-14）：与原 Python 版的 `report --json` 逐节点一致，Markdown 归一化后逐字一致。
  Python 参考实现已于 2026-09-15 删除，历史代码保留在 Git 仓库中（提交 `33cb981`）。
- 目录库回归检查基准见 [data.md](../reference/data.md#回归检查基准)。

## 测试技巧

本机已有存档时，可以通过 `SBSAVE_SAVE_DIRS`（多个目录用分号分隔）**替换**默认存档查找目录，
验证桌面应用的「未找到存档」界面。该变量对桌面应用和 CLI 均生效：

```powershell
$env:SBSAVE_SAVE_DIRS = "$env:TEMP\sbsave-empty"   # 分别使用存在和不存在的目录，可验证两种提示
cd apps/desktop
pnpm tauri dev
```

该变量仅供本地测试。程序会记住上次选择的存档，因此测试空状态前，请先在设置中点击「改为自动选择」或「恢复默认」。

## 文档站点

在线文档使用 VitePress 构建，源文件位于 `docs/`。中文为默认语言，英文页面位于 `docs/en/`。
本地预览和构建命令如下：

```powershell
cd docs
pnpm dev        # 本地预览（默认 http://localhost:5173）
pnpm build      # 构建；发现死链会直接失败（CI 与 Netlify 使用同一命令）
```

新增或修改页面时，必须同步更新两种语言：修改中文页面后，请更新 `docs/en/` 中的对应文件。
英文页面应使用与 `apps/desktop/src/locales/en.ts` 一致的界面术语，内部链接使用 `/en/...` 或同目录相对路径。
语言切换、导航、侧栏和页脚配置位于 `docs/.vitepress/config.ts` 顶层的 `locales[...].themeConfig` 中。

## CI 与发布

GitHub Actions 包含两个工作流（`.github/workflows/`）：

| 工作流 | 触发方式 | 内容 |
| --- | --- | --- |
| `ci.yml` | 推送到 `main` / PR / 手动 | Rust 作业（**windows-latest**）：`cargo fmt --all --check`、`clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、CLI 冒烟测试；前端作业（Ubuntu）：`pnpm install --frozen-lockfile` + `pnpm build` + `pnpm test`；文档作业（Ubuntu，`docs/`）：`pnpm install --frozen-lockfile` + `pnpm build`，构建 VitePress 站点，发现死链时失败；部署由 Netlify 负责 |
| `release.yml` | 推送 `v*` 标签 / 手动 | 先复用 `ci.yml` 完成全部检查，再校验标签是否与 `tauri.conf.json` 中的版本一致；随后构建桌面便携版 ZIP 和 CLI ZIP，使用 `gh release create` 创建 **draft** Release，人工复核后发布 |

发布流程：

```powershell
cd apps/desktop
pnpm bump 0.2.0        # 同步 tauri.conf.json / package.json / Cargo.toml，并刷新 Cargo.lock
cd ..\..
git commit -am ":bookmark: 发布 v0.2.0"
git tag v0.2.0
git push origin main v0.2.0
```

> 发布使用轻量标签（与 `v0.1.0`、`v0.2.0` 一致）。`git push --follow-tags` 不会推送轻量标签，因此必须在 push 命令中显式指定标签。
> 只有改用附注标签（`git tag -a v0.2.0 -m ":bookmark: 发布 v0.2.0"`）后，才能使用 `--follow-tags`。

`release.yml` 会校验标签与 `tauri.conf.json` 中的版本是否一致，不一致时直接失败。

版本号遵循 SemVer。`pnpm bump` 接受 `x.y.z` 和 `x.y.z-预发布号`（例如用于测试版的 `1.0.0-rc.1`），
不接受构建元数据（`+...`）。预发布版的流程相同，只需将标签写为 `v1.0.0-rc.1`。

> Rust 作业必须在 Windows 上运行：`sbsave-tools` 的集成测试会逐字节比对 `data/catalog.json`，
> 而生成结果使用平台默认换行符（Windows 为 CRLF）。`.gitattributes` 已将该文件的换行符固定为 CRLF。

## 布局约束

调整目录结构前，请务必遵守以下约束：

- `crates/` 必须位于仓库根目录下，因为 `sbsave-tools::repo_root()` 会上溯两级；
- `data/catalog.json` 必须位于 `crates/sbsave-core` 上溯三级的位置，以便通过 `include_str!` 在编译期嵌入；
- `apps/desktop/src-tauri` 必须列入 workspace 的 `members`，tauri-action 依赖该配置解析 workspace 的 `target/`。
