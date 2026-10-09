<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
  BookOpen,
  CalendarClock,
  ChevronRight,
  CircleCheck,
  Clock,
  Database,
  Gauge,
  HardDrive,
  IdCard,
  ListChecks,
  Package,
  Repeat,
  Save,
  TriangleAlert,
  Zap,
  type LucideIcon,
} from "@lucide/vue";
import type { Analysis, CategoryResult, SaveSlot } from "../types";
import { actionableRows, missableRows } from "../lib/actionable";
import { areaAccent, type AreaAccent } from "../lib/area";
import { areaLabel, categoryName, itemName, locationLine, saveSize, saveTime } from "../lib/display";
import { categoryIcon } from "../lib/icons";
import type { ItemRow } from "../lib/items";
import { settings } from "../lib/settings";
import DetailDrawer from "./DetailDrawer.vue";
import { UiBadge, UiButton, UiCard, UiProgress } from "./ui";

/**
 * Dashboard home (F0 decision ③): save info card, dual progress cards,
 * "ready to collect" list (missing items without a blocking reason, grouped
 * by area, missable items flagged first) and the category card grid.
 */
const props = defineProps<{ analysis: Analysis; slot: SaveSlot | null }>();

const emit = defineEmits<{ navigate: [key: string] }>();

const { t } = useI18n({ useScope: "global" });

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
  return t("summary.playTimeValue", { hours, minutes: String(minutes).padStart(2, "0") });
});

const infoFields = computed<{ key: string; icon: LucideIcon; label: string; value: string }[]>(
  () => [
    {
      key: "file",
      icon: HardDrive,
      label: t("summary.slot"),
      value: props.slot?.label ?? props.analysis.save.path.split(/[\\/]/).pop() ?? "—",
    },
    {
      key: "steam",
      icon: IdCard,
      label: "SteamID",
      value: props.analysis.save.steam_id ?? t("common.unknown"),
    },
    {
      key: "playthrough",
      icon: Repeat,
      label: t("summary.playthrough"),
      value: `${playthroughLabel.value} (NG+${props.analysis.save.ng_plus_count})`,
    },
    { key: "difficulty", icon: Gauge, label: t("summary.difficulty"), value: difficultyLabel.value },
    { key: "playtime", icon: Clock, label: t("summary.playTime"), value: playTimeLabel.value },
    {
      key: "size",
      icon: Database,
      label: t("summary.fileSize"),
      value: props.slot ? saveSize(props.slot.size) : "—",
    },
    {
      key: "modified",
      icon: CalendarClock,
      label: t("summary.modified"),
      value: props.slot ? saveTime(props.slot.mtimeMs) : "—",
    },
  ],
);

const sections = computed<
  { key: string; title: string; icon: LucideIcon; categories: CategoryResult[] }[]
>(() => [
  {
    key: "collections",
    title: t("summary.categoriesTitle"),
    icon: ListChecks,
    categories: props.analysis.categories.filter((category) => category.section !== "album"),
  },
  {
    key: "album",
    title: t("summary.albumSection"),
    icon: BookOpen,
    categories: props.analysis.categories.filter((category) => category.section === "album"),
  },
]);

function percentOf(category: CategoryResult): number {
  return category.total === 0 ? 0 : (category.obtained / category.total) * 100;
}

function blockedOf(category: CategoryResult): number {
  return category.missing.filter((item) => item.reason !== null).length;
}

const actionable = computed(() => actionableRows(props.analysis));
const missable = computed(() => missableRows(props.analysis));

const DISPLAY_LIMIT = 8;
const expanded = ref(false);
const visibleActionable = computed(() =>
  expanded.value ? actionable.value : actionable.value.slice(0, DISPLAY_LIMIT),
);

interface ActionGroup {
  key: string;
  label: string;
  accent: AreaAccent;
  items: { row: ItemRow; index: number }[];
}

const actionableGroups = computed<ActionGroup[]>(() => {
  const map = new Map<string, ActionGroup>();
  visibleActionable.value.forEach((row, index) => {
    const key = row.item.area ?? "";
    let group = map.get(key);
    if (!group) {
      group = {
        key,
        label: areaLabel(row.item, settings.contentLang) || t("common.uncategorized"),
        accent: areaAccent(row.item.area),
        items: [],
      };
      map.set(key, group);
    }
    group.items.push({ row, index });
  });
  return [...map.values()];
});

const selectedIndex = ref(-1);
const selectedRow = computed(() =>
  selectedIndex.value >= 0 ? (actionable.value[selectedIndex.value] ?? null) : null,
);

function openAction(index: number): void {
  selectedIndex.value = index;
}

function navigate(delta: number): void {
  const next = selectedIndex.value + delta;
  if (next >= 0 && next < actionable.value.length) selectedIndex.value = next;
}
</script>

<template>
  <div class="space-y-4">
    <UiCard padding="md">
      <template #header>
        <h2 class="flex items-center gap-1.5 text-sm font-semibold text-ink">
          <Save class="h-4 w-4 text-brand" aria-hidden="true" />
          {{ t("summary.saveInfo") }}
        </h2>
        <span
          class="hidden max-w-64 truncate font-mono text-[11px] text-ink-subtle min-[1100px]:inline"
          :title="analysis.save.path"
        >
          {{ analysis.save.path }}
        </span>
      </template>
      <div class="grid grid-cols-2 gap-x-4 gap-y-3 sm:grid-cols-3 xl:grid-cols-4">
        <div v-for="field in infoFields" :key="field.key" class="min-w-0">
          <div class="flex items-center gap-1 text-xs text-ink-subtle">
            <component :is="field.icon" class="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
            {{ field.label }}
          </div>
          <div class="mt-0.5 truncate text-sm text-ink" :title="field.value">{{ field.value }}</div>
        </div>
      </div>
      <template #footer>
        <span class="text-[11px] text-ink-subtle">
          {{
            t("summary.aliases", {
              obtained: summary.obtained_aliases,
              unmapped: summary.unmapped_aliases,
            })
          }}
        </span>
      </template>
    </UiCard>

    <div class="grid gap-3 md:grid-cols-2">
      <UiCard padding="md">
        <div class="flex items-center justify-between gap-3">
          <span class="inline-flex items-center gap-1.5 text-xs font-medium text-ink-muted">
            <Package class="h-4 w-4" aria-hidden="true" />
            {{ t("summary.catalogTitle") }}
          </span>
          <span class="text-3xl font-semibold tabular-nums text-ink">
            {{ summary.percent.toFixed(1) }}%
          </span>
        </div>
        <UiProgress class="mt-3" size="md" :value="summary.percent" tone="success" />
        <p class="mt-2 text-xs text-ink-muted">
          {{
            t("app.catalogProgress", {
              obtained: summary.catalog_obtained,
              total: summary.catalog_total,
            })
          }}
          · {{ t("summary.missingShort", { count: summary.missing_total }) }}
        </p>
      </UiCard>

      <UiCard padding="md">
        <div class="flex items-center justify-between gap-3">
          <span class="inline-flex items-center gap-1.5 text-xs font-medium text-ink-muted">
            <BookOpen class="h-4 w-4" aria-hidden="true" />
            {{ t("summary.albumTitle") }}
          </span>
          <span class="text-3xl font-semibold tabular-nums text-ink">
            {{ summary.album_percent.toFixed(1) }}%
          </span>
        </div>
        <UiProgress class="mt-3" size="md" :value="summary.album_percent" tone="album" />
        <p class="mt-2 text-xs text-ink-muted">
          {{
            t("app.albumProgressShort", {
              obtained: summary.album_obtained,
              total: summary.album_total,
            })
          }}
          · {{ t("summary.missingShort", { count: summary.album_missing_total }) }}
        </p>
      </UiCard>
    </div>

    <UiCard padding="md">
      <template #header>
        <h2 class="flex items-center gap-1.5 text-sm font-semibold text-ink">
          <Zap class="h-4 w-4 text-brand" aria-hidden="true" />
          {{ t("summary.priorityTitle") }}
          <UiBadge v-if="actionable.length > 0" tone="brand" size="xs">
            {{ actionable.length }}
          </UiBadge>
        </h2>
        <span class="hidden text-xs text-ink-muted min-[1100px]:inline">
          {{ t("summary.priorityHint") }}
        </span>
      </template>

      <div
        v-if="missable.length > 0"
        class="mb-3 flex items-center gap-2 rounded-md bg-warning/10 px-3 py-2 text-xs text-warning"
      >
        <TriangleAlert class="h-4 w-4 shrink-0" aria-hidden="true" />
        {{ t("summary.priorityMissable", { count: missable.length }) }}
      </div>

      <p v-if="actionable.length === 0" class="py-4 text-center text-xs text-ink-muted">
        {{ t("summary.priorityEmpty") }}
      </p>
      <template v-else>
        <div v-for="group in actionableGroups" :key="group.key" class="mt-3 first:mt-0">
          <div class="flex items-center gap-2 px-2 text-xs font-medium text-ink-muted">
            <span class="h-2 w-2 shrink-0 rounded-full" :class="group.accent.dot" aria-hidden="true"></span>
            {{ group.label }}
            <span class="tabular-nums text-ink-subtle">{{ group.items.length }}</span>
          </div>
          <ul class="mt-1">
            <li v-for="entry in group.items" :key="entry.row.id">
              <button
                type="button"
                class="flex w-full items-center justify-between gap-3 rounded-md px-2 py-1.5 text-left transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
                @click="openAction(entry.index)"
              >
                <span class="min-w-0">
                  <span class="block truncate text-sm text-ink">
                    {{ itemName(entry.row.item, settings.contentLang) }}
                  </span>
                  <span class="mt-0.5 block truncate text-[11px] text-ink-subtle">
                    {{ locationLine(entry.row.item, settings.contentLang) || entry.row.id }}
                  </span>
                </span>
                <span class="inline-flex shrink-0 items-center gap-1 text-xs text-brand">
                  {{ t("common.view") }}
                  <ChevronRight class="h-3.5 w-3.5" aria-hidden="true" />
                </span>
              </button>
            </li>
          </ul>
        </div>

        <div class="mt-3 flex items-center justify-between gap-3">
          <span class="text-[11px] tabular-nums text-ink-subtle">
            {{ t("summary.priorityCount", { count: actionable.length }) }}
          </span>
          <UiButton
            v-if="actionable.length > DISPLAY_LIMIT"
            variant="ghost"
            @click="expanded = !expanded"
          >
            {{ expanded ? t("summary.collapse") : t("summary.expandAll") }}
          </UiButton>
        </div>
      </template>
    </UiCard>

    <section v-for="section in sections" :key="section.key" class="space-y-3">
      <h2 class="flex items-center gap-1.5 text-sm font-semibold text-ink">
        <component :is="section.icon" class="h-4 w-4 text-ink-muted" aria-hidden="true" />
        {{ section.title }}
      </h2>
      <div class="grid grid-cols-2 gap-3 md:grid-cols-3 xl:grid-cols-4">
        <button
          v-for="category in section.categories"
          :key="category.key"
          type="button"
          class="rounded-card border border-edge bg-surface-card p-3 text-left shadow-card transition-colors duration-fast hover:border-brand/60 dark:shadow-none"
          @click="emit('navigate', category.key)"
        >
          <div class="flex items-center gap-2">
            <component
              :is="categoryIcon(category.key)"
              class="h-4 w-4 shrink-0 text-ink-muted"
              aria-hidden="true"
            />
            <span class="min-w-0 flex-1 truncate text-sm font-medium text-ink">
              {{ categoryName(category, settings.contentLang) }}
            </span>
            <UiBadge v-if="category.missing.length > 0" tone="danger" size="xs">
              {{ category.missing.length }}
            </UiBadge>
            <UiBadge v-else tone="success" size="xs">
              <template #icon>
                <CircleCheck class="h-3 w-3" />
              </template>
            </UiBadge>
          </div>
          <UiProgress
            class="mt-2"
            :value="percentOf(category)"
            :tone="section.key === 'album' ? 'album' : 'success'"
          />
          <div class="mt-1.5 flex items-center justify-between text-[11px] tabular-nums text-ink-subtle">
            <span>
              {{ category.obtained }}/{{ category.total }} ({{ percentOf(category).toFixed(0) }}%)
            </span>
            <span v-if="section.key === 'collections'">
              {{ t("summary.blockedShort", { count: blockedOf(category) }) }}
            </span>
            <span v-else>
              {{ t("summary.missingShort", { count: category.missing.length }) }}
            </span>
          </div>
        </button>
      </div>
    </section>

    <DetailDrawer
      v-if="selectedRow"
      :row="selectedRow"
      :ng-plus-count="analysis.save.ng_plus_count"
      :lang="settings.contentLang"
      :has-previous="selectedIndex > 0"
      :has-next="selectedIndex < actionable.length - 1"
      @close="selectedIndex = -1"
      @previous="navigate(-1)"
      @next="navigate(1)"
    />
  </div>
</template>
