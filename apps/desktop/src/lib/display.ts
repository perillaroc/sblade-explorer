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

interface LocatedItem {
  area?: string | null;
  area_zh?: string | null;
  location?: string | null;
  location_zh?: string | null;
}

export function locationLine(item: LocatedItem, lang: Lang): string {
  const area = localized(item.area_zh, item.area, lang);
  const location = localized(item.location_zh, item.location, lang);
  return [area, location].filter(Boolean).join(" · ");
}

export function obtainLabel(
  item: { obtain?: string | null; obtain_zh?: string | null },
  lang: Lang,
): string {
  return localized(item.obtain_zh, item.obtain, lang);
}
