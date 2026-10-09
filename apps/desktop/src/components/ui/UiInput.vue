<script setup lang="ts">
import { computed, ref, useSlots } from "vue";

/**
 * Text input with an optional leading icon (search boxes, keyword filters).
 * Focus ring: the outline comes from the global `:focus-visible` rule, the
 * border tint covers mouse focus as well.
 */
const props = withDefaults(
  defineProps<{
    modelValue?: string;
    type?: "text" | "search";
    id?: string;
    placeholder?: string;
    disabled?: boolean;
    readonly?: boolean;
    size?: "sm" | "md";
  }>(),
  { modelValue: "", type: "text", size: "sm" },
);

const emit = defineEmits<{ "update:modelValue": [value: string] }>();

const slots = useSlots();
const hasIcon = computed(() => Boolean(slots.icon));

const element = ref<HTMLInputElement | null>(null);

/** Lets callers focus/select the inner input (global search shortcut). */
defineExpose({
  focus: () => element.value?.focus(),
  select: () => {
    element.value?.focus();
    element.value?.select();
  },
});

const classes = computed(() => [
  "w-full rounded-md border border-edge bg-surface-card text-ink transition-colors duration-fast",
  "placeholder:text-ink-subtle focus:border-brand disabled:cursor-not-allowed disabled:opacity-50",
  props.size === "sm" ? "h-7 text-xs" : "h-9 text-sm",
  hasIcon.value ? "pl-7 pr-2.5" : "px-2.5",
]);

function onInput(event: Event) {
  emit("update:modelValue", (event.target as HTMLInputElement).value);
}
</script>

<template>
  <div class="relative inline-flex items-center">
    <span
      v-if="hasIcon"
      class="pointer-events-none absolute left-2 top-1/2 flex -translate-y-1/2 text-ink-subtle"
    >
      <slot name="icon" />
    </span>
    <input
      ref="element"
      :id="id"
      :type="type"
      :class="classes"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :readonly="readonly"
      @input="onInput"
    />
  </div>
</template>
