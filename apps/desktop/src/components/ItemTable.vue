<script setup lang="ts">
import { ref } from "vue";
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
import ItemDetailDialog from "./ItemDetailDialog.vue";

const { t } = useI18n({ useScope: "global" });

const props = defineProps<{
  rows: ItemRow[];
  ngPlusCount: number;
  lang: Lang;
  hideLocation?: boolean;
}>();

const selected = ref<ItemRow | null>(null);

function areaText(row: ItemRow): string {
  return areaLabel(row.item, props.lang);
}

function locationText(row: ItemRow): string {
  return locationLabel(row.item, props.lang);
}

function flags(row: ItemRow): string[] {
  return flagLabels(row.item);
}

function reason(row: ItemRow): string | null {
  return row.obtained ? null : reasonLabel(row.item, props.ngPlusCount);
}
</script>

<template>
  <div class="overflow-x-auto">
    <table class="w-full text-xs">
      <thead class="sticky top-0 bg-white dark:bg-slate-900">
        <tr class="text-left text-slate-500">
          <th class="px-4 py-2">{{ t("itemTable.status") }}</th>
          <th class="px-4 py-2">{{ t("itemTable.item") }}</th>
          <th v-if="!hideLocation" class="px-4 py-2">{{ t("itemTable.location") }}</th>
          <th class="px-4 py-2">{{ t("itemTable.obtain") }}</th>
          <th class="px-4 py-2">{{ t("itemTable.flags") }}</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="row in rows"
          :key="row.id"
          class="cursor-pointer border-t border-slate-200 dark:border-slate-800/70 align-top transition-colors hover:bg-slate-100 dark:hover:bg-slate-800/20"
          :class="row.obtained ? '' : 'bg-slate-50 dark:bg-slate-950/30'"
          @click="selected = row"
        >
          <td class="px-4 py-2">
            <span
              class="inline-flex items-center gap-0.5 rounded px-1.5 py-0.5 text-[11px]"
              :class="
                row.obtained
                  ? 'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-700 dark:text-emerald-300'
                  : 'bg-rose-100 dark:bg-rose-900/40 text-rose-700 dark:text-rose-300'
              "
            >
              <CircleCheck v-if="row.obtained" class="h-3 w-3" />
              <CircleX v-else class="h-3 w-3" />
              {{ row.obtained ? t("itemTable.obtained") : t("itemTable.missing") }}
            </span>
          </td>
          <td class="px-4 py-2">
            <div class="text-slate-900 dark:text-slate-100">{{ itemName(row.item, lang) }}</div>
            <div class="text-[11px] text-slate-500">{{ row.id }}</div>
          </td>
          <td v-if="!hideLocation" class="px-4 py-2">
            <div
              class="inline-flex max-w-72 flex-col rounded-r-md border-l-4 py-1 pl-2.5 pr-3"
              :class="[areaAccent(row.item.area).border, areaAccent(row.item.area).cell]"
            >
              <template v-if="locationText(row)">
                <span class="text-[11px] font-semibold" :class="areaAccent(row.item.area).text">
                  {{ areaText(row) || t("common.uncategorized") }}
                </span>
                <span class="text-sm font-medium text-slate-900 dark:text-slate-100">{{ locationText(row) }}</span>
              </template>
              <span
                v-else
                class="text-sm font-semibold"
                :class="areaAccent(row.item.area).text"
              >
                {{ areaText(row) || t("common.uncategorized") }}
              </span>
            </div>
          </td>
          <td class="px-4 py-2 text-slate-600 dark:text-slate-400">{{ obtainLabel(row.item, lang) || "—" }}</td>
          <td class="px-4 py-2">
            <span
              v-for="flag in flags(row)"
              :key="flag"
              class="mr-1 inline-block rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-700 dark:text-amber-300"
            >
              {{ flag }}
            </span>
            <span
              v-if="reason(row)"
              class="mr-1 inline-block rounded bg-amber-100 dark:bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-700 dark:text-amber-200"
            >
              {{ reason(row) }}
            </span>
            <span
              v-if="row.obtained && row.item.missable"
              class="mr-1 inline-flex items-center gap-0.5 rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-600 dark:text-slate-400"
            >
              <TriangleAlert class="h-3 w-3" />
              {{ t("itemTable.missable") }}
            </span>
          </td>
        </tr>
      </tbody>
    </table>

    <ItemDetailDialog
      v-if="selected"
      :row="selected"
      :ng-plus-count="ngPlusCount"
      :lang="lang"
      @close="selected = null"
    />
  </div>
</template>
