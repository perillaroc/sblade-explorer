<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import {
  FileJson,
  FileText,
  FolderSearch,
  Languages,
  LoaderCircle,
  TriangleAlert,
} from "@lucide/vue";
import AppSidebar from "./components/AppSidebar.vue";
import CategoryPage from "./components/CategoryPage.vue";
import SavePicker from "./components/SavePicker.vue";
import SummaryPage from "./components/SummaryPage.vue";
import { loadGuides } from "./lib/guides";
import type { Analysis, Lang, SaveSlot } from "./types";

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

const LANG_OPTIONS: { value: Lang; label: string }[] = [
  { value: "zh", label: "中文" },
  { value: "en", label: "English" },
  { value: "both", label: "双语" },
];

const saves = ref<SaveSlot[]>([]);
const selected = ref<SaveSlot | null>(null);
const analysis = ref<Analysis | null>(null);
const loading = ref(false);
const error = ref("");
const lang = ref<Lang>("zh");
const page = ref(SUMMARY_PAGE);
const notice = ref("");

const activeCategory = computed(
  () => analysis.value?.categories.find((category) => category.key === page.value) ?? null,
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

async function refreshSaves() {
  try {
    saves.value = await invoke<SaveSlot[]>("list_saves");
  } catch (reason) {
    error.value = String(reason);
    return;
  }
  if (selected.value === null && saves.value.length > 0) {
    await selectSave(saves.value[0]);
  }
}

async function selectSave(slot: SaveSlot) {
  selected.value = slot;
  loading.value = true;
  error.value = "";
  notice.value = "";
  try {
    analysis.value = await invoke<Analysis>("analyze_save", { path: slot.path });
  } catch (reason) {
    analysis.value = null;
    error.value = String(reason);
  } finally {
    loading.value = false;
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
    });
    notice.value = `已导出 ${target}`;
  } catch (reason) {
    error.value = String(reason);
  }
}

onMounted(() => {
  void loadGuides();
  void refreshSaves();
});
</script>

<template>
  <div class="flex h-screen bg-slate-950 text-slate-100">
    <AppSidebar :analysis="analysis" :active="page" @navigate="page = $event" />

    <div class="flex min-w-0 flex-1 flex-col">
      <header
        class="flex flex-wrap items-center justify-between gap-3 border-b border-slate-800 px-4 py-3"
      >
        <div class="flex items-baseline gap-3">
          <h2 class="text-sm font-semibold">
            {{ activeCategory ? activeCategory.name : "汇总" }}
          </h2>
          <span v-if="analysis" class="text-xs text-slate-500">
            目录进度 {{ analysis.summary.catalog_obtained }}/{{ analysis.summary.catalog_total }}
            ({{ analysis.summary.percent.toFixed(1) }}%) · 图鉴
            {{ analysis.summary.album_obtained }}/{{ analysis.summary.album_total }}
            ({{ analysis.summary.album_percent.toFixed(1) }}%)
          </span>
        </div>
        <div class="flex items-center gap-2">
          <Languages class="h-4 w-4 text-slate-500" />
          <div class="flex overflow-hidden rounded border border-slate-700 text-xs">
            <button
              v-for="option in LANG_OPTIONS"
              :key="option.value"
              type="button"
              class="px-2 py-1"
              :class="
                lang === option.value ? 'bg-slate-700 text-white' : 'text-slate-400 hover:bg-slate-800'
              "
              @click="lang = option.value"
            >
              {{ option.label }}
            </button>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="exportReport('json')"
          >
            <FileJson class="h-3.5 w-3.5" />
            导出 JSON
          </button>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="exportReport('markdown')"
          >
            <FileText class="h-3.5 w-3.5" />
            导出 Markdown
          </button>
        </div>
      </header>

      <SavePicker :saves="saves" :selected="selected" @select="selectSave" @refresh="refreshSaves" />

      <p v-if="notice" class="px-4 py-1 text-xs text-emerald-400">{{ notice }}</p>

      <main v-if="loading" class="flex flex-1 items-center justify-center text-sm text-slate-400">
        <LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
        读取存档中…
      </main>
      <main
        v-else-if="error"
        class="flex flex-1 items-center justify-center text-sm text-rose-400"
      >
        <TriangleAlert class="mr-2 h-4 w-4" />
        读取存档失败: {{ error }}
      </main>
      <main
        v-else-if="!analysis"
        class="flex flex-1 items-center justify-center text-sm text-slate-400"
      >
        <FolderSearch class="mr-2 h-4 w-4" />
        未找到存档，请将存档放入默认目录后点击刷新。
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
