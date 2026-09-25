# 桌面应用（Tauri 2 + Vue 3）

`sblade-explorer` 是项目的主要发布物：Tauri 2 外壳 + Vue 3 前端，Rust 侧全部调用 `sbsave-core`。

## 结构

- `src/` —— Vue 3 前端（Vite + TypeScript + Tailwind CSS v4）：
  - `components/` —— 汇总页、分类页、周目矩阵、详情弹窗、存档选择、关于对话框等
  - `lib/` —— 前端辅助逻辑（展示、区域、矩阵、状态、攻略链接等）
  - `App.vue` / `main.ts` / `types.ts` / `styles.css`
- `src-tauri/` —— Tauri shell：
  - `src/lib.rs` —— 三个 command：`list_saves` / `analyze_save` / `export_report`
  - `tauri.conf.json` —— 应用标识、窗口与打包配置（版本号由 `pnpm bump` 同步）
- `scripts/bump-version.mjs` —— 发版时同步版本号

## 常用命令

```powershell
pnpm install
pnpm dev                       # 仅前端（Vite）
pnpm build                     # vue-tsc --noEmit + vite build
pnpm tauri dev                 # 桌面应用开发
pnpm tauri build --no-bundle   # 只出便携版 exe：target/release/sblade-explorer.exe
pnpm tauri build               # NSIS/MSI 安装包：target/release/bundle/{nsis,msi}/
```

## 相关文档

- 开发环境、CI 与发布、布局不变量：[../../docs/development.md](../../docs/development.md)
- 数据来源与目录库：[../../docs/data.md](../../docs/data.md)
- CLI：[../../docs/cli.md](../../docs/cli.md)
