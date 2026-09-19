<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import type { Lang } from "../types";
import { matrixName, obtainLabel } from "../lib/display";
import { matrixPeriodLabel, statusEmoji, type MatrixUnit } from "../lib/matrix";

defineProps<{
  unit: MatrixUnit;
  areaLabel: string;
  locationLabel: string;
  ngPlusCount: number;
  lang: Lang;
}>();

const emit = defineEmits<{ close: [] }>();

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
        class="max-h-[80vh] w-[44rem] max-w-full overflow-y-auto rounded-lg border border-slate-700 bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-800 px-4 py-3">
          <div>
            <h3 class="text-sm font-semibold text-slate-100">{{ unit.name }}</h3>
            <p class="mt-0.5 text-xs text-slate-500">{{ areaLabel }} · {{ locationLabel }}</p>
          </div>
          <button
            type="button"
            class="rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="emit('close')"
          >
            关闭
          </button>
        </header>

        <div class="space-y-4 px-4 py-3">
          <div>
            <h4 class="text-xs font-semibold text-slate-400">获取方式</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-300">
              {{ unit.obtain || "—" }}
            </p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-400">周目变体</h4>
            <div
              v-for="entry in unit.cells.flat()"
              :key="entry.id"
              class="mt-2 rounded border border-slate-800 bg-slate-950/40 px-3 py-2"
            >
              <div class="flex flex-wrap items-baseline gap-2">
                <span class="text-xs text-slate-200">
                  {{ statusEmoji(entry, ngPlusCount) }} {{ matrixName(entry.item, lang) }}
                </span>
                <span class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-400">
                  {{ matrixPeriodLabel(entry.item) }}
                </span>
                <span class="text-[11px] text-slate-600">{{ entry.id }}</span>
                <span
                  v-for="flag in entry.flags"
                  :key="flag"
                  class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-300"
                >
                  {{ flag }}
                </span>
                <span
                  v-if="entry.reason"
                  class="rounded bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-200"
                >
                  {{ entry.reason }}
                </span>
                <span
                  v-if="entry.obtained && entry.item.missable"
                  class="rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-400"
                >
                  可错过
                </span>
              </div>
              <p class="mt-1 whitespace-pre-wrap text-[11px] leading-5 text-slate-400">
                {{ obtainLabel(entry.item, lang) }}
              </p>
            </div>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
