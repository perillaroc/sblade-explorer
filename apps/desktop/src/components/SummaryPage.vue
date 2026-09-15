<script setup lang="ts">
import type { Analysis } from "../types";
import CategoryTable from "./CategoryTable.vue";
import SummaryPanel from "./SummaryPanel.vue";

defineProps<{ analysis: Analysis }>();

const emit = defineEmits<{ navigate: [key: string] }>();
</script>

<template>
  <div class="space-y-4">
    <SummaryPanel :analysis="analysis" />
    <CategoryTable :analysis="analysis" @navigate="emit('navigate', $event)" />
  </div>
</template>
