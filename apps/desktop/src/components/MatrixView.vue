<script setup lang="ts">
import { computed, ref } from "vue";
import type { Lang } from "../types";
import { matrixName } from "../lib/display";
import type { ItemRow } from "../lib/items";
import {
  buildMatrix,
  MATRIX_COLUMNS,
  matrixColumns,
  statusEmoji,
  type MatrixUnit,
} from "../lib/matrix";
import MatrixDetailDialog from "./MatrixDetailDialog.vue";

const props = defineProps<{
  rows: ItemRow[];
  allRows: ItemRow[];
  ngPlusCount: number;
  lang: Lang;
}>();

interface SelectedUnit {
  unit: MatrixUnit;
  areaLabel: string;
  locationLabel: string;
}

const areas = computed(() => buildMatrix(props.rows, props.allRows, props.lang));
const columns = computed(() => matrixColumns(props.allRows));
const selected = ref<SelectedUnit | null>(null);

function selectUnit(areaLabel: string, locationLabel: string, unit: MatrixUnit) {
  selected.value = { unit, areaLabel, locationLabel };
}
</script>

<template>
  <div class="p-4">
    <p class="text-xs text-slate-500">✅ 已获得 · ❌ 未获得 · 🔒 需更高周目 · 🎁 DLC/特典 · ➖ 默认外观</p>
    <p class="mt-1 text-[11px] text-slate-600">
      同一行为同一获取点：高周目会替换该点的物品，横向对比即可查漏；点击行查看获取方式等详情。
    </p>
    <div class="mt-2 overflow-x-auto">
      <table class="w-full text-xs">
        <thead>
          <tr class="text-left text-slate-500">
            <th class="w-32 py-1 pr-3">地点</th>
            <th v-for="index in columns" :key="index" class="py-1 pr-3">
              {{ MATRIX_COLUMNS[index] }}
            </th>
            <th class="w-72 py-1 pr-3">获取方式</th>
          </tr>
        </thead>
        <tbody>
          <template v-for="area in areas" :key="area.key">
            <tr>
              <td
                :colspan="columns.length + 2"
                class="pb-1 pt-4 text-xs font-semibold text-slate-300"
              >
                {{ area.label }}
              </td>
            </tr>
            <template v-for="location in area.locations" :key="location.key">
              <tr
                v-for="(unit, unitIndex) in location.units"
                :key="unit.key"
                class="cursor-pointer border-t border-slate-800/70 align-top hover:bg-slate-800/20"
                @click="selectUnit(area.label, location.label, unit)"
              >
                <td
                  v-if="unitIndex === 0"
                  :rowspan="location.units.length"
                  class="py-1.5 pr-3 text-slate-300"
                >
                  {{ location.label }}
                </td>
                <td v-for="index in columns" :key="index" class="py-1.5 pr-3">
                  <div v-for="entry in unit.cells[index]" :key="entry.id">
                    {{ statusEmoji(entry, ngPlusCount) }} {{ matrixName(entry.item, lang) }}
                  </div>
                  <span v-if="unit.cells[index].length === 0" class="text-slate-600">—</span>
                </td>
                <td class="py-1.5 pr-3">
                  <div class="flex items-start gap-1">
                    <span class="mt-px text-slate-600">▸</span>
                    <p class="line-clamp-2 max-w-72 text-slate-400" :title="unit.obtain || undefined">
                      {{ unit.obtain || "—" }}
                    </p>
                  </div>
                </td>
              </tr>
            </template>
          </template>
        </tbody>
      </table>
    </div>

    <MatrixDetailDialog
      v-if="selected"
      :unit="selected.unit"
      :area-label="selected.areaLabel"
      :location-label="selected.locationLabel"
      :ng-plus-count="ngPlusCount"
      :lang="lang"
      @close="selected = null"
    />
  </div>
</template>
