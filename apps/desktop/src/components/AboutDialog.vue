<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { getVersion } from "@tauri-apps/api/app";
import { ExternalLink } from "@lucide/vue";
import { APP_COPYRIGHT, APP_ID, APP_LICENSE, DATA_SOURCES, PROJECT_LINKS } from "../lib/about";
import { APP_ICON } from "../lib/icons";
import { openExternalUrl } from "../lib/links";
import { UiButton, UiDialog } from "./ui";

/**
 * About modal rebuilt on the F1 dialog system: project links, data sources
 * and license, with all external URLs kept in `lib/about.ts`.
 */
const emit = defineEmits<{ close: [] }>();

const { t } = useI18n({ useScope: "global" });

const version = ref("");

getVersion()
  .then((value) => {
    version.value = value;
  })
  .catch(() => {
    version.value = "";
  });

const CARD_CLASS = "rounded-md border border-edge bg-graphite-50 px-3 py-2 dark:bg-white/4";
</script>

<template>
  <UiDialog
    :title="t('about.title', { app: t('common.appName') })"
    :description="`${APP_ID}${version ? ` v${version}` : ''}`"
    :icon="APP_ICON"
    size="lg"
    @close="emit('close')"
  >
    <div class="space-y-4">
      <p class="text-xs leading-5 text-ink-muted">{{ t("about.disclaimer") }}</p>

      <section>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("about.project") }}</h4>
        <div class="mt-2 space-y-2">
          <div
            v-for="link in PROJECT_LINKS"
            :key="link.url"
            :class="[CARD_CLASS, 'flex flex-wrap items-center justify-between gap-2']"
          >
            <div class="min-w-0">
              <div class="text-xs font-medium text-ink">
                {{ t(`about.projects.${link.key}Label`) }}
              </div>
              <p class="mt-0.5 text-[11px] leading-5 text-ink-muted">
                {{ t(`about.projects.${link.key}Detail`) }}
              </p>
              <p class="mt-0.5 select-text break-all font-mono text-[11px] text-ink-subtle">
                {{ link.url }}
              </p>
            </div>
            <UiButton
              class="shrink-0"
              :title="t('about.openInBrowser', { url: link.url })"
              @click="openExternalUrl(link.url)"
            >
              <template #icon>
                <ExternalLink class="h-3.5 w-3.5" />
              </template>
              {{ t(`about.projects.${link.key}Action`) }}
            </UiButton>
          </div>
        </div>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("about.sources") }}</h4>
        <ul class="mt-2 space-y-2">
          <li v-for="source in DATA_SOURCES" :key="source.key" :class="CARD_CLASS">
            <div class="text-xs font-medium text-ink">
              {{ t(`about.sourceNames.${source.key}`) }}
            </div>
            <p class="mt-0.5 text-[11px] leading-5 text-ink-muted">
              {{ t(`about.sourcesList.${source.key}Detail`) }}
            </p>
            <p
              v-for="url in source.urls ?? []"
              :key="url"
              class="mt-0.5 select-text break-all font-mono text-[11px] text-ink-subtle"
            >
              {{ url }}
            </p>
          </li>
        </ul>
        <p class="mt-2 text-[11px] leading-5 text-ink-subtle">{{ t("about.sourcesNote") }}</p>
      </section>

      <section>
        <h4 class="text-xs font-semibold text-ink-muted">{{ t("about.license") }}</h4>
        <p class="mt-1 text-xs text-ink-muted">{{ APP_LICENSE }} · {{ APP_COPYRIGHT }}</p>
      </section>
    </div>
  </UiDialog>
</template>
