<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { ChevronDown, Download, FileJson, FileText, Languages } from "@lucide/vue";
import type { Analysis, Lang } from "../types";
import { settings, type ThemeMode } from "../lib/settings";
import { themeOptions } from "../lib/theme";
import { UiButton, UiPopover, UiSegmented } from "./ui";

/**
 * Main topbar (h-12): page title + secondary progress line, then the global
 * controls [content language | theme | export ▾]. Below 1100px the secondary
 * text disappears and the export button shrinks to an icon (F0 §7).
 */
defineProps<{
  title: string;
  analysis: Analysis | null;
}>();

const emit = defineEmits<{ export: [format: "json" | "markdown"] }>();

const { t } = useI18n({ useScope: "global" });

const lang = computed({
  get: () => settings.contentLang,
  set: (value: string) => {
    settings.contentLang = value as Lang;
  },
});

const langOptions = computed(() => [
  { value: "zh", label: t("language.zhShort"), title: t("language.zh") },
  { value: "en", label: t("language.enShort"), title: t("language.en") },
  { value: "both", label: t("language.bothShort"), title: t("language.both") },
]);

const theme = computed({
  get: () => settings.theme,
  set: (value: string) => {
    settings.theme = value as ThemeMode;
  },
});

const themeChoices = computed(() =>
  themeOptions().map((option) => ({
    value: option.value,
    icon: option.icon,
    title: option.label,
    ariaLabel: t("app.appearance", { name: option.label }),
  })),
);

const exportOpen = ref(false);

const MENU_ITEM_CLASS =
  "flex w-full items-center gap-2 rounded-md px-2.5 py-2 text-left text-xs text-ink transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5";

function choose(format: "json" | "markdown"): void {
  exportOpen.value = false;
  emit("export", format);
}
</script>

<template>
  <header
    class="flex h-12 shrink-0 items-center justify-between gap-2 border-b border-edge bg-surface-card px-4"
  >
    <div class="flex min-w-0 items-baseline gap-3">
      <h2 class="truncate text-sm font-semibold text-ink">{{ title }}</h2>
      <span
        v-if="analysis"
        class="hidden min-w-0 truncate text-xs text-ink-muted min-[1100px]:inline"
      >
        {{
          t("app.catalogProgress", {
            obtained: analysis.summary.catalog_obtained,
            total: analysis.summary.catalog_total,
          })
        }}
        ({{ analysis.summary.percent.toFixed(1) }}%) ·
        {{
          t("app.albumProgressShort", {
            obtained: analysis.summary.album_obtained,
            total: analysis.summary.album_total,
          })
        }}
        ({{ analysis.summary.album_percent.toFixed(1) }}%)
      </span>
    </div>

    <div class="flex shrink-0 items-center gap-2">
      <span
        class="hidden items-center gap-1.5 min-[1100px]:inline-flex"
        :title="t('language.contentHint')"
      >
        <Languages class="h-4 w-4 text-ink-muted" aria-hidden="true" />
        <span class="text-xs text-ink-muted">{{ t("language.contentShort") }}</span>
      </span>
      <UiSegmented
        v-model="lang"
        :options="langOptions"
        :aria-label="t('language.contentLabel')"
      />

      <UiSegmented
        v-model="theme"
        :options="themeChoices"
        icon-only
        :aria-label="t('settings.appearance')"
      />

      <UiPopover v-model:open="exportOpen" placement="bottom-end" panel-class="w-44">
        <template #trigger>
          <UiButton
            class="max-[1099px]:hidden"
            variant="outline"
            :aria-expanded="exportOpen"
            aria-haspopup="menu"
            @click="exportOpen = !exportOpen"
          >
            <template #icon>
              <Download class="h-3.5 w-3.5" />
            </template>
            {{ t("app.export") }}
            <ChevronDown class="h-3 w-3" aria-hidden="true" />
          </UiButton>
          <UiButton
            class="hidden max-[1099px]:inline-flex"
            variant="outline"
            icon-only
            :title="t('app.export')"
            :aria-label="t('app.export')"
            :aria-expanded="exportOpen"
            aria-haspopup="menu"
            @click="exportOpen = !exportOpen"
          >
            <template #icon>
              <Download class="h-3.5 w-3.5" />
            </template>
          </UiButton>
        </template>
        <button type="button" role="menuitem" :class="MENU_ITEM_CLASS" @click="choose('json')">
          <FileJson class="h-3.5 w-3.5 shrink-0 text-ink-muted" aria-hidden="true" />
          {{ t("app.exportJson") }}
        </button>
        <button
          type="button"
          role="menuitem"
          :class="MENU_ITEM_CLASS"
          @click="choose('markdown')"
        >
          <FileText class="h-3.5 w-3.5 shrink-0 text-ink-muted" aria-hidden="true" />
          {{ t("app.exportMarkdown") }}
        </button>
      </UiPopover>
    </div>
  </header>
</template>
