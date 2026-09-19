<script setup lang="ts">
import { computed, ref } from "vue";
import type { CategoryResult, ItemFilter, Lang } from "../types";
import { categoryRows, matchesQuery, type ItemRow } from "../lib/items";
import ItemTable from "./ItemTable.vue";
import MatrixView from "./MatrixView.vue";

type ViewMode = "matrix" | "list";

interface RecordGroup {
  key: string;
  name: string;
  obtained: number;
  total: number;
  rows: ItemRow[];
}

const RECORD_TYPE_ORDER = [
  "memorystick",
  "document_log_data",
  "document_journal",
  "document_messages",
  "document_announcements",
  "document_series",
  "document_books",
  "document_information",
  "document_promotions",
  "document_prayers",
];

const props = defineProps<{
  category: CategoryResult;
  ngPlusCount: number;
  lang: Lang;
  matrix: boolean;
}>();

const FILTER_OPTIONS: { value: ItemFilter; label: string }[] = [
  { value: "all", label: "全部" },
  { value: "obtained", label: "已收集" },
  { value: "missing", label: "未收集" },
];

const VIEW_OPTIONS: { value: ViewMode; label: string }[] = [
  { value: "matrix", label: "周目矩阵" },
  { value: "list", label: "列表" },
];

const filter = ref<ItemFilter>("all");
const query = ref("");
const view = ref<ViewMode>(props.matrix ? "matrix" : "list");

const rows = computed(() =>
  categoryRows(props.category, filter.value).filter((row) => matchesQuery(row.item, query.value)),
);

const allRows = computed(() => categoryRows(props.category, "all"));

const isRecords = computed(() => props.category.key === "records");

const recordGroups = computed<RecordGroup[]>(() => {
  if (!isRecords.value) return [];
  const groups = new Map<string, RecordGroup>();
  const ensure = (row: ItemRow) => {
    const key = row.item.record_type ?? "unknown";
    let group = groups.get(key);
    if (!group) {
      group = {
        key,
        name: row.item.record_type_zh ?? "未分类",
        obtained: 0,
        total: 0,
        rows: [],
      };
      groups.set(key, group);
    }
    return group;
  };
  for (const row of allRows.value) {
    const group = ensure(row);
    group.total += 1;
    if (row.obtained) group.obtained += 1;
  }
  for (const row of rows.value) ensure(row).rows.push(row);
  const order = new Map(RECORD_TYPE_ORDER.map((key, index) => [key, index]));
  return [...groups.values()]
    .filter((group) => group.rows.length > 0)
    .sort((left, right) => (order.get(left.key) ?? 99) - (order.get(right.key) ?? 99));
});

const percent = computed(() =>
  props.category.total === 0 ? 0 : (props.category.obtained / props.category.total) * 100,
);

function count(value: ItemFilter): number {
  if (value === "obtained") return props.category.obtained;
  if (value === "missing") return props.category.missing.length;
  return props.category.total;
}

function onInput(event: Event) {
  query.value = (event.target as HTMLInputElement).value;
}
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60">
    <header class="space-y-3 border-b border-slate-800 px-4 py-3">
      <div class="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h2 class="text-base font-semibold">{{ category.name }}</h2>
          <p class="mt-0.5 text-xs text-slate-400">
            已收集 {{ category.obtained }}/{{ category.total }} · 未收集
            {{ category.missing.length }} · {{ percent.toFixed(0) }}%
          </p>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          <div
            v-if="matrix"
            class="flex overflow-hidden rounded border border-slate-700 text-xs"
          >
            <button
              v-for="option in VIEW_OPTIONS"
              :key="option.value"
              type="button"
              class="px-2 py-1"
              :class="
                view === option.value
                  ? 'bg-slate-700 text-white'
                  : 'text-slate-400 hover:bg-slate-800'
              "
              @click="view = option.value"
            >
              {{ option.label }}
            </button>
          </div>
          <input
            class="w-64 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-sm text-slate-100 placeholder:text-slate-600"
            type="search"
            placeholder="搜索名称 / 地点 / ID"
            :value="query"
            @input="onInput"
          />
        </div>
      </div>

      <div class="h-1.5 overflow-hidden rounded bg-slate-800">
        <div class="h-full rounded bg-emerald-500" :style="{ width: `${percent}%` }"></div>
      </div>

      <div class="flex overflow-hidden rounded border border-slate-700 text-xs">
        <button
          v-for="option in FILTER_OPTIONS"
          :key="option.value"
          type="button"
          class="px-3 py-1"
          :class="
            filter === option.value ? 'bg-slate-700 text-white' : 'text-slate-400 hover:bg-slate-800'
          "
          @click="filter = option.value"
        >
          {{ option.label }}
          <span class="ml-1 text-slate-500">{{ count(option.value) }}</span>
        </button>
      </div>
    </header>

    <p v-if="rows.length === 0" class="px-4 py-10 text-center text-xs text-slate-500">
      没有符合条件的物件
    </p>
    <MatrixView
      v-else-if="matrix && view === 'matrix'"
      :rows="rows"
      :all-rows="allRows"
      :ng-plus-count="ngPlusCount"
      :lang="lang"
    />
    <template v-else-if="isRecords">
      <section v-for="group in recordGroups" :key="group.key">
        <header
          class="flex flex-wrap items-baseline justify-between gap-2 border-t border-slate-800 bg-slate-900/80 px-4 py-2"
        >
          <h3 class="text-xs font-semibold text-slate-200">{{ group.name }}</h3>
          <span class="text-[11px] text-slate-500">
            已收集 {{ group.obtained }}/{{ group.total }}
          </span>
        </header>
        <ItemTable :rows="group.rows" :lang="lang" />
      </section>
    </template>
    <ItemTable v-else :rows="rows" :lang="lang" />
  </section>

  <p v-if="category.extra_obtained_aliases.length > 0" class="mt-2 px-1 text-[11px] text-slate-500">
    另有 {{ category.extra_obtained_aliases.length }} 个已获得别名未关联到目录条目。
  </p>
</template>
