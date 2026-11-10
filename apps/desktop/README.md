# 桌面应用（Tauri 2 + Vue 3）

`sblade-explorer` 是项目的主要发布物，由 Tauri 2 外壳和 Vue 3 前端组成，Rust 侧统一调用 `sbsave-core`。

## 结构

- `src/` —— Vue 3 前端（Vite + TypeScript + Tailwind CSS v4）：
  - `components/` —— 应用骨架（`AppSidebar` 可折叠导航、`GlobalSearch` 全局搜索、`AppTopbar` 顶栏和 `SaveCard` 存档卡）、
    仪表盘汇总页、分类页（吸顶工具栏、矩阵/列表、记录标签页和图鉴分组）、周目矩阵、统一详情抽屉（`DetailDrawer`）、
    启动空状态引导以及关于/设置对话框；页面统一使用设计令牌；
  - `components/ui/` —— 基础组件层（F1）：按钮、分段控件、输入框、卡片、徽章、进度条、BaseDialog、
    Toast、Tabs、表格、空状态、Tooltip 和 Popover。统一使用 `styles.css` 中的设计令牌
    （`bg-surface`、`text-ink` 等）；重做页面或弹窗时优先复用，导出见 `components/ui/index.ts`；
  - `lib/` —— 前端辅助逻辑，涵盖展示、区域、矩阵、状态、攻略链接、设置、主题、i18n 和视口等；
    `lib/viewport.ts` 提供 1100px 窄窗口响应式状态；`lib/actionable.ts` 生成仪表盘中的「可优先收集」
    （未收集且没有阻塞原因）和易错过提示；`lib/search.ts` 提供跨分类的全局搜索分组；
    `lib/toast.ts` 提供 Toast 队列（`showToast`，由 `UiToastHost` 渲染，已取代 `App.vue` 中的内联 notice）；
  - `locales/zh.ts` / `locales/en.ts` —— vue-i18n 语言包。`zh.ts` 是 `MessageSchema` 的事实来源，
    `en.ts` 使用 `satisfies` 进行约束；新增文案时必须同步更新两份文件；
  - `lib/i18n.ts` —— 初始化 i18n，同步 `<html lang>` 和窗口标题；`settings.uiLocale`（界面语言）
    与 `settings.contentLang`（内容语言）相互独立并持久化保存；`settings.sidebarCollapsed` 记录侧栏折叠状态，
    `settings.categoryViews` 和 `settings.categoryFilters` 按分类保存视图和筛选条件（不保存搜索词）；
  - `assets/app-icon.svg` —— 应用图标源文件，`pnpm icon` 根据它生成 `src-tauri/icons/`
    （造型、许可和重新生成方法见 `src-tauri/icons/README.md`）；
  - `App.vue` / `main.ts` / `types.ts` / `styles.css`
- `src-tauri/` —— Tauri shell：
  - `src/lib.rs` —— 命令：`list_saves` / `save_sources` / `inspect_save` / `open_save_dir` /
    `analyze_save` / `export_report` / `guide_links` / `list_browsers`（从注册表读取本机已安装的浏览器）；
  - `capabilities/default.json` —— opener 链接白名单（`app: true` 允许使用设置中选择的浏览器打开链接）；
  - `tauri.conf.json` —— 应用标识、窗口和打包配置（版本号由 `pnpm bump` 同步）；
- `scripts/bump-version.mjs` —— 发布时同步版本号。

## 常用命令

```powershell
pnpm install
pnpm dev                       # 仅前端（Vite）
pnpm build                     # vue-tsc --noEmit + vite build
pnpm test                      # vitest：语言包 key 对齐、插值与文案
pnpm icon                      # 由 src/assets/app-icon.svg 重新生成 src-tauri/icons/
pnpm tauri dev                 # 桌面应用开发
pnpm tauri build --no-bundle   # 只生成便携版 exe：target/release/sblade-explorer.exe
pnpm tauri build               # NSIS/MSI 安装包：target/release/bundle/{nsis,msi}/
```

## 安全（CSP）

`tauri.conf.json` 中的 `app.security.csp` 限制 WebView 可以加载的资源。该策略仅在 release 和便携版中生效
（通过自定义协议响应头发送；`pnpm tauri dev` 直接加载 Vite 开发服务器，不应用此策略）。当前配置为：

```text
default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self' ipc: http://ipc.localhost; object-src 'none'; base-uri 'self'; frame-ancestors 'none'; form-action 'none'
```

配置依据如下（修改前端依赖或构建配置前，请重新核对）：

- Vite 产物：`dist/index.html` 只引用外部 JS/CSS，没有内联脚本或样式；生产包不使用 `eval` 或 `new Function`
  （无需 `'unsafe-eval'`），Vue `:style` 通过 CSSOM 生效（无需 `style-src 'unsafe-inline'`）；
- Tailwind CSS v4：生产构建输出同一个外部 CSS 文件；开发模式下 Vite 会注入 `<style>`，但该模式不应用 CSP；
- Lucide 图标：`@lucide/vue` 打包为内联 SVG 组件；应用图标 `app-icon.svg` 被 Vite 内联为 `data:` URL，
  因此 `img-src` 需要允许 `data:`；
- 截图与字体：界面不展示远程截图，攻略链接通过 opener 插件交给系统浏览器打开；没有 `@font-face`，只使用系统字体栈；
- IPC：`@tauri-apps/api` 向 `http://ipc.localhost`（其他平台为 `ipc:`）发送请求，因此 `connect-src` 需要允许这两个来源。

`src-tauri/src/lib.rs` 中的 `csp_locks_down_remote_content_and_allows_ipc` 测试会检查上述指令是否齐全，
并确保未允许 `'unsafe-inline'` 或 `'unsafe-eval'`。

## 相关文档

- 开发环境、CI 与发布、布局约束：[../../docs/development/index.md](../../docs/development/index.md)
- 数据来源与目录库：[../../docs/reference/data.md](../../docs/reference/data.md)
- CLI：[../../docs/cli/index.md](../../docs/cli/index.md)
