import { watch } from "vue";
import { createI18n } from "vue-i18n";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { messages, type MessageSchema, type UiLocale } from "../locales";
import { settings } from "./settings";

/**
 * Global i18n instance. `settings.uiLocale` is the persisted source of truth;
 * the watcher below applies it to `<html lang>` and the window title.
 */
export const i18n = createI18n<[MessageSchema], UiLocale, false>({
  legacy: false,
  locale: settings.uiLocale,
  fallbackLocale: "zh",
  // Messages are bundled and trusted; `saveGuide.hint` intentionally contains
  // <code> markup rendered with v-html.
  warnHtmlMessage: false,
  messages,
});

/** Locale-aware translation for non-component modules (lib helpers). */
export function translate(key: string, params?: Record<string, unknown>): string {
  return params === undefined ? i18n.global.t(key) : i18n.global.t(key, params);
}

export function currentLocale(): UiLocale {
  return i18n.global.locale.value as UiLocale;
}

function applyShellLocale(): void {
  const locale = settings.uiLocale;
  const title = translate("common.appName");
  document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
  document.title = title;
  if (isTauri()) {
    getCurrentWindow()
      .setTitle(title)
      .catch((reason) => console.warn(translate("errors.themeSyncFailed"), reason));
  }
}

/** Applies the stored UI locale before mount and keeps the shell in sync. */
export function initI18n(): void {
  i18n.global.locale.value = settings.uiLocale;
  applyShellLocale();
  watch(
    () => settings.uiLocale,
    (value) => {
      i18n.global.locale.value = value;
      applyShellLocale();
    },
  );
}
