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
    ...item.aliases,
  ].some((value) => (value ?? "").toLowerCase().includes(needle));
}
