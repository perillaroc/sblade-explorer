# 原始数据说明

本目录保存构建目录库所需的原始资料和手工对照表。程序运行时不会联网。

## 目录结构

| 路径 | 内容 |
| --- | --- |
| `api/*.json` | stellarbladeguide.com API 响应（物品名称、位置描述和周目标签） |
| `api/i18n/levels.json`、`locations.json` | 攻略区域和地点的简体中文翻译（英文原文 → 中文） |
| `api/i18n/obtain_*.json` | 攻略获取说明的简体中文翻译（攻略站条目 id → 中文；`obtain_camps.json` 为营地文案） |
| `universe/aliases.json` | 从参考存档和本地存档提取的物品别名全集（不含未解锁 ID） |
| `crosswalk.json` | 手工维护的别名对照（site id → 别名 → 中文名，是目录库生成器的唯一事实来源）；`camps` 段维护营地别名与攻略站条目的映射，个别未收录营地直接内嵌中英文案；`record_type_overrides` 用于覆盖无法按名称自动匹配的记录类型；`nano_suits` 和 `appearance` 中未被攻略站收录的条目省略 `site_id`，直接内嵌 `name_en`、`area` 和 `obtain` |
| `fish_obtain.json` | 手工维护的鱼类获取说明（别名 → 中英文案：钓点和鱼饵；鱼类没有攻略站条目） |
| `missable.json` | 手工维护的易错过清单（希雍锁定、埃多斯9号可选区域和支线奖励；可按区域、营地区段或显式条目选择，`catalog build` 展开为 `missable`）。可选的 `void_after_quests` / `requires_quests` / `void_on_failed_quests` 给出机器可读的存档任务条件，`catalog build` 展开为条目的 `missable_rule` 并校验任务别名 |
| `quest_aliases.json` | `DataMap_SBQuest` 中见过的任务别名快照（本地、Demo、100% 三份存档，117 条）；`catalog build` 用它对 `missable.json` 的任务条件做拼写校验 |
| `memorystick_order.json` | 记忆棒在游戏内数据库中的排列顺序（按区域分组并记录组内顺序，覆盖全部 186 条；根据 Map Genie 和游民星空列表整理） |
| `guides.json` | 手工维护的中文攻略链接快照（游民星空图文和 B 站「喂狗组-文轩」全收集视频）；`catalog build` 将其解析为每个物品的 `guides.web` 和 `guides.video`；纳米战衣和设计图逐件对应游民星空服装图鉴分页 |
| `game/name_map.json` | 从游戏数据表和 `Game.locres` 提取的内部别名与官方名称映射（生成文件，包含 `items`、`camps`、`album`、`regions`、`quests` 五段；`regions` 是 `UI_RegionAlias_*` 区域代号表，`quests` 是 `Quest_*_Title` 任务标题；`camps` 合并 `game/camp_alias_rows.json` 中置信度不为 low 的对照） |
| `game/camp_alias_rows.json` | 营地别名与 `ZoneCampTable` 行号的对照（用于数据表未直接关联的营地，包含置信度和匹配方法；`low` 条目不并入 `name_map.json`） |
| `reference/Sources_TrainerCore_*.swift` | 中文名称对照（来自 stellar-blade-macos-save-editor） |
| `reference/*.sav` | 参考存档（仅供本地校验，已在 .gitignore 中忽略） |

## 数据来源

- <https://stellarbladeguide.com>：英文名称、位置、描述和 Base/NG+/NG++/DLC 标签；
- `api/upgrades__gear.json`：装备词条的拾取位置。标题末尾的 ★ 数对应稀有度（★ 普通、★★ 精良、★★★ 稀有），
  `catalog build` 结合 `game/name_map.json` 的官方英文名推导 `Gear_*_Common` / `_Uncommon` / `_Rare` 别名，
  并把升级后的 `_MK2` 并入同一条目；中文获取文案在 `api/i18n/obtain_gear.json`；
- <https://mapgenie.io/stellar-blade/guides/memory-sticks> 和 <https://www.gamersky.com/handbook/202507/1953527.shtml>：
  记忆棒在游戏内数据库中的排列顺序（`memorystick_order.json`，两份来源的顺序一致）；
- 文档和密码的区域、地点直接取自攻略条目，与其他收集物使用同一套区域/地点模型；
  记忆棒区域以 `memorystick_order.json` 为准，因为攻略与游戏内数据库存在个别归类差异；
- `api/i18n/`：上述英文攻略文案的简体中文手工翻译，物品名称仍以游戏本地化文本为准；
- 分类和记录类型的英文名直接维护在 `crates/sbsave-tools/src/catalog_build.rs` 的
  `CATEGORIES` 和 `RECORD_TYPES` 常量中，生成目录库时写入 `name_en` 和 `record_type_en`；
- <https://github.com/wuxiao00j/stellar-blade-macos-save-editor>：简体中文名称；
- <https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file>：别名全集校验；
- 游戏数据表（`ItemTable`、`ZoneCampTable`、`AlbumTable`）与 `Game.locres`（zh-Hans/en）：
  提供内部别名与官方名称的对应关系，通过 `cargo run -p sbsave-tools -- mine-names` 生成 `game/name_map.json`；
  `album` 段收录图鉴数据表中的孽奇拔（67 条）和角色（55 页）条目，包含官方说明文字；
  `regions` / `quests` 段来自 locres 的 `UI_RegionAlias_*` 与 `Quest_*_Title`，用于显示存档当前区域与主线；
- Steam《剑星》1.5.0 更新公告和 `Game.locres`（zh-Hans/en）：提供蓓优妮塔联动 8 件收集物
  （4 件纳米战衣、2 个眼镜、2 个耳饰）的官方中英文名称；攻略站尚未收录这些物品，因此通过
  `crosswalk.json` 中省略 `site_id` 的手工条目入库；
- 参考存档（100% 完成度）的 `DataMap_SBEnv`（别名和事件点坐标）、`UIStorage` 红点营地顺序、
  `SBAchievement` 成就顺序，以及 `ZoneCampTable` 的 `CoinSpawnPointName`：用于建立数据表未直接关联的
  营地别名与行号对照（`game/camp_alias_rows.json`；坐标法 36 条、区域内排除法 17 条、
  埃多斯9号 6 条按顺序推定并保持低置信度）；
- <https://www.gamersky.com>（《剑星》全收集攻略、全饮料罐、全钓鱼点、全宝箱密码等）和
  <https://www.bilibili.com>（喂狗组-文轩「剑星全收集」分 P 视频）：`guides.json` 中的中文攻略链接；
  只有在桌面端点击按钮时才会通过系统浏览器打开，程序运行时不会联网；
- <https://www.gamersky.com/handbook/202405/1747789.shtml>（全钓鱼点位置及可钓鱼类一览）：
  `fish_obtain.json` 中的钓点和鱼饵来源；妮姬联动鱼类根据联动攻略注明特殊鱼饵或奇怪鱼饵；
- PowerPyx（[全收集](https://www.powerpyx.com/stellar-blade-all-collectible-locations-guide/)和
  [埃多斯9号](https://www.powerpyx.com/stellar-blade-eidos-9-collectible-locations/)页面）、
  [Steam《Point Of No Return》指南](https://steamcommunity.com/sharedfiles/filedetails/?id=3516153280)、
  [巴哈姆特全结局攻略](https://forum.gamer.com.tw/C.php?bsn=36389&snA=203)和
  [游民星空全结局攻略](https://www.gamersky.com/handbook/202404/1737597.shtml)：`missable.json` 的依据，
  包括希雍城区在完成卢瓦底层后锁定、埃多斯9号需要百合进度达到 100% 才会开放，以及支线任务和布告栏委托会在第一个不可返回节点后失败。

## 中文攻略链接（guides.json）

`guides.json` 按「物品 > 地点 > 区域 > 品类」的优先级逐级覆盖，可设置 `web`（图文）和 `video`（视频）两类链接。
`catalog build` 会检查链接是否使用 HTTPS、域名是否属于 `www.gamersky.com` 或 `www.bilibili.com` 白名单，
以及键名能否匹配目录库；如果键名拼写错误，构建会直接失败。
桌面端列表和周目矩阵的详情弹窗据此显示「图文攻略」和「视频攻略」按钮，并始终提供
「搜索图文攻略」（默认使用必应，可在设置中切换为百度或谷歌）和「搜索视频攻略」（B 站）。
即使没有内置链接，也可以搜索对应条目的攻略。

纳米战衣和设计图逐件链接到游民星空《剑星》女主服装图鉴
（<https://www.gamersky.com/handbook/202404/1737082.shtml>，每套服装一页）的具体分页。
图鉴未收录的新版本或 DLC 服装，分别回退到完整版新增（1943736）、尼尔 DLC（1848445）和妮姬 DLC（1942435）图鉴。
设计图沿用同名服装的获取页和视频分 P。战衣视频逐套链接到 B 站「126 套纳米服全收集」
（<https://www.bilibili.com/video/BV1Wfj1ztEuj>，每套一个分 P）；少数未收录的套装回退到
喂狗组-文轩「服装&外观篇」（BV1or421L7XQ）。

## 重新生成目录库

```powershell
cargo run -p sbsave-tools -- catalog build
cargo run -p sbsave-cli -- catalog check
```

手工别名映射位于 `data/raw/crosswalk.json`，攻略文案翻译位于 `data/raw/api/i18n/`。
新增攻略站条目后，需要补齐 `obtain_*.json`；`sbsave-tools catalog build` 会按英文原文合并，
`crates/sbsave-core/tests/test_catalog.rs` 会校验覆盖率。
如果只需修正个别条目，建议优先使用 `%LOCALAPPDATA%\sbsave\catalog.user.json` 覆盖文件。

重复拾取产生的记录别名（`Item_Records_*_<N>`，如 `Item_Records_Xion_Memory_14_1`）会在生成时并入基础条目。
游戏数据库（`AlbumTable`）中每份记录只有一条，副本别名仅表示重复拾取。
合并后的 300 条记录与游戏数据库一致，副本别名仍参与「已获得」判定。

易错过清单位于 `data/raw/missable.json`，可按区域（希雍、埃多斯9号）、营地 EnvS 区段或显式条目选择。
构建时会检查每个选择器是否命中；带任务条件的分组会展开为 `missable_rule`，分析时结合存档任务状态显示「已错过 / 还来得及」，无规则或旧存档缺少任务数据时保留「可错过(注意节点)」。

## 更新游戏名称映射（需要本机安装游戏）

需要准备社区工具 `cue4parse.exe`、`repak.exe` 和社区 `.usmap` 文件。将它们放在同一目录后执行：

```powershell
cargo run -p sbsave-tools -- mine-names `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

`--game` 会将 `ItemTable`、`ZoneCampTable` 和 `AlbumTable` 导出到 `data/raw/game/dump`
（已在 .gitignore 中忽略），解包 `Game.{zh-Hans,en}.locres`，然后生成
`data/raw/game/name_map.json`（包含 `items`、`camps` 和 `album` 三段）。
对于数据表未直接关联的营地，程序根据 `data/raw/game/camp_alias_rows.json` 中的别名与行号对照补充名称，
跳过置信度为 `low` 的条目，对应条目在目录库中保持低置信度。
也可以先使用 `retoc` 或 `cue4parse` 自行导出，再通过 `--dump 目录` 离线运行。

> `repak` 解包使用 Oodle 压缩的 pak 时需要 `oo2core_9_win64.dll`，将其放在 tools 目录即可。
> 如果首次运行时自动下载失败，可以设置 `HTTP_PROXY` 或 `HTTPS_PROXY` 使用本机代理。

## 更新 API 数据（需要访问外网）

```powershell
curl.exe -x "http://127.0.0.1:50802" -A "Mozilla/5.0" `
  -o data/raw/api/collectibles__cans.json `
  "https://stellarbladeguide.com/api/collectibles/cans"
```

其余端点：`collectibles/{cans,documents,memorysticks,passcodes,camps}`、
`cosmetics/{nano-suits,hairstyles,glasses,earrings,drone,lily,adam}`。
