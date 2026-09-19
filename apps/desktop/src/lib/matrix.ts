import type { Lang } from "../types";
import type { AnyItem, ItemRow } from "./items";
import { localized, matrixObtain, stripDesignPrefix } from "./display";

export const MATRIX_COLUMNS = ["首周目", "二周目(NG+)", "三周目(NG++)", "DLC/特典"];

const MATRIX_COLUMN_COUNT = MATRIX_COLUMNS.length;

const AREA_ORDER: Record<string, number> = {
  Default: 0,
  "Eidos 7": 1,
  Xion: 2,
  Wasteland: 3,
  "Matrix 11": 5,
  "Great Desert": 6,
  "Abyss Levoire": 7,
  "Eidos 9": 8,
  "Spire 4": 9,
  "Boss Challenge": 11,
};

export interface MatrixUnit {
  key: string;
  name: string;
  obtain: string;
  cells: ItemRow[][];
}

export interface MatrixLocation {
  key: string;
  label: string;
  units: MatrixUnit[];
}

export interface MatrixArea {
  key: string;
  label: string;
  locations: MatrixLocation[];
}

export function matrixColumn(item: AnyItem): number {
  return item.dlc ? 3 : Math.min(item.ng_plus, 2);
}

export function matrixPeriodLabel(item: AnyItem): string {
  return MATRIX_COLUMNS[matrixColumn(item)] ?? "";
}

export function statusEmoji(row: ItemRow, ngPlusCount: number): string {
  if (row.obtained) return "✅";
  if (row.item.dlc) return "🎁";
  if (row.item.ng_plus > ngPlusCount) return "🔒";
  if (row.item.aliases.length === 0) return "➖";
  return "❌";
}

export function matrixColumns(rows: ItemRow[]): number[] {
  return Array.from({ length: MATRIX_COLUMN_COUNT }, (_, index) => index).filter((index) =>
    rows.some((row) => matrixColumn(row.item) === index),
  );
}

const REPLACE_ZH = /替换(.+?)(?=[。；;，,\n]|$)/g;
const REPLACE_EN = /Replaces\s+(.+?)\s+(?:on|in)\s+NG/g;

function replacementPhrases(item: AnyItem): string[] {
  const phrases: string[] = [];
  for (const text of [item.obtain_zh, item.obtain]) {
    if (!text) continue;
    for (const match of text.matchAll(REPLACE_ZH)) phrases.push(match[1].trim());
    for (const match of text.matchAll(REPLACE_EN)) phrases.push(match[1].trim());
  }
  return phrases;
}

interface CandidateMatch {
  item: AnyItem;
  length: number;
}

function preferMatch(
  current: CandidateMatch | null,
  candidate: AnyItem,
  length: number,
  longer: boolean,
): CandidateMatch {
  if (!current) return { item: candidate, length };
  if (longer ? length > current.length : length < current.length) {
    return { item: candidate, length };
  }
  if (length === current.length && matrixColumn(candidate) < matrixColumn(current.item)) {
    return { item: candidate, length };
  }
  return current;
}

function findPredecessor(candidates: AnyItem[], phrases: string[]): AnyItem | null {
  for (const phrase of phrases) {
    const haystack = phrase.toLowerCase();
    let exact: CandidateMatch | null = null;
    let contained: CandidateMatch | null = null;
    let contains: CandidateMatch | null = null;
    for (const candidate of candidates) {
      for (const raw of [candidate.name, candidate.name_en]) {
        const name = raw ? stripDesignPrefix(raw).toLowerCase() : "";
        if (!name) continue;
        if (name === haystack) {
          exact = preferMatch(exact, candidate, name.length, false);
        } else if (haystack.includes(name)) {
          contained = preferMatch(contained, candidate, name.length, true);
        } else if (name.includes(haystack)) {
          contains = preferMatch(contains, candidate, name.length, false);
        }
      }
    }
    const found = exact ?? contained ?? contains;
    if (found) return found.item;
  }
  return null;
}

function locationKey(item: AnyItem): string {
  return `${item.area ?? "未分类"}\u0000${item.location ?? "未分类"}`;
}

function displayName(item: AnyItem, lang: Lang): string {
  return stripDesignPrefix(lang === "en" ? item.name_en || item.name : item.name);
}

function firstColumn(unit: MatrixUnit): number {
  return unit.cells.findIndex((cell) => cell.length > 0);
}

function compareText(left: string, right: string, lang: Lang): number {
  return left.localeCompare(right, lang === "en" ? "en" : "zh-Hans");
}

export function buildMatrix(rows: ItemRow[], allRows: ItemRow[], lang: Lang): MatrixArea[] {
  const byId = new Map(allRows.map((row) => [row.id, row]));
  const byLocation = new Map<string, ItemRow[]>();
  for (const row of allRows) {
    const key = locationKey(row.item);
    const list = byLocation.get(key);
    if (list) list.push(row);
    else byLocation.set(key, [row]);
  }

  const predecessor = new Map<string, string>();
  for (const row of allRows) {
    const candidates = (byLocation.get(locationKey(row.item)) ?? [])
      .map((candidate) => candidate.item)
      .filter((item) => item.id !== row.item.id);
    const match = findPredecessor(candidates, replacementPhrases(row.item));
    if (match) predecessor.set(row.item.id, match.id);
  }

  const neighbors = new Map<string, string[]>();
  const link = (left: string, right: string) => {
    const leftList = neighbors.get(left);
    if (leftList) leftList.push(right);
    else neighbors.set(left, [right]);
    const rightList = neighbors.get(right);
    if (rightList) rightList.push(left);
    else neighbors.set(right, [left]);
  };
  for (const [itemId, predecessorId] of predecessor) link(itemId, predecessorId);

  const displayed = new Map(rows.map((row) => [row.id, row]));
  const visited = new Set<string>();
  const areaMap = new Map<
    string,
    { label: string; locations: Map<string, MatrixLocation> }
  >();
  for (const start of allRows) {
    if (visited.has(start.id)) continue;
    const component: ItemRow[] = [];
    const stack = [start.id];
    visited.add(start.id);
    while (stack.length > 0) {
      const id = stack.pop();
      if (id === undefined) continue;
      const row = byId.get(id);
      if (row) component.push(row);
      for (const next of neighbors.get(id) ?? []) {
        if (!visited.has(next)) {
          visited.add(next);
          stack.push(next);
        }
      }
    }
    const shown = component.filter((row) => displayed.has(row.id));
    if (shown.length === 0) continue;

    const root = component.find((row) => !predecessor.has(row.id)) ?? component[0];
    const item = root.item;
    const areaRaw = item.area ?? "未分类";
    const locationRaw = item.location ?? "未分类";
    let area = areaMap.get(areaRaw);
    if (!area) {
      area = {
        label: localized(item.area_zh, item.area, lang) || "未分类",
        locations: new Map(),
      };
      areaMap.set(areaRaw, area);
    }
    let location = area.locations.get(locationRaw);
    if (!location) {
      location = {
        key: `${areaRaw}\u0000${locationRaw}`,
        label: localized(item.location_zh, item.location, lang) || "未分类",
        units: [],
      };
      area.locations.set(locationRaw, location);
    }

    const cells: ItemRow[][] = Array.from({ length: MATRIX_COLUMN_COUNT }, () => []);
    for (const entry of shown) cells[matrixColumn(entry.item)].push(entry);
    for (const cell of cells) {
      cell.sort(
        (left, right) =>
          left.item.ng_plus - right.item.ng_plus ||
          compareText(displayName(left.item, lang), displayName(right.item, lang), lang),
      );
    }
    location.units.push({
      key: root.id,
      name: displayName(item, lang),
      obtain: matrixObtain(item, lang),
      cells,
    });
  }

  return [...areaMap.entries()]
    .sort(
      ([left], [right]) =>
        (AREA_ORDER[left] ?? 99) - (AREA_ORDER[right] ?? 99) ||
        left.localeCompare(right),
    )
    .map(([key, area]) => ({
      key,
      label: area.label,
      locations: [...area.locations.values()]
        .map((location) => ({
          ...location,
          units: location.units.sort(
            (left, right) =>
              firstColumn(left) - firstColumn(right) ||
              compareText(left.name, right.name, lang),
          ),
        }))
        .sort(
          (left, right) =>
            firstColumn(left.units[0]) - firstColumn(right.units[0]) ||
            compareText(left.label, right.label, lang),
        ),
    }));
}
