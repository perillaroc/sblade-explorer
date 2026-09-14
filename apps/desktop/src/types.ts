export interface SaveSlot {
  path: string;
  steamId: string | null;
  slot: number;
  mtimeMs: number;
  size: number;
  label: string;
}

export interface SaveInfo {
  path: string;
  steam_id: string | null;
  slot: number;
  playthrough: string;
  ng_plus_count: number;
  difficulty: number;
  play_time_seconds: number;
}

export interface Summary {
  catalog_total: number;
  catalog_obtained: number;
  missing_total: number;
  percent: number;
  obtained_aliases: number;
  unmapped_aliases: number;
}

export interface Item {
  id: string;
  name: string;
  name_en: string | null;
  aliases: string[];
  area: string | null;
  area_zh: string | null;
  location: string | null;
  location_zh: string | null;
  obtain: string | null;
  obtain_zh: string | null;
  ng_plus: number;
  dlc: string | null;
  missable: boolean;
  note: string | null;
  confidence: string;
  reason: string | null;
  flags: string[];
}

export interface ObtainedItem {
  id: string;
  name: string;
  category: string;
  aliases: string[];
  area: string | null;
  area_zh: string | null;
  location: string | null;
  location_zh: string | null;
  obtain: string | null;
  obtain_zh: string | null;
  ng_plus: number;
  dlc: string | null;
  missable: boolean;
  note: string | null;
  confidence: string;
  source: string | null;
  name_en: string | null;
}

export interface CategoryResult {
  key: string;
  name: string;
  total: number;
  obtained: number;
  missing: Item[];
  extra_obtained_aliases: string[];
  obtained_items: ObtainedItem[];
}

export interface Analysis {
  save: SaveInfo;
  summary: Summary;
  categories: CategoryResult[];
  unmapped_obtained: string[];
}

export type Lang = "zh" | "en" | "both";
