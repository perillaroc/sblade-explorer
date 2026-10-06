<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { FileJson, FileText, Languages, LoaderCircle } from "@lucide/vue";
import AppSidebar from "./components/AppSidebar.vue";
import CategoryPage from "./components/CategoryPage.vue";
import SaveGuide from "./components/SaveGuide.vue";
import SavePicker from "./components/SavePicker.vue";
import SummaryPage from "./components/SummaryPage.vue";
import { categoryName } from "./lib/display";
import { loadGuides } from "./lib/guides";
import { settings } from "./lib/settings";
import { themeOptions } from "./lib/theme";
import type { Analysis, Lang, SaveSlot, SaveSource } from "./types";

const { t } = useI18n({ useScope: "global" });

const SUMMARY_PAGE = "summary";

const MATRIX_CATEGORIES = [
  "nano_suits",
  "design_patterns",
  "earrings",
  "glasses",
  "drone_seals",
  "adam_costumes",
  "lily_costumes",
];

const lang = computed<Lang>({
  get: () => settings.contentLang,
  set: (value) => {
    settings.contentLang = value;
  },
});

const langOptions = computed<{ value: Lang; label: string }[]>(() => [
  { value: "zh", label: t("language.zh") },
  { value: "en", label: t("language.en") },
  { value: "both", label: t("language.both") },
]);

const themeChoices = computed(() => themeOptions());

const saves = ref<SaveSlot[]>([]);
const manualSlots = ref<SaveSlot[]>([]);
const sources = ref<SaveSource[]>([]);
const selected = ref<SaveSlot | null>(null);
const analysis = ref<Analysis | null>(null);
const scanning = ref(false);
const loading = ref(false);
const error = ref("");
const page = ref(SUMMARY_PAGE);
const notice = ref("");
const noticeError = ref(false);

/** Automatically discovered slots plus files picked manually this session. */
const allSaves = computed(() => {
  const discovered = new Set(saves.value.map((slot) => slot.path));
  return [...saves.value, ...manualSlots.value.filter((slot) => !discovered.has(slot.path))];
});

const activeCategory = computed(
  () => analysis.value?.categories.find((category) => category.key === page.value) ?? null,
);

const pageTitle = computed(() =>
  activeCategory.value ? categoryName(activeCategory.value, lang.value) : t("app.summary"),
);

watch(analysis, (value) => {
  if (
    value &&
    page.value !== SUMMARY_PAGE &&
    !value.categories.some((category) => category.key === page.value)
  ) {
    page.value = SUMMARY_PAGE;
  }
});

function showNotice(message: string, isError = false) {
  notice.value = message;
  noticeError.value = isError;
}

/** Re-checks slots picked manually; files that vanished are dropped. */
async function syncManualSlots() {
  const kept: SaveSlot[] = [];
  for (const slot of manualSlots.value) {
    if (saves.value.some((candidate) => candidate.path === slot.path)) {
      continue;
    }
    try {
      kept.push(
        await invoke<SaveSlot>("inspect_save", { path: slot.path, locale: settings.uiLocale }),
      );
    } catch {
      if (selected.value?.path === slot.path) {
        selected.value = null;
        analysis.value = null;
        if (settings.lastSavePath === slot.path) {
          settings.lastSavePath = "";
        }
      }
    }
  }
  manualSlots.value = kept;
}

async function refreshSaves() {
  scanning.value = true;
  try {
    const [discovered, sourceList] = await Promise.all([
      invoke<SaveSlot[]>("list_saves"),
      invoke<SaveSource[]>("save_sources"),
    ]);
    saves.value = discovered;
    sources.value = sourceList;
  } catch (reason) {
    error.value = String(reason);
    return;
  } finally {
    scanning.value = false;
  }
  await syncManualSlots();
  if (selected.value === null) {
    // A scan error would leave a stale message, so clear it before selecting.
    error.value = "";
    await selectPreferredSave();
  }
}

/** Opens the remembered save when it still exists, else the newest slot. */
async function selectPreferredSave() {
  const remembered = settings.lastSavePath;
  if (remembered) {
    const known = allSaves.value.find((slot) => slot.path === remembered);
    if (known) {
      await selectSave(known);
      return;
    }
    try {
      const slot = await invoke<SaveSlot>("inspect_save", {
        path: remembered,
        locale: settings.uiLocale,
      });
      manualSlots.value.push(slot);
      await selectSave(slot);
      return;
    } catch {
      settings.lastSavePath = "";
    }
  }
  const first = allSaves.value[0];
  if (first) {
    await selectSave(first, false);
  }
}

async function selectSave(slot: SaveSlot, remember = true) {
  selected.value = slot;
  loading.value = true;
  error.value = "";
  showNotice("");
  try {
    analysis.value = await invoke<Analysis>("analyze_save", {
      path: slot.path,
      locale: settings.uiLocale,
    });
    if (remember) {
      settings.lastSavePath = slot.path;
    }
  } catch (reason) {
    analysis.value = null;
    error.value = String(reason);
  } finally {
    loading.value = false;
  }
}

async function pickSaveFile() {
  try {
    const existing = sources.value.find((source) => source.exists);
    const picked = await open({
      multiple: false,
      directory: false,
      title: t("app.pickTitle"),
      defaultPath: existing?.path,
      filters: [{ name: t("app.pickFilter"), extensions: ["sav"] }],
    });
    if (typeof picked !== "string") return;
    const slot = await invoke<SaveSlot>("inspect_save", {
      path: picked,
      locale: settings.uiLocale,
    });
    if (!allSaves.value.some((candidate) => candidate.path === slot.path)) {
      manualSlots.value.push(slot);
    }
    await selectSave(slot);
  } catch (reason) {
    showNotice(t("app.pickError", { reason: String(reason) }), true);
  }
}

async function openSaveDir(path: string) {
  try {
    await invoke("open_save_dir", { path, locale: settings.uiLocale });
  } catch (reason) {
    showNotice(t("app.openDirError", { reason: String(reason) }), true);
  }
}

async function exportReport(format: "json" | "markdown") {
  if (!selected.value) return;
  const extension = format === "json" ? "json" : "md";
  const target = await save({
    defaultPath: `sblade-report.${extension}`,
    filters: [{ name: format === "json" ? "JSON" : "Markdown", extensions: [extension] }],
  });
  if (!target) return;
  try {
    await invoke("export_report", {
      path: selected.value.path,
      lang: lang.value,
      format,
      outPath: target,
      locale: settings.uiLocale,
    });
    showNotice(t("app.exported", { path: target }));
  } catch (reason) {
    showNotice(t("app.exportError", { reason: String(reason) }), true);
  }
}

onMounted(() => {
  void loadGuides();
  void refreshSaves();
});
</script>

<template>
  <div class="flex h-screen bg-slate-100 dark:bg-slate-950 text-slate-900 dark:text-slate-100">
    <AppSidebar :analysis="analysis" :active="page" @navigate="page = $event" />

    <div class="flex min-w-0 flex-1 flex-col">
      <header
        class="flex flex-wrap items-center justify-between gap-3 border-b border-slate-200 dark:border-slate-800 px-4 py-3"
      >
        <div class="flex items-baseline gap-3">
          <h2 class="text-sm font-semibold">
            {{ pageTitle }}
          </h2>
          <span v-if="analysis" class="text-xs text-slate-500">
            {{ t("app.catalogProgress", { obtained: analysis.summary.catalog_obtained, total: analysis.summary.catalog_total }) }}
            ({{ analysis.summary.percent.toFixed(1) }}%) ·
            {{ t("app.albumProgressShort", { obtained: analysis.summary.album_obtained, total: analysis.summary.album_total }) }}
            ({{ analysis.summary.album_percent.toFixed(1) }}%)
          </span>
        </div>
        <div class="flex items-center gap-2">
          <span
            class="inline-flex items-center gap-1.5"
            :title="t('language.contentHint')"
          >
            <Languages class="h-4 w-4 text-slate-500" />
            <span class="text-xs text-slate-500">{{ t("language.contentShort") }}</span>
          </span>
          <div
            class="flex overflow-hidden rounded border border-slate-300 dark:border-slate-700 text-xs"
            :title="t('language.contentHint')"
            :aria-label="t('language.contentLabel')"
          >
            <button
              v-for="option in langOptions"
              :key="option.value"
              type="button"
              class="px-2 py-1"
              :class="
                lang === option.value ? 'bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-white' : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
              "
              @click="lang = option.value"
            >
              {{ option.label }}
            </button>
          </div>
          <div class="flex overflow-hidden rounded border border-slate-300 dark:border-slate-700">
            <button
              v-for="option in themeChoices"
              :key="option.value"
              type="button"
              class="inline-flex items-center px-2 py-1"
              :class="
                settings.theme === option.value
                  ? 'bg-slate-200 dark:bg-slate-700 text-slate-900 dark:text-white'
                  : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
              "
              :title="option.label"
              :aria-label="t('app.appearance', { name: option.label })"
              :aria-pressed="settings.theme === option.value"
              @click="settings.theme = option.value"
            >
              <component :is="option.icon" class="h-3.5 w-3.5" />
            </button>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="exportReport('json')"
          >
            <FileJson class="h-3.5 w-3.5" />
            {{ t("app.exportJson") }}
          </button>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="exportReport('markdown')"
          >
            <FileText class="h-3.5 w-3.5" />
            {{ t("app.exportMarkdown") }}
          </button>
        </div>
      </header>

      <SavePicker
        :saves="allSaves"
        :selected="selected"
        :scanning="scanning"
        @select="selectSave"
        @refresh="refreshSaves"
        @pick="pickSaveFile"
      />

      <p
        v-if="notice"
        class="px-4 py-1 text-xs"
        :class="noticeError ? 'text-rose-600 dark:text-rose-400' : 'text-emerald-600 dark:text-emerald-400'"
      >
        {{ notice }}
      </p>

      <main v-if="loading" class="flex flex-1 items-center justify-center text-sm text-slate-600 dark:text-slate-400">
        <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
        {{ t("common.loading") }}
      </main>
      <main
        v-else-if="scanning && !analysis"
        class="flex flex-1 items-center justify-center text-sm text-slate-600 dark:text-slate-400"
      >
        <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
        {{ t("app.scanning") }}
      </main>
      <main v-else-if="error || !analysis" class="flex-1 overflow-y-auto p-4">
        <SaveGuide
          :sources="sources"
          :error="error"
          :scanning="scanning"
          :active-path="selected?.path ?? null"
          @pick="pickSaveFile"
          @refresh="refreshSaves"
          @open="openSaveDir"
        />
      </main>
      <main v-else class="flex-1 overflow-y-auto p-4">
        <SummaryPage
          v-if="page === SUMMARY_PAGE"
          :analysis="analysis"
          @navigate="page = $event"
        />
        <CategoryPage
          v-else-if="activeCategory"
          :key="activeCategory.key"
          :category="activeCategory"
          :ng-plus-count="analysis.save.ng_plus_count"
          :lang="lang"
          :matrix="MATRIX_CATEGORIES.includes(activeCategory.key)"
        />
      </main>
    </div>
  </div>
</template>
