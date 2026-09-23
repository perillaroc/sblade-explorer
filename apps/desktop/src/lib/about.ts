export const APP_NAME = "剑星存档分析";
export const APP_ID = "sblade-explorer";
export const APP_LICENSE = "Apache License 2.0";
export const APP_COPYRIGHT = "Copyright 2026 perillaroc";

export interface AboutSource {
  name: string;
  detail: string;
  urls?: string[];
}

export const DATA_SOURCES: AboutSource[] = [
  {
    name: "Stellar Blade Guide",
    detail: "物品名称（英文）、位置描述、获取方式与 Base / NG+ / NG++ / DLC 周目标签。",
    urls: ["https://stellarbladeguide.com"],
  },
  {
    name: "stellar-blade-macos-save-editor",
    detail: "简体中文物品名称对照与别名映射。",
    urls: ["https://github.com/wuxiao00j/stellar-blade-macos-save-editor"],
  },
  {
    name: "Stellar-Blade-100-completion-save-file",
    detail: "别名全集与数据校验。",
    urls: ["https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file"],
  },
  {
    name: "Map Genie 与游民星空",
    detail: "记忆棒的游戏内数据库选单顺序（两份来源顺序一致）。",
    urls: [
      "https://mapgenie.io/stellar-blade/guides/memory-sticks",
      "https://www.gamersky.com/handbook/202507/1953527.shtml",
    ],
  },
  {
    name: "《剑星》游戏数据（Shift Up）",
    detail:
      "游戏数据表（ItemTable、ZoneCampTable）与 Game.locres（zh-Hans/en）提取的内部别名与官方简中名称。",
  },
  {
    name: "游民星空与哔哩哔哩（中文攻略链接）",
    detail:
      "data/raw/guides.json：详情页「中文攻略」按钮指向的游民星空图文攻略（含《剑星》女主服装图鉴）与 B 站视频（126 套纳米服全收集、喂狗组-文轩全收集系列）；仅在点击时由系统浏览器打开。",
    urls: [
      "https://www.gamersky.com/handbook/202404/1738505.shtml",
      "https://www.gamersky.com/handbook/202404/1737082.shtml",
      "https://www.bilibili.com/video/BV1Wfj1ztEuj",
      "https://www.bilibili.com/video/BV1or421L7XQ",
    ],
  },
  {
    name: "本项目手工翻译",
    detail:
      "data/raw/api/i18n：攻略区域、地点与获取描述的简体中文翻译（名称仍以游戏本地化为准）。",
  },
  {
    name: "社区解包工具",
    detail:
      "构建期使用 cue4parse、repak 与社区 .usmap 映射导出游戏数据；相关工具未随本程序分发。",
  },
];

export const SOURCES_NOTE =
  "以上第三方数据与工具版权归各自作者所有。目录库与全部数据内置于程序，运行时不联网；「中文攻略」按钮仅在点击后由系统浏览器打开外部网页。";

export const DISCLAIMER =
  "本工具为个人非商业项目，与 Shift Up / Sony Interactive Entertainment 无关；《剑星》名称、物品名称与游戏文本版权归原权利人所有，仅供个人存档分析使用。工具只读，绝不修改存档。";
