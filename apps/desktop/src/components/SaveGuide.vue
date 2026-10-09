<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { FileSearch, FolderOpen, FolderX, RefreshCw, TriangleAlert } from "@lucide/vue";
import type { SaveSource } from "../types";
import { APP_ICON } from "../lib/icons";
import { UiButton, UiCard } from "./ui";

/**
 * Startup / empty-state guide: big brand icon, scanned directory list with
 * per-directory open action, manual file picker and rescan (F0 wireframe §6).
 */
defineProps<{
  sources: SaveSource[];
  error: string;
  scanning: boolean;
  activePath: string | null;
}>();

const emit = defineEmits<{
  pick: [];
  refresh: [];
  open: [path: string];
}>();

const { t } = useI18n({ useScope: "global" });
</script>

<template>
  <div class="mx-auto flex w-full max-w-3xl flex-col gap-4 py-4">
    <div class="flex flex-col items-center gap-2 text-center">
      <component :is="APP_ICON" class="h-10 w-10 text-brand/80" aria-hidden="true" />
      <h2 class="flex items-center gap-2 text-base font-semibold text-ink">
        <TriangleAlert v-if="error" class="h-4 w-4 text-danger" aria-hidden="true" />
        {{ error ? t("saveGuide.readError") : t("saveGuide.notFound") }}
      </h2>
      <p
        v-if="error"
        class="max-w-xl break-all font-mono text-xs leading-5 text-danger"
      >
        {{ error }}
      </p>
      <p
        v-else
        class="max-w-xl text-xs leading-5 text-ink-muted"
        v-html="t('saveGuide.hint')"
      ></p>
      <p v-if="activePath" class="break-all font-mono text-[11px] text-ink-subtle">
        {{ t("saveGuide.currentFile", { path: activePath }) }}
      </p>
    </div>

    <UiCard padding="sm">
      <ul class="space-y-1.5">
        <li
          v-for="source in sources"
          :key="source.path"
          class="flex flex-wrap items-center justify-between gap-3 rounded-md border px-3 py-2"
          :class="source.exists ? 'border-edge' : 'border-dashed border-edge/70'"
        >
          <div class="min-w-0">
            <div
              class="flex items-center gap-1.5 text-xs"
              :class="source.exists ? 'text-ink-muted' : 'text-ink-subtle'"
            >
              <FolderOpen v-if="source.exists" class="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
              <FolderX v-else class="h-3.5 w-3.5 shrink-0" aria-hidden="true" />
              {{ source.exists ? t("saveGuide.directoryExists") : t("saveGuide.directoryMissing") }}
            </div>
            <div class="mt-0.5 break-all font-mono text-[11px] text-ink-subtle">
              {{ source.path }}
            </div>
          </div>
          <UiButton v-if="source.exists" @click="emit('open', source.path)">
            <template #icon>
              <FolderOpen class="h-3.5 w-3.5" />
            </template>
            {{ t("saveGuide.open") }}
          </UiButton>
        </li>
        <li v-if="sources.length === 0" class="px-3 py-2 text-xs text-ink-subtle">
          {{ t("saveGuide.loadingDirs") }}
        </li>
      </ul>

      <div class="mt-3 flex flex-wrap items-center gap-2 border-t border-edge pt-3">
        <UiButton variant="primary" @click="emit('pick')">
          <template #icon>
            <FileSearch class="h-3.5 w-3.5" />
          </template>
          {{ t("saveGuide.pick") }}
        </UiButton>
        <UiButton :disabled="scanning" @click="emit('refresh')">
          <template #icon>
            <RefreshCw class="h-3.5 w-3.5" :class="scanning && 'animate-spin'" />
          </template>
          {{ t("saveGuide.rescan") }}
        </UiButton>
        <span class="text-[11px] leading-5 text-ink-subtle">
          {{ t("saveGuide.note") }}
        </span>
      </div>
    </UiCard>
  </div>
</template>
