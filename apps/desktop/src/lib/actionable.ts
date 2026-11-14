import type { Analysis, Item } from "../types";
import type { ItemRow } from "./items";

/**
 * Dashboard "ready to collect" derivation (F0 decision ③).
 *
 * `actionable`: missing items without a blocking reason (no NG+ / DLC /
 * missable flag), plus missable entries whose quest rule is still satisfied;
 * i.e. obtainable in the current playthrough.
 * `missable`: the still-obtainable subset, shown as the "collect before
 * leaving" strip above the list. Entries the save marks as missed are excluded.
 */
function missingRow(item: Item): ItemRow {
  return {
    id: item.id,
    item,
    obtained: false,
    flags: item.flags,
    reason: item.reason,
    source: null,
    missableState: item.missable_state,
  };
}

export function actionableRows(analysis: Analysis): ItemRow[] {
  const rows: ItemRow[] = [];
  for (const category of analysis.categories) {
    // Growth entries (gear upgrades) are not part of the collection checklist.
    if (category.section !== "collection") continue;
    for (const item of category.missing) {
      // Missable entries whose quest rule is satisfied are still collectable.
      if (item.reason === null || item.missable_state === "open") rows.push(missingRow(item));
    }
  }
  return rows;
}

/** Missable entries the player can still collect before leaving the area. */
export function missableRows(analysis: Analysis): ItemRow[] {
  const rows: ItemRow[] = [];
  for (const category of analysis.categories) {
    // Growth entries (gear upgrades) are not part of the collection checklist.
    if (category.section !== "collection") continue;
    for (const item of category.missing) {
      if (item.missable_state === "open") rows.push(missingRow(item));
    }
  }
  return rows;
}
