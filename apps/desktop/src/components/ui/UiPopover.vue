<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";

/**
 * Anchored popover (save card menu, export menu, future dropdowns).
 *
 * No dependencies: absolute positioning inside a relative root; open state via
 * `v-model:open`. Closes on outside pointerdown or Esc (Esc returns focus to
 * the first focusable element in the trigger area, so it works with menu
 * buttons). The panel uses `z-overlay`, staying below Toast (z-toast) and
 * above page content / sticky toolbars.
 *
 * Placements: bottom-start / bottom-end / right-start (collapsed rail flyout).
 */
const props = withDefaults(
  defineProps<{
    open: boolean;
    placement?: "bottom-start" | "bottom-end" | "right-start";
    panelClass?: string;
    panelRole?: string;
  }>(),
  { placement: "bottom-start", panelRole: "dialog" },
);

const emit = defineEmits<{ "update:open": [value: boolean] }>();

const root = ref<HTMLElement | null>(null);

const PANEL_CLASS =
  "absolute z-overlay min-w-40 rounded-card border border-edge bg-surface-raised p-1.5 shadow-overlay animate-modal-in";

const POSITIONS = {
  "bottom-start": "left-0 top-full mt-1",
  "bottom-end": "right-0 top-full mt-1",
  "right-start": "left-full top-0 ml-1",
} as const;

const panelPosition = computed(() => POSITIONS[props.placement]);

function onPointerDown(event: PointerEvent): void {
  const target = event.target as Node | null;
  if (target && root.value && !root.value.contains(target)) {
    emit("update:open", false);
  }
}

function onKeydown(event: KeyboardEvent): void {
  if (event.key !== "Escape") return;
  // Swallow Esc so stacked overlays (details drawer, settings modal) stay open.
  event.stopPropagation();
  emit("update:open", false);
  root.value?.querySelector<HTMLElement>("button, [href], input, select, textarea")?.focus();
}

function listen(enabled: boolean): void {
  if (enabled) {
    document.addEventListener("pointerdown", onPointerDown, true);
    window.addEventListener("keydown", onKeydown, true);
  } else {
    document.removeEventListener("pointerdown", onPointerDown, true);
    window.removeEventListener("keydown", onKeydown, true);
  }
}

watch(() => props.open, listen);

onBeforeUnmount(() => listen(false));
</script>

<template>
  <div ref="root" class="relative">
    <slot name="trigger" />
    <div v-if="open" :role="panelRole" :class="[PANEL_CLASS, panelPosition, panelClass]">
      <slot />
    </div>
  </div>
</template>
