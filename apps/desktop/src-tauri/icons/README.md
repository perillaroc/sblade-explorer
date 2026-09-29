# 应用图标

本目录下的图标由 `apps/desktop/src/assets/app-icon.svg` 生成，**不要手工编辑**。
Tauri 配置（`tauri.conf.json` 的 `bundle.icon`）只引用其中 5 个桌面图标，
其余 `Square*Logo.png` / `StoreLogo.png` 供 AppX/MSIX 目标预留，与 CLI 输出保持一致。

## 重新生成

```powershell
cd apps/desktop
pnpm icon   # = tauri icon src/assets/app-icon.svg
```

该命令会覆盖本目录下的桌面图标，并额外输出 `android/`、`ios/`（移动端资源，
桌面项目已在 `src-tauri/.gitignore` 中忽略）。改动图标后重新打包即可看到新图标：
窗口/任务栏、便携版 exe（`pnpm tauri build --no-bundle`）与 NSIS/MSI 安装包。

## 设计与许可

- 造型：深靛 → 紫渐变圆角方底 + 白色剑形；`sword` 字形取自
  [Lucide](https://lucide.dev/icons/sword)，路径数据照抄自 `@lucide/vue`。
- 许可：Lucide 采用 ISC 许可（`sword` 不属于 Lucide 中源自 Feather 的图标，故仅需 ISC 声明）。
  下列许可与版权声明随图标一并分发，照抄自 `@lucide/vue@1.47.0` 的 `LICENSE`。

```text
ISC License

Copyright (c) 2026 Lucide Icons and Contributors

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
```

许可原文：<https://lucide.dev/license>
