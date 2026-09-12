# 剑星修改器 macOS

一款为腾讯应用宝 Mac 版《剑星》制作的原生 macOS 存档修改器。

> 这是非官方工具。使用前请在游戏中正常保存并完全退出。修改器每次写入前都会自动备份存档。

## 功能

- 修改货币、纳米材料、聚合材料、微型线圈/马达/驱动装置、核心升级材料、弹药和消耗品。
- 修改 `SPExp` 技能经验值，由游戏在获得下一次经验时结算技能点。
- 按中文名称勾选解锁 116 件非 DLC 伊芙服饰。
- 解锁 14 个非 DLC 伊芙发型、19 个眼镜/面饰、24 个耳饰、7 件亚当服饰、6 件莉莉服饰和 8 个无人机外观。
- NieR:Automata 与 NIKKE DLC 商店外观不写入存档，避免出现“已拥有但无法购买、图鉴未登记”的问题。
- 总览页提供“修复 DLC 商店”，可删除旧版提前写入的 DLC 外观库存记录，同时保留两种 DLC 货币。
- 首次选择存档后记住位置，也可自动查找应用宝存档。
- 游戏运行时禁止写入。
- 写入前自动备份，写入后重新解析并验证 CRC32。
- 内置备份恢复页面，可单独删除或一键清空备份。

## 下载

请在 [Releases](https://github.com/wuxiao00j/stellar-blade-macos-save-editor/releases) 页面下载最新的 macOS 压缩包。

系统要求：macOS 14 或更高版本，Apple Silicon Mac。

第一次打开如被 macOS 拦截，请在 Finder 中右键应用，然后选择“打开”。

## 存档位置

腾讯应用宝 Mac 版《剑星》存档通常位于：

```text
~/Library/Application Support/com.tencent.yybmac.wine.engine/wine/drive_c/users/tencentyyb/AppData/Local/SB/Saved/SaveGames/
```

进入其中的数字账号文件夹，选择 `StellarBladeSave00.sav`。更完整的步骤请查看 [使用说明](./使用说明.md)。

## 从源码构建

```bash
swift build -c release
./Scripts/build-app.sh
```

构建完成后，应用会位于 `dist/剑星修改器.app`。

`swift test` 会使用本机现有的 `StellarBladeSave00.sav` 创建临时副本，因此只能在已安装并保存过应用宝 Mac 版《剑星》的机器上运行。测试不会写入真实存档。

## 安全设计

- 存档必须具有有效的 `EVAS/GVAS` 文件头和 CRC32。
- 每次写入前会在存档目录中创建 `CodexBackups` 备份文件夹。
- 已拥有的非 DLC 服饰和外观不会重复写入。
- DLC 商店外观必须在游戏内正常购买；请勿使用 v0.7 或更早版本直接解锁这些项目。
- “修复 DLC 商店”只应用于旧版提前解锁造成的异常；已经在游戏内正常购买的 DLC 外观不要清除。
- 本项目是存档修改器，不包含无敌、一击必杀等运行时内存修改功能。

## 仓库内容

- `Sources/TrainerCore`：存档解析、CRC32、备份和写入逻辑。
- `Sources/EVETrainer`：macOS SwiftUI 界面。
- `Tests/TrainerCoreTests`：临时存档副本的解析与写入验证。
- `Packaging`：应用图标和 `Info.plist`。

仓库不包含《剑星》游戏文件或用户存档。
