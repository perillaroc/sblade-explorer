import { computed, ref } from "vue";

/** Tone of a toast; success is informational, danger announces an error. */
export type ToastTone = "success" | "danger";

export interface Toast {
  id: number;
  message: string;
  tone: ToastTone;
  /** Auto-dismiss delay in ms; `0` keeps the toast until dismissed. */
  timeoutMs: number;
}

const DEFAULT_TIMEOUT_MS = 4000;

const items = ref<Toast[]>([]);
let nextId = 1;

/** Read-only view consumed by `UiToastHost`. */
export const toasts = computed(() => items.value);

/**
 * Queue a toast. The host (`UiToastHost`) renders and auto-dismisses them,
 * so this helper stays usable from any module (F4 replaces the inline
 * notices in `App.vue` with this).
 */
export function showToast(
  message: string,
  tone: ToastTone = "success",
  timeoutMs: number = DEFAULT_TIMEOUT_MS,
): number {
  const id = nextId;
  nextId += 1;
  items.value = [...items.value, { id, message, tone, timeoutMs }];
  return id;
}

export function dismissToast(id: number): void {
  items.value = items.value.filter((toast) => toast.id !== id);
}

export function clearToasts(): void {
  items.value = [];
}
