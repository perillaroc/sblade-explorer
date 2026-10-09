<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { LoaderCircle } from "@lucide/vue";
import AppSidebar from "./components/AppSidebar.vue";
import AppTopbar from "./components/AppTopbar.vue";
import CategoryPage from "./components/CategoryPage.vue";
import SaveGuide from "./components/SaveGuide.vue";
import SummaryPage from "./components/SummaryPage.vue";
import { categoryName } from "./lib/display";
import { loadGuides } from "./lib/guides";
import { settings } from "./lib/settings";
import { isNarrow } from "./lib/viewport";
import type { Analysis, SaveSlot, SaveSource } from "./types";

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
  activeCategory.value ? categoryName(activeCategory.value, settings.contentLang) : t("app.summary"),
);

/**
 * Sidebar rail state: the stored preference wins once the user toggles in
 * this session; otherwise narrow windows (< 1100px) auto-collapse.
 */
const sidebarTouched = ref(false);
const sidebarCollapsed = computed(() =>
  sidebarTouched.value
    ? settings.sidebarCollapsed
    : settings.sidebarCollapsed || isNarrow.value,
);

function toggleSidebar() {
  settings.sidebarCollapsed = !sidebarCollapsed.value;
  sidebarTouched.value = true;
}

/**
 * Per-page scroll memory. The scroll container is shared by the kept-alive
 * pages, so positions are saved/restored on page changes.
 */
const mainRef = ref<HTMLElement | null>(null);
const pageScroll = new Map<string, number>();

function rememberScroll() {
  if (mainRef.value) pageScroll.set(page.value, mainRef.value.scrollTop);
}

watch(page, async (next, previous) => {
  if (mainRef.value) pageScroll.set(previous, mainRef.value.scrollTop);
  await nextTick();
  if (mainRef.value) mainRef.value.scrollTop = pageScroll.get(next) ?? 0;
});

watch(loading, async (value, previous) => {
  if (value) {
    rememberScroll();
  } else if (previous) {
    await nextTick();
    if (mainRef.value) mainRef.value.scrollTop = pageScroll.get(page.value) ?? 0;
  }
});

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
      lang: settings.contentLang,
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
  <div class="flex h-screen bg-surface text-ink">
    <AppSidebar
      :analysis="analysis"
      :active="page"
      :collapsed="sidebarCollapsed"
      :saves="allSaves"
      :selected="selected"
      :sources="sources"
      :scanning="scanning"
      @navigate="page = $event"
      @toggle-collapse="toggleSidebar"
      @select-save="selectSave"
      @refresh-saves="refreshSaves"
      @pick-save="pickSaveFile"
      @open-dir="openSaveDir"
    />

    <div class="flex min-w-0 flex-1 flex-col">
      <AppTopbar :title="pageTitle" :analysis="analysis" @export="exportReport" />

      <p
        v-if="notice"
        class="shrink-0 px-4 py-1.5 text-xs"
        :class="noticeError ? 'text-danger' : 'text-success'"
      >
        {{ notice }}
      </p>

      <main
        v-if="loading"
        class="flex min-h-0 flex-1 items-center justify-center gap-2 text-sm text-ink-muted"
      >
        <LoaderCircle class="h-4 w-4 animate-spin" aria-hidden="true" />
        {{ t("common.loading") }}
      </main>
      <main
        v-else-if="scanning && !analysis"
        class="flex min-h-0 flex-1 items-center justify-center gap-2 text-sm text-ink-muted"
      >
        <LoaderCircle class="h-4 w-4 animate-spin" aria-hidden="true" />
        {{ t("app.scanning") }}
      </main>
      <main v-else-if="error || !analysis" class="min-h-0 flex-1 overflow-y-auto p-4">
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
      <main v-else ref="mainRef" class="min-h-0 flex-1 overflow-y-auto">
        <div class="mx-auto w-full max-w-[1600px] p-4">
          <KeepAlive>
            <SummaryPage
              v-if="page === SUMMARY_PAGE"
              :key="SUMMARY_PAGE"
              :analysis="analysis"
              @navigate="page = $event"
            />
            <CategoryPage
              v-else-if="activeCategory"
              :key="activeCategory.key"
              :category="activeCategory"
              :ng-plus-count="analysis.save.ng_plus_count"
              :lang="settings.contentLang"
              :matrix="MATRIX_CATEGORIES.includes(activeCategory.key)"
            />
          </KeepAlive>
        </div>
      </main>
    </div>
  </div>
</template>
