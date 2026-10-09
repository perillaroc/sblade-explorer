import type { Analysis, Item } from "../types";
import type { ItemRow } from "./items";

/**
 * Dashboard "ready to collect" derivation (F0 decision ③).
 *
 * `actionable`: missing items without a blocking reason (no NG+ / DLC /
 * missable flag), i.e. obtainable in the current playthrough.
 * `missable`: missing items that are obtainable now but easy to miss, shown
 * as a highlighted warning strip above the list.
 */
function missingRow(item: Item): ItemRow {
  return { id: item.id, item, obtained: false, flags: item.flags, reason: item.reason, source: null };
}

export function actionableRows(analysis: Analysis): ItemRow[] {
  const rows: ItemRow[] = [];
  for (const category of analysis.categories) {
    for (const item of category.missing) {
      if (item.reason === null) rows.push(missingRow(item));
    }
  }
  return rows;
}

export function missableRows(analysis: Analysis): ItemRow[] {
  const count = analysis.save.ng_plus_count;
  const rows: ItemRow[] = [];
  for (const category of analysis.categories) {
    for (const item of category.missing) {
      if (item.missable && item.dlc === null && item.ng_plus <= count) {
        rows.push(missingRow(item));
      }
    }
  }
  return rows;
}
