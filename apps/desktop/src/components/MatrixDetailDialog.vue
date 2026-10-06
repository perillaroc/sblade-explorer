<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { Image, Search, Video, X } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import { flagLabels, matrixName, obtainLabel, reasonLabel } from "../lib/display";
import { guideFor, searchVideoUrl, searchWebUrl } from "../lib/guides";
import { openExternalUrl } from "../lib/links";
import { searchEngine } from "../lib/settings";
import { matrixPeriodLabel, type MatrixUnit } from "../lib/matrix";
import type { ItemRow } from "../lib/items";
import MatrixStatusIcon from "./MatrixStatusIcon.vue";

const { t } = useI18n({ useScope: "global" });

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
const engineName = computed(() => t(`settings.engines.${searchEngine().id}`));
const webSearchUrl = computed(() => searchWebUrl(searchName.value));
const videoSearchUrl = computed(() => searchVideoUrl(searchName.value));

function flags(entry: ItemRow): string[] {
  return flagLabels(entry.item);
}

function reason(entry: ItemRow): string | null {
  return entry.obtained ? null : reasonLabel(entry.item, props.ngPlusCount);
}

function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") emit("close");
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-900/30 dark:bg-slate-950/70 p-4"
      @click.self="emit('close')"
    >
      <section
        class="max-h-[80vh] w-[44rem] max-w-full overflow-y-auto rounded-lg border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-200 dark:border-slate-800 px-4 py-3">
          <div>
            <h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">{{ unit.name }}</h3>
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
              <span class="text-sm font-medium text-slate-800 dark:text-slate-200">{{ locationLabel }}</span>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            {{ t("common.close") }}
          </button>
        </header>

        <div class="space-y-4 px-4 py-3">
          <div>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.obtain") }}</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-700 dark:text-slate-300">
              {{ unit.obtain || "—" }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.chineseGuides") }}</h4>
            <div class="mt-1.5 flex flex-wrap gap-2">
              <button
                v-if="webLink"
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-800 dark:text-slate-200 hover:border-sky-600 hover:bg-slate-100 dark:hover:bg-slate-800"
                :title="webLink.title"
                @click="openExternalUrl(webLink.url)"
              >
                <Image class="h-3.5 w-3.5" />
                {{ t("itemDetail.imageGuide") }}
              </button>
              <button
                v-if="videoLink"
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-800 dark:text-slate-200 hover:border-rose-600 hover:bg-slate-100 dark:hover:bg-slate-800"
                :title="videoLink.title"
                @click="openExternalUrl(videoLink.url)"
              >
                <Video class="h-3.5 w-3.5" />
                {{ t("itemDetail.videoGuide") }}
              </button>
              <span
                v-if="webLink || videoLink"
                class="mx-0.5 h-5 w-px self-center bg-slate-200 dark:bg-slate-700"
                aria-hidden="true"
              ></span>
              <button
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:border-sky-600 hover:bg-slate-100 dark:hover:bg-slate-800"
                :title="t('itemDetail.searchWebTitle', { engine: engineName, name: searchName })"
                @click="openExternalUrl(webSearchUrl)"
              >
                <Search class="h-3.5 w-3.5" />
                {{ t("itemDetail.searchWeb") }}
              </button>
              <button
                type="button"
                class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:border-rose-600 hover:bg-slate-100 dark:hover:bg-slate-800"
                :title="t('itemDetail.searchVideoTitle', { name: searchName })"
                @click="openExternalUrl(videoSearchUrl)"
              >
                <Search class="h-3.5 w-3.5" />
                {{ t("itemDetail.searchVideo") }}
              </button>
            </div>
            <p v-if="webLink" class="mt-1.5 text-[11px] leading-4 text-slate-500">
              {{ t("itemDetail.guideSource", { title: webLink.title }) }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.cycleVariants") }}</h4>
            <div
              v-for="entry in unit.cells.flat()"
              :key="entry.id"
              class="mt-2 rounded border border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-950/40 px-3 py-2"
            >
              <div class="flex flex-wrap items-baseline gap-2">
                <span class="inline-flex items-center gap-1 text-xs text-slate-800 dark:text-slate-200">
                  <MatrixStatusIcon :row="entry" :ng-plus-count="ngPlusCount" />
                  {{ matrixName(entry.item, lang) }}
                </span>
                <span class="rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                  {{ matrixPeriodLabel(entry.item) }}
                </span>
                <span class="text-[11px] text-slate-500 dark:text-slate-600">{{ entry.id }}</span>
                <span
                  v-for="flag in flags(entry)"
                  :key="flag"
                  class="rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-700 dark:text-amber-300"
                >
                  {{ flag }}
                </span>
                <span
                  v-if="reason(entry)"
                  class="rounded bg-amber-100 dark:bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-700 dark:text-amber-200"
                >
                  {{ reason(entry) }}
                </span>
                <span
                  v-if="entry.obtained && entry.item.missable"
                  class="rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-600 dark:text-slate-400"
                >
                  {{ t("itemTable.missable") }}
                </span>
              </div>
              <p class="mt-1 whitespace-pre-wrap text-[11px] leading-5 text-slate-600 dark:text-slate-400">
                {{ obtainLabel(entry.item, lang) }}
              </p>
            </div>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
