import type { CategoryResult, Item, ItemFilter, ObtainedItem } from "../types";

export type AnyItem = Item | ObtainedItem;

export interface ItemRow {
  id: string;
  item: AnyItem;
  obtained: boolean;
  flags: string[];
  reason: string | null;
  source: string | null;
}

function canNumber(id: string): number {
  const match = /^Can_(\d+)$/.exec(id);
  return match ? Number(match[1]) : Number.MAX_SAFE_INTEGER;
}

export function categoryRows(category: CategoryResult, filter: ItemFilter): ItemRow[] {
  const rows: ItemRow[] = [];
  if (filter !== "missing") {
    rows.push(
      ...category.obtained_items.map((item) => ({
        id: item.id,
        item,
        obtained: true,
        flags: [],
        reason: null,
        source: item.source,
      })),
    );
  }
  if (filter !== "obtained") {
    rows.push(
      ...category.missing.map((item) => ({
        id: item.id,
        item,
        obtained: false,
        flags: item.flags,
        reason: item.reason,
        source: null,
      })),
    );
  }
  if (category.key === "cans") {
    rows.sort((left, right) => canNumber(left.id) - canNumber(right.id));
  }
  return rows;
}

export function matchesQuery(item: AnyItem, query: string): boolean {
  const needle = query.trim().toLowerCase();
  if (!needle) return true;
  return [
    item.id,
    item.name,
    item.name_en,
    item.area,
    item.area_zh,
    item.location,
    item.location_zh,
    item.obtain,
    item.obtain_zh,
    item.record_type,
    item.record_type_zh,
    item.desc_zh,
    item.desc_en,
    ...item.aliases,
  ].some((value) => (value ?? "").toLowerCase().includes(needle));
}
