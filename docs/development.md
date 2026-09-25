# 开发

## 环境要求

- Rust（stable 工具链）与 Cargo
- Node.js + pnpm（前端 / 桌面）
- Windows 打包需 WebView2（Windows 10/11 通常已安装）

## 常用命令

```powershell
# Rust（仓库根目录）
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
cargo run -p sbsave-cli -- --version

# 前端 / 桌面（apps/desktop）
pnpm install
pnpm dev            # 仅前端
pnpm build          # vue-tsc --noEmit + vite build
pnpm tauri dev      # 桌面应用开发
pnpm tauri build    # Windows 安装包（NSIS + MSI）
```

## 项目结构

```text
crates/sbsave-core/    # Rust 核心库：GVAS/EVAS 解析、存档提取、目录库、差集分析、报告渲染
crates/sbsave-cli/     # clap CLI（二进制 sbsave），命令与输出沿用原 Python 版约定
crates/sbsave-tools/   # 构建期数据管线（二进制 sbsave-tools）：catalog build / mine-names
apps/desktop/          # Tauri 2 桌面应用：前端 Vue 3 + Vite + TS + Tailwind CSS v4（工程根）
apps/desktop/src-tauri # Tauri shell：list_saves / analyze_save / export_report 三个 command
data/raw/              # 已提交的数据快照（含手工维护的 crosswalk.json）；运行时绝不联网
data/catalog.json      # 由 sbsave-tools catalog build 生成的目录库；禁止手改
```

`crates/sbsave-core` 的模块划分：`gvas`（GVAS/EVAS 解析）、`savegame`（存档探测与提取）、
`catalog`（目录库）、`analyze`（差集与 NG+/DLC 逻辑）、`report`（JSON 契约/Markdown/控制台）。
桌面应用自身的结构与命令见 [../apps/desktop/README.md](../apps/desktop/README.md)。

## 数据管线

```powershell
# 重新生成 data/catalog.json
cargo run -p sbsave-tools -- catalog build
cargo run -p sbsave-cli -- catalog check

# 集成测试：生成结果与已提交文件逐字节比对
cargo test -p sbsave-tools
```

修改目录库数据源后的顺序：先 `catalog build`，再 `catalog check`。人工别名映射在
`data/raw/crosswalk.json`（唯一事实来源）；攻略文案翻译在 `data/raw/api/i18n/`。

刷新游戏名称映射（需要本机安装游戏与 `cue4parse.exe`/`repak.exe`，详见 `data/raw/README.md`）：

```powershell
cargo run -p sbsave-tools -- mine-names `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

## 测试与验收

- 测试使用逐字节构造的合成档案，绝不修改真实存档。
- GVAS 解析器保持只读与防御性：无法解析的 struct/array/map/set 回退为 `RawValue`，
  reader 始终重同步到 `tag_start + size`。
- 迁移验收（2026-09-14）：与原 Python 版 `report --json` 逐节点一致、Markdown 归一化后逐字一致；
  Python 参考实现已于 2026-09-15 删除，代码历史保留在 repo git（提交 `33cb981`）。
- 目录库回归哨兵见 [data.md](data.md#回归哨兵)。

## CI 与发布

GitHub Actions 两个工作流（`.github/workflows/`）：

| 工作流 | 触发 | 内容 |
| --- | --- | --- |
| `ci.yml` | push `main` / PR / 手动 | Rust 作业（**windows-latest**）：`cargo fmt --all --check`、`clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、CLI 冒烟；前端作业（ubuntu）：`pnpm install --frozen-lockfile` + `pnpm build` |
| `release.yml` | push tag `v*` / 手动 | 先复用 `ci.yml` 全量把关，再校验 tag 与 `tauri.conf.json` 版本一致，构建桌面便携版 zip 与 CLI zip，用 `gh release create` 创建 **draft** Release；人工复核后 Publish |

发布流程：

```powershell
cd apps/desktop
pnpm bump 0.2.0        # 同步 tauri.conf.json / package.json / Cargo.toml，并刷新 Cargo.lock
cd ..\..
git commit -am ":bookmark: 发布 v0.2.0"
git tag v0.2.0
git push --follow-tags
```

`release.yml` 会校验 tag 与 `tauri.conf.json` 的版本一致，不一致直接失败。

> Rust 作业必须跑 Windows：`sbsave-tools` 的集成测试对 `data/catalog.json` 做逐字节比对，
> 而生成结果按平台换行（Windows CRLF）。`.gitattributes` 已把该文件的换行固定为 CRLF。

## 布局不变量

调整目录结构前必读：

- `crates/` 必须在仓库根下（`sbsave-tools::repo_root()` 上溯两级）；
- `data/catalog.json` 必须在 `crates/sbsave-core` 上溯三级（`include_str!` 编译期嵌入）；
- `apps/desktop/src-tauri` 必须列在 workspace `members` 中（tauri-action 依此解析 workspace 的 `target/`）。
