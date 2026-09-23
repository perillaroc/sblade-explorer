# 原始数据说明

本目录保存构建目录库所需的原始资料与人工对照表，运行时不会联网。

## 目录结构

| 路径 | 内容 |
| --- | --- |
| `api/*.json` | stellarbladeguide.com API 响应（物品名称、位置描述、周目标签） |
| `api/i18n/levels.json`、`locations.json` | 攻略区域/地点的简体中文翻译（英文原文 -> 中文） |
| `api/i18n/obtain_*.json` | 攻略获取描述的简体中文翻译（站点条目 id -> 中文） |
| `universe/aliases.json` | 从参考存档与本地存档提取的物品别名全集（不含未解锁 ID） |
| `crosswalk.json` | 手工维护的别名对照（site id -> 别名 -> 中文名，目录库生成器的唯一事实来源）；`record_type_overrides` 覆盖无法按名称自动匹配的记录类型 |
| `memorystick_order.json` | 记忆棒的游戏内数据库选单顺序（按区域分组、组内顺序，覆盖全部 186 条；据 Map Genie 与游民星空列表整理） |
| `guides.json` | 手工维护的中文攻略链接快照（游民星空图文 + B 站「喂狗组-文轩」全收集视频）；`catalog build` 解析为每条物品的 `guides.web` / `guides.video`；纳米战衣与设计图逐件对应游民星空服装图鉴分页 |
| `game/name_map.json` | 游戏数据表 + `Game.locres` 提取的内部别名→官方名称映射（生成物） |
| `reference/Sources_TrainerCore_*.swift` | 中文名称对照（来自 stellar-blade-macos-save-editor） |
| `reference/*.sav` | 参考存档（仅本地校验用，已在 .gitignore 中忽略） |

## 数据来源

- <https://stellarbladeguide.com> —— 英文名称、位置、描述、Base/NG+/NG++/DLC 标签
- <https://mapgenie.io/stellar-blade/guides/memory-sticks>、<https://www.gamersky.com/handbook/202507/1953527.shtml>
  —— 记忆棒的游戏内数据库选单顺序（`memorystick_order.json`，两份来源顺序一致）
- 文档与密码的区域/地点直接取自攻略条目（与其它收集物同一套区域/地点模型）；记忆棒区域以
  `memorystick_order.json` 为准（攻略与游戏内数据库存在个别归类差异）
- `api/i18n/` —— 上述英文攻略文案的手工简体中文翻译（名称仍以游戏本地化为准）
- <https://github.com/wuxiao00j/stellar-blade-macos-save-editor> —— 简体中文名称
- <https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file> —— 别名全集校验
- 游戏本体数据表（`ItemTable`、`ZoneCampTable`）与 `Game.locres`（zh-Hans/en）——
  内部别名到官方名称的精确映射，经 `cargo run -p sbsave-tools -- mine-names` 生成 `game/name_map.json`
- <https://www.gamersky.com>（《剑星》全收集攻略、全饮料罐、全钓鱼点、全宝箱密码等）与
  <https://www.bilibili.com>（喂狗组-文轩「剑星全收集」分 P 视频）—— `guides.json` 的中文攻略链接；
  仅在桌面端点按按钮时由系统浏览器打开，程序运行时不联网

## 中文攻略链接（guides.json）

`guides.json` 按 `物品 > 地点 > 区域 > 品类` 逐级覆盖，可写 `web`（图文）与 `video`（视频）两类链接；
`catalog build` 会校验链接必须为 https 且域名在 `www.gamersky.com` / `www.bilibili.com` 白名单内，
同时校验键名必须能在目录库中匹配到（拼写错误会直接让构建失败）。桌面端列表与周目矩阵的详情弹窗
据此显示「图文攻略」「视频攻略」按钮，缺少链接时回退为必应 / B 站搜索。

纳米战衣、设计图逐件链接到游民星空《剑星》女主服装图鉴
（<https://www.gamersky.com/handbook/202404/1737082.shtml>，每套服装一页）的具体分页；
图鉴未收录的新版本/DLC 服装回退到完整版新增（1943736）、尼尔 DLC（1848445）、妮姬 DLC（1942435）图鉴；
设计图沿用同名服装的获取页与视频分 P。

## 重新生成目录库

```powershell
cargo run -p sbsave-tools -- catalog build
cargo run -p sbsave-cli -- catalog check
```

人工别名映射在 `data/raw/crosswalk.json`；攻略文案翻译在 `data/raw/api/i18n/`
（新增站点条目后需补齐 `obtain_*.json`，`sbsave-tools catalog build` 会按英文原文合并，
`crates/sbsave-core/tests/test_catalog.rs` 会校验覆盖率）。如只需修正个别条目，优先使用
`%LOCALAPPDATA%\sbsave\catalog.user.json` 覆盖文件。

## 刷新游戏名称映射（需要本机安装游戏）

需要社区工具 `cue4parse.exe`、`repak.exe` 与社区 `.usmap`。把它们放在同一目录后执行：

```powershell
cargo run -p sbsave-tools -- mine-names `
  --game "H:\SteamLibrary\steamapps\common\StellarBlade" `
  --tools path\to\tools
```

`--game` 会把数据表导出到 `data/raw/game/dump`（已在 .gitignore 中忽略）并解包
`Game.{zh-Hans,en}.locres`，然后写出 `data/raw/game/name_map.json`。也可以先用
`retoc`/`cue4parse` 自行导出，再用 `--dump 目录` 离线运行。

## 刷新 API 数据（需要可访问外网）

```powershell
curl.exe -x "http://127.0.0.1:50802" -A "Mozilla/5.0" `
  -o data/raw/api/collectibles__cans.json `
  "https://stellarbladeguide.com/api/collectibles/cans"
```

其余端点：`collectibles/{cans,documents,memorysticks,passcodes,camps}`、
`cosmetics/{nano-suits,hairstyles,glasses,earrings,drone,lily,adam}`。
