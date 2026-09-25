import { openUrl } from "@tauri-apps/plugin-opener";
import { settings } from "./settings";

/**
 * Opens an external `https` link in the browser chosen in the settings dialog.
 * When no browser is configured (or it fails to launch) the system default is
 * used; the browser-only dev server falls back to a new tab.
 */
export async function openExternalUrl(url: string): Promise<void> {
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
