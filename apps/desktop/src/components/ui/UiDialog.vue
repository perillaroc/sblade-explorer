<script lang="ts">
/**
 * BaseDialog — shared chrome for modals and drawers.
 *
 * z-index scale (styles.css utilities):
 *   z-page 0 < z-sticky 10 < z-overlay 80 < z-tooltip 85 < z-toast 90
 *
 * Behaviour: Teleport to body, backdrop click / Esc close, focus trap,
 * initial focus, body scroll lock, focus restore. Every overlay must use
 * this component so the scale and keyboard behaviour stay consistent.
 *
 * Esc and Tab only act on the topmost dialog, so stacked overlays (e.g. a
 * settings modal opened from the detail drawer) behave predictably.
 */

// Module-scope state shared by every instance.
const dialogStack: symbol[] = [];

let scrollLocks = 0;
let previousOverflow = "";

function lockBodyScroll(): void {
  if (scrollLocks === 0) {
    previousOverflow = document.body.style.overflow;
    document.body.style.overflow = "hidden";
  }
  scrollLocks += 1;
}

function unlockBodyScroll(): void {
  scrollLocks = Math.max(0, scrollLocks - 1);
  if (scrollLocks === 0) {
    document.body.style.overflow = previousOverflow;
  }
}
</script>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, useId } from "vue";
import { useI18n } from "vue-i18n";
import { X, type LucideIcon } from "@lucide/vue";
import UiButton from "./UiButton.vue";

const props = withDefaults(
  defineProps<{
    /** `modal` centers a panel; `drawer` slides in from the right. */
    variant?: "modal" | "drawer";
    /** Width tier: modal max-width / drawer width. */
    size?: "sm" | "md" | "lg" | "xl";
    title?: string;
    description?: string;
    icon?: LucideIcon;
    /** Used as `aria-label` when no `title` is given. */
    ariaLabel?: string;
    closable?: boolean;
    closeOnBackdrop?: boolean;
    closeOnEsc?: boolean;
    /** Apply the default body padding (`px-4 py-3`). */
    bodyPadding?: boolean;
  }>(),
  {
    variant: "modal",
    size: "md",
    closable: true,
    closeOnBackdrop: true,
    closeOnEsc: true,
    bodyPadding: true,
  },
);

const emit = defineEmits<{ close: [] }>();

const { t } = useI18n({ useScope: "global" });

const MODAL_SIZES = {
  sm: "max-w-md",
  md: "max-w-[36rem]",
  lg: "max-w-[42rem]",
  xl: "max-w-[48rem]",
} as const;

const DRAWER_SIZES = {
  sm: "w-[380px]",
  md: "w-[420px]",
  lg: "w-[520px]",
  xl: "w-[640px]",
} as const;

const rootClass = computed(() =>
  props.variant === "drawer"
    ? "fixed inset-0 z-overlay flex justify-end"
    : "fixed inset-0 z-overlay flex items-center justify-center p-4",
);

const panelClass = computed(() => [
  "relative flex w-full flex-col border-edge bg-surface-raised shadow-overlay focus:outline-none",
  props.variant === "drawer"
    ? [
        "h-full border-l animate-drawer-in max-[1099px]:w-full",
        DRAWER_SIZES[props.size],
      ]
    : ["max-h-[85vh] rounded-xl border animate-modal-in", MODAL_SIZES[props.size]],
]);

const titleId = useId();
const descriptionId = useId();

const panel = ref<HTMLElement | null>(null);
const dialogId = Symbol("ui-dialog");

let previouslyFocused: HTMLElement | null = null;

function isTop(): boolean {
  return dialogStack[dialogStack.length - 1] === dialogId;
}

const FOCUSABLE =
  'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

function trapTab(event: KeyboardEvent): void {
  const root = panel.value;
  if (!root) return;
  const items = Array.from(root.querySelectorAll<HTMLElement>(FOCUSABLE)).filter(
    (element) => element.getClientRects().length > 0,
  );
  if (items.length === 0) {
    event.preventDefault();
    root.focus();
    return;
  }
  const first = items[0];
  const last = items[items.length - 1];
  const active = document.activeElement as HTMLElement | null;
  if (!active || !root.contains(active)) {
    event.preventDefault();
    (event.shiftKey ? last : first).focus();
    return;
  }
  if (event.shiftKey && active === first) {
    event.preventDefault();
    last.focus();
  } else if (!event.shiftKey && active === last) {
    event.preventDefault();
    first.focus();
  }
}

function requestClose(): void {
  if (props.closable) emit("close");
}

function onBackdrop(): void {
  if (props.closeOnBackdrop) requestClose();
}

function onKeydown(event: KeyboardEvent): void {
  if (!isTop()) return;
  if (event.key === "Escape") {
    // Swallow Esc so lower overlays never close behind this one.
    event.stopPropagation();
    if (props.closeOnEsc) {
      event.preventDefault();
      requestClose();
    }
    return;
  }
  if (event.key === "Tab") {
    trapTab(event);
  }
}

onMounted(() => {
  dialogStack.push(dialogId);
  lockBodyScroll();
  previouslyFocused =
    document.activeElement instanceof HTMLElement ? document.activeElement : null;
  window.addEventListener("keydown", onKeydown, true);
  void nextTick(() => panel.value?.focus());
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", onKeydown, true);
  const index = dialogStack.indexOf(dialogId);
  if (index >= 0) dialogStack.splice(index, 1);
  unlockBodyScroll();
  if (previouslyFocused?.isConnected) previouslyFocused.focus();
});
</script>

<template>
  <Teleport to="body">
    <div :class="rootClass">
      <div
        class="absolute inset-0 animate-overlay-in bg-graphite-950/40 dark:bg-black/60"
        aria-hidden="true"
        @click="onBackdrop"
      ></div>
      <section
        ref="panel"
        :class="panelClass"
        role="dialog"
        aria-modal="true"
        :aria-labelledby="title ? titleId : undefined"
        :aria-label="title ? undefined : ariaLabel"
        :aria-describedby="description ? descriptionId : undefined"
        tabindex="-1"
      >
        <slot name="header">
          <header
            class="flex items-start justify-between gap-3 border-b border-edge px-4 py-3"
          >
            <div class="flex min-w-0 items-start gap-2">
              <component
                :is="icon"
                v-if="icon"
                class="mt-0.5 h-4 w-4 shrink-0 text-brand"
                aria-hidden="true"
              />
              <div class="min-w-0">
                <h2 :id="titleId" class="text-sm font-semibold text-ink">{{ title }}</h2>
                <p v-if="description" :id="descriptionId" class="mt-0.5 text-xs text-ink-muted">
                  {{ description }}
                </p>
              </div>
            </div>
            <UiButton
              v-if="closable"
              variant="ghost"
              icon-only
              :aria-label="t('common.close')"
              :title="t('common.close')"
              @click="requestClose"
            >
              <template #icon>
                <X class="h-4 w-4" />
              </template>
            </UiButton>
          </header>
        </slot>

        <div
          class="min-h-0 flex-1 overflow-y-auto overscroll-contain"
          :class="bodyPadding && 'px-4 py-3'"
        >
          <slot />
        </div>

        <footer
          v-if="$slots.footer"
          class="flex items-center justify-between gap-3 border-t border-edge px-4 py-3"
        >
          <slot name="footer" />
        </footer>
      </section>
    </div>
  </Teleport>
</template>
