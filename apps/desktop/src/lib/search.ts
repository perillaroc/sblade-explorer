import type { Analysis, CategoryResult } from "../types";
import { categoryRows, matchesQuery, type ItemRow } from "./items";

/** One category's slice of the global search results. */
export interface SearchGroup {
  key: string;
  category: CategoryResult;
  rows: ItemRow[];
}

const DEFAULT_PER_CATEGORY = 6;

/**
 * Cross-category search used by the sidebar global search (F4): reuses
 * `matchesQuery` (name / location / obtain / aliases, both languages) and
 * groups hits by category, capped per category to keep the panel light.
 */
export function searchCatalog(
  analysis: Analysis,
  query: string,
  perCategory: number = DEFAULT_PER_CATEGORY,
): SearchGroup[] {
  const needle = query.trim();
  if (!needle) return [];
  const groups: SearchGroup[] = [];
  for (const category of analysis.categories) {
    const rows = categoryRows(category, "all")
      .filter((row) => matchesQuery(row.item, needle))
      .slice(0, perCategory);
    if (rows.length > 0) groups.push({ key: category.key, category, rows });
  }
  return groups;
}
