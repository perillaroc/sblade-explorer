<script setup lang="ts" generic="T">
interface UiTableColumn {
  key: string;
  label?: string;
  headerClass?: string;
  cellClass?: string;
}

/**
 * Generic table: sticky header, hover and (when `clickable`) row activation
 * via click, Enter or Space. Cell content comes from `cell-<column.key>`
 * scoped slots; `empty` / `emptyText` cover the no-rows state.
 */
const props = withDefaults(
  defineProps<{
    columns: readonly UiTableColumn[];
    rows: readonly T[];
    rowKey: (row: T, index: number) => string | number;
    clickable?: boolean;
    stickyHeader?: boolean;
    ariaLabel?: string;
    emptyText?: string;
    rowClass?: (row: T, index: number) => string | undefined;
  }>(),
  { clickable: false, stickyHeader: true },
);

const emit = defineEmits<{ rowActivate: [row: T, index: number] }>();

function onRowClick(row: T, index: number): void {
  if (props.clickable) emit("rowActivate", row, index);
}

function onRowKeydown(event: KeyboardEvent, row: T, index: number): void {
  if (!props.clickable) return;
  if (event.key === "Enter" || event.key === " ") {
    event.preventDefault();
    emit("rowActivate", row, index);
  }
}

function cellValue(row: T, key: string): unknown {
  return (row as Record<string, unknown>)[key];
}
</script>

<template>
  <div class="overflow-x-auto">
    <table class="w-full text-xs" :aria-label="ariaLabel">
      <thead class="bg-surface-card" :class="stickyHeader ? 'sticky top-0 z-sticky' : ''">
        <tr class="border-b border-edge text-left text-ink-muted">
          <th
            v-for="column in columns"
            :key="column.key"
            class="px-4 py-2 font-medium"
            :class="column.headerClass"
          >
            {{ column.label }}
          </th>
        </tr>
      </thead>
      <tbody v-if="rows.length > 0">
        <tr
          v-for="(row, index) in rows"
          :key="rowKey(row, index)"
          :data-key="rowKey(row, index)"
          class="border-t border-edge align-top transition-colors duration-fast"
          :class="[
            clickable && 'cursor-pointer hover:bg-graphite-100 dark:hover:bg-white/5',
            clickable && 'focus-visible:bg-graphite-100 dark:focus-visible:bg-white/5',
            rowClass?.(row, index),
          ]"
          :tabindex="clickable ? 0 : undefined"
          @click="onRowClick(row, index)"
          @keydown="onRowKeydown($event, row, index)"
        >
          <td
            v-for="column in columns"
            :key="column.key"
            class="px-4 py-2"
            :class="column.cellClass"
          >
            <slot :name="`cell-${column.key}`" :row="row" :index="index" :value="cellValue(row, column.key)">
              {{ cellValue(row, column.key) ?? "" }}
            </slot>
          </td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr>
          <td :colspan="columns.length" class="px-4 py-10 text-center text-xs text-ink-muted">
            <slot name="empty">{{ emptyText }}</slot>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
