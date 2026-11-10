# 存档位置

## 自动查找

程序启动后，会在以下目录中递归查找 `StellarBladeSave*.sav`，并忽略所有名为 `Backup` 的目录：

| 位置 | 说明 |
| --- | --- |
| `%LOCALAPPDATA%\SB\Saved\SaveGames\<SteamID64>\StellarBladeSave00.sav` | 正式版主存档 |
| `%USERPROFILE%\Documents\StellarBlade\<SteamID64>\` | 部分版本或试玩版存档 |
| `%LOCALAPPDATA%\SB_Demo\Saved\SaveGames\` | 试玩版存档 |

- 存档槽位依次命名为 `StellarBladeSave00.sav`、`StellarBladeSave01.sav` 等，`<SteamID64>` 是以数字命名的账号目录；
- 默认打开**最近写入的主槽位**（`StellarBladeSave00`），也可以通过顶部的下拉菜单切换其他槽位；
- 程序会记住上次手动选择的存档，包括其他槽位和通过「选择存档文件…」打开的文件，并在下次启动时直接打开。
  如果文件已不存在，则自动改为最近写入的主槽位；也可以在设置中点击「改为自动选择」；
- 选中存档后，界面会显示文件大小、修改时间和完整路径。

## 找不到存档

- 界面会列出实际扫描的目录，并标明目录是否存在。对于已存在的目录，可以点击「打开」在文件资源管理器中查看；
- 如果存档位于备份、其他磁盘或 Steam 云副本等默认目录之外的位置，点击「选择存档文件…」即可直接打开，文件名不必遵循游戏的命名规则；
- 也可以将存档放入上表中的任一目录，再点击顶部的「刷新」重新扫描；
- 程序不会自动查找 `Backup` 目录中的备份文件，请通过「选择存档文件…」手动打开。

## 使用 CLI 排查

`sbsave saves` 会列出自动找到的存档，包括槽位、SteamID、大小、修改时间和路径。
使用 `report --save <路径>` 或 `--slot <槽位>` 可以指定要分析的存档，详见 [CLI 文档](../cli/index.md)。
