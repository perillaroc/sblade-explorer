import { reactive, watch } from "vue";

/** A browser detected on this machine by the `list_browsers` command. */
export interface BrowserInfo {
  id: string;
  name: string;
  path: string;
}

interface SearchEngine {
  id: SearchEngineId;
  name: string;
  url: string;
}

export type SearchEngineId = "bing" | "baidu" | "google";

export const SEARCH_ENGINES: readonly SearchEngine[] = [
  { id: "bing", name: "必应", url: "https://www.bing.com/search?q=" },
  { id: "baidu", name: "百度", url: "https://www.baidu.com/s?wd=" },
  { id: "google", name: "谷歌", url: "https://www.google.com/search?q=" },
];

export interface Settings {
  searchEngine: SearchEngineId;
  /** Executable of the browser used to open links; empty means the system default. */
  browserPath: string;
  /** Display name of `browserPath` (used when it is not in the detected browser list). */
  browserName: string;
}

const STORAGE_KEY = "sbsave.settings.v1";

function defaultSettings(): Settings {
  return { searchEngine: "bing", browserPath: "", browserName: "" };
}

function load(): Settings {
  const fallback = defaultSettings();
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return fallback;
    const parsed = JSON.parse(raw) as Partial<Settings>;
    const engine = SEARCH_ENGINES.find((candidate) => candidate.id === parsed.searchEngine);
    return {
      searchEngine: engine ? engine.id : fallback.searchEngine,
      browserPath: typeof parsed.browserPath === "string" ? parsed.browserPath : "",
      browserName: typeof parsed.browserName === "string" ? parsed.browserName : "",
    };
  } catch (reason) {
    console.warn("读取设置失败，使用默认设置", reason);
    return fallback;
  }
}

/**
 * Settings live in `localStorage` (kept in the app's WebView2 profile) and are
 * applied immediately; the save file is never touched.
 */
export const settings = reactive<Settings>(load());

watch(
  settings,
  (value) => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
    } catch (reason) {
      console.warn("保存设置失败", reason);
    }
  },
  { deep: true },
);

export function searchEngine(): SearchEngine {
  return (
    SEARCH_ENGINES.find((candidate) => candidate.id === settings.searchEngine) ?? SEARCH_ENGINES[0]
  );
}

export function resetSettings(): void {
  Object.assign(settings, defaultSettings());
}
