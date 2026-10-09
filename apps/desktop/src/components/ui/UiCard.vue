<script setup lang="ts">
import { computed, useSlots } from "vue";

/**
 * Surface card: card surface + border + radius (`rounded-card`) + light
 * shadow (dark theme separates layers with borders, so the shadow is off).
 * `header` / default (body) / `footer` slots; body padding via `padding`.
 */
const props = withDefaults(
  defineProps<{
    padding?: "none" | "sm" | "md";
  }>(),
  { padding: "none" },
);

const slots = useSlots();

const bodyClass = computed(() => {
  if (props.padding === "sm") return "px-4 py-2.5";
  if (props.padding === "md") return "px-4 py-3";
  return "";
});
</script>

<template>
  <section class="rounded-card border border-edge bg-surface-card shadow-card dark:shadow-none">
    <header
      v-if="slots.header"
      class="flex items-center justify-between gap-3 border-b border-edge px-4 py-2.5"
    >
      <slot name="header" />
    </header>
    <div :class="bodyClass">
      <slot />
    </div>
    <footer v-if="slots.footer" class="border-t border-edge px-4 py-2.5">
      <slot name="footer" />
    </footer>
  </section>
</template>
