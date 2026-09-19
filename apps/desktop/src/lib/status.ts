import { CircleCheck, CircleX, Gift, Lock, Minus, type LucideIcon } from "@lucide/vue";
import type { ItemRow } from "./items";

export interface MatrixStatus {
  icon: LucideIcon;
  label: string;
  className: string;
}

const OBTAINED: MatrixStatus = { icon: CircleCheck, label: "已获得", className: "text-emerald-400" };
const MISSING: MatrixStatus = { icon: CircleX, label: "未获得", className: "text-rose-400" };
const LOCKED: MatrixStatus = { icon: Lock, label: "需更高周目", className: "text-slate-500" };
const DLC: MatrixStatus = { icon: Gift, label: "DLC/特典", className: "text-violet-400" };
const DEFAULT: MatrixStatus = { icon: Minus, label: "默认外观", className: "text-slate-600" };

export const MATRIX_STATUS_LEGEND: MatrixStatus[] = [OBTAINED, MISSING, LOCKED, DLC, DEFAULT];

export function matrixStatus(row: ItemRow, ngPlusCount: number): MatrixStatus {
  if (row.obtained) return OBTAINED;
  if (row.item.dlc) return DLC;
  if (row.item.ng_plus > ngPlusCount) return LOCKED;
  if (row.item.aliases.length === 0) return DEFAULT;
  return MISSING;
}
