<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Check, RotateCcw, Search, Settings, X } from "@lucide/vue";
import {
  resetSettings,
  SEARCH_ENGINES,
  settings,
  type BrowserInfo,
  type SearchEngineId,
} from "../lib/settings";
import { themeOptions } from "../lib/theme";
import { UI_LOCALES, type UiLocale } from "../locales";

const { t } = useI18n({ useScope: "global" });

const emit = defineEmits<{ close: [] }>();

const browsers = ref<BrowserInfo[]>([]);
const browsersLoaded = ref(false);

const themeChoices = computed(() => themeOptions());

const localeOptions = computed<{ value: UiLocale; label: string }[]>(() =>
  UI_LOCALES.map((value) => ({ value, label: t(`language.${value}`) })),
);

async function loadBrowsers() {
  try {
    browsers.value = await invoke<BrowserInfo[]>("list_browsers");
  } catch (reason) {
    console.warn(t("errors.browserListFailed"), reason);
  } finally {
    browsersLoaded.value = true;
  }
}

function selectEngine(id: SearchEngineId) {
  settings.searchEngine = id;
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

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") emit("close");
}

onMounted(() => {
  window.addEventListener("keydown", onKeydown);
  void loadBrowsers();
});
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/30 dark:bg-slate-950/70 p-4"
      @click.self="emit('close')"
    >
      <section
        class="flex max-h-[85vh] w-[36rem] max-w-full flex-col rounded-lg border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-200 dark:border-slate-800 px-4 py-3">
          <div class="flex items-start gap-2">
            <Settings class="mt-0.5 h-4 w-4 shrink-0 text-emerald-600 dark:text-emerald-400" />
            <div>
              <h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">{{ t("settings.title") }}</h3>
              <p class="mt-0.5 text-xs text-slate-500">{{ t("settings.note") }}</p>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            {{ t("common.close") }}
          </button>
        </header>

        <div class="space-y-5 overflow-y-auto px-4 py-3">
          <section>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("language.interface") }}</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              {{ t("language.interfaceHint") }}
            </p>
            <div class="mt-2 grid grid-cols-3 gap-2">
              <button
                v-for="option in localeOptions"
                :key="option.value"
                type="button"
                class="inline-flex items-center justify-center gap-1.5 rounded border px-2 py-1.5 text-xs"
                :class="
                  settings.uiLocale === option.value
                    ? 'border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-200'
                    : 'border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800'
                "
                @click="settings.uiLocale = option.value"
              >
                {{ option.label }}
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("settings.appearance") }}</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              {{ t("settings.appearanceHint") }}
            </p>
            <div class="mt-2 grid grid-cols-3 gap-2">
              <button
                v-for="option in themeChoices"
                :key="option.value"
                type="button"
                class="inline-flex items-center justify-center gap-1.5 rounded border px-2 py-1.5 text-xs"
                :class="
                  settings.theme === option.value
                    ? 'border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-200'
                    : 'border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800'
                "
                @click="settings.theme = option.value"
              >
                <component :is="option.icon" class="h-3.5 w-3.5" />
                {{ option.label }}
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("settings.startup") }}</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              {{ t("settings.startupHint") }}
            </p>
            <div
              class="mt-2 flex items-center justify-between gap-3 rounded border border-slate-300 dark:border-slate-700 px-3 py-2 text-xs"
            >
              <span class="min-w-0">
                <span class="block text-slate-700 dark:text-slate-300">
                  {{ settings.lastSavePath ? t("settings.remembered") : t("settings.autoSelect") }}
                </span>
                <span
                  v-if="settings.lastSavePath"
                  class="mt-0.5 block truncate font-mono text-[10px] text-slate-500"
                >
                  {{ settings.lastSavePath }}
                </span>
              </span>
              <button
                v-if="settings.lastSavePath"
                type="button"
                class="shrink-0 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800"
                @click="forgetLastSave"
              >
                {{ t("settings.switchToAuto") }}
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("settings.searchEngine") }}</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              {{ t("settings.searchEngineHint") }}
            </p>
            <div class="mt-2 grid grid-cols-3 gap-2">
              <button
                v-for="engine in SEARCH_ENGINES"
                :key="engine.id"
                type="button"
                class="inline-flex items-center justify-center gap-1.5 rounded border px-2 py-1.5 text-xs"
                :class="
                  settings.searchEngine === engine.id
                    ? 'border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-200'
                    : 'border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800'
                "
                @click="selectEngine(engine.id)"
              >
                <Search class="h-3.5 w-3.5" />
                {{ t(`settings.engines.${engine.id}`) }}
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("settings.browser") }}</h4>
            <p class="mt-1 text-[11px] leading-5 text-slate-500">
              {{ t("settings.browserHint") }}
            </p>
            <div class="mt-2 space-y-1.5">
              <button
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border px-3 py-2 text-left text-xs"
                :class="
                  settings.browserPath === ''
                    ? 'border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-200'
                    : 'border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800'
                "
                @click="selectSystemDefault"
              >
                <span>{{ t("settings.systemDefault") }}</span>
                <Check v-if="settings.browserPath === ''" class="h-3.5 w-3.5 shrink-0" />
              </button>

              <button
                v-for="browser in browsers"
                :key="browser.id"
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border px-3 py-2 text-left text-xs"
                :class="
                  isSelected(browser.path)
                    ? 'border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-200'
                    : 'border-slate-300 dark:border-slate-700 text-slate-700 dark:text-slate-300 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800'
                "
                @click="selectBrowser(browser)"
              >
                <span class="min-w-0">
                  <span class="block truncate font-medium">{{ browser.name }}</span>
                  <span class="mt-0.5 block truncate font-mono text-[10px] text-slate-500">
                    {{ browser.path }}
                  </span>
                </span>
                <Check v-if="isSelected(browser.path)" class="h-3.5 w-3.5 shrink-0" />
              </button>

              <div
                v-if="settings.browserPath !== '' && !browsers.some((b) => b.path === settings.browserPath)"
                class="flex w-full items-center justify-between gap-2 rounded border border-emerald-600 bg-emerald-50 dark:bg-emerald-900/30 px-3 py-2 text-left text-xs text-emerald-700 dark:text-emerald-200"
              >
                <span class="min-w-0">
                  <span class="block truncate font-medium">
                    {{ settings.browserName || t("settings.customBrowser") }}
                  </span>
                  <span class="mt-0.5 block truncate font-mono text-[10px] text-emerald-700/70 dark:text-emerald-300/70">
                    {{ settings.browserPath }}
                  </span>
                </span>
                <Check class="h-3.5 w-3.5 shrink-0" />
              </div>

              <button
                type="button"
                class="flex w-full items-center justify-between gap-2 rounded border border-dashed border-slate-300 dark:border-slate-700 px-3 py-2 text-left text-xs text-slate-600 dark:text-slate-400 hover:border-slate-400 dark:hover:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800"
                @click="pickCustomBrowser"
              >
                <span>
                  {{ settings.browserPath !== '' ? t("settings.repickBrowser") : t("settings.pickBrowser") }}
                </span>
                <span class="shrink-0 text-[10px] text-slate-500">.exe</span>
              </button>
            </div>
            <p v-if="browsersLoaded && browsers.length === 0" class="mt-1.5 text-[11px] leading-4 text-slate-500">
              {{ t("settings.noBrowsers") }}
            </p>
          </section>

          <div class="flex items-center justify-between border-t border-slate-200 dark:border-slate-800 pt-3">
            <button
              type="button"
              class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800"
              @click="resetSettings"
            >
              <RotateCcw class="h-3.5 w-3.5" />
              {{ t("settings.reset") }}
            </button>
            <button
              type="button"
              class="rounded border border-emerald-700 px-3 py-1 text-xs text-emerald-700 dark:text-emerald-200 hover:bg-emerald-50 dark:hover:bg-emerald-900/30"
              @click="emit('close')"
            >
              {{ t("settings.done") }}
            </button>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
