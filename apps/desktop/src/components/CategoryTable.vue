<script setup lang="ts">
import type { Analysis, CategoryResult } from "../types";

const props = defineProps<{
  analysis: Analysis;
  selected: string | null;
}>();

const emit = defineEmits<{ select: [key: string | null] }>();

function toggle(key: string) {
  emit("select", props.selected === key ? null : key);
}

function percent(category: CategoryResult): string {
  if (category.total === 0) return "0";
  return ((category.obtained / category.total) * 100).toFixed(0);
}

function blocked(category: CategoryResult): number {
  return category.missing.filter((item) => item.reason !== null).length;
}
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60">
    <header class="flex items-center justify-between border-b border-slate-800 px-4 py-2">
      <h2 class="text-sm font-semibold">分类汇总</h2>
      <button
        v-if="selected"
        type="button"
        class="text-xs text-slate-400 hover:text-slate-200"
        @click="emit('select', null)"
      >
        清除筛选
      </button>
    </header>
    <table class="w-full text-sm">
      <thead>
        <tr class="text-left text-xs text-slate-500">
          <th class="px-4 py-2">分类</th>
          <th class="px-4 py-2 text-right">进度</th>
          <th class="px-4 py-2 text-right">缺失</th>
          <th class="px-4 py-2 text-right">多周目/DLC</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="category in analysis.categories"
          :key="category.key"
          class="cursor-pointer border-t border-slate-800/70 hover:bg-slate-800/40"
          :class="{ 'bg-slate-800/60': selected === category.key }"
          @click="toggle(category.key)"
        >
          <td class="px-4 py-2">{{ category.name }}</td>
          <td class="px-4 py-2 text-right">
            {{ category.obtained }}/{{ category.total }} ({{ percent(category) }}%)
          </td>
          <td class="px-4 py-2 text-right">{{ category.missing.length }}</td>
          <td class="px-4 py-2 text-right">{{ blocked(category) }}</td>
        </tr>
      </tbody>
    </table>
  </section>
</template>
