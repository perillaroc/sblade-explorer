import type { Lang } from "../types";
import { translate } from "./i18n";
import { settings } from "./settings";

export function localized(
  zh: string | null | undefined,
  en: string | null | undefined,
  lang: Lang,
): string {
  if (lang === "en") {
    return en || zh || "";
  }
  if (lang === "both" && zh && en && zh !== en) {
    return `${zh}（${en}）`;
  }
  return zh || en || "";
}

interface NamedItem {
  name: string;
  name_en?: string | null;
}

export function itemName(item: NamedItem, lang: Lang): string {
  if (lang === "en") {
    return item.name_en || item.name;
  }
  if (lang === "both" && item.name_en && item.name_en !== item.name) {
    return `${item.name}（${item.name_en}）`;
  }
  return item.name;
}

export function stripDesignPrefix(value: string): string {
  return value.replace(/^设计图：/, "").replace(/^Design Pattern:\s*/, "").trim();
}

export function matrixName(item: NamedItem, lang: Lang): string {
  const zh = stripDesignPrefix(item.name);
  const en = item.name_en ? stripDesignPrefix(item.name_en) : "";
  if (lang === "en") {
    return en || zh;
  }
  if (lang === "both" && en && en !== zh) {
    return `${zh}（${en}）`;
  }
  return zh || en;
}

interface LocatedItem {
  area?: string | null;
  area_zh?: string | null;
  location?: string | null;
  location_zh?: string | null;
}

export function areaLabel(item: LocatedItem, lang: Lang): string {
  return localized(item.area_zh, item.area, lang);
}

export function locationLabel(item: LocatedItem, lang: Lang): string {
  const area = areaLabel(item, lang);
  const location = localized(item.location_zh, item.location, lang);
  return location === area ? "" : location;
}

export function locationLine(item: LocatedItem, lang: Lang): string {
  return [areaLabel(item, lang), locationLabel(item, lang)].filter(Boolean).join(" · ");
}

export function obtainLabel(
  item: { obtain?: string | null; obtain_zh?: string | null },
  lang: Lang,
): string {
  return localized(item.obtain_zh, item.obtain, lang);
}

export function descLabel(
  item: { desc_zh?: string | null; desc_en?: string | null },
  lang: Lang,
): string {
  return localized(item.desc_zh, item.desc_en, lang);
}

interface NamedCategory {
  name: string;
  name_en?: string | null;
}

/** Category display name for the content language. */
export function categoryName(category: NamedCategory, lang: Lang): string {
  return localized(category.name, category.name_en, lang);
}

interface TypedItem {
  record_type?: string | null;
  record_type_zh?: string | null;
  record_type_en?: string | null;
}

/** Record type group label for the content language. */
export function recordTypeLabel(item: TypedItem, lang: Lang): string {
  const zh = item.record_type_zh ?? item.record_type ?? "";
  const en = item.record_type_en ?? item.record_type ?? "";
  return localized(zh, en, lang) || translate("common.uncategorized");
}

/** Cycle label used in flags and details (interface locale). */
export function cycleLabel(ngPlus: number): string {
  if (ngPlus <= 0) return translate("cycles.base");
  if (ngPlus === 1) return translate("cycles.ngPlus");
  if (ngPlus === 2) return translate("cycles.ngPlusPlus");
  return translate("cycles.ngPlusN", { count: ngPlus });
}

const KNOWN_DLC = ["nier", "nikke", "deluxe", "preorder", "summer"] as const;

/** DLC label used in flags and details (interface locale). */
export function dlcLabel(dlc: string): string {
  return (KNOWN_DLC as readonly string[]).includes(dlc) ? translate(`dlc.${dlc}`) : dlc;
}

interface FlagItem {
  dlc?: string | null;
  ng_plus: number;
  missable?: boolean;
  confidence?: string;
}

/** Localized flags for a missing item (mirrors the Rust report). */
export function flagLabels(item: FlagItem): string[] {
  const flags: string[] = [];
  if (item.dlc) flags.push(dlcLabel(item.dlc));
  if (item.ng_plus > 0) flags.push(cycleLabel(item.ng_plus));
  if (item.missable) flags.push(translate("itemTable.missable"));
  if (item.confidence && item.confidence !== "high") {
    flags.push(translate("itemTable.mappingUnconfirmed"));
  }
  return flags;
}

/** Localized explanation of why the item is missing. */
export function reasonLabel(item: FlagItem, ngPlusCount: number): string | null {
  if (item.ng_plus > ngPlusCount) {
    return translate("reasons.requires", { label: cycleLabel(item.ng_plus) });
  }
  if (item.dlc) {
    return translate("reasons.dlcOnly", { label: dlcLabel(item.dlc) });
  }
  if (item.missable) return translate("reasons.missable");
  return null;
}

const REPLACE_LEAD_ZH = /^在\s*NG\+\+?\s*中替换[^。]*。\s*/;
const REPLACE_LEAD_EN = /^Replaces\s+.+?\s+(?:on|in)\s+NG\+\+?[.!]?\s*/i;

function stripReplaceLead(value: string): string {
  return value.replace(REPLACE_LEAD_ZH, "").replace(REPLACE_LEAD_EN, "").trim();
}

export function matrixObtain(
  item: { obtain?: string | null; obtain_zh?: string | null },
  lang: Lang,
): string {
  const zh = item.obtain_zh ? stripReplaceLead(item.obtain_zh) : "";
  const en = item.obtain ? stripReplaceLead(item.obtain) : "";
  if (lang === "en") {
    return en || zh;
  }
  if (lang === "both" && zh && en && en !== zh) {
    return `${zh}（${en}）`;
  }
  return zh || en;
}

/** Save file size in MB with one decimal (sidebar save card / slot list). */
export function saveSize(size: number): string {
  return `${(size / 1024 / 1024).toFixed(1)} MB`;
}

/** Save file timestamp for the current interface locale. */
export function saveTime(mtimeMs: number): string {
  const locale = settings.uiLocale === "zh" ? "zh-CN" : "en-US";
  return new Date(mtimeMs).toLocaleString(locale, { hour12: false });
}
