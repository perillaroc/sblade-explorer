# 存档位置

## 自动探测

程序启动后会自动在以下目录（递归）查找 `StellarBladeSave*.sav`，任何名为 `Backup`
的目录都会被忽略：

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本/试玩版 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版 |

- 槽位文件名为 `StellarBladeSave00.sav`、`StellarBladeSave01.sav` 等，`<SteamID64>` 是数字账号目录；
- 程序默认选中**最新写入的主槽位**（`StellarBladeSave00`），也可以在顶部下拉中切换其它槽位；
- 选中的存档会显示文件大小、修改时间与完整路径。

## 找不到存档

- 确认存档位于上表目录之一，且文件名以 `StellarBladeSave` 开头、以 `.sav` 结尾；
- 将存档放入默认目录后，点击顶部「刷新」重新扫描；
- `Backup` 目录中的备份文件不会被识别，请不要选择它们。

## 用 CLI 排查

`sbsave saves` 会列出自动探测到的存档（槽位、SteamID、大小、修改时间、路径）。
使用 `report --save <路径>` 或 `--slot <槽位>` 可以指定任意存档；见 [CLI 文档](../cli/index.md)。
