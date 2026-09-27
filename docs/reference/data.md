# 数据来源与目录库

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com) —— 物品英文名称、位置描述、获取方式与
  Base/NG+/NG++/DLC 标签（`data/raw/api/*.json`）
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor) ——
  简体中文物品名称与别名映射（`data/raw/reference/Sources_TrainerCore_*.swift`）
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file) ——
  别名全集与数据校验（`data/raw/universe/aliases.json`）
- 游戏本体数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）——
  内部别名到官方名称的精确映射，经 `cargo run -p sbsave-tools -- mine-names` 生成
  `data/raw/game/name_map.json`
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与
  [游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml) ——
  记忆棒的游戏内数据库选单顺序（`data/raw/memorystick_order.json`，两份来源顺序一致）
- [游民星空](https://www.gamersky.com/handbook/202404/1738505.shtml)（全收集图文攻略、饮料罐、
  钓鱼点、密码、[女主服装图鉴](https://www.gamersky.com/handbook/202404/1737082.shtml)）与
  [哔哩哔哩](https://www.bilibili.com/video/BV1Wfj1ztEuj)（126 套纳米服全收集、
  [喂狗组-文轩「剑星全收集」](https://www.bilibili.com/video/BV1or421L7XQ) 分 P 视频）——
  `data/raw/guides.json` 中的中文攻略链接
- `data/raw/api/i18n/` —— 上述英文攻略文案的手工简体中文翻译（名称仍以游戏本地化为准）

原始数据快照的逐项说明与刷新方式（API 抓取、`mine-names`）见 [../data/raw/README.md](../data/raw/README.md)。
程序运行时不联网，全部数据内置于程序。

## 目录库与别名模型

- 目录库 `data/catalog.json` 由 `sbsave-tools catalog build` 生成，禁止手改；
  收集品的唯一事实来源是 `data/raw/crosswalk.json`（site id → 别名 → 中文名，手工维护），
  图鉴条目由游戏数据挖掘（`AlbumTable` + `Game.locres`）生成。
- 只要物品的任一 `aliases` 出现在存档物品集合、从成就记录派生的别名或成就别名（图鉴）中，
  即视为「已获得」。
- 读取存档中的 `NewGamePlusPlayCount`，标注「需要二周目(NG+)」/「需要三周目(NG++)」。
- 记录/密码/罐子/设计图案/外观名称由游戏数据挖掘覆盖为官方简中名称并置 `confidence="high"`；
  少量记录版本变体与未关联营地保留低置信度（共 69 条），报告中标记「映射待确认」。
- 分类参数（CLI `--category` 与 UI 筛选）同时接受分类键（`nano_suits`）和中文名（`纳米战衣`）。
- 分类分两段：`section=collection`（13 类收集品，计入目录进度）与 `section=album`
  （图鉴：孽奇拔 67、角色 55，**不计入目录进度**，在报告/UI 中单列）。

## 图鉴（AlbumTable）

游戏图鉴数据表 `SB/Content/Local/Data/AlbumTable.uasset` 共 524 条，按 `ESBAlbumType` 分为
记忆棒/文档/密码/孽奇拔（Native）/鱼类/角色/其他/拍照挑战/尼尔 DLC。本项目收录其中两类，
标记为 `section=album`：

- `naytiba`（孽奇拔，67 条）：小兵 12、战士 36、精锐 10、阿尔法 6、上古 3；
- `characters`（角色，55 条）：20 名角色 × 2~5 页剧情资料，名称形如 `艾德姆（资料 3/5）`。

每条图鉴的 `aliases` 是存档成就别名（`Ach_Album_Unlock_*`），在成就列表中出现即视为已解锁；
条目带官方 `desc_zh`/`desc_en` 说明（怪物为「生态情报/战斗情报」，角色为累积剧情文字，入库时已清洗
`<Orange>`/`<NewLine>` 富文本标记）。图鉴不计入目录进度，仅在报告与 UI 中单列。
记忆棒/文档/密码/鱼类已在收集品分类中覆盖，故未重复收录。

## 自定义目录库

如发现条目缺失或映射错误，可创建用户覆盖文件（无需改代码、无需重新生成）：

`%LOCALAPPDATA%\sbsave\catalog.user.json`

```json
{
  "items": [
    {
      "id": "Can_007",
      "name": "狄俄尼索斯C",
      "category": "cans",
      "aliases": ["Can_007"],
      "area": "希雍",
      "location": "公告板附近地面",
      "obtain": "前往公告板路上捡取",
      "confidence": "high"
    }
  ]
}
```

CLI 亦可用 `--catalog 路径.json` 临时附加；条目内 `area_zh`/`location_zh`/`obtain_zh`
优先级高于内置翻译。

## 中文攻略链接

`data/raw/guides.json` 按 `物品 > 地点 > 区域 > 品类` 逐级覆盖，可写 `web`（图文）与 `video`（视频）
两类链接；`catalog build` 会校验链接必须为 https 且域名在 `www.gamersky.com` / `www.bilibili.com`
白名单内，同时校验键名必须能在目录库中匹配到（拼写错误会直接让构建失败）。

桌面端详情弹窗据此显示「图文攻略」「视频攻略」按钮，并始终提供「搜索图文攻略」（默认必应，可在设置中
切换百度 / 谷歌）与「搜索视频攻略」（B 站）按钮，内置链接缺失时也能检索到对应条目。
纳米战衣与设计图逐件指向游民星空《剑星》女主服装图鉴的具体分页，战衣视频逐套指向 B 站
「126 套纳米服全收集」的对应分 P；罐子/鱼类精确到单个物品，记录/密码精确到区域分 P。
完整规则见 [../data/raw/README.md](../data/raw/README.md)。

## 回归哨兵

- 目录库 932 条 / 15 分类：13 类收集品 810 条 + 图鉴 122 条（孽奇拔 67、角色 55）；
  纳米战衣 126、罐子 49、营地 89、低置信度 69。
- 图鉴按页入库：孽奇拔分组 12/36/10/6/3，角色 55 页（艾德姆 5 页、迅驰 3 页、母主领域 1 页），
  全部带 `desc_zh`/`desc_en`，且无内置攻略链接。
- 记录 310 条全部带游戏内类型（记忆棒 187、文档 123）与攻略获取方式；记忆棒 186 条带游戏内选单顺序
  （版本变体继承基础条目）；密码 23 条全部带攻略获取方式。
- 中文攻略链接：收集品 808/810 条有内置链接（图文 804、视频 696；发型类的默认马尾与首领挑战奖励无内置
  链接，可直接用「搜索图文攻略 / 搜索视频攻略」检索）；链接均为 https 且域名在白名单内。
