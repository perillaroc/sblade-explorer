<script setup lang="ts">
import { ref } from "vue";

interface UiTab {
  value: string;
  label: string;
  count?: string | number;
  disabled?: boolean;
  /** Dim the tab (e.g. a record-type group with no results). */
  muted?: boolean;
}

/**
 * Tab strip (record types and generic tabs). Active tab: brand underline.
 * Keyboard: buttons respond to Enter/Space natively; Arrow/Home/End move the
 * roving tabindex and activate, per the WAI-ARIA tabs pattern.
 */
const props = defineProps<{
  modelValue: string;
  tabs: readonly UiTab[];
  ariaLabel?: string;
}>();

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

const items = ref<(HTMLButtonElement | null)[]>([]);

function enabledIndexes(): number[] {
  return props.tabs
    .map((tab, index) => (tab.disabled ? -1 : index))
    .filter((index) => index >= 0);
}

function activate(index: number): void {
  const tab = props.tabs[index];
  if (tab && !tab.disabled) emit("update:modelValue", tab.value);
}

function focusTab(index: number): void {
  items.value[index]?.focus();
  activate(index);
}

function onKeydown(event: KeyboardEvent, index: number): void {
  const indexes = enabledIndexes();
  const current = indexes.indexOf(index);
  if (current < 0) return;
  let next: number | undefined;
  if (event.key === "ArrowRight") next = indexes[(current + 1) % indexes.length];
  else if (event.key === "ArrowLeft") next = indexes[(current - 1 + indexes.length) % indexes.length];
  else if (event.key === "Home") next = indexes[0];
  else if (event.key === "End") next = indexes[indexes.length - 1];
  if (next !== undefined) {
    event.preventDefault();
    focusTab(next);
  }
}
</script>

<template>
  <nav
    class="flex gap-0.5 overflow-x-auto border-b border-edge bg-surface-sunken px-2"
    role="tablist"
    :aria-label="ariaLabel"
  >
    <button
      v-for="(tab, index) in tabs"
      :key="tab.value"
      ref="items"
      type="button"
      role="tab"
      :aria-selected="modelValue === tab.value"
      :disabled="tab.disabled"
      :tabindex="modelValue === tab.value ? 0 : -1"
      class="relative flex shrink-0 items-baseline gap-1.5 border-b-2 px-3 py-2 text-xs transition-colors duration-fast disabled:cursor-not-allowed disabled:opacity-40"
      :class="[
        modelValue === tab.value
          ? 'border-brand font-medium text-ink'
          : 'border-transparent text-ink-muted hover:bg-graphite-100/70 hover:text-ink dark:hover:bg-white/5',
        tab.muted && 'opacity-40',
      ]"
      @click="activate(index)"
      @keydown="onKeydown($event, index)"
    >
      {{ tab.label }}
      <span
        v-if="tab.count !== undefined"
        class="text-[11px]"
        :class="modelValue === tab.value ? 'text-brand' : 'text-ink-subtle'"
      >
        {{ tab.count }}
      </span>
    </button>
  </nav>
</template>
