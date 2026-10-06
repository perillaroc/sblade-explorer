export const APP_ID = "sblade-explorer";
export const APP_LICENSE = "Apache License 2.0";
export const APP_COPYRIGHT = "Copyright 2026 perillaroc";

// 项目链接与 src-tauri/capabilities/default.json 的 opener 白名单保持一致。
export interface AboutProjectLink {
  /** i18n key suffix under `about.projects`. */
  key: "repo" | "docs";
  url: string;
}

export const PROJECT_LINKS: AboutProjectLink[] = [
  {
    key: "repo",
    url: "https://github.com/perillaroc/sblade-explorer",
  },
  {
    key: "docs",
    url: "https://sblade-explorer.perillaroc.wang/",
  },
];

export interface AboutSource {
  /** i18n key suffix under `about.sourceNames` / `about.sourcesList`. */
  key: "guide" | "saveEditor" | "completion" | "mapgenie" | "game" | "guides" | "translation" | "unpack";
  urls?: string[];
}

export const DATA_SOURCES: AboutSource[] = [
  {
    key: "guide",
    urls: ["https://stellarbladeguide.com"],
  },
  {
    key: "saveEditor",
    urls: ["https://github.com/wuxiao00j/stellar-blade-macos-save-editor"],
  },
  {
    key: "completion",
    urls: ["https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file"],
  },
  {
    key: "mapgenie",
    urls: [
      "https://mapgenie.io/stellar-blade/guides/memory-sticks",
      "https://www.gamersky.com/handbook/202507/1953527.shtml",
    ],
  },
  {
    key: "game",
  },
  {
    key: "guides",
    urls: [
      "https://www.gamersky.com/handbook/202404/1738505.shtml",
      "https://www.gamersky.com/handbook/202404/1737082.shtml",
      "https://www.bilibili.com/video/BV1Wfj1ztEuj",
      "https://www.bilibili.com/video/BV1or421L7XQ",
    ],
  },
  {
    key: "translation",
  },
  {
    key: "unpack",
  },
];
