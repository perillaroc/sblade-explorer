<script setup lang="ts">
import { computed, ref, watch } from "vue";
import {
  ChevronDown,
  ChevronRight,
  LayoutGrid,
  List,
  ListFilter,
  Search,
  SearchX,
  type LucideIcon,
} from "@lucide/vue";
import type { CategoryResult, ItemFilter, Lang } from "../types";
import { areaAccent } from "../lib/area";
import { categoryRows, matchesQuery, type ItemRow } from "../lib/items";
import ItemTable from "./ItemTable.vue";
import MatrixView from "./MatrixView.vue";

type ViewMode = "matrix" | "list";

interface RecordAreaGroup {
  key: string;
  label: string;
  obtained: number;
  total: number;
  rows: ItemRow[];
}

interface RecordGroup {
  key: string;
  name: string;
  obtained: number;
  total: number;
  rows: ItemRow[];
  areas: RecordAreaGroup[];
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

const VIEW_OPTIONS: { value: ViewMode; label: string; icon: LucideIcon }[] = [
  { value: "matrix", label: "周目矩阵", icon: LayoutGrid },
  { value: "list", label: "列表", icon: List },
];

const filter = ref<ItemFilter>("all");
const query = ref("");
const view = ref<ViewMode>(props.matrix ? "matrix" : "list");
const activeRecordType = ref<string | null>(null);
const collapsedAreas = ref<Set<string>>(new Set());

function toggleArea(key: string) {
  const next = new Set(collapsedAreas.value);
  if (next.has(key)) next.delete(key);
  else next.add(key);
  collapsedAreas.value = next;
}

const rows = computed(() =>
  categoryRows(props.category, filter.value).filter((row) => matchesQuery(row.item, query.value)),
);

const allRows = computed(() => categoryRows(props.category, "all"));

const isRecords = computed(() => props.category.key === "records");

function orderOf(row: ItemRow): number {
  return row.item.order > 0 ? row.item.order : Number.MAX_SAFE_INTEGER;
}

function compareOrder(left: ItemRow, right: ItemRow): number {
  return orderOf(left) - orderOf(right) || left.id.localeCompare(right.id);
}

function memorystickAreas(allRows: ItemRow[], visibleRows: ItemRow[]): RecordAreaGroup[] {
  const areas = new Map<string, RecordAreaGroup>();
  const ensure = (row: ItemRow) => {
    const key = row.item.area ?? "";
    let area = areas.get(key);
    if (!area) {
      area = {
        key,
        label: row.item.area_zh || row.item.area || "未分类",
        obtained: 0,
        total: 0,
        rows: [],
      };
      areas.set(key, area);
    }
    return area;
  };
  for (const row of allRows) {
    const area = ensure(row);
    area.total += 1;
    if (row.obtained) area.obtained += 1;
  }
  for (const row of visibleRows) ensure(row).rows.push(row);
  return [...areas.values()]
    .filter((area) => area.rows.length > 0)
    .map((area) => ({ ...area, rows: [...area.rows].sort(compareOrder) }))
    .sort((left, right) => orderOf(left.rows[0]) - orderOf(right.rows[0]));
}

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
        areas: [],
      };
      groups.set(key, group);
    }
    return group;
  };
  const allByGroup = new Map<string, ItemRow[]>();
  for (const row of allRows.value) {
    const group = ensure(row);
    group.total += 1;
    if (row.obtained) group.obtained += 1;
    const list = allByGroup.get(group.key);
    if (list) list.push(row);
    else allByGroup.set(group.key, [row]);
  }
  for (const row of rows.value) ensure(row).rows.push(row);
  for (const group of groups.values()) {
    if (group.key === "memorystick") {
      group.areas = memorystickAreas(allByGroup.get(group.key) ?? [], group.rows);
    }
  }
  const order = new Map(RECORD_TYPE_ORDER.map((key, index) => [key, index]));
  return [...groups.values()].sort(
    (left, right) => (order.get(left.key) ?? 99) - (order.get(right.key) ?? 99),
  );
});

const activeRecordGroup = computed<RecordGroup | null>(() => {
  const groups = recordGroups.value;
  return groups.find((group) => group.key === activeRecordType.value) ?? groups[0] ?? null;
});

watch(recordGroups, (groups) => {
  if (!isRecords.value) return;
  const active = groups.find((group) => group.key === activeRecordType.value);
  if (active && active.rows.length > 0) return;
  const fallback = groups.find((group) => group.rows.length > 0) ?? groups[0] ?? null;
  activeRecordType.value = fallback?.key ?? null;
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
              class="inline-flex items-center gap-1 px-2 py-1"
              :class="
                view === option.value
                  ? 'bg-slate-700 text-white'
                  : 'text-slate-400 hover:bg-slate-800'
              "
              @click="view = option.value"
            >
              <component :is="option.icon" class="h-3.5 w-3.5" />
              {{ option.label }}
            </button>
          </div>
          <div class="relative">
            <Search
              class="pointer-events-none absolute left-2 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-500"
            />
            <input
              class="w-64 rounded border border-slate-700 bg-slate-950 py-1 pl-7 pr-2 text-sm text-slate-100 placeholder:text-slate-600"
              type="search"
              placeholder="搜索名称 / 地点 / ID"
              :value="query"
              @input="onInput"
            />
          </div>
        </div>
      </div>

      <div class="h-1.5 overflow-hidden rounded bg-slate-800">
        <div class="h-full rounded bg-emerald-500" :style="{ width: `${percent}%` }"></div>
      </div>

      <div class="flex items-center gap-1.5">
        <ListFilter class="h-3.5 w-3.5 text-slate-500" />
        <div class="flex overflow-hidden rounded border border-slate-700 text-xs">
          <button
            v-for="option in FILTER_OPTIONS"
            :key="option.value"
            type="button"
            class="px-3 py-1"
            :class="
              filter === option.value
                ? 'bg-slate-700 text-white'
                : 'text-slate-400 hover:bg-slate-800'
            "
            @click="filter = option.value"
          >
            {{ option.label }}
            <span class="ml-1 text-slate-500">{{ count(option.value) }}</span>
          </button>
        </div>
        <span v-if="!matrix || view === 'list'" class="ml-auto text-[11px] text-slate-600">
          点击条目查看完整详情
        </span>
      </div>
    </header>

    <p
      v-if="rows.length === 0"
      class="flex items-center justify-center gap-2 px-4 py-10 text-center text-xs text-slate-500"
    >
      <SearchX class="h-4 w-4" />
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
      <nav
        class="flex gap-0.5 overflow-x-auto border-b border-slate-800 bg-slate-950/40 px-2"
        role="tablist"
      >
        <button
          v-for="group in recordGroups"
          :key="group.key"
          type="button"
          role="tab"
          :aria-selected="activeRecordGroup?.key === group.key"
          class="flex shrink-0 items-baseline gap-1.5 border-b-2 px-3 py-2 text-xs transition-colors"
          :class="[
            activeRecordGroup?.key === group.key
              ? 'border-emerald-500 bg-slate-900/80 text-slate-100'
              : 'border-transparent text-slate-400 hover:bg-slate-900/50 hover:text-slate-200',
            group.rows.length === 0 ? 'opacity-40' : '',
          ]"
          @click="activeRecordType = group.key"
        >
          {{ group.name }}
          <span
            class="text-[11px]"
            :class="activeRecordGroup?.key === group.key ? 'text-emerald-400' : 'text-slate-500'"
          >
            {{ group.obtained }}/{{ group.total }}
          </span>
        </button>
      </nav>
      <template v-if="activeRecordGroup">
        <p
          v-if="activeRecordGroup.rows.length === 0"
          class="flex items-center justify-center gap-2 px-4 py-10 text-center text-xs text-slate-500"
        >
          <SearchX class="h-4 w-4" />
          没有符合条件的物件
        </p>
        <template v-else-if="activeRecordGroup.areas.length > 0">
          <section v-for="area in activeRecordGroup.areas" :key="area.key">
            <button
              type="button"
              class="flex w-full flex-wrap items-center justify-between gap-2 border-t border-slate-800/70 px-4 py-2 text-left"
              :class="areaAccent(area.key).band"
              :aria-expanded="!collapsedAreas.has(area.key)"
              @click="toggleArea(area.key)"
            >
              <span
                class="flex items-center gap-2 text-sm font-semibold"
                :class="areaAccent(area.key).text"
              >
                <component
                  :is="collapsedAreas.has(area.key) ? ChevronRight : ChevronDown"
                  class="h-4 w-4 shrink-0"
                />
                <span
                  class="h-2.5 w-2.5 shrink-0 rounded-full"
                  :class="areaAccent(area.key).dot"
                ></span>
                {{ area.label }}
              </span>
              <span class="text-xs text-slate-400">
                已收集 {{ area.obtained }}/{{ area.total }}
              </span>
            </button>
            <ItemTable
              v-if="!collapsedAreas.has(area.key)"
              :rows="area.rows"
              :lang="lang"
              hide-location
            />
          </section>
        </template>
        <ItemTable v-else :rows="activeRecordGroup.rows" :lang="lang" />
      </template>
    </template>
    <ItemTable v-else :rows="rows" :lang="lang" />
  </section>

  <p v-if="category.extra_obtained_aliases.length > 0" class="mt-2 px-1 text-[11px] text-slate-500">
    另有 {{ category.extra_obtained_aliases.length }} 个已获得别名未关联到目录条目。
  </p>
</template>
