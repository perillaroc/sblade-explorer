<script setup lang="ts">
import { toasts } from "../../lib/toast";
import UiToast from "./UiToast.vue";

/**
 * Toast container: top-right, above dialogs (z-toast), pointer-events only
 * on the single toasts so the page underneath stays usable.
 * Render once per app skeleton (F2/F4 wire it into `App.vue`).
 */
</script>

<template>
  <Teleport to="body">
    <div
      class="pointer-events-none fixed right-4 top-4 z-toast flex w-80 max-w-[calc(100vw-2rem)] flex-col gap-2"
      aria-live="polite"
      aria-atomic="false"
    >
      <TransitionGroup name="ui-toast">
        <UiToast v-for="toast in toasts" :key="toast.id" :toast="toast" />
      </TransitionGroup>
    </div>
  </Teleport>
</template>
