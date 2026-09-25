<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import { ExternalLink, Info, X } from "@lucide/vue";
import {
  APP_COPYRIGHT,
  APP_ID,
  APP_LICENSE,
  APP_NAME,
  APP_REPO_DETAIL,
  APP_REPO_LABEL,
  APP_REPO_URL,
  DATA_SOURCES,
  DISCLAIMER,
  SOURCES_NOTE,
} from "../lib/about";
import { openExternalUrl } from "../lib/links";

const emit = defineEmits<{ close: [] }>();

const version = ref("");

getVersion()
  .then((value) => {
    version.value = value;
  })
  .catch(() => {
    version.value = "";
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
      class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/70 p-4"
      @click.self="emit('close')"
    >
      <section
        class="flex max-h-[85vh] w-[42rem] max-w-full flex-col rounded-lg border border-slate-700 bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-800 px-4 py-3">
          <div class="flex items-start gap-2">
            <Info class="mt-0.5 h-4 w-4 shrink-0 text-emerald-400" />
            <div>
              <h3 class="text-sm font-semibold text-slate-100">关于 {{ APP_NAME }}</h3>
              <p class="mt-0.5 text-xs text-slate-500">
                {{ APP_ID }}<template v-if="version"> v{{ version }}</template>
              </p>
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

        <div class="space-y-4 overflow-y-auto px-4 py-3">
          <p class="text-xs leading-5 text-slate-300">{{ DISCLAIMER }}</p>

          <section>
            <h4 class="text-xs font-semibold text-slate-400">项目</h4>
            <div
              class="mt-2 flex flex-wrap items-center justify-between gap-2 rounded border border-slate-800 bg-slate-950/40 px-3 py-2"
            >
              <div class="min-w-0">
                <div class="text-xs font-medium text-slate-200">{{ APP_REPO_LABEL }}</div>
                <p class="mt-0.5 text-[11px] leading-5 text-slate-400">{{ APP_REPO_DETAIL }}</p>
                <p class="mt-0.5 select-text break-all font-mono text-[11px] text-slate-500">
                  {{ APP_REPO_URL }}
                </p>
              </div>
              <button
                type="button"
                class="inline-flex shrink-0 items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-200 hover:border-sky-600 hover:bg-slate-800"
                :title="`在浏览器中打开 ${APP_REPO_URL}`"
                @click="openExternalUrl(APP_REPO_URL)"
              >
                <ExternalLink class="h-3.5 w-3.5" />
                打开仓库
              </button>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-400">数据来源</h4>
            <ul class="mt-2 space-y-2">
              <li
                v-for="source in DATA_SOURCES"
                :key="source.name"
                class="rounded border border-slate-800 bg-slate-950/40 px-3 py-2"
              >
                <div class="text-xs font-medium text-slate-200">{{ source.name }}</div>
                <p class="mt-0.5 text-[11px] leading-5 text-slate-400">{{ source.detail }}</p>
                <p
                  v-for="url in source.urls ?? []"
                  :key="url"
                  class="mt-0.5 select-text break-all font-mono text-[11px] text-slate-500"
                >
                  {{ url }}
                </p>
              </li>
            </ul>
            <p class="mt-2 text-[11px] leading-5 text-slate-500">{{ SOURCES_NOTE }}</p>
          </section>

          <section>
            <h4 class="text-xs font-semibold text-slate-400">协议</h4>
            <p class="mt-1 text-xs text-slate-300">
              {{ APP_LICENSE }} · {{ APP_COPYRIGHT }}
            </p>
          </section>
        </div>
      </section>
    </div>
  </Teleport>
</template>
