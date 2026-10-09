<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ChevronLeft, ChevronRight, CircleCheck, CircleX, Image, Search, Video } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import {
  areaLabel,
  cycleLabel,
  descLabel,
  dlcLabel,
  flagLabels,
  itemName,
  locationLabel,
  obtainLabel,
  reasonLabel,
  recordTypeLabel,
} from "../lib/display";
import { guideFor, searchVideoUrl, searchWebUrl } from "../lib/guides";
import { openExternalUrl } from "../lib/links";
import type { ItemRow } from "../lib/items";
import { matrixPeriodLabel, type MatrixUnit } from "../lib/matrix";
import { searchEngine } from "../lib/settings";
import MatrixStatusIcon from "./MatrixStatusIcon.vue";
import { UiBadge, UiButton, UiDialog } from "./ui";

/**
 * Unified detail drawer (F0 decision ④), replacing the old
 * `ItemDetailDialog` + `MatrixDetailDialog` pair. Two shapes:
 *
 * - item mode (`row`): list rows / dashboard entries; optional prev/next
 *   navigation inside the source list.
 * - group mode (`unit`): a matrix acquisition spot; lists the cycle variants
 *   first and drills down into a single entry with a back action.
 *
 * The Modal/Drawer chrome, Esc, backdrop close, focus trap and body scroll
 * lock come from `UiDialog`.
 */
const props = withDefaults(
  defineProps<{
    row?: ItemRow;
    unit?: { unit: MatrixUnit; areaKey: string; areaLabel: string; locationLabel: string };
    ngPlusCount: number;
    lang: Lang;
    hasPrevious?: boolean;
    hasNext?: boolean;
  }>(),
  { hasPrevious: false, hasNext: false },
);

const emit = defineEmits<{
  close: [];
  previous: [];
  next: [];
}>();

const { t } = useI18n({ useScope: "global" });

/** Group mode: the variant entry the user drilled into (null = variant list). */
const drilled = ref<ItemRow | null>(null);

watch(
  () => props.unit,
  () => {
    drilled.value = null;
  },
);

const activeRow = computed<ItemRow | null>(() =>
  props.unit ? drilled.value : (props.row ?? null),
);

const title = computed(() => {
  if (props.unit && !drilled.value) return props.unit.unit.name;
  return activeRow.value ? itemName(activeRow.value.item, props.lang) : "";
});

const description = computed(() => {
  if (props.unit && !drilled.value) {
    return [props.unit.areaLabel, props.unit.locationLabel].filter(Boolean).join(" · ");
  }
  return activeRow.value?.id ?? "";
});

const guides = computed(() => {
  if (activeRow.value) return guideFor(activeRow.value.id);
  const unit = props.unit;
  if (!unit) return null;
  const direct = guideFor(unit.unit.key);
  if (direct) return direct;
  for (const cell of unit.unit.cells) {
    for (const entry of cell) {
      const found = guideFor(entry.id);
      if (found) return found;
    }
  }
  return null;
});

const webLink = computed(() => guides.value?.web ?? null);
const videoLink = computed(() => guides.value?.video ?? null);
const searchName = computed(() =>
  activeRow.value ? itemName(activeRow.value.item, "zh") : (props.unit?.unit.name ?? ""),
);
const engineName = computed(() => t(`settings.engines.${searchEngine().id}`));
const webSearchUrl = computed(() => searchWebUrl(searchName.value));
const videoSearchUrl = computed(() => searchVideoUrl(searchName.value));

const reason = computed(() =>
  activeRow.value && !activeRow.value.obtained
    ? reasonLabel(activeRow.value.item, props.ngPlusCount)
    : null,
);

const aliases = computed(() => {
  const item = activeRow.value?.item;
  if (!item) return [];
  return (item.aliases.length > 0 ? item.aliases : [item.id]).filter((alias) => alias !== item.id);
});

const source = computed(() => {
  const raw = activeRow.value?.source;
  if (!raw) return "";
  if (raw === "game") return t("itemDetail.sourceGame");
  const host = /^https?:\/\/([^/]+)/.exec(raw);
  return host ? t("itemDetail.sourceGuideSite", { host: host[1] }) : raw;
});

const navVisible = computed(
  () => Boolean(props.row) && !drilled.value && (props.hasPrevious || props.hasNext),
);
</script>

<template>
  <UiDialog
    variant="drawer"
    :title="title"
    :description="description"
    @close="emit('close')"
  >
    <template v-if="navVisible" #footer>
      <UiButton variant="ghost" :disabled="!hasPrevious" @click="emit('previous')">
        <template #icon>
          <ChevronLeft class="h-3.5 w-3.5" />
        </template>
        {{ t("itemDetail.previous") }}
      </UiButton>
      <UiButton variant="ghost" :disabled="!hasNext" @click="emit('next')">
        {{ t("itemDetail.next") }}
        <ChevronRight class="h-3.5 w-3.5" />
      </UiButton>
    </template>

    <!-- Matrix acquisition spot: variant list first. -->
    <div v-if="unit && !drilled" class="space-y-4">
      <div class="flex flex-wrap items-center gap-2">
        <span
          class="inline-flex items-center gap-1.5 rounded-md px-2 py-0.5 text-xs font-semibold"
          :class="[areaAccent(unit.areaKey).band, areaAccent(unit.areaKey).text]"
        >
          <span class="h-2 w-2 shrink-0 rounded-full" :class="areaAccent(unit.areaKey).dot"></span>
          {{ unit.areaLabel }}
        </span>
        <span v-if="unit.locationLabel" class="text-sm font-medium text-ink">
          {{ unit.locationLabel }}
        </span>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.obtain") }}</h4>
        <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-ink-muted">
          {{ unit.unit.obtain || "—" }}
        </p>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.chineseGuides") }}</h4>
        <div class="mt-1.5 flex flex-wrap gap-2">
          <UiButton v-if="webLink" :title="webLink.title" @click="openExternalUrl(webLink.url)">
            <template #icon><Image class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.imageGuide") }}
          </UiButton>
          <UiButton v-if="videoLink" :title="videoLink.title" @click="openExternalUrl(videoLink.url)">
            <template #icon><Video class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.videoGuide") }}
          </UiButton>
          <UiButton
            :title="t('itemDetail.searchWebTitle', { engine: engineName, name: searchName })"
            @click="openExternalUrl(webSearchUrl)"
          >
            <template #icon><Search class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.searchWeb") }}
          </UiButton>
          <UiButton
            :title="t('itemDetail.searchVideoTitle', { name: searchName })"
            @click="openExternalUrl(videoSearchUrl)"
          >
            <template #icon><Search class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.searchVideo") }}
          </UiButton>
        </div>
        <p v-if="webLink" class="mt-1.5 text-[11px] leading-4 text-ink-subtle">
          {{ t("itemDetail.guideSource", { title: webLink.title }) }}
        </p>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.cycleVariants") }}</h4>
        <button
          v-for="entry in unit.unit.cells.flat()"
          :key="entry.id"
          type="button"
          class="mt-2 flex w-full items-start gap-3 rounded-md border border-edge bg-surface-sunken px-3 py-2 text-left transition-colors duration-fast hover:border-brand/60"
          @click="drilled = entry"
        >
          <MatrixStatusIcon :row="entry" :ng-plus-count="ngPlusCount" class="mt-0.5" />
          <span class="min-w-0 flex-1">
            <span class="flex flex-wrap items-baseline gap-2">
              <span class="text-xs text-ink">{{ itemName(entry.item, lang) }}</span>
              <UiBadge tone="neutral" size="xs">{{ matrixPeriodLabel(entry.item) }}</UiBadge>
              <span class="font-mono text-[11px] text-ink-subtle">{{ entry.id }}</span>
              <UiBadge v-for="flag in flagLabels(entry.item)" :key="flag" tone="warning" size="xs">
                {{ flag }}
              </UiBadge>
              <UiBadge v-if="reasonLabel(entry.item, ngPlusCount) && !entry.obtained" tone="warning" size="xs">
                {{ reasonLabel(entry.item, ngPlusCount) }}
              </UiBadge>
              <UiBadge v-if="entry.obtained && entry.item.missable" tone="neutral" size="xs">
                {{ t("itemTable.missable") }}
              </UiBadge>
            </span>
            <span class="mt-1 block whitespace-pre-wrap text-[11px] leading-5 text-ink-muted">
              {{ obtainLabel(entry.item, lang) || "—" }}
            </span>
          </span>
          <ChevronRight class="mt-0.5 h-4 w-4 shrink-0 text-ink-subtle" aria-hidden="true" />
        </button>
      </div>
    </div>

    <!-- Single item (list row, dashboard entry or drilled matrix variant). -->
    <div v-else-if="activeRow" class="space-y-4">
      <UiButton v-if="drilled" variant="ghost" @click="drilled = null">
        <template #icon>
          <ChevronLeft class="h-3.5 w-3.5" />
        </template>
        {{ t("itemDetail.backToVariants") }}
      </UiButton>

      <div class="flex flex-wrap items-center gap-2">
        <UiBadge :tone="activeRow.obtained ? 'success' : 'danger'" size="sm">
          <template #icon>
            <CircleCheck v-if="activeRow.obtained" class="h-3 w-3" />
            <CircleX v-else class="h-3 w-3" />
          </template>
          {{ activeRow.obtained ? t("itemTable.obtained") : t("itemTable.missing") }}
        </UiBadge>
        <span
          class="inline-flex items-center gap-1.5 rounded-md px-2 py-0.5 text-xs font-semibold"
          :class="[areaAccent(activeRow.item.area).band, areaAccent(activeRow.item.area).text]"
        >
          <span
            class="h-2 w-2 shrink-0 rounded-full"
            :class="areaAccent(activeRow.item.area).dot"
          ></span>
          {{ areaLabel(activeRow.item, lang) || t("common.uncategorized") }}
        </span>
        <span
          v-if="locationLabel(activeRow.item, lang)"
          class="text-sm font-medium text-ink"
        >
          {{ locationLabel(activeRow.item, lang) }}
        </span>
        <UiBadge v-if="activeRow.item.missable" tone="warning" size="sm">
          {{ t("itemTable.missable") }}
        </UiBadge>
      </div>

      <div v-if="descLabel(activeRow.item, lang)">
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.albumDescription") }}</h4>
        <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-ink-muted">
          {{ descLabel(activeRow.item, lang) }}
        </p>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.obtain") }}</h4>
        <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-ink-muted">
          {{ obtainLabel(activeRow.item, lang) || "—" }}
        </p>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.chineseGuides") }}</h4>
        <div class="mt-1.5 flex flex-wrap gap-2">
          <UiButton v-if="webLink" :title="webLink.title" @click="openExternalUrl(webLink.url)">
            <template #icon><Image class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.imageGuide") }}
          </UiButton>
          <UiButton v-if="videoLink" :title="videoLink.title" @click="openExternalUrl(videoLink.url)">
            <template #icon><Video class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.videoGuide") }}
          </UiButton>
          <UiButton
            :title="t('itemDetail.searchWebTitle', { engine: engineName, name: searchName })"
            @click="openExternalUrl(webSearchUrl)"
          >
            <template #icon><Search class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.searchWeb") }}
          </UiButton>
          <UiButton
            :title="t('itemDetail.searchVideoTitle', { name: searchName })"
            @click="openExternalUrl(videoSearchUrl)"
          >
            <template #icon><Search class="h-3.5 w-3.5" /></template>
            {{ t("itemDetail.searchVideo") }}
          </UiButton>
        </div>
        <p v-if="webLink" class="mt-1.5 text-[11px] leading-4 text-ink-subtle">
          {{ t("itemDetail.guideSource", { title: webLink.title }) }}
        </p>
      </div>

      <div v-if="reason">
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.missingReason") }}</h4>
        <p class="mt-1 text-xs leading-5 text-warning">{{ reason }}</p>
      </div>

      <div>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.details") }}</h4>
        <dl class="mt-1.5 grid grid-cols-[5.5rem_1fr] gap-x-3 gap-y-1.5 text-xs">
          <dt class="text-ink-subtle">{{ t("itemDetail.playthrough") }}</dt>
          <dd class="text-ink-muted">{{ cycleLabel(activeRow.item.ng_plus) }}</dd>
          <template v-if="activeRow.item.dlc">
            <dt class="text-ink-subtle">{{ t("itemDetail.dlc") }}</dt>
            <dd class="text-ink-muted">{{ dlcLabel(activeRow.item.dlc) }}</dd>
          </template>
          <template v-if="activeRow.item.missable">
            <dt class="text-ink-subtle">{{ t("itemDetail.missable") }}</dt>
            <dd class="text-warning">{{ t("common.yes") }}</dd>
          </template>
          <template v-if="recordTypeLabel(activeRow.item, lang)">
            <dt class="text-ink-subtle">{{ t("itemDetail.recordType") }}</dt>
            <dd class="text-ink-muted">{{ recordTypeLabel(activeRow.item, lang) }}</dd>
          </template>
          <dt class="text-ink-subtle">{{ t("itemDetail.confidence") }}</dt>
          <dd :class="activeRow.item.confidence === 'high' ? 'text-ink-muted' : 'text-warning'">
            {{
              activeRow.item.confidence === "high"
                ? t("itemDetail.confidenceHigh")
                : t("itemDetail.confidenceLow")
            }}
          </dd>
          <template v-if="source">
            <dt class="text-ink-subtle">{{ t("itemDetail.source") }}</dt>
            <dd class="text-ink-muted">{{ source }}</dd>
          </template>
        </dl>
      </div>

      <div v-if="aliases.length > 0">
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.aliases") }}</h4>
        <div class="mt-1.5 flex flex-wrap gap-1">
          <span
            v-for="alias in aliases"
            :key="alias"
            class="break-all rounded bg-graphite-100 px-1.5 py-0.5 font-mono text-[11px] text-ink-muted dark:bg-white/6"
          >
            {{ alias }}
          </span>
        </div>
      </div>

      <div v-if="activeRow.item.note">
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("itemDetail.note") }}</h4>
        <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-ink-muted">
          {{ activeRow.item.note }}
        </p>
      </div>
    </div>
  </UiDialog>
</template>
