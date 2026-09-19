<script setup lang="ts">
import { computed } from "vue";
import { Clock, Gauge, IdCard, Package, Repeat } from "@lucide/vue";
import type { Analysis } from "../types";

const props = defineProps<{ analysis: Analysis }>();

const summary = computed(() => props.analysis.summary);

const difficultyLabel = computed(() => {
  const value = props.analysis.save.difficulty;
  if (value === 0) return "简单";
  if (value === 1) return "普通";
  if (value === 2) return "困难";
  return `未知(${value})`;
});

const playTimeLabel = computed(() => {
  const seconds = props.analysis.save.play_time_seconds;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${hours}小时${String(minutes).padStart(2, "0")}分`;
});
</script>

<template>
  <section class="rounded-lg border border-slate-800 bg-slate-900/60 p-4">
    <div class="grid grid-cols-2 gap-3 text-sm md:grid-cols-4">
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <IdCard class="h-3.5 w-3.5" />
          SteamID
        </div>
        <div>{{ analysis.save.steam_id ?? "未知" }}</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Repeat class="h-3.5 w-3.5" />
          周目
        </div>
        <div>{{ analysis.save.playthrough }} (NG+{{ analysis.save.ng_plus_count }})</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Gauge class="h-3.5 w-3.5" />
          难度
        </div>
        <div>{{ difficultyLabel }}</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Clock class="h-3.5 w-3.5" />
          游玩时间
        </div>
        <div>{{ playTimeLabel }}</div>
      </div>
    </div>
    <div class="mt-4">
      <div class="mb-1 flex justify-between text-xs text-slate-400">
        <span class="inline-flex items-center gap-1">
          <Package class="h-3.5 w-3.5" />
          目录进度 {{ summary.catalog_obtained }}/{{ summary.catalog_total }}
        </span>
        <span>{{ summary.percent.toFixed(1) }}% · 未收集 {{ summary.missing_total }}</span>
      </div>
      <div class="h-2 overflow-hidden rounded bg-slate-800">
        <div class="h-full rounded bg-emerald-500" :style="{ width: `${summary.percent}%` }"></div>
      </div>
      <div class="mt-2 text-xs text-slate-500">
        已获得别名 {{ summary.obtained_aliases }} · 未映射 {{ summary.unmapped_aliases }}
      </div>
    </div>
  </section>
</template>
