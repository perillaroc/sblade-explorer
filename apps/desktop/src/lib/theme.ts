import { watch } from "vue";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Monitor, Moon, Sun, type LucideIcon } from "@lucide/vue";
import { translate } from "./i18n";
import { settings, type ThemeMode } from "./settings";

export interface ThemeOption {
  value: ThemeMode;
  label: string;
  icon: LucideIcon;
}

/** Theme choices with labels in the current interface language. */
export function themeOptions(): ThemeOption[] {
  return [
    { value: "dark", label: translate("theme.dark"), icon: Moon },
    { value: "light", label: translate("theme.light"), icon: Sun },
    { value: "system", label: translate("theme.system"), icon: Monitor },
  ];
}

type ResolvedTheme = "dark" | "light";

const DARK_SCHEME = window.matchMedia("(prefers-color-scheme: dark)");

function resolveTheme(): ResolvedTheme {
  if (settings.theme === "system") return DARK_SCHEME.matches ? "dark" : "light";
  return settings.theme;
}

/** Keep the native window chrome (title bar, WebView2 color scheme) in sync. */
function syncWindowTheme(theme: ResolvedTheme): void {
  if (!isTauri()) return;
  getCurrentWindow()
    .setTheme(settings.theme === "system" ? null : theme)
    .catch((reason) => console.warn(translate("errors.themeSyncFailed"), reason));
}

export function applyTheme(): void {
  const theme = resolveTheme();
  document.documentElement.classList.toggle("dark", theme === "dark");
  document.documentElement.style.colorScheme = theme;
  syncWindowTheme(theme);
}

/**
 * Apply the stored appearance before the app mounts, then follow OS changes
 * while the mode is `system`.
 */
export function initTheme(): void {
  applyTheme();
  DARK_SCHEME.addEventListener("change", () => {
    if (settings.theme === "system") applyTheme();
  });
  watch(() => settings.theme, applyTheme);
}
