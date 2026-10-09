<script setup lang="ts">
import type { Component } from "vue";

/**
 * Empty state: icon + optional title / description + an `action` slot
 * (buttons) and a default slot for fully custom content.
 */
defineProps<{
  icon?: Component;
  title?: string;
  description?: string;
}>();
</script>

<template>
  <div class="flex flex-col items-center justify-center gap-2 px-4 py-10 text-center">
    <component :is="icon" v-if="icon" class="h-8 w-8 text-ink-subtle" aria-hidden="true" />
    <p v-if="title" class="text-sm font-medium text-ink">{{ title }}</p>
    <p v-if="description" class="max-w-md text-xs leading-5 text-ink-muted">{{ description }}</p>
    <div v-if="$slots.action" class="mt-1 flex flex-wrap items-center justify-center gap-2">
      <slot name="action" />
    </div>
    <slot />
  </div>
</template>
