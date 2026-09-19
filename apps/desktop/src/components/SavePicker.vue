<script setup lang="ts">
import { Clock, HardDrive, RefreshCw } from "@lucide/vue";
import type { SaveSlot } from "../types";

const props = defineProps<{
  saves: SaveSlot[];
  selected: SaveSlot | null;
}>();

const emit = defineEmits<{
  select: [slot: SaveSlot];
  refresh: [];
}>();

function onChange(event: Event) {
  const path = (event.target as HTMLSelectElement).value;
  const slot = props.saves.find((candidate) => candidate.path === path);
  if (slot) {
    emit("select", slot);
  }
}

function formatTime(mtimeMs: number): string {
  return new Date(mtimeMs).toLocaleString("zh-CN", { hour12: false });
}

function formatSize(size: number): string {
  return `${(size / 1024 / 1024).toFixed(1)} MB`;
}
</script>

<template>
  <section class="flex flex-wrap items-center gap-3 border-b border-slate-800 bg-slate-900/60 px-4 py-3">
    <label class="text-xs text-slate-400" for="save-select">存档</label>
    <select
      id="save-select"
      class="min-w-80 rounded border border-slate-700 bg-slate-950 px-2 py-1 text-sm text-slate-100"
      :value="selected?.path ?? ''"
      @change="onChange"
    >
      <option v-if="saves.length === 0" value="" disabled>未找到存档</option>
      <option v-for="slot in saves" :key="slot.path" :value="slot.path">
        {{ slot.label }}
      </option>
    </select>
    <button
      type="button"
      class="inline-flex items-center gap-1 rounded border border-slate-700 px-2 py-1 text-sm text-slate-300 hover:bg-slate-800"
      @click="emit('refresh')"
    >
      <RefreshCw class="h-3.5 w-3.5" />
      刷新
    </button>
    <span
      v-if="selected"
      class="inline-flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-slate-500"
    >
      <span class="inline-flex items-center gap-1">
        <HardDrive class="h-3.5 w-3.5" />
        {{ formatSize(selected.size) }}
      </span>
      <span class="inline-flex items-center gap-1">
        <Clock class="h-3.5 w-3.5" />
        {{ formatTime(selected.mtimeMs) }}
      </span>
      <span>{{ selected.path }}</span>
    </span>
  </section>
</template>
