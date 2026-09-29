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
- 程序会记住上次手动选择的存档（含其它槽位与「选择存档文件…」打开的任意文件），
  下次启动直接打开；文件不存在时自动回退到最新写入的主槽位，可在设置里「改为自动选择」；
- 选中的存档会显示文件大小、修改时间与完整路径。

## 找不到存档

- 界面上会列出实际扫描的目录，并标注每个目录是否存在；存在的目录可点「打开」在资源管理器中查看；
- 点「选择存档文件…」可直接打开不在默认目录中的存档（备份、其它磁盘、Steam 云副本等），
  文件名不必符合游戏命名规则；
- 也可以将存档放入上表目录之一，再点顶部「刷新」重新扫描；
- `Backup` 目录中的备份文件不会被自动识别，请用「选择存档文件…」手动打开或不要选择它们。

## 用 CLI 排查

`sbsave saves` 会列出自动探测到的存档（槽位、SteamID、大小、修改时间、路径）。
使用 `report --save <路径>` 或 `--slot <槽位>` 可以指定任意存档；见 [CLI 文档](../cli/index.md)。
