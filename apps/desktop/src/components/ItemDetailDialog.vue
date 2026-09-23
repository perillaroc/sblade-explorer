<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { CircleCheck, CircleX, X } from "@lucide/vue";
import type { Lang } from "../types";
import { areaAccent } from "../lib/area";
import { areaLabel, itemName, locationLabel, obtainLabel } from "../lib/display";
import type { ItemRow } from "../lib/items";

const props = defineProps<{
  row: ItemRow;
  lang: Lang;
}>();

const emit = defineEmits<{ close: [] }>();

const NG_PLUS_LABELS = ["首周目", "二周目(NG+)", "三周目(NG++)"];

const DLC_LABELS: Record<string, string> = {
  nier: "尼尔 DLC",
  nikke: "NIKKE DLC",
  deluxe: "豪华版",
  preorder: "预购特典",
  summer: "夏日更新",
};

const item = computed(() => props.row.item);
const name = computed(() => itemName(item.value, props.lang));
const area = computed(() => areaLabel(item.value, props.lang));
const location = computed(() => locationLabel(item.value, props.lang));
const obtain = computed(() => obtainLabel(item.value, props.lang));
const ngPlus = computed(() => NG_PLUS_LABELS[item.value.ng_plus] ?? `NG+${item.value.ng_plus}`);
const dlc = computed(() =>
  item.value.dlc ? (DLC_LABELS[item.value.dlc] ?? item.value.dlc) : null,
);
const recordType = computed(() => item.value.record_type_zh ?? item.value.record_type ?? "");
const confidence = computed(() =>
  item.value.confidence === "high" ? "高" : "低（映射待确认）",
);
const aliases = computed(() =>
  (item.value.aliases.length > 0 ? item.value.aliases : [item.value.id]).filter(
    (alias) => alias !== item.value.id,
  ),
);
const source = computed(() => {
  const raw = props.row.source;
  if (!raw) return "";
  if (raw === "game") return "游戏数据（官方简中名称）";
  const host = /^https?:\/\/([^/]+)/.exec(raw);
  return host ? `攻略站数据（${host[1]}）` : raw;
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
        class="max-h-[80vh] w-[38rem] max-w-full overflow-y-auto rounded-lg border border-slate-700 bg-slate-900 shadow-xl"
      >
        <header class="flex items-start justify-between gap-3 border-b border-slate-800 px-4 py-3">
          <div class="min-w-0">
            <h3 class="text-sm font-semibold text-slate-100">{{ name }}</h3>
            <p class="mt-0.5 break-all font-mono text-[11px] text-slate-500">{{ row.id }}</p>
            <div class="mt-1.5 flex flex-wrap items-center gap-2">
              <span
                class="inline-flex items-center gap-0.5 rounded px-1.5 py-0.5 text-[11px]"
                :class="
                  row.obtained
                    ? 'bg-emerald-900/40 text-emerald-300'
                    : 'bg-rose-900/40 text-rose-300'
                "
              >
                <CircleCheck v-if="row.obtained" class="h-3 w-3" />
                <CircleX v-else class="h-3 w-3" />
                {{ row.obtained ? "已收集" : "未收集" }}
              </span>
              <span
                class="inline-flex items-center gap-1.5 rounded-md px-2 py-0.5 text-xs font-semibold"
                :class="[areaAccent(item.area).band, areaAccent(item.area).text]"
              >
                <span
                  class="h-2 w-2 shrink-0 rounded-full"
                  :class="areaAccent(item.area).dot"
                ></span>
                {{ area || "未分类" }}
              </span>
              <span v-if="location" class="text-sm font-medium text-slate-200">
                {{ location }}
              </span>
            </div>
          </div>
          <button
            type="button"
            class="inline-flex shrink-0 items-center gap-1 rounded border border-slate-700 px-2 py-1 text-xs text-slate-300 hover:bg-slate-800"
            @click="emit('close')"
          >
            <X class="h-3.5 w-3.5" />
            关闭
          </button>
        </header>

        <div class="space-y-4 px-4 py-3">
          <div>
            <h4 class="text-xs font-semibold text-slate-400">获取方式</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-300">
              {{ obtain || "—" }}
            </p>
          </div>

          <div v-if="row.reason">
            <h4 class="text-xs font-semibold text-slate-400">未收集原因</h4>
            <p class="mt-1 text-xs leading-5 text-amber-200">{{ row.reason }}</p>
          </div>

          <div>
            <h4 class="text-xs font-semibold text-slate-400">详情</h4>
            <dl class="mt-1.5 grid grid-cols-[5.5rem_1fr] gap-x-3 gap-y-1.5 text-xs">
              <dt class="text-slate-500">周目</dt>
              <dd class="text-slate-300">{{ ngPlus }}</dd>
              <template v-if="dlc">
                <dt class="text-slate-500">DLC</dt>
                <dd class="text-slate-300">{{ dlc }}</dd>
              </template>
              <template v-if="item.missable">
                <dt class="text-slate-500">可错过</dt>
                <dd class="text-amber-200">是</dd>
              </template>
              <template v-if="recordType">
                <dt class="text-slate-500">记录类型</dt>
                <dd class="text-slate-300">{{ recordType }}</dd>
              </template>
              <dt class="text-slate-500">映射置信度</dt>
              <dd :class="item.confidence === 'high' ? 'text-slate-300' : 'text-amber-200'">
                {{ confidence }}
              </dd>
              <template v-if="source">
                <dt class="text-slate-500">数据来源</dt>
                <dd class="text-slate-300">{{ source }}</dd>
              </template>
            </dl>
          </div>

          <div v-if="aliases.length > 0">
            <h4 class="text-xs font-semibold text-slate-400">别名</h4>
            <div class="mt-1.5 flex flex-wrap gap-1">
              <span
                v-for="alias in aliases"
                :key="alias"
                class="break-all rounded bg-slate-800 px-1.5 py-0.5 font-mono text-[11px] text-slate-400"
              >
                {{ alias }}
              </span>
            </div>
          </div>

          <div v-if="item.note">
            <h4 class="text-xs font-semibold text-slate-400">备注</h4>
            <p class="mt-1 whitespace-pre-wrap text-xs leading-5 text-slate-400">
              {{ item.note }}
            </p>
          </div>
        </div>
      </section>
    </div>
  </Teleport>
</template>
