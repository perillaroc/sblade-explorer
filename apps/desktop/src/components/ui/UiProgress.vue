<script setup lang="ts">
import { computed } from "vue";

/**
 * Progress bar. The fill uses `transform: scaleX()` (CSP-safe, GPU-friendly)
 * with `transform-origin: left`; the transition is disabled when the user
 * prefers reduced motion (also covered by the global downgrade in styles.css).
 */
const props = withDefaults(
  defineProps<{
    value: number;
    tone?: "brand" | "success" | "album" | "danger";
    size?: "sm" | "md";
    glow?: boolean;
    ariaLabel?: string;
  }>(),
  { tone: "brand", size: "sm", glow: false },
);

const TONES = {
  brand: "bg-brand-500 dark:bg-brand-400",
  success: "bg-emerald-500",
  album: "bg-sky-500",
  danger: "bg-rose-500",
} as const;

const percent = computed(() => Math.min(100, Math.max(0, props.value)));

const trackClass = computed(() => [
  "overflow-hidden rounded-full bg-graphite-200/80 dark:bg-white/8",
  props.size === "sm" ? "h-1" : "h-2",
]);
</script>

<template>
  <div
    :class="trackClass"
    role="progressbar"
    :aria-valuenow="Math.round(percent)"
    aria-valuemin="0"
    aria-valuemax="100"
    :aria-label="ariaLabel"
  >
    <div
      class="h-full w-full origin-left rounded-full transition-transform duration-normal ease-out-soft motion-reduce:transition-none"
      :class="[TONES[tone], glow && 'shadow-glow-brand']"
      :style="{ transform: `scaleX(${percent / 100})` }"
    ></div>
  </div>
</template>
