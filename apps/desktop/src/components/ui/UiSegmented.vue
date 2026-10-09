<script setup lang="ts">
import { computed } from "vue";
import type { Component } from "vue";

interface SegmentedOption {
  value: string;
  label?: string;
  icon?: Component;
  count?: number | string;
  title?: string;
  /** Accessible name for icon-only options (falls back to `title`). */
  ariaLabel?: string;
  disabled?: boolean;
}

/**
 * Segmented control (language / theme / view / filter toggles).
 * Supports icon-only options and a trailing count per option.
 * Focus relies on the global `:focus-visible` ring (styles.css).
 */
const props = withDefaults(
  defineProps<{
    modelValue: string;
    options: readonly SegmentedOption[];
    size?: "sm" | "md";
    iconOnly?: boolean;
    ariaLabel?: string;
  }>(),
  { size: "sm", iconOnly: false },
);

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

const containerClass = computed(() => [
  "inline-flex overflow-hidden rounded-md border border-edge bg-surface-card",
  props.size === "sm" ? "text-xs" : "text-sm",
]);

function optionClass(option: SegmentedOption): (string | false | undefined)[] {
  const active = props.modelValue === option.value;
  return [
    "inline-flex items-center justify-center gap-1 whitespace-nowrap transition-colors duration-fast",
    props.iconOnly
      ? props.size === "sm"
        ? "h-7 w-7"
        : "h-8 w-8"
      : props.size === "sm"
        ? "h-7 px-2.5"
        : "h-8 px-3",
    active
      ? "bg-brand-soft font-medium text-brand"
      : "text-ink-muted hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5",
    option.disabled && "cursor-not-allowed opacity-40",
  ];
}
</script>

<template>
  <div :class="containerClass" role="group" :aria-label="ariaLabel">
    <button
      v-for="option in options"
      :key="option.value"
      type="button"
      :class="optionClass(option)"
      :disabled="option.disabled"
      :title="option.title"
      :aria-label="option.ariaLabel ?? option.title"
      :aria-pressed="modelValue === option.value"
      @click="emit('update:modelValue', option.value)"
    >
      <component :is="option.icon" v-if="option.icon" class="h-3.5 w-3.5 shrink-0" />
      <span v-if="!iconOnly && option.label">{{ option.label }}</span>
      <span v-if="option.count !== undefined" class="opacity-70">{{ option.count }}</span>
    </button>
  </div>
</template>
