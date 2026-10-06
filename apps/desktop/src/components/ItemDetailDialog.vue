<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { CircleCheck, CircleX, Image, Search, Video, X } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import {
  areaLabel,
  cycleLabel,
  descLabel,
  dlcLabel,
  itemName,
  locationLabel,
  obtainLabel,
  reasonLabel,
  recordTypeLabel,
} from "../lib/display";
import { guideFor, searchVideoUrl, searchWebUrl } from "../lib/guides";
import { openExternalUrl } from "../lib/links";
import { searchEngine } from "../lib/settings";
import type { ItemRow } from "../lib/items";

const { t } = useI18n({ useScope: "global" });

const props = defineProps<{
  row: ItemRow;
  ngPlusCount: number;
  lang: Lang;
}>();

const emit = defineEmits<{ close: [] }>();

const item = computed(() => props.row.item);
const name = computed(() => itemName(item.value, props.lang));
const area = computed(() => areaLabel(item.value, props.lang));
const location = computed(() => locationLabel(item.value, props.lang));
const obtain = computed(() => obtainLabel(item.value, props.lang));
const desc = computed(() => descLabel(item.value, props.lang));
const guides = computed(() => guideFor(props.row.id));
const webLink = computed(() => guides.value?.web ?? null);
const videoLink = computed(() => guides.value?.video ?? null);
const searchName = computed(() => itemName(item.value, "zh"));
const engineName = computed(() => t(`settings.engines.${searchEngine().id}`));
const webSearchUrl = computed(() => searchWebUrl(searchName.value));
const videoSearchUrl = computed(() => searchVideoUrl(searchName.value));
const ngPlus = computed(() => cycleLabel(item.value.ng_plus));
const dlc = computed(() => (item.value.dlc ? dlcLabel(item.value.dlc) : null));
const recordType = computed(() => recordTypeLabel(item.value, props.lang));
const confidence = computed(() =>
  item.value.confidence === "high"
    ? t("itemDetail.confidenceHigh")
    : t("itemDetail.confidenceLow"),
);
const reason = computed(() =>
  props.row.obtained ? null : reasonLabel(item.value, props.ngPlusCount),
);
const aliases = computed(() =>
  (item.value.aliases.length > 0 ? item.value.aliases : [item.value.id]).filter(
    (alias) => alias !== item.value.id,
  ),
);
const source = computed(() => {
  const raw = props.row.source;
  if (!raw) return "";
  if (raw === "game") return t("itemDetail.sourceGame");
  const host = /^https?:\/\/([^/]+)/.exec(raw);
  return host ? t("itemDetail.sourceGuideSite", { host: host[1] }) : raw;
});

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
        class="max-h-[80vh] w-[38rem] max-w-full overflow-y-auto rounded-lg border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-200 dark:border-slate-800 px-4 py-3">
          <div class="min-w-0">
            <h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">{{ name }}</h3>
            <p class="mt-0.5 break-all font-mono text-[11px] text-slate-500">{{ row.id }}</p>
            <div class="mt-1.5 flex flex-wrap items-center gap-2">
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
              <span
                class="inline-flex items-center gap-1.5 rounded-md px-2 py-0.5 text-xs font-semibold"
                :class="[areaAccent(item.area).band, areaAccent(item.area).text]"
              >
                <span
                  class="h-2 w-2 shrink-0 rounded-full"
                  :class="areaAccent(item.area).dot"
                ></span>
                {{ area || t("common.uncategorized") }}
              </span>
              <span v-if="location" class="text-sm font-medium text-slate-800 dark:text-slate-200">
                {{ location }}
              </span>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex shrink-0 items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            {{ t("common.close") }}
          </button>
        </header>

        <div class="space-y-4 px-4 py-3">
          <div v-if="desc">
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.albumDescription") }}</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-700 dark:text-slate-300">
              {{ desc }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.obtain") }}</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-700 dark:text-slate-300">
              {{ obtain || "—" }}
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

          <div v-if="reason">
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.missingReason") }}</h4>
            <p class="mt-1 text-xs leading-5 text-amber-700 dark:text-amber-200">{{ reason }}</p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.details") }}</h4>
            <dl class="mt-1.5 grid grid-cols-[5.5rem_1fr] gap-x-3 gap-y-1.5 text-xs">
              <dt class="text-slate-500">{{ t("itemDetail.playthrough") }}</dt>
              <dd class="text-slate-700 dark:text-slate-300">{{ ngPlus }}</dd>
              <template v-if="dlc">
                <dt class="text-slate-500">{{ t("itemDetail.dlc") }}</dt>
                <dd class="text-slate-700 dark:text-slate-300">{{ dlc }}</dd>
              </template>
              <template v-if="item.missable">
                <dt class="text-slate-500">{{ t("itemDetail.missable") }}</dt>
                <dd class="text-amber-700 dark:text-amber-200">{{ t("common.yes") }}</dd>
              </template>
              <template v-if="recordType">
                <dt class="text-slate-500">{{ t("itemDetail.recordType") }}</dt>
                <dd class="text-slate-700 dark:text-slate-300">{{ recordType }}</dd>
              </template>
              <dt class="text-slate-500">{{ t("itemDetail.confidence") }}</dt>
              <dd :class="item.confidence === 'high' ? 'text-slate-700 dark:text-slate-300' : 'text-amber-700 dark:text-amber-200'">
                {{ confidence }}
              </dd>
              <template v-if="source">
                <dt class="text-slate-500">{{ t("itemDetail.source") }}</dt>
                <dd class="text-slate-700 dark:text-slate-300">{{ source }}</dd>
              </template>
            </dl>
          </div>

          <div v-if="aliases.length > 0">
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.aliases") }}</h4>
            <div class="mt-1.5 flex flex-wrap gap-1">
              <span
                v-for="alias in aliases"
                :key="alias"
                class="break-all rounded bg-slate-100 dark:bg-slate-800 px-1.5 py-0.5 font-mono text-[11px] text-slate-600 dark:text-slate-400"
              >
                {{ alias }}
              </span>
            </div>
          </div>

          <div v-if="item.note">
            <h4 class="text-xs font-semibold text-slate-600 dark:text-slate-400">{{ t("itemDetail.note") }}</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-600 dark:text-slate-400">
              {{ item.note }}
            </p>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
