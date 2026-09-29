<script setup lang="ts">
import { FileSearch, FolderOpen, FolderX, RefreshCw, TriangleAlert } from "@lucide/vue";
import type { SaveSource } from "../types";

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
</script>

<template>
  <div class="mx-auto flex w-full max-w-3xl flex-col gap-3">
    <section
      class="rounded-lg border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-900/60 p-4"
    >
      <div class="flex items-start gap-2">
        <TriangleAlert
          v-if="error"
          class="mt-0.5 h-4 w-4 shrink-0 text-rose-600 dark:text-rose-400"
        />
        <FolderX v-else class="mt-0.5 h-4 w-4 shrink-0 text-amber-500 dark:text-amber-400" />
        <div class="min-w-0">
          <h3 class="text-sm font-semibold">{{ error ? "读取存档失败" : "未找到存档" }}</h3>
          <p
            v-if="error"
            class="mt-1 break-all font-mono text-xs text-rose-600 dark:text-rose-400"
          >
            {{ error }}
          </p>
          <p v-else class="mt-1 text-xs leading-5 text-slate-600 dark:text-slate-400">
            程序会递归扫描下列目录中的 <code>StellarBladeSave*.sav</code>，名为
            <code>Backup</code> 的目录会被忽略。
          </p>
          <p v-if="activePath" class="mt-1 break-all font-mono text-[11px] text-slate-500">
            当前文件：{{ activePath }}
          </p>
        </div>
      </div>

      <ul class="mt-3 space-y-1.5">
        <li
          v-for="source in sources"
          :key="source.path"
          class="flex items-center justify-between gap-3 rounded border px-3 py-2"
          :class="
            source.exists
              ? 'border-slate-200 dark:border-slate-800'
              : 'border-dashed border-slate-300 dark:border-slate-700'
          "
        >
          <div class="min-w-0">
            <div
              class="flex items-center gap-1.5 text-xs"
              :class="source.exists ? 'text-slate-700 dark:text-slate-300' : 'text-slate-500'"
            >
              <FolderOpen v-if="source.exists" class="h-3.5 w-3.5 shrink-0" />
              <FolderX v-else class="h-3.5 w-3.5 shrink-0" />
              {{ source.exists ? "目录存在" : "目录不存在" }}
            </div>
            <div class="mt-0.5 break-all font-mono text-[11px] text-slate-500">
              {{ source.path }}
            </div>
          </div>
          <button
            v-if="source.exists"
            type="button"
            class="inline-flex shrink-0 items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800"
            @click="emit('open', source.path)"
          >
            <FolderOpen class="h-3.5 w-3.5" />
            打开
          </button>
        </li>
        <li v-if="sources.length === 0" class="px-3 py-2 text-xs text-slate-500">
          正在读取扫描目录…
        </li>
      </ul>
    </section>

    <div class="flex flex-wrap items-center gap-2">
      <button
        type="button"
        class="inline-flex items-center gap-1 rounded border border-emerald-700 px-3 py-1 text-xs text-emerald-700 dark:text-emerald-200 hover:bg-emerald-50 dark:hover:bg-emerald-900/30"
        @click="emit('pick')"
      >
        <FileSearch class="h-3.5 w-3.5" />
        选择存档文件…
      </button>
      <button
        type="button"
        class="inline-flex items-center gap-1 rounded border border-slate-300 dark:border-slate-700 px-2 py-1 text-xs text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="scanning"
        @click="emit('refresh')"
      >
        <RefreshCw class="h-3.5 w-3.5" :class="scanning ? 'animate-spin' : ''" />
        重新扫描
      </button>
      <span class="text-[11px] leading-5 text-slate-500">
        存档不在默认目录时，可直接选择文件（只读打开）。
      </span>
    </div>
  </div>
</template>
