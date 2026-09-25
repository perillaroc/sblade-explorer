<script setup lang="ts">
import { computed, ref } from "vue";
import { Info, Settings } from "@lucide/vue";
import type { Analysis } from "../types";
import { APP_ICON, categoryIcon, SUMMARY_ICON } from "../lib/icons";
import AboutDialog from "./AboutDialog.vue";
import SettingsDialog from "./SettingsDialog.vue";

const props = defineProps<{
  analysis: Analysis | null;
  active: string;
}>();

const emit = defineEmits<{ navigate: [key: string] }>();

const aboutOpen = ref(false);
const settingsOpen = ref(false);

const overallPercent = computed(() => {
  const summary = props.analysis?.summary;
  if (!summary || summary.catalog_total === 0) return 0;
  return (summary.catalog_obtained / summary.catalog_total) * 100;
});

function categoryPercent(obtained: number, total: number): number {
  if (total === 0) return 0;
  return (obtained / total) * 100;
}

const categoryGroups = computed(() => {
  const categories = props.analysis?.categories ?? [];
  return [
    {
      label: "可收集物分类",
      categories: categories.filter((category) => category.section !== "album"),
    },
    {
      label: "图鉴（不计入总进度）",
      categories: categories.filter((category) => category.section === "album"),
    },
  ];
});
</script>

<template>
  <aside class="flex w-60 shrink-0 flex-col border-r border-slate-800 bg-slate-900/40">
    <div class="flex items-center gap-2 border-b border-slate-800 px-4 py-3">
      <component :is="APP_ICON" class="h-4 w-4 text-emerald-400" />
      <h1 class="text-sm font-bold tracking-wide">剑星存档分析</h1>
    </div>
    <nav class="flex-1 space-y-1 overflow-y-auto p-2">
      <button
        type="button"
        class="w-full rounded px-3 py-2 text-left text-sm transition-colors"
        :class="
          active === 'summary'
            ? 'bg-slate-800 text-white'
            : 'text-slate-300 hover:bg-slate-800/60'
        "
        @click="emit('navigate', 'summary')"
      >
        <div class="flex items-center justify-between gap-2">
          <span class="inline-flex items-center gap-2 font-medium">
            <component :is="SUMMARY_ICON" class="h-4 w-4 text-slate-400" />
            汇总
          </span>
          <span v-if="analysis" class="text-xs text-slate-400">
            {{ analysis.summary.percent.toFixed(0) }}%
          </span>
        </div>
        <div v-if="analysis" class="mt-1.5 h-1 overflow-hidden rounded bg-slate-800">
          <div class="h-full rounded bg-emerald-500" :style="{ width: `${overallPercent}%` }"></div>
        </div>
      </button>

      <template v-if="analysis">
        <template v-for="group in categoryGroups" :key="group.label">
          <div class="px-3 pb-1 pt-3 text-[11px] text-slate-500">{{ group.label }}</div>
          <button
            v-for="category in group.categories"
            :key="category.key"
            type="button"
            class="w-full rounded px-3 py-2 text-left text-sm transition-colors"
            :class="
              active === category.key
                ? 'bg-slate-800 text-white'
                : 'text-slate-300 hover:bg-slate-800/60'
            "
            @click="emit('navigate', category.key)"
          >
            <div class="flex items-center justify-between gap-2">
              <span class="inline-flex min-w-0 items-center gap-2">
                <component
                  :is="categoryIcon(category.key)"
                  class="h-4 w-4 shrink-0 text-slate-400"
                />
                <span class="truncate">{{ category.name }}</span>
              </span>
              <span class="shrink-0 text-xs text-slate-400">
                {{ category.obtained }}/{{ category.total }}
              </span>
            </div>
            <div class="mt-1.5 h-1 overflow-hidden rounded bg-slate-800">
              <div
                class="h-full rounded"
                :class="category.section === 'album' ? 'bg-sky-500' : 'bg-emerald-500'"
                :style="{ width: `${categoryPercent(category.obtained, category.total)}%` }"
              ></div>
            </div>
          </button>
        </template>
      </template>
    </nav>
    <footer class="space-y-1 border-t border-slate-800 p-2">
      <button
        type="button"
        class="flex w-full items-center gap-2 rounded px-3 py-2 text-left text-sm text-slate-400 transition-colors hover:bg-slate-800/60 hover:text-slate-200"
        title="设置默认搜索引擎与打开链接的浏览器"
        @click="settingsOpen = true"
      >
        <Settings class="h-4 w-4 shrink-0" />
        设置
      </button>
      <button
        type="button"
        class="flex w-full items-center gap-2 rounded px-3 py-2 text-left text-sm text-slate-400 transition-colors hover:bg-slate-800/60 hover:text-slate-200"
        title="关于本工具与数据来源"
        @click="aboutOpen = true"
      >
        <Info class="h-4 w-4 shrink-0" />
        关于
      </button>
    </footer>

    <AboutDialog v-if="aboutOpen" @close="aboutOpen = false" />
    <SettingsDialog v-if="settingsOpen" @close="settingsOpen = false" />
  </aside>
</template>
