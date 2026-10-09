<script setup lang="ts">
import { computed } from "vue";
import type { Component } from "vue";

/**
 * Small status / meta badge (missing counts, flags, area chips, aliases…).
 * `tone` picks the semantic colour pair; `dot` adds a leading colour dot
 * (area chips), `icon` / the `icon` slot add a leading icon.
 */
const props = withDefaults(
  defineProps<{
    tone?: "neutral" | "brand" | "success" | "warning" | "danger" | "album";
    size?: "xs" | "sm";
    dot?: boolean;
    icon?: Component;
  }>(),
  { tone: "neutral", size: "xs", dot: false },
);

const TONES = {
  neutral: "bg-graphite-100 text-ink-muted dark:bg-white/6",
  brand: "bg-brand-soft text-brand",
  success: "bg-success/15 text-success",
  warning: "bg-warning/15 text-warning",
  danger: "bg-danger/15 text-danger",
  album: "bg-album/15 text-album",
} as const;

const DOTS = {
  neutral: "bg-ink-subtle",
  brand: "bg-brand",
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
  album: "bg-album",
} as const;

const classes = computed(() => [
  "inline-flex items-center",
  props.size === "xs"
    ? "gap-1 rounded px-1.5 py-0.5 text-[11px]"
    : "gap-1.5 rounded-md px-2 py-0.5 text-xs",
  TONES[props.tone],
]);
</script>

<template>
  <span :class="classes">
    <span
      v-if="dot"
      class="h-1.5 w-1.5 shrink-0 rounded-full"
      :class="DOTS[tone]"
      aria-hidden="true"
    ></span>
    <slot name="icon">
      <component :is="icon" v-if="icon" class="h-3 w-3 shrink-0" aria-hidden="true" />
    </slot>
    <slot />
  </span>
</template>
