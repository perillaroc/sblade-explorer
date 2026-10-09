<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { Check, ChevronDown, FileSearch, FolderOpen, HardDrive, RefreshCw } from "@lucide/vue";
import type { Analysis, SaveSlot, SaveSource } from "../types";
import { saveSize, saveTime } from "../lib/display";
import { UiBadge, UiButton, UiPopover, UiProgress } from "./ui";

/**
 * Sidebar save card (F0 decision ②): shows the active slot and opens a
 * popover to switch slots / refresh / pick a file / open the scanned folder.
 * Collapsed to the w-14 rail it becomes an icon button with a flyout.
 */
const props = defineProps<{
  saves: SaveSlot[];
  selected: SaveSlot | null;
  sources: SaveSource[];
  scanning: boolean;
  analysis: Analysis | null;
  collapsed: boolean;
}>();

const emit = defineEmits<{
  select: [slot: SaveSlot];
  refresh: [];
  pick: [];
  open: [path: string];
}>();

const { t } = useI18n({ useScope: "global" });

const open = ref(false);

const meta = computed(() => {
  if (!props.selected) return "";
  return `${saveTime(props.selected.mtimeMs)} · ${saveSize(props.selected.size)}`;
});

const playthrough = computed(() => {
  const count = props.analysis?.save.ng_plus_count ?? 0;
  return count <= 0 ? t("summary.playthroughFirst") : t("summary.playthroughNgPlus", { count });
});

/** `open_save_dir` only accepts directories reported by `save_sources`. */
const openDirPath = computed(() => {
  const selected = props.selected;
  if (!selected) return null;
  const dir = selected.path.replace(/[\\/][^\\/]*$/, "").toLowerCase();
  const source = props.sources.find(
    (candidate) => candidate.exists && candidate.path.toLowerCase() === dir,
  );
  return source?.path ?? null;
});

function choose(slot: SaveSlot): void {
  open.value = false;
  emit("select", slot);
}

function pick(): void {
  open.value = false;
  emit("pick");
}

function openDir(): void {
  open.value = false;
  if (openDirPath.value) emit("open", openDirPath.value);
}
</script>

<template>
  <UiPopover
    v-model:open="open"
    :placement="collapsed ? 'right-start' : 'bottom-start'"
    :panel-class="collapsed ? 'w-72' : 'w-full'"
  >
    <template #trigger>
      <button
        v-if="collapsed"
        type="button"
        class="mx-auto flex h-8 w-8 items-center justify-center rounded-md text-ink-muted transition-colors duration-fast hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5"
        :title="selected?.label ?? t('savePicker.label')"
        :aria-label="t('savePicker.label')"
        :aria-expanded="open"
        @click="open = !open"
      >
        <HardDrive class="h-4 w-4" />
      </button>
      <button
        v-else
        type="button"
        class="w-full rounded-card border border-edge bg-surface-card px-3 py-2 text-left transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
        :aria-expanded="open"
        @click="open = !open"
      >
        <div class="flex items-center gap-2">
          <HardDrive class="h-4 w-4 shrink-0 text-brand" aria-hidden="true" />
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-1.5">
              <span class="truncate text-xs font-medium text-ink">
                {{
                  selected?.label ??
                  (scanning ? t("savePicker.scanning") : t("savePicker.notFound"))
                }}
              </span>
              <UiBadge v-if="analysis" tone="brand" size="xs">{{ playthrough }}</UiBadge>
            </div>
            <span v-if="meta" class="mt-0.5 block truncate text-[11px] text-ink-subtle">
              {{ meta }}
            </span>
          </div>
          <ChevronDown
            class="h-3.5 w-3.5 shrink-0 text-ink-subtle transition-transform duration-fast"
            :class="open && 'rotate-180'"
            aria-hidden="true"
          />
        </div>
        <UiProgress
          v-if="analysis"
          :value="analysis.summary.percent"
          tone="success"
          class="mt-2"
          :aria-label="t('app.summary')"
        />
      </button>
    </template>

    <div class="flex items-center justify-between gap-2 px-1 pb-0.5">
      <span class="text-xs font-semibold text-ink">{{ t("savePicker.label") }}</span>
      <UiButton
        variant="ghost"
        icon-only
        :disabled="scanning"
        :title="t('savePicker.refresh')"
        :aria-label="t('savePicker.refresh')"
        @click="emit('refresh')"
      >
        <template #icon>
          <RefreshCw class="h-3.5 w-3.5" :class="scanning && 'animate-spin'" />
        </template>
      </UiButton>
    </div>

    <div class="max-h-56 space-y-0.5 overflow-y-auto">
      <button
        v-for="slot in saves"
        :key="slot.path"
        type="button"
        class="flex w-full items-start gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
        :class="slot.path === selected?.path && 'bg-brand-soft'"
        @click="choose(slot)"
      >
        <Check
          v-if="slot.path === selected?.path"
          class="mt-0.5 h-3.5 w-3.5 shrink-0 text-brand"
          aria-hidden="true"
        />
        <span v-else class="mt-0.5 h-3.5 w-3.5 shrink-0" aria-hidden="true"></span>
        <span class="min-w-0 flex-1">
          <span class="block truncate text-ink">{{ slot.label }}</span>
          <span class="mt-0.5 block truncate text-[11px] text-ink-subtle">
            {{ saveTime(slot.mtimeMs) }} · {{ saveSize(slot.size) }}
          </span>
        </span>
      </button>
      <p v-if="saves.length === 0" class="px-2 py-1.5 text-[11px] text-ink-subtle">
        {{ scanning ? t("savePicker.scanning") : t("savePicker.notFound") }}
      </p>
    </div>

    <div class="mt-1 space-y-0.5 border-t border-edge pt-1.5">
      <button
        type="button"
        class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs text-ink transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
        @click="pick"
      >
        <FileSearch class="h-3.5 w-3.5 shrink-0 text-ink-muted" aria-hidden="true" />
        {{ t("savePicker.pick") }}
      </button>
      <button
        v-if="openDirPath"
        type="button"
        class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs text-ink transition-colors duration-fast hover:bg-graphite-100 dark:hover:bg-white/5"
        @click="openDir"
      >
        <FolderOpen class="h-3.5 w-3.5 shrink-0 text-ink-muted" aria-hidden="true" />
        {{ t("savePicker.openDir") }}
      </button>
      <p v-else-if="selected" class="select-text break-all px-2 py-1 font-mono text-[11px] leading-4 text-ink-subtle" :title="selected.path">
        {{ selected.path }}
      </p>
    </div>
  </UiPopover>
</template>
