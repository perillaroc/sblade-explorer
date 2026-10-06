import { openUrl } from "@tauri-apps/plugin-opener";
import { translate } from "./i18n";
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
        console.warn(translate("errors.browserOpenFailed", { browser }), reason);
      }
    }
    await openUrl(url);
  } catch (reason) {
    console.warn(translate("errors.openUrlFailed"), reason);
    if (!("__TAURI_INTERNALS__" in window)) {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  }
}
