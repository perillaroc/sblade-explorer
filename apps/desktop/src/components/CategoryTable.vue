<script setup lang="ts">
import { ChevronRight, ListChecks } from "@lucide/vue";
import type { Analysis, CategoryResult } from "../types";

defineProps<{ analysis: Analysis }>();

const emit = defineEmits<{ navigate: [key: string] }>();

function percent(category: CategoryResult): number {
  if (category.total === 0) return 0;
  return (category.obtained / category.total) * 100;
}

function blocked(category: CategoryResult): number {
  return category.missing.filter((item) => item.reason !== null).length;
}
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60">
    <header class="flex items-center justify-between border-b border-slate-800 px-4 py-2">
      <h2 class="flex items-center gap-1.5 text-sm font-semibold">
        <ListChecks class="h-4 w-4 text-slate-400" />
        分类汇总
      </h2>
      <span class="text-xs text-slate-500">点击分类查看明细</span>
    </header>
    <table class="w-full text-sm">
      <thead>
        <tr class="text-left text-xs text-slate-500">
          <th class="px-4 py-2">分类</th>
          <th class="px-4 py-2">进度</th>
          <th class="px-4 py-2 text-right">缺失</th>
          <th class="px-4 py-2 text-right">多周目/DLC</th>
          <th class="px-4 py-2"></th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="category in analysis.categories"
          :key="category.key"
          class="cursor-pointer border-t border-slate-800/70 hover:bg-slate-800/40"
          @click="emit('navigate', category.key)"
        >
          <td class="px-4 py-2">{{ category.name }}</td>
          <td class="px-4 py-2">
            <div class="flex items-center gap-2">
              <div class="h-1.5 w-24 overflow-hidden rounded bg-slate-800">
                <div
                  class="h-full rounded bg-emerald-500"
                  :style="{ width: `${percent(category)}%` }"
                ></div>
              </div>
              <span class="text-xs text-slate-400">
                {{ category.obtained }}/{{ category.total }} ({{ percent(category).toFixed(0) }}%)
              </span>
            </div>
          </td>
          <td class="px-4 py-2 text-right">{{ category.missing.length }}</td>
          <td class="px-4 py-2 text-right">{{ blocked(category) }}</td>
          <td class="px-4 py-2 text-right text-xs text-slate-500">
            <span class="inline-flex items-center gap-0.5">
              查看
              <ChevronRight class="h-3.5 w-3.5" />
            </span>
          </td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
