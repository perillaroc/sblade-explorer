<script setup lang="ts">
import { computed } from "vue";
import type { ItemRow } from "../lib/items";
import { matrixStatus } from "../lib/status";

const props = defineProps<{
  row: ItemRow;
  ngPlusCount: number;
}>();

const status = computed(() => matrixStatus(props.row, props.ngPlusCount));
</script>

<template>
  <component
    :is="status.icon"
    class="h-4 w-4 shrink-0"
    :class="status.className"
    :aria-label="status.label"
  >
    <title>{{ status.label }}</title>
  </component>
</template>
