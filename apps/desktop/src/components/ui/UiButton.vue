<script setup lang="ts">
import { computed } from "vue";
import { LoaderCircle } from "@lucide/vue";

/**
 * Base button.
 *
 * - variant: primary (brand) / outline / ghost / danger
 * - size: sm (h-7) / md (h-8); `iconOnly` keeps it square
 * - `icon` slot renders before the label; `loading` swaps it for a spinner
 * - Focus relies on the global `:focus-visible` ring (styles.css).
 */
const props = withDefaults(
  defineProps<{
    variant?: "primary" | "outline" | "ghost" | "danger";
    size?: "sm" | "md";
    type?: "button" | "submit" | "reset";
    disabled?: boolean;
    loading?: boolean;
    iconOnly?: boolean;
    block?: boolean;
  }>(),
  {
    variant: "outline",
    size: "sm",
    type: "button",
    disabled: false,
    loading: false,
    iconOnly: false,
    block: false,
  },
);

const VARIANTS = {
  primary:
    "bg-brand-700 text-white hover:bg-brand-800 dark:bg-brand-400 dark:text-graphite-950 dark:hover:bg-brand-300",
  outline:
    "border border-edge bg-surface-card text-ink hover:bg-graphite-100 dark:hover:bg-white/5",
  ghost: "text-ink-muted hover:bg-graphite-100 hover:text-ink dark:hover:bg-white/5",
  danger: "bg-rose-600 text-white hover:bg-rose-700",
} as const;

const classes = computed(() => [
  "inline-flex shrink-0 select-none items-center justify-center gap-1.5 rounded-md font-medium transition-colors duration-fast disabled:cursor-not-allowed disabled:opacity-50",
  props.iconOnly
    ? props.size === "sm"
      ? "h-7 w-7"
      : "h-8 w-8"
    : props.size === "sm"
      ? "h-7 px-2.5 text-xs"
      : "h-8 px-3 text-sm",
  VARIANTS[props.variant],
  props.block && "w-full",
]);
</script>

<template>
  <button :type="type" :class="classes" :disabled="disabled || loading">
    <LoaderCircle v-if="loading" class="h-3.5 w-3.5 animate-spin" aria-hidden="true" />
    <slot v-else name="icon" />
    <slot />
  </button>
</template>
