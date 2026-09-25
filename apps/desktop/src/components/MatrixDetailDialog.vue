<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { Image, Search, Video, X } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import { matrixName, obtainLabel } from "../lib/display";
import { guideFor, searchVideoUrl, searchWebUrl } from "../lib/guides";
import { openExternalUrl } from "../lib/links";
import { searchEngine } from "../lib/settings";
import { matrixPeriodLabel, type MatrixUnit } from "../lib/matrix";
import MatrixStatusIcon from "./MatrixStatusIcon.vue";

const props = defineProps<{
  unit: MatrixUnit;
  areaKey: string;
  areaLabel: string;
  locationLabel: string;
  ngPlusCount: number;
  lang: Lang;
}>();

const emit = defineEmits<{ close: [] }>();

// Guide links follow the unit root (base playthrough item); fall back to the
// first variant that has links when the root is unmapped.
const guides = computed(() => {
  const direct = guideFor(props.unit.key);
  if (direct) return direct;
  for (const cell of props.unit.cells) {
    for (const row of cell) {
      const found = guideFor(row.id);
      if (found) return found;
    }
  }
  return null;
});
const webLink = computed(() => guides.value?.web ?? null);
const videoLink = computed(() => guides.value?.video ?? null);
const searchName = computed(() => props.unit.name);
const engineName = computed(() => searchEngine().name);
const webSearchUrl = computed(() => searchWebUrl(searchName.value));
const videoSearchUrl = computed(() => searchVideoUrl(searchName.value));

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") emit("close");
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/70 p-4"
      @click.self="emit('close')"
    >
      <section
        class="max-h-[80vh] w-[44rem] max-w-full overflow-y-auto rounded-lg border border-slate-700 bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-800 px-4 py-3">
          <div>
            <h3 class="text-sm font-semibold text-slate-100">{{ unit.name }}</h3>
            <div class="mt-1.5 flex flex-wrap items-center gap-2">
              <span
                class="inline-flex items-center gap-1.5 rounded-md px-2 py-0.5 text-xs font-semibold"
                :class="[areaAccent(areaKey).band, areaAccent(areaKey).text]"
              >
                <span
                  class="h-2 w-2 shrink-0 rounded-full"
                  :class="areaAccent(areaKey).dot"
                ></span>
                {{ areaLabel }}
              </span>
              <span class="text-sm font-medium text-slate-200">{{ locationLabel }}</span>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            关闭
          </button>
        </header>

        <div class="space-y-4 px-4 py-3">
          <div>
            <h4 class="text-xs font-semibold text-slate-400">获取方式</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-300">
              {{ unit.obtain || "—" }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-400">中文攻略</h4>
            <div class="mt-1.5 flex flex-wrap gap-2">
              <button
                v-if="webLink"
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-200 hover:border-sky-600 hover:bg-slate-800"
                :title="webLink.title"
                @click="openExternalUrl(webLink.url)"
              >
                <Image class="h-3.5 w-3.5" />
                图文攻略
              </button>
              <button
                v-if="videoLink"
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-200 hover:border-rose-600 hover:bg-slate-800"
                :title="videoLink.title"
                @click="openExternalUrl(videoLink.url)"
              >
                <Video class="h-3.5 w-3.5" />
                视频攻略
              </button>
              <span
                v-if="webLink || videoLink"
                class="mx-0.5 h-5 w-px self-center bg-slate-700"
                aria-hidden="true"
              ></span>
              <button
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:border-sky-600 hover:bg-slate-800"
                :title="`在${engineName}搜索「${searchName}」的图文攻略`"
                @click="openExternalUrl(webSearchUrl)"
              >
                <Search class="h-3.5 w-3.5" />
                搜索图文攻略
              </button>
              <button
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:border-rose-600 hover:bg-slate-800"
                :title="`在 B 站搜索「${searchName}」的视频攻略`"
                @click="openExternalUrl(videoSearchUrl)"
              >
                <Search class="h-3.5 w-3.5" />
                搜索视频攻略
              </button>
            </div>
            <p v-if="webLink" class="mt-1.5 text-[11px] leading-4 text-slate-500">
              图文来源：{{ webLink.title }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-400">周目变体</h4>
            <div
              v-for="entry in unit.cells.flat()"
              :key="entry.id"
              class="mt-2 rounded border border-slate-800 bg-slate-950/40 px-3 py-2"
            >
              <div class="flex flex-wrap items-baseline gap-2">
                <span class="inline-flex items-center gap-1 text-xs text-slate-200">
                  <MatrixStatusIcon :row="entry" :ng-plus-count="ngPlusCount" />
                  {{ matrixName(entry.item, lang) }}
                </span>
                <span class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-400">
                  {{ matrixPeriodLabel(entry.item) }}
                </span>
                <span class="text-[11px] text-slate-600">{{ entry.id }}</span>
                <span
                  v-for="flag in entry.flags"
                  :key="flag"
                  class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-300"
                >
                  {{ flag }}
                </span>
                <span
                  v-if="entry.reason"
                  class="rounded bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-200"
                >
                  {{ entry.reason }}
                </span>
                <span
                  v-if="entry.obtained && entry.item.missable"
                  class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-400"
                >
                  可错过
                </span>
              </div>
              <p class="mt-1 whitespace-pre-wrap text-[11px] leading-5 text-slate-400">
                {{ obtainLabel(entry.item, lang) }}
              </p>
            </div>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
