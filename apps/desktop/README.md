# 桌面应用（Tauri 2 + Vue 3）

`sblade-explorer` 是项目的主要发布物：Tauri 2 外壳 + Vue 3 前端，Rust 侧全部调用 `sbsave-core`。

## 结构

- `src/` —— Vue 3 前端（Vite + TypeScript + Tailwind CSS v4）：
  - `components/` —— 应用骨架（`AppSidebar` 可折叠导航、`AppTopbar` 顶栏、`SaveCard` 存档卡）、
    汇总页、分类页、周目矩阵、详情弹窗、关于/设置对话框等
  - `components/ui/` —— 基础组件层（F1）：按钮 / 分段 / 输入 / 卡片 / 徽章 / 进度条 / BaseDialog /
    Toast / Tabs / 表格 / 空态 / Tooltip / Popover；统一消费 `styles.css` 的设计令牌
    （`bg-surface`、`text-ink` 等），页面与弹窗重做时优先复用（见 `components/ui/index.ts`）
  - `lib/` —— 前端辅助逻辑（展示、区域、矩阵、状态、攻略链接、设置、主题、i18n、视口等）；
    `lib/viewport.ts` 提供 1100px 窄窗响应式状态；`lib/toast.ts` 提供 Toast 队列
    （`showToast`，由 `UiToastHost` 渲染，F4 起替换 `App.vue` 内联 notice）
  - `locales/zh.ts` / `locales/en.ts` —— vue-i18n 语言包（`zh.ts` 是 `MessageSchema` 事实来源，
    `en.ts` 用 `satisfies` 约束；新增文案必须同时补两份）
  - `lib/i18n.ts` —— i18n 初始化、`<html lang>` 与窗口标题同步；`settings.uiLocale`（界面语言）
    与 `settings.contentLang`（内容语言）相互独立并持久化；`settings.sidebarCollapsed` 记住侧栏折叠态
  - `assets/app-icon.svg` —— 应用图标 master，`pnpm icon` 由它生成 `src-tauri/icons/`
    （造型、许可与重新生成说明见 `src-tauri/icons/README.md`）
  - `App.vue` / `main.ts` / `types.ts` / `styles.css`
- `src-tauri/` —— Tauri shell：
  - `src/lib.rs` —— command：`list_saves` / `save_sources` / `inspect_save` / `open_save_dir` /
    `analyze_save` / `export_report` / `guide_links` / `list_browsers`（从注册表读取本机已安装浏览器）
  - `capabilities/default.json` —— opener 链接白名单（`app: true` 允许用设置中选择的浏览器打开）
  - `tauri.conf.json` —— 应用标识、窗口与打包配置（版本号由 `pnpm bump` 同步）
- `scripts/bump-version.mjs` —— 发版时同步版本号

## 常用命令

```powershell
pnpm install
pnpm dev                       # 仅前端（Vite）
pnpm build                     # vue-tsc --noEmit + vite build
pnpm test                      # vitest：语言包 key 对齐、插值与文案
pnpm icon                      # 由 src/assets/app-icon.svg 重新生成 src-tauri/icons/
pnpm tauri dev                 # 桌面应用开发
pnpm tauri build --no-bundle   # 只出便携版 exe：target/release/sblade-explorer.exe
pnpm tauri build               # NSIS/MSI 安装包：target/release/bundle/{nsis,msi}/
```

## 安全（CSP）

`tauri.conf.json` 的 `app.security.csp` 限制 WebView 可加载的资源。该策略只在 release/便携版生效
（由自定义协议响应头下发；`pnpm tauri dev` 直接加载 Vite 服务器，不套用策略），当前为：

```text
default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self' ipc: http://ipc.localhost; object-src 'none'; base-uri 'self'; frame-ancestors 'none'; form-action 'none'
```

盘点依据（改动前端依赖或构建配置前请重新核对）：

- Vite 产物：`dist/index.html` 只引用外部 JS/CSS，无内联脚本/样式；生产包无 `eval`/`new Function`
  （不需要 `'unsafe-eval'`），Vue `:style` 走 CSSOM（不需要 `style-src 'unsafe-inline'`）；
- Tailwind CSS v4：生产构建输出到同一个外部 CSS 文件；dev 模式 Vite 会注入 `<style>`，但该模式不套用 CSP；
- Lucide 图标：`@lucide/vue` 打包为内联 SVG 组件；应用图标 `app-icon.svg` 被 Vite 内联为 `data:` URL，
  因此 `img-src` 需放行 `data:`；
- 截图与字体：界面不展示远程截图（攻略链接经 opener 插件交给系统浏览器），无 `@font-face`，只用系统字体栈；
- IPC：`@tauri-apps/api` 向 `http://ipc.localhost`（其他平台为 `ipc:`）发请求，`connect-src` 需放行这两个来源。

哨兵：`src-tauri/src/lib.rs` 的 `csp_locks_down_remote_content_and_allows_ipc` 校验上述指令齐全、
且未放行 `'unsafe-inline'`/`'unsafe-eval'`。

## 相关文档

- 开发环境、CI 与发布、布局不变量：[../../docs/development/index.md](../../docs/development/index.md)
- 数据来源与目录库：[../../docs/reference/data.md](../../docs/reference/data.md)
- CLI：[../../docs/cli/index.md](../../docs/cli/index.md)
