<script setup lang="ts">
import type { Lang } from "../types";
import { itemName, locationLine, obtainLabel } from "../lib/display";
import type { ItemRow } from "../lib/items";

defineProps<{
  rows: ItemRow[];
  lang: Lang;
}>();
</script>

<template>
  <div class="overflow-x-auto">
    <table class="w-full text-xs">
      <thead class="sticky top-0 bg-slate-900">
        <tr class="text-left text-slate-500">
          <th class="px-4 py-2">状态</th>
          <th class="px-4 py-2">物品</th>
          <th class="px-4 py-2">位置</th>
          <th class="px-4 py-2">获取</th>
          <th class="px-4 py-2">标记</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="row in rows"
          :key="row.id"
          class="border-t border-slate-800/70 align-top"
          :class="row.obtained ? '' : 'bg-slate-950/30'"
        >
          <td class="px-4 py-2">
            <span
              class="inline-block rounded px-1.5 py-0.5 text-[11px]"
              :class="
                row.obtained
                  ? 'bg-emerald-900/40 text-emerald-300'
                  : 'bg-rose-900/40 text-rose-300'
              "
            >
              {{ row.obtained ? "已收集" : "未收集" }}
            </span>
          </td>
          <td class="px-4 py-2">
            <div class="text-slate-100">{{ itemName(row.item, lang) }}</div>
            <div class="text-[11px] text-slate-500">{{ row.id }}</div>
          </td>
          <td class="px-4 py-2 text-slate-300">{{ locationLine(row.item, lang) || "—" }}</td>
          <td class="px-4 py-2 text-slate-400">{{ obtainLabel(row.item, lang) || "—" }}</td>
          <td class="px-4 py-2">
            <span
              v-for="flag in row.flags"
              :key="flag"
              class="mr-1 inline-block rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-amber-300"
            >
              {{ flag }}
            </span>
            <span
              v-if="row.reason"
              class="mr-1 inline-block rounded bg-amber-900/40 px-1.5 py-0.5 text-[11px] text-amber-200"
            >
              {{ row.reason }}
            </span>
            <span
              v-if="row.obtained && row.item.missable"
              class="mr-1 inline-block rounded bg-slate-800 px-1.5 py-0.5 text-[11px] text-slate-400"
            >
              可错过
            </span>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
