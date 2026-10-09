import { CircleCheck, CircleX, Gift, Lock, Minus, type LucideIcon } from "@lucide/vue";
import { translate } from "./i18n";
import type { ItemRow } from "./items";

export interface MatrixStatus {
  icon: LucideIcon;
  label: string;
  className: string;
}

const STYLES = {
  obtained: "text-emerald-600 dark:text-emerald-400",
  missing: "text-rose-600 dark:text-rose-400",
  locked: "text-slate-500",
  dlc: "text-violet-600 dark:text-violet-400",
  defaultAppearance: "text-slate-500 dark:text-slate-400",
} as const;

const ICONS: Record<keyof typeof STYLES, LucideIcon> = {
  obtained: CircleCheck,
  missing: CircleX,
  locked: Lock,
  dlc: Gift,
  defaultAppearance: Minus,
};

function status(key: keyof typeof STYLES): MatrixStatus {
  return {
    icon: ICONS[key],
    label: translate(`obtainedStatus.${key}`),
    className: STYLES[key],
  };
}

/** Legend for the cycle matrix (labels follow the interface language). */
export function matrixStatusLegend(): MatrixStatus[] {
  return (Object.keys(STYLES) as (keyof typeof STYLES)[]).map(status);
}

export function matrixStatus(row: ItemRow, ngPlusCount: number): MatrixStatus {
  if (row.obtained) return status("obtained");
  if (row.item.dlc) return status("dlc");
  if (row.item.ng_plus > ngPlusCount) return status("locked");
  if (row.item.aliases.length === 0) return status("defaultAppearance");
  return status("missing");
}
