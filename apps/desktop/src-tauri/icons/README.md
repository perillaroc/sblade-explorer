# 应用图标

本目录中的图标由 `apps/desktop/src/assets/app-icon.svg` 生成，**请勿手动编辑**。
Tauri 配置（`tauri.conf.json` 的 `bundle.icon`）只引用其中 5 个桌面图标，
其余 `Square*Logo.png` 和 `StoreLogo.png` 为 AppX/MSIX 目标预留，与 CLI 输出保持一致。

## 重新生成

```powershell
cd apps/desktop
pnpm icon   # 等同于 tauri icon src/assets/app-icon.svg
```

该命令会覆盖本目录中的桌面图标，并额外生成 `android/` 和 `ios/` 目录（移动端资源，
桌面项目已在 `src-tauri/.gitignore` 中忽略）。更换图标后重新打包，即可在窗口/任务栏、
便携版 exe（`pnpm tauri build --no-bundle`）和 NSIS/MSI 安装包中看到新图标。

> **更换图标后必须强制重新编译一次**：窗口和 exe 图标在编译期嵌入（Windows 使用本目录中的
> `icon.ico`，代码通过 `include_bytes!(<OUT_DIR>/<sha256>)` 引入），但 `tauri-build` 不会针对
> 图标发出 `rerun-if-changed`。直接运行 `pnpm tauri dev` 可能复用旧二进制并继续显示旧图标。
> 在 `src-tauri/` 下执行 `cargo clean -p sblade-explorer`，然后重新构建即可。

## 设计与许可

- 造型：深靛色到紫色的渐变圆角方底，搭配白色剑形图案；`sword` 字形取自
  [Lucide](https://lucide.dev/icons/sword)，路径数据来自 `@lucide/vue`。
- 许可：Lucide 采用 ISC 许可证。`sword` 不属于 Lucide 中源自 Feather 的图标，因此只需保留 ISC 声明。
  以下许可和版权声明随图标一并分发，内容来自 `@lucide/vue@1.47.0` 的 `LICENSE`。

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
