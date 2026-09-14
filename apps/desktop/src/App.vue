<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import CategoryTable from "./components/CategoryTable.vue";
import MatrixView from "./components/MatrixView.vue";
import MissingList from "./components/MissingList.vue";
import SavePicker from "./components/SavePicker.vue";
import SummaryPanel from "./components/SummaryPanel.vue";
import type { Analysis, Lang, SaveSlot } from "./types";

const MATRIX_CATEGORIES = [
  "nano_suits",
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
const categoryFilter = ref<string | null>(null);
const query = ref("");
const notice = ref("");

const matrixCategories = computed(() =>
  (analysis.value?.categories ?? []).filter((category) =>
    MATRIX_CATEGORIES.includes(category.key),
  ),
);

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

function onSelectCategory(key: string | null) {
  categoryFilter.value = key;
}

function onUpdateQuery(value: string) {
  query.value = value;
}

onMounted(refreshSaves);
</script>

<template>
  <div class="flex h-screen flex-col bg-slate-950 text-slate-100">
    <header class="flex flex-wrap items-center justify-between gap-3 border-b border-slate-800 px-4 py-3">
      <div class="flex items-baseline gap-3">
        <h1 class="text-lg font-bold tracking-wide">剑星存档分析</h1>
        <span v-if="analysis" class="text-xs text-slate-500">
          目录进度 {{ analysis.summary.catalog_obtained }}/{{ analysis.summary.catalog_total }}
          ({{ analysis.summary.percent.toFixed(1) }}%)
        </span>
      </div>
      <div class="flex items-center gap-2">
        <div class="flex overflow-hidden rounded border border-slate-700 text-xs">
          <button
            v-for="option in LANG_OPTIONS"
            :key="option.value"
            type="button"
            class="px-2 py-1"
            :class="lang === option.value ? 'bg-slate-700 text-white' : 'text-slate-400 hover:bg-slate-800'"
            @click="lang = option.value"
          >
            {{ option.label }}
          </button>
        </div>
        <button
          type="button"
          class="rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
          @click="exportReport('json')"
        >
          导出 JSON
        </button>
        <button
          type="button"
          class="rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
          @click="exportReport('markdown')"
        >
          导出 Markdown
        </button>
      </div>
    </header>

    <SavePicker :saves="saves" :selected="selected" @select="selectSave" @refresh="refreshSaves" />

    <p v-if="notice" class="px-4 py-1 text-xs text-emerald-400">{{ notice }}</p>

    <main v-if="loading" class="flex flex-1 items-center justify-center text-sm text-slate-400">
      读取存档中…
    </main>
    <main v-else-if="error" class="flex flex-1 items-center justify-center text-sm text-rose-400">
      读取存档失败: {{ error }}
    </main>
    <main
      v-else-if="!analysis"
      class="flex flex-1 items-center justify-center text-sm text-slate-400"
    >
      未找到存档，请将存档放入默认目录后点击刷新。
    </main>
    <main v-else class="flex-1 space-y-4 overflow-y-auto p-4">
      <SummaryPanel :analysis="analysis" />
      <CategoryTable :analysis="analysis" :selected="categoryFilter" @select="onSelectCategory" />
      <MatrixView
        v-for="category in matrixCategories"
        :key="category.key"
        :category="category"
        :ng-plus-count="analysis.save.ng_plus_count"
        :lang="lang"
      />
      <MissingList
        :analysis="analysis"
        :lang="lang"
        :selected="categoryFilter"
        :query="query"
        @update:query="onUpdateQuery"
      />
    </main>
  </div>
</template>
