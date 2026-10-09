<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
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
import { areaLabel, categoryName, recordTypeLabel } from "../lib/display";
import { categoryRows, matchesQuery, type ItemRow } from "../lib/items";
import { settings, type CategoryView } from "../lib/settings";
import ItemTable from "./ItemTable.vue";
import MatrixView from "./MatrixView.vue";
import { UiCard, UiEmpty, UiInput, UiProgress, UiSegmented, UiTabs } from "./ui";

/**
 * Category page: sticky toolbar (title / progress / view / search / filter)
 * plus the list, cycle matrix, record-type tabs and album groupings.
 */
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
  /** Global-search target: reset view/filter and open this item (F4). */
  focusItemId?: string | null;
  focusNonce?: number;
}>();

const { t } = useI18n({ useScope: "global" });

const FILTER_OPTIONS = computed<{ value: ItemFilter; label: string; count: number }[]>(() => [
  { value: "all", label: t("categoryPage.filterAll"), count: props.category.total },
  { value: "obtained", label: t("categoryPage.filterObtained"), count: props.category.obtained },
  { value: "missing", label: t("categoryPage.filterMissing"), count: props.category.missing.length },
]);

const VIEW_OPTIONS = computed<{ value: ViewMode; label: string; icon: LucideIcon }[]>(() => [
  { value: "matrix", label: t("categoryPage.viewMatrix"), icon: LayoutGrid },
  { value: "list", label: t("categoryPage.viewList"), icon: List },
]);

const filter = ref<ItemFilter>(settings.categoryFilters[props.category.key] ?? "all");
const query = ref("");
const view = ref<CategoryView>(
  settings.categoryViews[props.category.key] ?? (props.matrix ? "matrix" : "list"),
);
const activeRecordType = ref<string | null>(null);
const collapsedAreas = ref<Set<string>>(new Set());

/** Programmatic resets (global-search focus) must not overwrite preferences. */
let applyingFocus = false;

watch(view, (value) => {
  if (!applyingFocus) settings.categoryViews[props.category.key] = value;
});

watch(filter, (value) => {
  if (!applyingFocus) settings.categoryFilters[props.category.key] = value;
});

function setView(value: string): void {
  view.value = value as ViewMode;
}

function setFilter(value: string): void {
  filter.value = value as ItemFilter;
}

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

const isNaytiba = computed(() => props.category.key === "naytiba");

function orderOf(row: ItemRow): number {
  return row.item.order > 0 ? row.item.order : Number.MAX_SAFE_INTEGER;
}

function compareOrder(left: ItemRow, right: ItemRow): number {
  return orderOf(left) - orderOf(right) || left.id.localeCompare(right.id);
}

function groupRowsByArea(allAreaRows: ItemRow[], visibleRows: ItemRow[]): RecordAreaGroup[] {
  const areas = new Map<string, RecordAreaGroup>();
  const ensure = (row: ItemRow) => {
    const key = row.item.area ?? "";
    let area = areas.get(key);
    if (!area) {
      area = {
        key,
        label: areaLabel(row.item, props.lang) || t("common.uncategorized"),
        obtained: 0,
        total: 0,
        rows: [],
      };
      areas.set(key, area);
    }
    return area;
  };
  for (const row of allAreaRows) {
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
        name: recordTypeLabel(row.item, props.lang),
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
      group.areas = groupRowsByArea(allByGroup.get(group.key) ?? [], group.rows);
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

const recordTabs = computed(() =>
  recordGroups.value.map((group) => ({
    value: group.key,
    label: group.name,
    count: `${group.obtained}/${group.total}`,
    muted: group.rows.length === 0,
  })),
);

const naytibaGroups = computed<RecordAreaGroup[]>(() => {
  if (!isNaytiba.value) return [];
  return groupRowsByArea(allRows.value, rows.value);
});

watch(recordGroups, (groups) => {
  if (!isRecords.value) return;
  const active = groups.find((group) => group.key === activeRecordType.value);
  if (active && active.rows.length > 0) return;
  const fallback = groups.find((group) => group.rows.length > 0) ?? groups[0] ?? null;
  activeRecordType.value = fallback?.key ?? null;
});

/**
 * Global-search focus: reset to a neutral list view so the item exists in the
 * rendered rows, then let the matching ItemTable open its detail drawer.
 */
watch(
  () => [props.focusItemId, props.focusNonce] as const,
  ([itemId]) => {
    if (!itemId) return;
    applyingFocus = true;
    filter.value = "all";
    query.value = "";
    collapsedAreas.value = new Set();
    if (view.value === "matrix") view.value = "list";
    if (isRecords.value) {
      const group = recordGroups.value.find((candidate) =>
        candidate.rows.some((row) => row.id === itemId),
      );
      if (group) activeRecordType.value = group.key;
    }
    void nextTick(() => {
      applyingFocus = false;
    });
  },
  { immediate: true },
);

const percent = computed(() =>
  props.category.total === 0 ? 0 : (props.category.obtained / props.category.total) * 100,
);
</script>

<template>
  <div class="space-y-3">
    <div class="sticky top-0 z-toolbar -mx-4 -mt-4 border-b border-edge bg-surface-card px-4 py-3">
      <div class="flex flex-wrap items-start justify-between gap-3">
        <div class="min-w-0">
          <h2 class="truncate text-base font-semibold text-ink">
            {{ categoryName(category, lang) }}
          </h2>
          <p class="mt-0.5 text-xs text-ink-muted">
            {{
              t("categoryPage.progress", {
                obtained: category.obtained,
                total: category.total,
                missing: category.missing.length,
                percent: percent.toFixed(0),
              })
            }}
          </p>
        </div>
        <div class="flex flex-wrap items-center gap-2">
          <UiSegmented
            v-if="matrix"
            :model-value="view"
            :options="VIEW_OPTIONS"
            :aria-label="t('categoryPage.viewLabel')"
            @update:model-value="setView"
          />
          <UiInput
            v-model="query"
            type="search"
            :placeholder="t('categoryPage.searchPlaceholder')"
            class="w-64 max-w-full"
          >
            <template #icon>
              <Search class="h-3.5 w-3.5" />
            </template>
          </UiInput>
        </div>
      </div>

      <UiProgress
        class="mt-2.5"
        size="md"
        :value="percent"
        :tone="category.section === 'album' ? 'album' : 'success'"
      />

      <div class="mt-2.5 flex flex-wrap items-center gap-2">
        <ListFilter class="h-3.5 w-3.5 shrink-0 text-ink-subtle" aria-hidden="true" />
        <UiSegmented
          :model-value="filter"
          :options="FILTER_OPTIONS"
          :aria-label="t('categoryPage.filterLabel')"
          @update:model-value="setFilter"
        />
        <span class="ml-auto hidden text-[11px] text-ink-subtle min-[1100px]:inline">
          {{ t("categoryPage.clickHint") }}
        </span>
      </div>
    </div>

    <UiEmpty v-if="rows.length === 0" :icon="SearchX" :description="t('categoryPage.empty')" />

    <template v-else-if="matrix && view === 'matrix'">
      <UiCard padding="none">
        <MatrixView
          :rows="rows"
          :all-rows="allRows"
          :ng-plus-count="ngPlusCount"
          :lang="lang"
        />
      </UiCard>
    </template>

    <template v-else-if="isRecords">
      <UiCard padding="none">
        <UiTabs
          :model-value="activeRecordGroup?.key ?? ''"
          :tabs="recordTabs"
          :aria-label="t('categoryPage.recordTypes')"
          @update:model-value="activeRecordType = $event"
        />
        <template v-if="activeRecordGroup">
          <UiEmpty
            v-if="activeRecordGroup.rows.length === 0"
            :icon="SearchX"
            :description="t('categoryPage.empty')"
          />
          <template v-else-if="activeRecordGroup.areas.length > 0">
            <section v-for="area in activeRecordGroup.areas" :key="area.key">
              <button
                type="button"
                class="flex w-full flex-wrap items-center justify-between gap-2 border-t border-edge px-4 py-2 text-left transition-colors duration-fast first:border-t-0"
                :class="areaAccent(area.key).band"
                :aria-expanded="!collapsedAreas.has(area.key)"
                @click="toggleArea(area.key)"
              >
                <span class="flex items-center gap-2 text-sm font-semibold" :class="areaAccent(area.key).text">
                  <component
                    :is="collapsedAreas.has(area.key) ? ChevronRight : ChevronDown"
                    class="h-4 w-4 shrink-0"
                    aria-hidden="true"
                  />
                  <span class="h-2.5 w-2.5 shrink-0 rounded-full" :class="areaAccent(area.key).dot"></span>
                  {{ area.label }}
                </span>
                <span class="text-xs text-ink-muted">
                  {{ t("categoryPage.areaProgress", { obtained: area.obtained, total: area.total }) }}
                </span>
              </button>
              <ItemTable
                v-if="!collapsedAreas.has(area.key)"
                :rows="area.rows"
                :ng-plus-count="ngPlusCount"
                :lang="lang"
                hide-location
                :open-item-id="focusItemId"
                :open-nonce="focusNonce"
              />
            </section>
          </template>
          <ItemTable
            v-else
            :rows="activeRecordGroup.rows"
            :ng-plus-count="ngPlusCount"
            :lang="lang"
            :open-item-id="focusItemId"
            :open-nonce="focusNonce"
          />
        </template>
      </UiCard>
    </template>

    <template v-else-if="isNaytiba">
      <UiCard v-for="area in naytibaGroups" :key="area.key" padding="none">
        <button
          type="button"
          class="flex w-full flex-wrap items-center justify-between gap-2 px-4 py-2 text-left transition-colors duration-fast"
          :class="areaAccent(area.key).band"
          :aria-expanded="!collapsedAreas.has(area.key)"
          @click="toggleArea(area.key)"
        >
          <span class="flex items-center gap-2 text-sm font-semibold" :class="areaAccent(area.key).text">
            <component
              :is="collapsedAreas.has(area.key) ? ChevronRight : ChevronDown"
              class="h-4 w-4 shrink-0"
              aria-hidden="true"
            />
            <span class="h-2.5 w-2.5 shrink-0 rounded-full" :class="areaAccent(area.key).dot"></span>
            {{ area.label }}
          </span>
          <span class="text-xs text-ink-muted">
            {{ t("categoryPage.areaProgress", { obtained: area.obtained, total: area.total }) }}
          </span>
        </button>
        <ItemTable
          v-if="!collapsedAreas.has(area.key)"
          :rows="area.rows"
          :ng-plus-count="ngPlusCount"
          :lang="lang"
          hide-location
          :open-item-id="focusItemId"
          :open-nonce="focusNonce"
        />
      </UiCard>
    </template>

    <UiCard v-else padding="none">
      <ItemTable
        :rows="rows"
        :ng-plus-count="ngPlusCount"
        :lang="lang"
        :open-item-id="focusItemId"
        :open-nonce="focusNonce"
      />
    </UiCard>

    <p v-if="category.extra_obtained_aliases.length > 0" class="px-1 text-[11px] text-ink-subtle">
      {{ t("categoryPage.extraAliases", { count: category.extra_obtained_aliases.length }) }}
    </p>
  </div>
</template>
