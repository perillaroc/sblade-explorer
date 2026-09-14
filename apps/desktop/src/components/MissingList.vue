<script setup lang="ts">
import { computed } from "vue";
import type { Analysis, Item, Lang } from "../types";
import { itemName, locationLine, obtainLabel } from "../lib/display";

const props = defineProps<{
  analysis: Analysis;
  lang: Lang;
  selected: string | null;
  query: string;
}>();

const emit = defineEmits<{ "update:query": [value: string] }>();

interface Row {
  category: string;
  item: Item;
}

const rows = computed<Row[]>(() => {
  const needle = props.query.trim().toLowerCase();
  return props.analysis.categories
    .filter((category) => !props.selected || category.key === props.selected)
    .flatMap((category) =>
      category.missing.map((item) => ({ category: category.name, item })),
    )
    .filter(({ item }) => {
      if (!needle) return true;
      return [
        item.id,
        item.name,
        item.name_en,
        item.area,
        item.area_zh,
        item.location,
        item.location_zh,
      ].some((value) => (value ?? "").toLowerCase().includes(needle));
    });
});

function onInput(event: Event) {
  emit("update:query", (event.target as HTMLInputElement).value);
}
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60">
    <header class="flex flex-wrap items-center justify-between gap-2 border-b border-slate-800 px-4 py-2">
      <h2 class="text-sm font-semibold">
        缺失清单
        <span class="ml-1 text-xs font-normal text-slate-400">{{ rows.length }} 条</span>
      </h2>
      <input
        class="w-64 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-sm text-slate-100 placeholder:text-slate-600"
        type="search"
        placeholder="搜索名称 / 地点 / ID"
        :value="query"
        @input="onInput"
      />
    </header>
    <div class="max-h-96 overflow-y-auto">
      <table class="w-full text-xs">
        <thead class="sticky top-0 bg-slate-900">
          <tr class="text-left text-slate-500">
            <th class="px-4 py-2">分类</th>
            <th class="px-4 py-2">物品</th>
            <th class="px-4 py-2">位置</th>
            <th class="px-4 py-2">获取</th>
            <th class="px-4 py-2">标记</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="row in rows"
            :key="`${row.category}-${row.item.id}`"
            class="border-t border-slate-800/70 align-top"
          >
            <td class="px-4 py-2 text-slate-400">{{ row.category }}</td>
            <td class="px-4 py-2">
              <div class="text-slate-100">{{ itemName(row.item, lang) }}</div>
              <div class="text-[11px] text-slate-500">{{ row.item.id }}</div>
            </td>
            <td class="px-4 py-2 text-slate-300">{{ locationLine(row.item, lang) || "—" }}</td>
            <td class="px-4 py-2 text-slate-400">{{ obtainLabel(row.item, lang) || "—" }}</td>
            <td class="px-4 py-2">
              <span
                v-for="flag in row.item.flags"
                :key="flag"
                class="mr-1 inline-block rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-300"
              >
                {{ flag }}
              </span>
              <span
                v-if="row.item.reason"
                class="mr-1 inline-block rounded bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-200"
              >
                {{ row.item.reason }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>
