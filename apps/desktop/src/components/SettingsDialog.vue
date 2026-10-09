<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, RotateCcw, Settings } from "@lucide/vue";
import {
  resetSettings,
  SEARCH_ENGINES,
  settings,
  type BrowserInfo,
  type SearchEngineId,
  type ThemeMode,
} from "../lib/settings";
import { themeOptions } from "../lib/theme";
import { UI_LOCALES, type UiLocale } from "../locales";
import { UiButton, UiDialog, UiSegmented } from "./ui";

/**
 * Settings modal rebuilt on the F1 dialog system. Settings keys and their
 * defaults are unchanged (backward compatible with older settings files);
 * Esc / backdrop close and focus handling come from `UiDialog`.
 */
const emit = defineEmits<{ close: [] }>();

const { t } = useI18n({ useScope: "global" });

const browsers = ref<BrowserInfo[]>([]);
const browsersLoaded = ref(false);

const localeOptions = computed(() =>
  UI_LOCALES.map((value) => ({ value, label: t(`language.${value}`) })),
);

const localeValue = computed({
  get: () => settings.uiLocale,
  set: (value: string) => {
    settings.uiLocale = value as UiLocale;
  },
});

const themeChoices = computed(() =>
  themeOptions().map((option) => ({
    value: option.value,
    label: option.label,
    icon: option.icon,
    ariaLabel: t("app.appearance", { name: option.label }),
  })),
);

const themeValue = computed({
  get: () => settings.theme,
  set: (value: string) => {
    settings.theme = value as ThemeMode;
  },
});

const engineOptions = computed(() =>
  SEARCH_ENGINES.map((engine) => ({
    value: engine.id,
    label: t(`settings.engines.${engine.id}`),
  })),
);

const engineValue = computed({
  get: () => settings.searchEngine,
  set: (value: string) => {
    settings.searchEngine = value as SearchEngineId;
  },
});

async function loadBrowsers() {
  try {
    const detected = await invoke<BrowserInfo[] | null>("list_browsers");
    browsers.value = Array.isArray(detected) ? detected : [];
  } catch (reason) {
    console.warn(t("errors.browserListFailed"), reason);
  } finally {
    browsersLoaded.value = true;
  }
}

function forgetLastSave() {
  settings.lastSavePath = "";
}

function selectSystemDefault() {
  settings.browserPath = "";
  settings.browserName = "";
}

function selectBrowser(browser: BrowserInfo) {
  settings.browserPath = browser.path;
  settings.browserName = browser.name;
}

function isSelected(path: string): boolean {
  return settings.browserPath !== "" && settings.browserPath === path;
}

function fileName(path: string): string {
  return path.split(/[\\/]/).pop() || path;
}

async function pickCustomBrowser() {
  try {
    const picked = await open({
      multiple: false,
      directory: false,
      title: t("settings.pickBrowserTitle"),
      filters: [{ name: t("settings.pickBrowserFilter"), extensions: ["exe"] }],
    });
    if (typeof picked === "string") {
      settings.browserPath = picked;
      settings.browserName = fileName(picked);
    }
  } catch (reason) {
    console.warn(t("errors.browserPickFailed"), reason);
  }
}

const BROWSER_ROW_CLASS =
  "flex w-full items-center justify-between gap-2 rounded-md border px-3 py-2 text-left text-xs transition-colors duration-fast";
const BROWSER_IDLE_CLASS =
  "border-edge text-ink-muted hover:bg-graphite-100 dark:hover:bg-white/5";
const BROWSER_ACTIVE_CLASS = "border-brand bg-brand-soft text-brand";

onMounted(() => {
  void loadBrowsers();
});
</script>

<template>
  <UiDialog
    :title="t('settings.title')"
    :description="t('settings.note')"
    :icon="Settings"
    size="md"
    @close="emit('close')"
  >
    <div class="space-y-5">
      <section>
        <h4 class="text-xs font-semibold text-ink">{{ t("language.interface") }}</h4>
        <p class="mt-1 text-[11px] leading-5 text-ink-subtle">
          {{ t("language.interfaceHint") }}
        </p>
        <div class="mt-2">
          <UiSegmented
            v-model="localeValue"
            :options="localeOptions"
            size="md"
            :aria-label="t('language.interface')"
          />
        </div>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink">{{ t("settings.appearance") }}</h4>
        <p class="mt-1 text-[11px] leading-5 text-ink-subtle">
          {{ t("settings.appearanceHint") }}
        </p>
        <div class="mt-2">
          <UiSegmented
            v-model="themeValue"
            :options="themeChoices"
            size="md"
            :aria-label="t('settings.appearance')"
          />
        </div>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink">{{ t("settings.startup") }}</h4>
        <p class="mt-1 text-[11px] leading-5 text-ink-subtle">
          {{ t("settings.startupHint") }}
        </p>
        <div
          class="mt-2 flex items-center justify-between gap-3 rounded-md border border-edge px-3 py-2 text-xs"
        >
          <span class="min-w-0">
            <span class="block text-ink-muted">
              {{ settings.lastSavePath ? t("settings.remembered") : t("settings.autoSelect") }}
            </span>
            <span
              v-if="settings.lastSavePath"
              class="mt-0.5 block truncate font-mono text-[10px] text-ink-subtle"
            >
              {{ settings.lastSavePath }}
            </span>
          </span>
          <UiButton
            v-if="settings.lastSavePath"
            class="shrink-0"
            @click="forgetLastSave"
          >
            {{ t("settings.switchToAuto") }}
          </UiButton>
        </div>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink">{{ t("settings.searchEngine") }}</h4>
        <p class="mt-1 text-[11px] leading-5 text-ink-subtle">
          {{ t("settings.searchEngineHint") }}
        </p>
        <div class="mt-2">
          <UiSegmented
            v-model="engineValue"
            :options="engineOptions"
            size="md"
            :aria-label="t('settings.searchEngine')"
          />
        </div>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink">{{ t("settings.browser") }}</h4>
        <p class="mt-1 text-[11px] leading-5 text-ink-subtle">
          {{ t("settings.browserHint") }}
        </p>
        <div class="mt-2 space-y-1.5">
          <button
            type="button"
            :class="[
              BROWSER_ROW_CLASS,
              settings.browserPath === '' ? BROWSER_ACTIVE_CLASS : BROWSER_IDLE_CLASS,
            ]"
            @click="selectSystemDefault"
          >
            <span>{{ t("settings.systemDefault") }}</span>
            <Check v-if="settings.browserPath === ''" class="h-3.5 w-3.5 shrink-0" />
          </button>

          <button
            v-for="browser in browsers"
            :key="browser.id"
            type="button"
            :class="[
              BROWSER_ROW_CLASS,
              isSelected(browser.path) ? BROWSER_ACTIVE_CLASS : BROWSER_IDLE_CLASS,
            ]"
            @click="selectBrowser(browser)"
          >
            <span class="min-w-0">
              <span class="block truncate font-medium">{{ browser.name }}</span>
              <span class="mt-0.5 block truncate font-mono text-[10px] text-ink-subtle">
                {{ browser.path }}
              </span>
            </span>
            <Check v-if="isSelected(browser.path)" class="h-3.5 w-3.5 shrink-0" />
          </button>

          <div
            v-if="settings.browserPath !== '' && !browsers.some((b) => b.path === settings.browserPath)"
            :class="[BROWSER_ROW_CLASS, BROWSER_ACTIVE_CLASS]"
          >
            <span class="min-w-0">
              <span class="block truncate font-medium">
                {{ settings.browserName || t("settings.customBrowser") }}
              </span>
              <span class="mt-0.5 block truncate font-mono text-[10px] text-ink-subtle">
                {{ settings.browserPath }}
              </span>
            </span>
            <Check class="h-3.5 w-3.5 shrink-0" />
          </div>

          <button
            type="button"
            class="flex w-full items-center justify-between gap-2 rounded-md border border-dashed border-edge px-3 py-2 text-left text-xs text-ink-muted transition-colors duration-fast hover:border-brand/60 hover:text-ink"
            @click="pickCustomBrowser"
          >
            <span>
              {{ settings.browserPath !== "" ? t("settings.repickBrowser") : t("settings.pickBrowser") }}
            </span>
            <span class="shrink-0 text-[10px] text-ink-subtle">.exe</span>
          </button>
        </div>
        <p
          v-if="browsersLoaded && browsers.length === 0"
          class="mt-1.5 text-[11px] leading-4 text-ink-subtle"
        >
          {{ t("settings.noBrowsers") }}
        </p>
      </section>
    </div>

    <template #footer>
      <UiButton variant="ghost" @click="resetSettings">
        <template #icon>
          <RotateCcw class="h-3.5 w-3.5" />
        </template>
        {{ t("settings.reset") }}
      </UiButton>
      <UiButton variant="primary" @click="emit('close')">{{ t("settings.done") }}</UiButton>
    </template>
  </UiDialog>
</template>
