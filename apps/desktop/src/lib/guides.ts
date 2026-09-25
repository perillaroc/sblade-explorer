import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { searchEngine, settings } from "./settings";

export interface GuideLink {
  title: string;
  url: string;
}

export interface Guides {
  web: GuideLink | null;
  video: GuideLink | null;
}

const state = reactive<{
  loaded: boolean;
  links: Record<string, Guides>;
}>({
  loaded: false,
  links: {},
});

/**
 * Loads the per-item Chinese guide links from the bundled catalog once.
 * The links are resolved at build time (`sbsave-tools catalog build`); the app
 * itself never fetches the network, it only hands URLs to the system browser.
 */
export async function loadGuides(): Promise<void> {
  if (state.loaded) return;
  state.loaded = true;
  try {
    const payload = await invoke<Record<string, Partial<Guides>>>("guide_links");
    const links: Record<string, Guides> = {};
    for (const [id, guides] of Object.entries(payload)) {
      links[id] = {
        web: guides.web ?? null,
        video: guides.video ?? null,
      };
    }
    state.links = links;
  } catch (reason) {
    // Browser-only dev server has no Tauri IPC; fall back to search links.
    console.warn("读取攻略链接失败", reason);
  }
}

export function guideFor(id: string): Guides | null {
  return state.links[id] ?? null;
}

const VIDEO_SEARCH = "https://search.bilibili.com/all?keyword=";

// The web search engine is configurable in the settings dialog; the video
// search always goes to Bilibili's own search page.
export function searchWebUrl(name: string): string {
  return searchEngine().url + encodeURIComponent(`剑星 ${name} 攻略`);
}

export function searchVideoUrl(name: string): string {
  return VIDEO_SEARCH + encodeURIComponent(`剑星 ${name} 全收集`);
}

/**
 * Opens a guide page in the browser chosen in the settings dialog. When no
 * browser is configured (or it fails to launch) the system default is used;
 * the browser-only dev server falls back to a new tab.
 */
export async function openGuide(url: string): Promise<void> {
  if (!url.startsWith("https://")) return;
  const browser = settings.browserPath;
  try {
    if (browser) {
      try {
        await openUrl(url, browser);
        return;
      } catch (reason) {
        console.warn(`用指定浏览器打开失败（${browser}），回退系统默认浏览器`, reason);
      }
    }
    await openUrl(url);
  } catch (reason) {
    console.warn("打开外部链接失败", reason);
    if (!("__TAURI_INTERNALS__" in window)) {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  }
}
