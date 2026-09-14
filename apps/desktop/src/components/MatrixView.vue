<script setup lang="ts">
import { computed } from "vue";
import type { CategoryResult, Item, Lang, ObtainedItem } from "../types";
import { itemName, localized } from "../lib/display";

const props = defineProps<{
  category: CategoryResult;
  ngPlusCount: number;
  lang: Lang;
}>();

const MATRIX_COLUMNS = ["首周目", "二周目(NG+)", "三周目(NG++)", "DLC/特典"];

const AREA_ORDER: Record<string, number> = {
  Default: 0,
  "Eidos 7": 1,
  Xion: 2,
  Wasteland: 3,
  "Matrix 11": 5,
  "Great Desert": 6,
  "Abyss Levoire": 7,
  "Eidos 9": 8,
  "Spire 4": 9,
  "Boss Challenge": 11,
};

interface MatrixItem {
  id: string;
  name: string;
  name_en?: string | null;
  area?: string | null;
  area_zh?: string | null;
  location?: string | null;
  location_zh?: string | null;
  ng_plus: number;
  dlc: string | null;
  aliases: string[];
  obtained: boolean;
}

interface LocationRow {
  label: string;
  cells: string[][];
}

interface AreaRow {
  label: string;
  locations: LocationRow[];
}

const items = computed<MatrixItem[]>(() => [
  ...props.category.obtained_items.map((item: ObtainedItem) => ({ ...item, obtained: true })),
  ...props.category.missing.map((item: Item) => ({ ...item, obtained: false })),
]);

function emoji(item: MatrixItem): string {
  if (item.obtained) return "✅";
  if (item.dlc) return "🎁";
  if (item.ng_plus > props.ngPlusCount) return "🔒";
  if (item.aliases.length === 0) return "➖";
  return "❌";
}

const areas = computed<AreaRow[]>(() => {
  const map = new Map<string, { label: string; locations: Map<string, LocationRow> }>();
  for (const item of items.value) {
    const areaRaw = item.area ?? "未分类";
    const locationRaw = item.location ?? "未分类";
    let area = map.get(areaRaw);
    if (!area) {
      area = {
        label: localized(item.area_zh, item.area, props.lang) || "未分类",
        locations: new Map(),
      };
      map.set(areaRaw, area);
    }
    let location = area.locations.get(locationRaw);
    if (!location) {
      location = {
        label: localized(item.location_zh, item.location, props.lang) || "未分类",
        cells: [[], [], [], []],
      };
      area.locations.set(locationRaw, location);
    }
    const column = item.dlc ? 3 : Math.min(item.ng_plus, 2);
    location.cells[column].push(`${emoji(item)} ${itemName(item, props.lang)}`);
  }
  return [...map.entries()]
    .sort(
      (left, right) =>
        (AREA_ORDER[left[0]] ?? 99) - (AREA_ORDER[right[0]] ?? 99) ||
        left[0].localeCompare(right[0]),
    )
    .map(([, area]) => ({ label: area.label, locations: [...area.locations.values()] }));
});

function usedColumns(locations: LocationRow[]): number[] {
  return MATRIX_COLUMNS.map((_, index) => index).filter((index) =>
    locations.some((row) => row.cells[index].length > 0),
  );
}
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60 p-4">
    <h2 class="text-sm font-semibold">
      {{ category.name }}
      <span class="ml-2 text-xs font-normal text-slate-400">
        已获得 {{ category.obtained }}/{{ category.total }}
      </span>
    </h2>
    <p class="mt-1 text-xs text-slate-500">✅ 已获得 · ❌ 未获得 · 🔒 需更高周目 · 🎁 DLC/特典 · ➖ 默认外观</p>
    <div v-for="area in areas" :key="area.label" class="mt-3">
      <h3 class="text-xs font-semibold text-slate-300">{{ area.label }}</h3>
      <table class="mt-1 w-full text-xs">
        <thead>
          <tr class="text-left text-slate-500">
            <th class="py-1 pr-2">地点</th>
            <th v-for="index in usedColumns(area.locations)" :key="index" class="py-1 pr-2">
              {{ MATRIX_COLUMNS[index] }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in area.locations" :key="row.label" class="border-t border-slate-800/70 align-top">
            <td class="py-1 pr-2 text-slate-300">{{ row.label }}</td>
            <td v-for="index in usedColumns(area.locations)" :key="index" class="py-1 pr-2">
              <div v-for="(entry, entryIndex) in row.cells[index]" :key="entryIndex">{{ entry }}</div>
              <span v-if="row.cells[index].length === 0" class="text-slate-600">—</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>
