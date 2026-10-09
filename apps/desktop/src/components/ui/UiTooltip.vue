<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref, watch } from "vue";

/**
 * Tooltip replacing the native `title` attribute (delayed hover/focus hint).
 * Pure Popper-free implementation: Teleport + fixed positioning so it is not
 * clipped by scroll containers (dialogs, tables); flips vertically when it
 * would leave the viewport. Delay defaults to 300ms.
 */
const props = withDefaults(
  defineProps<{
    content: string;
    placement?: "top" | "bottom";
    delay?: number;
    disabled?: boolean;
  }>(),
  { placement: "top", delay: 300, disabled: false },
);

const visible = ref(false);
const trigger = ref<HTMLElement | null>(null);
const tip = ref<HTMLElement | null>(null);
const position = ref({ left: 0, top: 0 });

let timer: number | undefined;

function clearTimer(): void {
  if (timer !== undefined) {
    window.clearTimeout(timer);
    timer = undefined;
  }
}

function show(): void {
  if (props.disabled || !props.content) return;
  clearTimer();
  timer = window.setTimeout(async () => {
    visible.value = true;
    await nextTick();
    place();
  }, props.delay);
}

function hide(): void {
  clearTimer();
  visible.value = false;
}

function onFocusOut(event: FocusEvent): void {
  const next = event.relatedTarget;
  if (!(next instanceof Node) || !trigger.value?.contains(next)) hide();
}

function place(): void {
  const anchor = trigger.value;
  const element = tip.value;
  if (!anchor || !element) return;
  const gap = 6;
  const rect = anchor.getBoundingClientRect();
  const tipRect = element.getBoundingClientRect();
  let top = props.placement === "top" ? rect.top - tipRect.height - gap : rect.bottom + gap;
  if (props.placement === "top" && top < 4) top = rect.bottom + gap;
  if (props.placement === "bottom" && top + tipRect.height > window.innerHeight - 4) {
    top = rect.top - tipRect.height - gap;
  }
  const halfWidth = tipRect.width / 2;
  const left = Math.min(
    Math.max(rect.left + rect.width / 2, halfWidth + 4),
    window.innerWidth - halfWidth - 4,
  );
  position.value = { left, top };
}

watch(visible, (value) => {
  if (value) {
    window.addEventListener("scroll", hide, true);
    window.addEventListener("resize", hide);
  } else {
    window.removeEventListener("scroll", hide, true);
    window.removeEventListener("resize", hide);
  }
});

onBeforeUnmount(() => {
  clearTimer();
  window.removeEventListener("scroll", hide, true);
  window.removeEventListener("resize", hide);
});
</script>

<template>
  <span
    ref="trigger"
    class="inline-flex"
    @pointerenter="show"
    @pointerleave="hide"
    @focusin="show"
    @focusout="onFocusOut"
    @keydown.esc="hide"
  >
    <slot />
  </span>
  <Teleport to="body">
    <div
      v-if="visible"
      ref="tip"
      role="tooltip"
      class="pointer-events-none fixed z-tooltip w-max max-w-[18rem] -translate-x-1/2 rounded-md border border-edge bg-surface-raised px-2 py-1 text-[11px] leading-4 text-ink shadow-overlay"
      :style="{ left: `${position.left}px`, top: `${position.top}px` }"
    >
      {{ content }}
    </div>
  </Teleport>
</template>
