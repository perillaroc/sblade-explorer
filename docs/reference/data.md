# 数据来源与目录库

## 数据来源

- [stellarbladeguide.com](https://stellarbladeguide.com)：物品英文名称、位置描述、获取方式和 Base/NG+/NG++/DLC 标签（`data/raw/api/*.json`）；
- [stellar-blade-macos-save-editor](https://github.com/wuxiao00j/stellar-blade-macos-save-editor)：简体中文物品名称和别名映射（`data/raw/reference/Sources_TrainerCore_*.swift`）；
- [Stellar-Blade-100-completion-save-file](https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file)：用于数据校验的完整别名集合（`data/raw/universe/aliases.json`）；
- 游戏数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）：提供内部别名与官方名称的对应关系。
  通过 `cargo run -p sbsave-tools -- mine-names` 生成 `data/raw/game/name_map.json`；
  对于数据表中未直接关联的营地，使用 `data/raw/game/camp_alias_rows.json` 对照 `ZoneCampTable` 的行号；
  `name_map.json` 的 `regions` / `quests` 段来自 locres 的 `UI_RegionAlias_*` 与 `Quest_*_Title`，
  用于显示存档当前区域与主线；
- [mapgenie.io](https://mapgenie.io/stellar-blade/guides/memory-sticks) 与[游民星空](https://www.gamersky.com/handbook/202507/1953527.shtml)：
  记忆棒在游戏内数据库中的排列顺序（`data/raw/memorystick_order.json`，两份来源的顺序一致）；
- [游民星空](https://www.gamersky.com/handbook/202404/1738505.shtml)（全收集图文攻略、饮料罐、钓鱼点、密码和[女主服装图鉴](https://www.gamersky.com/handbook/202404/1737082.shtml)）与
  [哔哩哔哩](https://www.bilibili.com/video/BV1Wfj1ztEuj)（126 套纳米服全收集和[喂狗组-文轩「剑星全收集」](https://www.bilibili.com/video/BV1or421L7XQ)分 P 视频）：
  `data/raw/guides.json` 中的中文攻略链接；
- `data/raw/fish_obtain.json`：鱼类获取说明（钓点和鱼饵，手工维护），内容来自上述游民星空[全钓鱼点攻略](https://www.gamersky.com/handbook/202405/1747789.shtml)和游戏内钓鱼点信息；妮姬联动鱼类会注明联动鱼饵；
- `data/raw/api/i18n/`：上述英文攻略文案的简体中文手工翻译，物品名称仍以游戏本地化文本为准；其中 `obtain_camps.json` 包含营地条目的中文获取说明；
- PowerPyx（全收集和埃多斯9号页面）、Steam《Point Of No Return》指南、巴哈姆特与游民星空的全结局攻略：提供两个不可返回节点和可选区域的信息，用于维护 `data/raw/missable.json`（易错过清单）。
- `data/raw/quest_aliases.json`：本地、Demo 和 100% 三份存档中出现的任务别名快照（117 条），
  用于校验 `missable.json` 的任务条件，避免规则拼写错误。

原始数据快照的逐项说明和更新方法（API 抓取、`mine-names`）见 [data/raw/README.md](https://github.com/perillaroc/sblade-explorer/blob/main/data/raw/README.md)。
所有数据均内置于程序，运行时不会联网。

## 目录库与别名模型

- 目录库 `data/catalog.json` 由 `sbsave-tools catalog build` 生成，请勿手动修改。
  收集品数据以手工维护的 `data/raw/crosswalk.json` 为准（site id → 别名 → 中文名）。
  其中，`camps` 段维护营地别名与攻略站条目的映射，用于提供营地获取说明；
  营地别名与 `ZoneCampTable` 行号的对应关系维护在 `data/raw/game/camp_alias_rows.json`；
  图鉴条目则从游戏数据（`AlbumTable` 和 `Game.locres`）中提取。
- 只要物品的任一 `aliases` 出现在存档物品集合、成就记录派生的别名或图鉴成就别名中，就视为「已获得」。
- 程序读取存档中的 `NewGamePlusPlayCount`，据此标注「需要二周目(NG+)」或「需要三周目(NG++)」。
- 记录、密码、罐子、设计图案和外观的名称使用从游戏数据中提取的官方简体中文名称，并标记为 `confidence="high"`。
  所有营地别名与 `ZoneCampTable` 行号的对应关系均已确认；埃多斯9号的 6 座营地通过本机存档
  `DataMap_SBEnv.ZoneEnvStorageList` 的坐标与各行的 `CoinSpawnPointName` 位置邻近匹配（距离 138–719），
  不再有低置信度条目。
- 重复拾取产生的记录别名（如 `Item_Records_Xion_Memory_14_1`）会并入基础条目。
  游戏数据库（`AlbumTable`）中每份记录只有一条，副本别名仅表示重复拾取；合并后的 300 条记录与游戏数据库一致。
- 易错过条目由 `data/raw/missable.json` 维护，涵盖希雍城区、埃多斯9号以及支线任务和布告栏奖励。
  `catalog build` 会将其展开为 `missable` 字段和可选的 `missable_rule`；其中 `xion_lock` / `eidos9_optional`
  两组规则会结合存档任务状态显示「已错过 / 还来得及 / 未知」，其余 21 条支线奖励仍为静态标记。
  详见[存档解析与判定](#存档解析与判定)。
- 获取说明：收集品使用攻略站的 `description`（英文）和 `api/i18n/obtain_*.json`（中文）。
  鱼类没有攻略站条目，其说明维护在 `data/raw/fish_obtain.json`；
  少数未被攻略站快照收录的营地，则在 `crosswalk.json` 的 `camps` 段中直接提供中英文说明。
- 分类参数（CLI `--category` 和 UI 筛选）接受分类键（`nano_suits`）、中文名（`纳米战衣`）和英文名（`Nano Suits`）。
- 目录库分为两部分：`section=collection`（13 类收集品，计入目录进度）和
  `section=album`（图鉴：孽奇拔 67 条、角色 55 页，**不计入目录进度**，在报告和 UI 中单独列出）。
- 分类包含 `name`（中文）和 `name_en`（英文），记录条目包含 `record_type`、`record_type_zh` 和 `record_type_en`。
  英文名称维护在 `crates/sbsave-tools/src/catalog_build.rs` 的 `CATEGORIES` 和 `RECORD_TYPES` 中，随目录库一起生成；
  analysis JSON 也会输出这些字段，供桌面端按内容语言显示。

## 存档解析与判定

报告和桌面端在目录库差集之外，还会读取以下存档状态：

| 存档字段 | 用途 |
| --- | --- |
| `DataMap_SBItem.ItemOtaineSet` | 已获得物品别名 |
| `SBAchievement.AchievementList` | 成就及其派生别名（图鉴解锁、营地等） |
| `DataMap_int32` / `DataMap_FString` | 周目、难度、游玩时间、`MapName` 等计数器 |
| `DataMap_SBQuest.CompleteQuestList` / `FailedQuestList` / `ProgressQuestList` | 已完成 / 失败 / 进行中的任务别名 |
| `DataMap_FString.MapName` | 当前区域代号（如 `E04`） |

缺字段、类型异常或旧存档时，任务状态为空集，不会因此报错或误判。
当前区域代号通过 `catalog.json` 的 `map_names` 转成区域名；未知代号显示原始代号，不猜测。
当前主线取进行中任务里的第一个 `Quest_Epic_*`，标题通过 `quest_titles` 显示；没有进行中的主线时隐藏。

### 易错过三态

`data/raw/missable.json` 的每个分组可用 `void_after_quests`、`requires_quests`、
`void_on_failed_quests` 声明存档任务条件。`catalog build` 会把条件展开到条目的 `missable_rule`，
并用 `data/raw/quest_aliases.json` 校验任务别名；拼写错误会直接使构建失败。目前启用两组：

| 分组 | 条件 | 条目数 |
| --- | --- | --- |
| `xion_lock` | 完成 `Quest_Epic_08` 后失效 | 156 |
| `eidos9_optional` | 需要 `Quest_Epic_Hidden` 已开始 | 34 |

支线奖励分组（21 条）暂未启用机器规则，仍按静态 `missable` 展示。

分析结果分为：

- `open`（可错过，仍可获得）：条件表明尚未错过；
- `missed`（已错过）：存档中的任务状态已越过不可返回节点；
- `unknown`（未知）：没有规则、没有任务数据或条件不足；此时保留旧的「可错过(注意节点)」文案，
  **不会误报已错过**。

需要更高周目或未拥有的 DLC 条目先按周目 / DLC 阻塞处理，不参与易错过三态。

### DLC 拥有判定

存档没有可靠的 DLC 授权字段（`SaveFileVersionInfo` 只有版本号）。桌面端设置或 CLI
`--dlc nier,nikke` 提供「已拥有」列表：

- 未拥有的 DLC：条目按 DLC 限定阻塞；
- 已拥有的 DLC：不再阻塞，标记「已拥有，尚未获得」，可进入「可优先收集」；
- `save.owned_dlcs` 只反映设置 / 参数，不来自存档。

### JSON 附加字段

以下字段只增不改，旧消费者可以忽略：

- `save.owned_dlcs`：已拥有的 DLC 列表；
- `save.current_map` / `current_map_name` / `current_map_name_zh`：当前区域代号与中英名称
  （未知代号时名称字段为 `null`）；
- `save.current_epic_quest` / `current_epic_quest_title` / `current_epic_quest_title_zh`：
  当前主线别名与标题；
- 条目 `missable_state`：`open` / `missed` / `unknown`，非易错过或已获得条目为 `null`；
- 条目 `dlc_owned`：该条目的 DLC 是否已声明拥有；
- `summary.missed_count` / `summary.missable_open_count`：已错过 / 还来得及的缺失条目数；
- `dump --json` 的 `summary.quests.{completed,failed,active}`：任务列表计数。

## 图鉴（AlbumTable）

游戏图鉴数据表 `SB/Content/Local/Data/AlbumTable.uasset` 共有 524 条记录，按 `ESBAlbumType` 分为
记忆棒、文档、密码、孽奇拔（Native）、鱼类、角色、其他、拍照挑战和尼尔 DLC。
本项目收录其中两类，并标记为 `section=album`：

- `naytiba`（孽奇拔，67 条）：小兵 12 条、战士 36 条、精锐 10 条、阿尔法 6 条、上古 3 条；
- `characters`（角色，55 页）：20 名角色，每人 2～5 页剧情资料，名称形如 `艾德姆（资料 3/5）`。

每个图鉴条目的 `aliases` 都是存档成就别名（`Ach_Album_Unlock_*`），出现在成就列表中即视为已解锁。
条目带有官方说明 `desc_zh` 和 `desc_en`：孽奇拔包含「生态情报」和「战斗情报」，角色则包含累积的剧情文本；
入库时已清除 `<Orange>`、`<NewLine>` 等富文本标记。
图鉴不计入目录进度，只在报告和 UI 中单独列出。记忆棒、文档、密码和鱼类已包含在收集品分类中，因此不再重复收录。

## 自定义目录库

如果发现条目缺失或映射错误，可以创建用户覆盖文件，无需修改代码或重新生成目录库：

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

CLI 也可以通过 `--catalog 路径.json` 临时加载覆盖文件。
条目中的 `area_zh`、`location_zh` 和 `obtain_zh` 字段优先于内置翻译。

## 中文攻略链接

`data/raw/guides.json` 按「物品 > 地点 > 区域 > 品类」的优先级逐级覆盖，可设置 `web`（图文）和 `video`（视频）两类链接。
`catalog build` 会检查链接是否使用 HTTPS、域名是否属于 `www.gamersky.com` 或 `www.bilibili.com` 白名单，
以及键名能否匹配目录库；如果键名拼写错误，构建会直接失败。

桌面端详情弹窗据此显示「图文攻略」和「视频攻略」按钮，并始终提供「搜索图文攻略」
（默认使用必应，可在设置中切换为百度或谷歌）和「搜索视频攻略」（B 站）。
即使没有内置链接，也可以搜索对应条目的攻略。
纳米战衣和设计图逐件链接到游民星空《剑星》女主服装图鉴的具体分页，战衣视频逐套链接到 B 站
「126 套纳米服全收集」的对应分 P；罐子和鱼类精确到单个物品，记录和密码精确到区域分 P。
完整规则见 [data/raw/README.md](https://github.com/perillaroc/sblade-explorer/blob/main/data/raw/README.md)。

## 回归检查基准

- 目录库共 1019 条、17 个分类：13 类收集品 808 条、装备（成长区块）77 条、辅助脊椎（成长区块）12 条、
  图鉴 122 条（孽奇拔 67 条、角色 55 页）；其中纳米战衣 130 条、罐子 49 条、营地 89 条，低置信度条目 0 条。
- 获取说明：808 条收集品、77 条装备和 12 条辅助脊椎全部包含 `obtain` 和 `obtain_zh`。其中，35 条鱼类来自 `fish_obtain.json`，
  89 条营地来自攻略站 `description` 和 `obtain_camps.json`，1.5.0 联动的 8 条来自 `crosswalk.json` 中的手工文案，
  77 条装备来自 `api/i18n/obtain_gear.json`，12 条辅助脊椎来自 `api/i18n/obtain_exospines.json`。
  只有 122 条图鉴按设计使用官方 `desc`，不提供上述字段。
- 装备（Gear）：77 条可拾取装备单独归入 `section=growth`，**不计入目录进度**。
  条目别名按「游戏英文名 + 星级稀有度」推导为 `Gear_*_Common` / `_Uncommon` / `_Rare`，
  升级后的 `_MK2` 别名并入同一条目（升级也算已获得），共 154 个别名；装备不参与易错过标记。
- 辅助脊椎（Exospines）：12 条归入 `section=growth`，**不计入目录进度**；攻略标题与游戏英文名一一对应
  （`Judgement` 统一为官方拼写 `Judgment`），别名 `PT_*` 与 `PT_*_MK2` 并入同一条目，共 24 个别名。
  获得判定来自存档 `ItemEnhanceState.ExospineEnhanceStateMap`（`DataMap_SBActor` 与 NG+ 基础数据），
  而不是物品背包；不参与易错过标记。
- 材料（补给箱 151、高级补给箱 42）不进入目录库：它们是通用容器，内容不进入物品清单；
  存档用 `ChangeState_ZoneEnv_*_SupplyBox` 成就记录开启状态（本机 287 条、100% 参考存档 293 条），
  但攻略条目只有区域/地点描述、没有容器别名，两个数量也无法一一对应，因此不提供逐项或计数进度。
- 图鉴按页收录：孽奇拔各组数量为 12/36/10/6/3，角色共 55 页（艾德姆 5 页、迅驰 3 页、母主领域 1 页），
  全部包含 `desc_zh` 和 `desc_en`，且不内置攻略链接。
- 300 条记录全部包含游戏内类型（记忆棒 186 条、文档 114 条）和攻略获取方式；
  186 条记忆棒包含游戏内选单顺序，重复拾取副本并入基础条目；23 条密码全部包含攻略获取方式。
- 易错过：211 条带有 `missable`，包括希雍 156 条、埃多斯9号 34 条（含 6 座营地），以及其他支线奖励和任务区域 21 条。
  其中 190 条带有 `missable_rule`（希雍 156、埃多斯9号 34），21 条仅保留静态标记；
  分析结果区分「已错过 / 还来得及 / 未知」。
- 区域与任务：`catalog.json` 含 12 个 `map_names`（`UI_RegionAlias_*`）和 89 条 `quest_titles`
  （`Quest_*_Title`）；`quest_aliases.json` 含 117 条任务别名。
- 中文攻略链接：808 条收集品、77 条装备和 12 条辅助脊椎中有 895 条内置链接（图文 887 条、视频 694 条）。
  发型类中的默认马尾和首领挑战奖励没有内置链接，可直接使用「搜索图文攻略」或「搜索视频攻略」。
  所有链接均使用 HTTPS，且域名在白名单内。
