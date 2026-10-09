<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Check, ChevronDown, ChevronLeft, ChevronRight, Info, Settings } from "@lucide/vue";
import type { Analysis, CategoryResult, SaveSlot, SaveSource } from "../types";
import { categoryName } from "../lib/display";
import { APP_ICON, categoryIcon, SUMMARY_ICON } from "../lib/icons";
import { settings } from "../lib/settings";
import AboutDialog from "./AboutDialog.vue";
import SaveCard from "./SaveCard.vue";
import SettingsDialog from "./SettingsDialog.vue";
import { UiBadge, UiButton, UiProgress } from "./ui";

/**
 * Application sidebar (F0 decision ②): w-64 expanded / w-14 icon rail,
 * group folding, per-category missing badge (✓ when complete) and thin
 * progress bars, save card on top, settings/about at the bottom.
 */
const props = defineProps<{
  analysis: Analysis | null;
  active: string;
  collapsed: boolean;
  saves: SaveSlot[];
  selected: SaveSlot | null;
  sources: SaveSource[];
  scanning: boolean;
}>();

const emit = defineEmits<{
  navigate: [key: string];
  toggleCollapse: [];
  selectSave: [slot: SaveSlot];
  refreshSaves: [];
  pickSave: [];
  openDir: [path: string];
}>();

const { t } = useI18n({ useScope: "global" });

const aboutOpen = ref(false);
const settingsOpen = ref(false);
const collapsedGroups = ref<Set<string>>(new Set());

const groups = computed(() => {
  const categories = props.analysis?.categories ?? [];
  return [
    {
      key: "collections",
      label: t("sidebar.collectionsGroup"),
      categories: categories.filter((category) => category.section !== "album"),
    },
    {
      key: "album",
      label: t("sidebar.albumGroup"),
      categories: categories.filter((category) => category.section === "album"),
    },
  ];
});

function toggleGroup(key: string): void {
  const next = new Set(collapsedGroups.value);
  if (next.has(key)) next.delete(key);
  else next.add(key);
  collapsedGroups.value = next;
}

function percentOf(category: CategoryResult): number {
  if (category.total === 0) return 0;
  return (category.obtained / category.total) * 100;
}

function itemTitle(category: CategoryResult): string {
  return `${categoryName(category, settings.contentLang)} · ${t("categoryPage.progress", {
    obtained: category.obtained,
    total: category.total,
    missing: category.missing.length,
    percent: percentOf(category).toFixed(0),
  })}`;
}

const ITEM_CLASS =
  "w-full rounded-md text-left transition-colors duration-fast " +
  "text-ink-muted hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5";

const RAIL_ITEM_CLASS =
  "relative mx-auto flex h-8 w-8 items-center justify-center rounded-md transition-colors duration-fast " +
  "text-ink-muted hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5";

function itemClass(key: string): (string | false)[] {
  return [
    props.collapsed ? RAIL_ITEM_CLASS : `${ITEM_CLASS} relative px-2 py-2`,
    props.active === key && "bg-brand-soft",
  ];
}

function footerClass(): (string | false)[] {
  return [props.collapsed ? RAIL_ITEM_CLASS : `${ITEM_CLASS} flex items-center gap-2 px-2 py-2 text-sm`];
}
</script>

<template>
  <aside
    class="flex shrink-0 flex-col border-r border-edge bg-surface-card transition-[width] duration-normal ease-out-soft"
    :class="collapsed ? 'w-14' : 'w-64'"
  >
    <header class="flex items-center gap-2 px-2.5 py-3">
      <button
        v-if="collapsed"
        type="button"
        class="mx-auto flex h-8 w-8 items-center justify-center rounded-md text-brand transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
        :title="t('sidebar.expand')"
        :aria-label="t('sidebar.expand')"
        @click="emit('toggleCollapse')"
      >
        <component :is="APP_ICON" class="h-4 w-4" />
      </button>
      <template v-else>
        <component :is="APP_ICON" class="h-4 w-4 shrink-0 text-brand" aria-hidden="true" />
        <h1 class="min-w-0 flex-1 truncate text-sm font-bold tracking-wide text-ink">
          {{ t("common.appName") }}
        </h1>
        <UiButton
          variant="ghost"
          icon-only
          :title="t('sidebar.collapse')"
          :aria-label="t('sidebar.collapse')"
          @click="emit('toggleCollapse')"
        >
          <template #icon>
            <ChevronLeft class="h-4 w-4" />
          </template>
        </UiButton>
      </template>
    </header>

    <div class="px-2 pb-2">
      <SaveCard
        :saves="saves"
        :selected="selected"
        :sources="sources"
        :scanning="scanning"
        :analysis="analysis"
        :collapsed="collapsed"
        @select="emit('selectSave', $event)"
        @refresh="emit('refreshSaves')"
        @pick="emit('pickSave')"
        @open="emit('openDir', $event)"
      />
    </div>

    <nav class="min-h-0 flex-1 space-y-0.5 overflow-y-auto px-2 py-1">
      <button
        type="button"
        :class="itemClass('summary')"
        :title="t('app.summary')"
        @click="emit('navigate', 'summary')"
      >
        <template v-if="collapsed">
          <component :is="SUMMARY_ICON" class="h-4 w-4" :class="active === 'summary' && 'text-brand'" />
        </template>
        <template v-else>
          <div class="flex items-center justify-between gap-2">
            <span class="inline-flex min-w-0 items-center gap-2">
              <component
                :is="SUMMARY_ICON"
                class="h-4 w-4 shrink-0"
                :class="active === 'summary' ? 'text-brand' : ''"
              />
              <span class="truncate text-sm font-medium">{{ t("app.summary") }}</span>
            </span>
            <span v-if="analysis" class="text-xs tabular-nums text-ink-muted">
              {{ analysis.summary.percent.toFixed(0) }}%
            </span>
          </div>
          <UiProgress
            v-if="analysis"
            class="mt-1.5"
            :value="analysis.summary.percent"
            tone="brand"
          />
        </template>
      </button>

      <template v-if="analysis">
        <template v-for="group in groups" :key="group.key">
          <button
            v-if="!collapsed"
            type="button"
            class="flex w-full items-center gap-1 rounded-md px-2 pb-1 pt-3 text-left text-[11px] font-medium tracking-wide text-ink-subtle transition-colors duration-fast hover:text-ink"
            :aria-expanded="!collapsedGroups.has(group.key)"
            @click="toggleGroup(group.key)"
          >
            <component
              :is="collapsedGroups.has(group.key) ? ChevronRight : ChevronDown"
              class="h-3 w-3 shrink-0"
              aria-hidden="true"
            />
            {{ group.label }}
          </button>
          <div v-else class="pt-2" aria-hidden="true"></div>
          <template v-if="collapsed || !collapsedGroups.has(group.key)">
            <button
              v-for="category in group.categories"
              :key="category.key"
              type="button"
              :class="itemClass(category.key)"
              :title="collapsed ? categoryName(category, settings.contentLang) : itemTitle(category)"
              @click="emit('navigate', category.key)"
            >
              <template v-if="collapsed">
                <component
                  :is="categoryIcon(category.key)"
                  class="h-4 w-4"
                  :class="active === category.key ? 'text-brand' : ''"
                />
                <span
                  class="absolute right-0.5 top-0.5 h-1.5 w-1.5 rounded-full"
                  :class="category.missing.length > 0 ? 'bg-danger' : 'bg-success'"
                  aria-hidden="true"
                ></span>
              </template>
              <template v-else>
                <div class="flex items-center justify-between gap-2">
                  <span class="inline-flex min-w-0 items-center gap-2">
                    <component
                      :is="categoryIcon(category.key)"
                      class="h-4 w-4 shrink-0"
                      :class="active === category.key ? 'text-brand' : ''"
                    />
                    <span
                      class="truncate text-sm"
                      :class="active === category.key ? 'font-medium text-ink' : ''"
                    >
                      {{ categoryName(category, settings.contentLang) }}
                    </span>
                  </span>
                  <UiBadge
                    v-if="category.missing.length > 0"
                    tone="danger"
                    size="xs"
                    :title="itemTitle(category)"
                  >
                    {{ category.missing.length }}
                  </UiBadge>
                  <UiBadge v-else tone="success" size="xs" :title="t('sidebar.complete')">
                    <template #icon>
                      <Check class="h-3 w-3" aria-hidden="true" />
                    </template>
                  </UiBadge>
                </div>
                <UiProgress
                  class="mt-1.5"
                  :value="percentOf(category)"
                  :tone="category.section === 'album' ? 'album' : 'success'"
                />
              </template>
            </button>
          </template>
        </template>
      </template>
    </nav>

    <footer class="space-y-0.5 border-t border-edge p-2">
      <button
        type="button"
        :class="footerClass()"
        :title="t('sidebar.settingsTitle')"
        @click="settingsOpen = true"
      >
        <Settings class="h-4 w-4 shrink-0" aria-hidden="true" />
        <span v-if="!collapsed">{{ t("sidebar.settings") }}</span>
      </button>
      <button
        type="button"
        :class="footerClass()"
        :title="t('sidebar.aboutTitle')"
        @click="aboutOpen = true"
      >
        <Info class="h-4 w-4 shrink-0" aria-hidden="true" />
        <span v-if="!collapsed">{{ t("sidebar.about") }}</span>
      </button>
    </footer>

    <AboutDialog v-if="aboutOpen" @close="aboutOpen = false" />
    <SettingsDialog v-if="settingsOpen" @close="settingsOpen = false" />
  </aside>
</template>
