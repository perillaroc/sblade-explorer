import type { Lang } from "../types";

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
