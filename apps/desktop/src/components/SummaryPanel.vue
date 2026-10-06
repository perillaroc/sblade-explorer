<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { BookOpen, Clock, Gauge, IdCard, Package, Repeat } from "@lucide/vue";
import type { Analysis } from "../types";

const { t } = useI18n({ useScope: "global" });

const props = defineProps<{ analysis: Analysis }>();

const summary = computed(() => props.analysis.summary);

const difficultyLabel = computed(() => {
  const value = props.analysis.save.difficulty;
  if (value === 0) return t("summary.difficultyEasy");
  if (value === 1) return t("summary.difficultyNormal");
  if (value === 2) return t("summary.difficultyHard");
  return t("summary.difficultyUnknown", { value });
});

const playthroughLabel = computed(() => {
  const count = props.analysis.save.ng_plus_count;
  return count <= 0 ? t("summary.playthroughFirst") : t("summary.playthroughNgPlus", { count });
});

const playTimeLabel = computed(() => {
  const seconds = props.analysis.save.play_time_seconds;
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return t("summary.playTimeValue", {
    hours,
    minutes: String(minutes).padStart(2, "0"),
  });
});
</script>

<template>
  <section class="rounded-lg border border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900/60 p-4">
    <div class="grid grid-cols-2 gap-3 text-sm md:grid-cols-4">
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <IdCard class="h-3.5 w-3.5" />
          SteamID
        </div>
        <div>{{ analysis.save.steam_id ?? t("common.unknown") }}</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Repeat class="h-3.5 w-3.5" />
          {{ t("summary.playthrough") }}
        </div>
        <div>{{ playthroughLabel }} (NG+{{ analysis.save.ng_plus_count }})</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Gauge class="h-3.5 w-3.5" />
          {{ t("summary.difficulty") }}
        </div>
        <div>{{ difficultyLabel }}</div>
      </div>
      <div>
        <div class="flex items-center gap-1 text-xs text-slate-500">
          <Clock class="h-3.5 w-3.5" />
          {{ t("summary.playTime") }}
        </div>
        <div>{{ playTimeLabel }}</div>
      </div>
    </div>
    <div class="mt-4">
      <div class="mb-1 flex justify-between text-xs text-slate-600 dark:text-slate-400">
        <span class="inline-flex items-center gap-1">
          <Package class="h-3.5 w-3.5" />
          {{ t("app.catalogProgress", { obtained: summary.catalog_obtained, total: summary.catalog_total }) }}
        </span>
        <span>{{ t("summary.percentMissing", { percent: summary.percent.toFixed(1), missing: summary.missing_total }) }}</span>
      </div>
      <div class="h-2 overflow-hidden rounded bg-slate-200 dark:bg-slate-800">
        <div class="h-full rounded bg-emerald-500" :style="{ width: `${summary.percent}%` }"></div>
      </div>
      <div class="mt-2 text-xs text-slate-500">
        {{ t("summary.aliases", { obtained: summary.obtained_aliases, unmapped: summary.unmapped_aliases }) }}
      </div>
    </div>
    <div class="mt-3">
      <div class="mb-1 flex justify-between text-xs text-slate-600 dark:text-slate-400">
        <span class="inline-flex items-center gap-1">
          <BookOpen class="h-3.5 w-3.5" />
          {{ t("app.albumProgressShort", { obtained: summary.album_obtained, total: summary.album_total }) }}
        </span>
        <span>
          {{ t("summary.albumPercentMissing", { percent: summary.album_percent.toFixed(1), missing: summary.album_missing_total }) }}
        </span>
      </div>
      <div class="h-2 overflow-hidden rounded bg-slate-200 dark:bg-slate-800">
        <div class="h-full rounded bg-sky-500" :style="{ width: `${summary.album_percent}%` }"></div>
      </div>
    </div>
  </section>
</template>
