<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { ChevronRight, Search, SearchX } from "@lucide/vue";
import type { Analysis } from "../types";
import { categoryName, itemName, locationLine } from "../lib/display";
import { categoryIcon } from "../lib/icons";
import { searchCatalog, type SearchGroup } from "../lib/search";
import { settings } from "../lib/settings";
import { UiInput, UiPopover } from "./ui";

/**
 * Sidebar global search (F0 decision ② / F4): cross-category hits grouped by
 * category, arrow-key navigation, Ctrl+K or `/` to focus, select jumps to the
 * category page and opens the item's detail drawer.
 */
const props = defineProps<{ analysis: Analysis | null; collapsed: boolean }>();

const emit = defineEmits<{
  select: [hit: { categoryKey: string; itemId: string }];
  expand: [];
}>();

const { t } = useI18n({ useScope: "global" });

const query = ref("");
const open = ref(false);
const activeIndex = ref(0);
const input = ref<InstanceType<typeof UiInput> | null>(null);

let pendingFocus = false;

const groups = computed<SearchGroup[]>(() =>
  props.analysis ? searchCatalog(props.analysis, query.value) : [],
);

const hits = computed(() =>
  groups.value.flatMap((group) => group.rows.map((row) => ({ group, row }))),
);

const total = computed(() => groups.value.reduce((sum, group) => sum + group.rows.length, 0));

watch(query, (value) => {
  activeIndex.value = 0;
  open.value = Boolean(value.trim()) && !props.collapsed && Boolean(props.analysis);
});

watch(
  () => props.collapsed,
  (value) => {
    if (!value && pendingFocus) {
      pendingFocus = false;
      void nextTick(() => input.value?.select());
    }
  },
);

function requestFocus(): void {
  if (!props.analysis) return;
  if (props.collapsed) {
    pendingFocus = true;
    emit("expand");
    return;
  }
  input.value?.select();
}

function isEditable(target: EventTarget | null): boolean {
  const element = target as HTMLElement | null;
  if (!element) return false;
  return (
    element.tagName === "INPUT" ||
    element.tagName === "TEXTAREA" ||
    element.tagName === "SELECT" ||
    element.isContentEditable
  );
}

function onShortcut(event: KeyboardEvent): void {
  // Do not hijack the shortcut while a modal/drawer is open.
  if (document.querySelector("[role=dialog]")) return;
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k") {
    event.preventDefault();
    requestFocus();
    return;
  }
  if (event.key === "/" && !isEditable(event.target)) {
    event.preventDefault();
    requestFocus();
  }
}

function flatIndex(groupIndex: number, rowIndex: number): number {
  let index = rowIndex;
  for (let i = 0; i < groupIndex; i += 1) index += groups.value[i].rows.length;
  return index;
}

function choose(group: SearchGroup, itemId: string): void {
  open.value = false;
  query.value = "";
  emit("select", { categoryKey: group.key, itemId });
}

function onKeydown(event: KeyboardEvent): void {
  // Never intercept keys while an IME composition is in progress (Chinese input).
  if (event.isComposing) return;
  if (event.key === "Escape") {
    query.value = "";
    activeIndex.value = 0;
    return;
  }
  const list = hits.value;
  if (!open.value || list.length === 0) return;
  if (event.key === "ArrowDown") {
    event.preventDefault();
    activeIndex.value = (activeIndex.value + 1) % list.length;
  } else if (event.key === "ArrowUp") {
    event.preventDefault();
    activeIndex.value = (activeIndex.value - 1 + list.length) % list.length;
  } else if (event.key === "Enter") {
    event.preventDefault();
    const hit = list[activeIndex.value];
    if (hit) choose(hit.group, hit.row.id);
  }
}

onMounted(() => window.addEventListener("keydown", onShortcut));
onBeforeUnmount(() => window.removeEventListener("keydown", onShortcut));
</script>

<template>
  <UiPopover
    v-model:open="open"
    placement="bottom-start"
    panel-class="w-80 max-h-[60vh] overflow-y-auto"
    panel-role="listbox"
  >
    <template #trigger>
      <button
        v-if="collapsed"
        type="button"
        class="mx-auto flex h-8 w-8 items-center justify-center rounded-md text-ink-muted transition-colors duration-fast hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5"
        :title="t('search.hint')"
        :aria-label="t('search.placeholder')"
        @click="requestFocus"
      >
        <Search class="h-4 w-4" />
      </button>
      <div v-else class="relative">
        <UiInput
          ref="input"
          v-model="query"
          type="search"
          :disabled="!analysis"
          :placeholder="t('search.placeholder')"
          class="w-full [&_input]:pr-14"
          @keydown="onKeydown"
          @focusin="open = Boolean(query.trim()) && Boolean(analysis)"
        >
          <template #icon>
            <Search class="h-3.5 w-3.5" />
          </template>
        </UiInput>
        <span
          v-show="!query"
          class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 text-[10px] text-ink-subtle"
        >
          {{ t("search.shortcut") }}
        </span>
      </div>
    </template>

    <div v-if="query.trim() && analysis">
      <template v-if="hits.length > 0">
        <template v-for="(group, groupIndex) in groups" :key="group.key">
          <div class="flex items-center gap-1.5 px-2 pb-1 pt-2 text-[11px] font-medium text-ink-subtle">
            <component :is="categoryIcon(group.category.key)" class="h-3.5 w-3.5" aria-hidden="true" />
            {{ categoryName(group.category, settings.contentLang) }}
            <span class="ml-auto tabular-nums">{{ group.rows.length }}</span>
          </div>
          <button
            v-for="(row, rowIndex) in group.rows"
            :key="row.id"
            type="button"
            role="option"
            :aria-selected="activeIndex === flatIndex(groupIndex, rowIndex)"
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left transition-colors duration-fast"
            :class="
              activeIndex === flatIndex(groupIndex, rowIndex)
                ? 'bg-brand-soft'
                : 'hover:bg-graphite-100 dark:hover:bg-white/5'
            "
            @mousemove="activeIndex = flatIndex(groupIndex, rowIndex)"
            @click="choose(group, row.id)"
          >
            <span class="min-w-0 flex-1">
              <span class="block truncate text-xs text-ink">{{ itemName(row.item, settings.contentLang) }}</span>
              <span class="mt-0.5 block truncate text-[11px] text-ink-subtle">
                {{ locationLine(row.item, settings.contentLang) || row.id }}
              </span>
            </span>
            <ChevronRight class="h-3.5 w-3.5 shrink-0 text-ink-subtle" aria-hidden="true" />
          </button>
        </template>
        <div class="px-2 py-1.5 text-[11px] tabular-nums text-ink-subtle">
          {{ t("search.hits", { count: total }) }}
        </div>
      </template>
      <div v-else class="flex items-center justify-center gap-2 px-2 py-6 text-xs text-ink-subtle">
        <SearchX class="h-4 w-4" aria-hidden="true" />
        {{ t("search.empty") }}
      </div>
    </div>
  </UiPopover>
</template>
