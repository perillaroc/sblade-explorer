<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { CircleAlert, CircleCheck, X } from "@lucide/vue";
import { dismissToast, type Toast } from "../../lib/toast";

/**
 * Single toast ("success" / "danger"), auto-dismissed after
 * `toast.timeoutMs` and closable by hand. Mount through `UiToastHost`.
 */
const props = defineProps<{ toast: Toast }>();

const { t } = useI18n({ useScope: "global" });

let timer: number | undefined;

onMounted(() => {
  if (props.toast.timeoutMs > 0) {
    timer = window.setTimeout(() => dismissToast(props.toast.id), props.toast.timeoutMs);
  }
});

onBeforeUnmount(() => {
  if (timer !== undefined) window.clearTimeout(timer);
});
</script>

<template>
  <div
    :role="toast.tone === 'danger' ? 'alert' : 'status'"
    class="pointer-events-auto flex w-full items-start gap-2 rounded-card border border-edge bg-surface-raised px-3 py-2.5 shadow-overlay animate-modal-in"
  >
    <CircleCheck
      v-if="toast.tone === 'success'"
      class="mt-0.5 h-4 w-4 shrink-0 text-success"
      aria-hidden="true"
    />
    <CircleAlert v-else class="mt-0.5 h-4 w-4 shrink-0 text-danger" aria-hidden="true" />
    <p class="min-w-0 flex-1 break-words text-xs leading-5 text-ink">{{ toast.message }}</p>
    <button
      type="button"
      class="-m-0.5 shrink-0 rounded p-0.5 text-ink-subtle transition-colors duration-fast hover:text-ink"
      :aria-label="t('common.close')"
      :title="t('common.close')"
      @click="dismissToast(toast.id)"
    >
      <X class="h-3.5 w-3.5" />
    </button>
  </div>
</template>
