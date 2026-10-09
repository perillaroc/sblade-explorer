import { reactive, watch } from "vue";
import type { UiLocale } from "../locales";
import { UI_LOCALES } from "../locales";
import type { ItemFilter, Lang } from "../types";

/** A browser detected on this machine by the `list_browsers` command. */
export interface BrowserInfo {
  id: string;
  name: string;
  path: string;
}

interface SearchEngine {
  id: SearchEngineId;
  url: string;
}

export type SearchEngineId = "bing" | "baidu" | "google";

/** UI appearance; `system` follows the OS light/dark setting. */
export type ThemeMode = "dark" | "light" | "system";

const THEME_MODES: readonly ThemeMode[] = ["dark", "light", "system"];

const LANG_MODES: readonly Lang[] = ["zh", "en", "both"];

export const SEARCH_ENGINES: readonly SearchEngine[] = [
  { id: "bing", url: "https://www.bing.com/search?q=" },
  { id: "baidu", url: "https://www.baidu.com/s?wd=" },
  { id: "google", url: "https://www.google.com/search?q=" },
];

export interface Settings {
  /** Language of menus, dialogs, errors and the window title. */
  uiLocale: UiLocale;
  /** Language of collectible names, locations and descriptions. */
  contentLang: Lang;
  theme: ThemeMode;
  searchEngine: SearchEngineId;
  /** Executable of the browser used to open links; empty means the system default. */
  browserPath: string;
  /** Display name of `browserPath` (used when it is not in the detected browser list). */
  browserName: string;
  /** Save opened last time; empty means "pick the newest save on startup". */
  lastSavePath: string;
  /** Left sidebar collapsed to the icon rail (w-14); new in v0.3.0. */
  sidebarCollapsed: boolean;
  /** Per-category list/matrix preference; missing key = catalog default. */
  categoryViews: Partial<Record<string, CategoryView>>;
  /** Per-category filter preference; missing key = "all". Search terms are not stored. */
  categoryFilters: Partial<Record<string, ItemFilter>>;
}

/** Category page view preference persisted across restarts (F4). */
export type CategoryView = "matrix" | "list";

const STORAGE_KEY = "sbsave.settings.v1";

function detectUiLocale(): UiLocale {
  const language = typeof navigator === "undefined" ? "" : navigator.language;
  return language.toLowerCase().startsWith("zh") ? "zh" : "en";
}

function defaultSettings(): Settings {
  return {
    uiLocale: detectUiLocale(),
    contentLang: "zh",
    theme: "system",
    searchEngine: "bing",
    browserPath: "",
    browserName: "",
    lastSavePath: "",
    sidebarCollapsed: false,
    categoryViews: {},
    categoryFilters: {},
  };
}

function sanitizeViews(value: unknown): Partial<Record<string, CategoryView>> {
  const views: Partial<Record<string, CategoryView>> = {};
  if (value && typeof value === "object") {
    for (const [key, view] of Object.entries(value as Record<string, unknown>)) {
      if (view === "matrix" || view === "list") views[key] = view;
    }
  }
  return views;
}

function sanitizeFilters(value: unknown): Partial<Record<string, ItemFilter>> {
  const filters: Partial<Record<string, ItemFilter>> = {};
  if (value && typeof value === "object") {
    for (const [key, filter] of Object.entries(value as Record<string, unknown>)) {
      if (filter === "all" || filter === "obtained" || filter === "missing") filters[key] = filter;
    }
  }
  return filters;
}

function load(): Settings {
  const fallback = defaultSettings();
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return fallback;
    const parsed = JSON.parse(raw) as Partial<Settings>;
    const engine = SEARCH_ENGINES.find((candidate) => candidate.id === parsed.searchEngine);
    const theme = THEME_MODES.find((candidate) => candidate === parsed.theme);
    const uiLocale = UI_LOCALES.find((candidate) => candidate === parsed.uiLocale);
    const contentLang = LANG_MODES.find((candidate) => candidate === parsed.contentLang);
    return {
      uiLocale: uiLocale ?? fallback.uiLocale,
      contentLang: contentLang ?? fallback.contentLang,
      theme: theme ?? fallback.theme,
      searchEngine: engine ? engine.id : fallback.searchEngine,
      browserPath: typeof parsed.browserPath === "string" ? parsed.browserPath : "",
      browserName: typeof parsed.browserName === "string" ? parsed.browserName : "",
      lastSavePath: typeof parsed.lastSavePath === "string" ? parsed.lastSavePath : "",
      sidebarCollapsed:
        typeof parsed.sidebarCollapsed === "boolean"
          ? parsed.sidebarCollapsed
          : fallback.sidebarCollapsed,
      categoryViews: sanitizeViews(parsed.categoryViews),
      categoryFilters: sanitizeFilters(parsed.categoryFilters),
    };
  } catch (reason) {
    console.warn("Failed to load settings; using defaults", reason);
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
      console.warn("Failed to save settings", reason);
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
