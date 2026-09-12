# 原始数据说明

本目录保存构建目录库所需的原始资料与人工对照表，运行时不会联网。

## 目录结构

| 路径 | 内容 |
| --- | --- |
| `api/*.json` | stellarbladeguide.com API 响应（物品名称、位置描述、周目标签） |
| `universe/aliases.json` | 从参考存档与本地存档提取的物品别名全集（不含未解锁 ID） |
| `reference/Sources_TrainerCore_*.swift` | 中文名称对照（来自 stellar-blade-macos-save-editor） |
| `reference/*.sav` | 参考存档（仅本地校验用，已在 .gitignore 中忽略） |

## 数据来源

- <https://stellarbladeguide.com> —— 英文名称、位置、描述、Base/NG+/NG++/DLC 标签
- <https://github.com/wuxiao00j/stellar-blade-macos-save-editor> —— 简体中文名称
- <https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file> —— 别名全集校验

## 重新生成目录库

```powershell
uv run python tools/build_catalog.py
uv run sbsave catalog check
```

人工别名映射在 `tools/crosswalk_data.py`；如只需修正个别条目，优先使用
`%LOCALAPPDATA%\sbsave\catalog.user.json` 覆盖文件。

## 刷新 API 数据（需要可访问外网）

```powershell
curl.exe -x "http://127.0.0.1:50802" -A "Mozilla/5.0" `
  -o data/raw/api/collectibles__cans.json `
  "https://stellarbladeguide.com/api/collectibles/cans"
```

其余端点：`collectibles/{cans,documents,memorysticks,passcodes,camps}`、
`cosmetics/{nano-suits,hairstyles,glasses,earrings,drone,lily,adam}`。
