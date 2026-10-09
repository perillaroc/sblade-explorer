<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { CircleCheck, CircleX, TriangleAlert } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import {
  areaLabel,
  flagLabels,
  itemName,
  locationLabel,
  obtainLabel,
  reasonLabel,
} from "../lib/display";
import type { ItemRow } from "../lib/items";
import DetailDrawer from "./DetailDrawer.vue";
import { UiBadge, UiTable } from "./ui";

/**
 * Item table (list view and area groups inside the category page).
 * Rows are clickable / keyboard-activatable and open the unified detail
 * drawer with prev/next navigation inside the passed rows.
 */
const props = defineProps<{
  rows: ItemRow[];
  ngPlusCount: number;
  lang: Lang;
  hideLocation?: boolean;
}>();

const { t } = useI18n({ useScope: "global" });

const columns = computed(() => {
  const list = [
    { key: "status", label: t("itemTable.status"), headerClass: "w-24" },
    { key: "item", label: t("itemTable.item") },
  ];
  if (!props.hideLocation) list.push({ key: "location", label: t("itemTable.location") });
  list.push({ key: "obtain", label: t("itemTable.obtain") });
  list.push({ key: "flags", label: t("itemTable.flags"), headerClass: "w-56" });
  return list;
});

const selectedIndex = ref(-1);

const selected = computed(() =>
  selectedIndex.value >= 0 ? (props.rows[selectedIndex.value] ?? null) : null,
);

watch(
  () => props.rows,
  () => {
    selectedIndex.value = -1;
  },
);

function open(index: number): void {
  selectedIndex.value = index;
}

function navigate(delta: number): void {
  const next = selectedIndex.value + delta;
  if (next >= 0 && next < props.rows.length) selectedIndex.value = next;
}

function reason(row: ItemRow): string | null {
  return row.obtained ? null : reasonLabel(row.item, props.ngPlusCount);
}
</script>

<template>
  <UiTable
    :columns="columns"
    :rows="rows"
    :row-key="(row) => row.id"
    clickable
    :row-class="(row) => (row.obtained ? undefined : 'bg-graphite-50/70 dark:bg-black/20')"
    @row-activate="(_row, index) => open(index)"
  >
    <template #cell-status="{ row }">
      <UiBadge :tone="row.obtained ? 'success' : 'danger'">
        <template #icon>
          <CircleCheck v-if="row.obtained" class="h-3 w-3" />
          <CircleX v-else class="h-3 w-3" />
        </template>
        {{ row.obtained ? t("itemTable.obtained") : t("itemTable.missing") }}
      </UiBadge>
    </template>

    <template #cell-item="{ row }">
      <div class="text-ink">{{ itemName(row.item, lang) }}</div>
      <div class="font-mono text-[11px] text-ink-subtle">{{ row.id }}</div>
    </template>

    <template #cell-location="{ row }">
      <div
        class="inline-flex max-w-72 flex-col rounded-r-md border-l-4 py-1 pl-2.5 pr-3"
        :class="[areaAccent(row.item.area).border, areaAccent(row.item.area).cell]"
      >
        <template v-if="locationLabel(row.item, lang)">
          <span class="text-[11px] font-semibold" :class="areaAccent(row.item.area).text">
            {{ areaLabel(row.item, lang) || t("common.uncategorized") }}
          </span>
          <span class="text-sm font-medium text-ink">{{ locationLabel(row.item, lang) }}</span>
        </template>
        <span v-else class="text-sm font-semibold" :class="areaAccent(row.item.area).text">
          {{ areaLabel(row.item, lang) || t("common.uncategorized") }}
        </span>
      </div>
    </template>

    <template #cell-obtain="{ row }">
      <span class="text-ink-muted">{{ obtainLabel(row.item, lang) || "—" }}</span>
    </template>

    <template #cell-flags="{ row }">
      <span class="inline-flex flex-wrap items-center gap-1">
        <UiBadge v-for="flag in flagLabels(row.item)" :key="flag" tone="warning">
          {{ flag }}
        </UiBadge>
        <UiBadge v-if="reason(row)" tone="warning">
          {{ reason(row) }}
        </UiBadge>
        <UiBadge v-if="row.obtained && row.item.missable" tone="neutral">
          <template #icon>
            <TriangleAlert class="h-3 w-3" />
          </template>
          {{ t("itemTable.missable") }}
        </UiBadge>
      </span>
    </template>
  </UiTable>

  <DetailDrawer
    v-if="selected"
    :row="selected"
    :ng-plus-count="ngPlusCount"
    :lang="lang"
    :has-previous="selectedIndex > 0"
    :has-next="selectedIndex < rows.length - 1"
    @close="selectedIndex = -1"
    @previous="navigate(-1)"
    @next="navigate(1)"
  />
</template>
