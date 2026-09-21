<script setup lang="ts">
import { computed, ref } from "vue";
import { ChevronDown, ChevronRight } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import { matrixName } from "../lib/display";
import type { ItemRow } from "../lib/items";
import {
  buildMatrix,
  MATRIX_COLUMNS,
  matrixColumns,
  type MatrixArea,
  type MatrixUnit,
} from "../lib/matrix";
import { MATRIX_STATUS_LEGEND } from "../lib/status";
import MatrixDetailDialog from "./MatrixDetailDialog.vue";
import MatrixStatusIcon from "./MatrixStatusIcon.vue";

const props = defineProps<{
  rows: ItemRow[];
  allRows: ItemRow[];
  ngPlusCount: number;
  lang: Lang;
}>();

interface SelectedUnit {
  unit: MatrixUnit;
  areaKey: string;
  areaLabel: string;
  locationLabel: string;
}

const areas = computed(() => buildMatrix(props.rows, props.allRows, props.lang));
const columns = computed(() => matrixColumns(props.allRows));
const selected = ref<SelectedUnit | null>(null);
const collapsedAreas = ref<Set<string>>(new Set());
const collapsedLocations = ref<Set<string>>(new Set());

function toggleKey(set: Set<string>, key: string): Set<string> {
  const next = new Set(set);
  if (next.has(key)) next.delete(key);
  else next.add(key);
  return next;
}

function toggleArea(key: string) {
  collapsedAreas.value = toggleKey(collapsedAreas.value, key);
}

function toggleLocation(key: string) {
  collapsedLocations.value = toggleKey(collapsedLocations.value, key);
}

function areaUnitCount(area: MatrixArea): number {
  return area.locations.reduce((total, location) => total + location.units.length, 0);
}

function selectUnit(areaKey: string, areaLabel: string, locationLabel: string, unit: MatrixUnit) {
  selected.value = { unit, areaKey, areaLabel, locationLabel };
}
</script>

<template>
  <div class="p-4">
    <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500">
      <span
        v-for="status in MATRIX_STATUS_LEGEND"
        :key="status.label"
        class="inline-flex items-center gap-1"
      >
        <component :is="status.icon" class="h-3.5 w-3.5" :class="status.className" />
        {{ status.label }}
      </span>
    </div>
    <p class="mt-1 text-[11px] text-slate-600">
      同一行为同一获取点：高周目会替换该点的物品，横向对比即可查漏；点击行查看获取方式等详情。
    </p>
    <div class="mt-2 overflow-x-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-left text-slate-500">
            <th class="w-40 py-1 pr-3 text-sm font-semibold text-slate-300">地点</th>
            <th v-for="index in columns" :key="index" class="py-1 pr-3">
              {{ MATRIX_COLUMNS[index] }}
            </th>
            <th class="w-72 py-1 pr-3">获取方式</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="area in areas" :key="area.key">
            <tr>
              <td :colspan="columns.length + 2" class="pb-1 pt-4">
                <button
                  type="button"
                  class="flex w-full items-center gap-2 rounded-md px-3 py-1.5 text-sm font-bold"
                  :class="[areaAccent(area.key).band, areaAccent(area.key).text]"
                  :aria-expanded="!collapsedAreas.has(area.key)"
                  @click="toggleArea(area.key)"
                >
                  <component
                    :is="collapsedAreas.has(area.key) ? ChevronRight : ChevronDown"
                    class="h-4 w-4 shrink-0"
                  />
                  <span
                    class="h-2.5 w-2.5 shrink-0 rounded-full"
                    :class="areaAccent(area.key).dot"
                  ></span>
                  <span>{{ area.label }}</span>
                  <span class="ml-auto text-xs font-normal text-slate-400">
                    {{ areaUnitCount(area) }} 项
                  </span>
                </button>
              </td>
            </tr>
            <template v-if="!collapsedAreas.has(area.key)">
              <template v-for="location in area.locations" :key="location.key">
                <tr
                  v-if="collapsedLocations.has(location.key)"
                  class="cursor-pointer border-t border-slate-800/70 align-top hover:bg-slate-800/20"
                  @click="toggleLocation(location.key)"
                >
                  <td
                    class="border-l-4 p-0 align-top"
                    :class="[areaAccent(area.key).border, areaAccent(area.key).cell]"
                  >
                    <button
                      type="button"
                      class="flex w-full items-start gap-1.5 px-3 py-2 text-left text-sm font-semibold text-slate-100"
                      :aria-expanded="false"
                      @click.stop="toggleLocation(location.key)"
                    >
                      <ChevronRight class="mt-0.5 h-3.5 w-3.5 shrink-0 text-slate-400" />
                      <span>{{ location.label }}</span>
                    </button>
                  </td>
                  <td :colspan="columns.length + 1" class="py-2 pr-3 text-xs text-slate-500">
                    已折叠 {{ location.units.length }} 项
                  </td>
                </tr>
                <template v-else>
                  <tr
                    v-for="(unit, unitIndex) in location.units"
                    :key="unit.key"
                    class="cursor-pointer border-t border-slate-800/70 align-top hover:bg-slate-800/20"
                    @click="selectUnit(area.key, area.label, location.label, unit)"
                  >
                    <td
                      v-if="unitIndex === 0"
                      :rowspan="location.units.length"
                      class="cursor-pointer border-l-4 p-0 align-top"
                      :class="[areaAccent(area.key).border, areaAccent(area.key).cell]"
                      @click.stop="toggleLocation(location.key)"
                    >
                      <button
                        type="button"
                        class="flex w-full items-start gap-1.5 px-3 py-2 text-left text-sm font-semibold text-slate-100"
                        :aria-expanded="true"
                        @click.stop="toggleLocation(location.key)"
                      >
                        <ChevronDown class="mt-0.5 h-3.5 w-3.5 shrink-0 text-slate-400" />
                        <span>{{ location.label }}</span>
                      </button>
                    </td>
                    <td v-for="index in columns" :key="index" class="py-1.5 pr-3">
                      <div
                        v-for="entry in unit.cells[index]"
                        :key="entry.id"
                        class="flex items-start gap-1"
                      >
                        <MatrixStatusIcon
                          :row="entry"
                          :ng-plus-count="ngPlusCount"
                          class="mt-px"
                        />
                        <span>{{ matrixName(entry.item, lang) }}</span>
                      </div>
                      <span v-if="unit.cells[index].length === 0" class="text-slate-600">—</span>
                    </td>
                    <td class="py-1.5 pr-3">
                      <div class="flex items-start gap-1">
                        <span class="mt-px text-slate-600">▸</span>
                        <p
                          class="line-clamp-2 max-w-72 text-slate-400"
                          :title="unit.obtain || undefined"
                        >
                          {{ unit.obtain || "—" }}
                        </p>
                      </div>
                    </td>
                  </tr>
                </template>
              </template>
            </template>
          </template>
        </tbody>
      </table>
    </div>

    <MatrixDetailDialog
      v-if="selected"
      :unit="selected.unit"
      :area-key="selected.areaKey"
      :area-label="selected.areaLabel"
      :location-label="selected.locationLabel"
      :ng-plus-count="ngPlusCount"
      :lang="lang"
      @close="selected = null"
    />
  </div>
</template>
