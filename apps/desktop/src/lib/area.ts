export interface AreaAccent {
  band: string;
  cell: string;
  border: string;
  dot: string;
  text: string;
}

const NEUTRAL_ACCENT: AreaAccent = {
  band: "bg-slate-700/40",
  cell: "bg-slate-700/20",
  border: "border-slate-500/60",
  dot: "bg-slate-400",
  text: "text-slate-300",
};

const PALETTE: AreaAccent[] = [
  {
    band: "bg-sky-500/15",
    cell: "bg-sky-500/10",
    border: "border-sky-400/70",
    dot: "bg-sky-400",
    text: "text-sky-200",
  },
  {
    band: "bg-emerald-500/15",
    cell: "bg-emerald-500/10",
    border: "border-emerald-400/70",
    dot: "bg-emerald-400",
    text: "text-emerald-200",
  },
  {
    band: "bg-amber-500/15",
    cell: "bg-amber-500/10",
    border: "border-amber-400/70",
    dot: "bg-amber-400",
    text: "text-amber-200",
  },
  {
    band: "bg-rose-500/15",
    cell: "bg-rose-500/10",
    border: "border-rose-400/70",
    dot: "bg-rose-400",
    text: "text-rose-200",
  },
  {
    band: "bg-violet-500/15",
    cell: "bg-violet-500/10",
    border: "border-violet-400/70",
    dot: "bg-violet-400",
    text: "text-violet-200",
  },
  {
    band: "bg-cyan-500/15",
    cell: "bg-cyan-500/10",
    border: "border-cyan-400/70",
    dot: "bg-cyan-400",
    text: "text-cyan-200",
  },
  {
    band: "bg-orange-500/15",
    cell: "bg-orange-500/10",
    border: "border-orange-400/70",
    dot: "bg-orange-400",
    text: "text-orange-200",
  },
  {
    band: "bg-lime-500/15",
    cell: "bg-lime-500/10",
    border: "border-lime-400/70",
    dot: "bg-lime-400",
    text: "text-lime-200",
  },
  {
    band: "bg-fuchsia-500/15",
    cell: "bg-fuchsia-500/10",
    border: "border-fuchsia-400/70",
    dot: "bg-fuchsia-400",
    text: "text-fuchsia-200",
  },
  {
    band: "bg-teal-500/15",
    cell: "bg-teal-500/10",
    border: "border-teal-400/70",
    dot: "bg-teal-400",
    text: "text-teal-200",
  },
  {
    band: "bg-indigo-500/15",
    cell: "bg-indigo-500/10",
    border: "border-indigo-400/70",
    dot: "bg-indigo-400",
    text: "text-indigo-200",
  },
  {
    band: "bg-pink-500/15",
    cell: "bg-pink-500/10",
    border: "border-pink-400/70",
    dot: "bg-pink-400",
    text: "text-pink-200",
  },
];

const AREA_COLOR_INDEX: Record<string, number> = {
  default: 0,
  "eidos-7": 1,
  xion: 2,
  wasteland: 3,
  "matrix-11": 4,
  "great-desert": 5,
  "abyss-levoire": 6,
  "eidos-9": 7,
  "spire-4": 8,
  "boss-challenge": 9,
};

const NEUTRAL_KEYS = new Set(["未分类", "其他"]);

function canonicalArea(area: string): string {
  const key = area.trim().toLowerCase();
  if (key.startsWith("大沙漠")) return "great-desert";
  if (key.startsWith("矩阵11")) return "matrix-11";
  if (key.startsWith("尖塔")) return "spire-4";
  if (key.startsWith("废土")) return "wasteland";
  if (key === "希雍") return "xion";
  if (key === "深渊勒瓦尔") return "abyss-levoire";
  return key;
}

export function areaAccent(area: string | null | undefined): AreaAccent {
  const raw = (area ?? "").trim();
  if (!raw) return NEUTRAL_ACCENT;
  if (NEUTRAL_KEYS.has(raw)) return NEUTRAL_ACCENT;
  const key = canonicalArea(raw);
  const fixed = AREA_COLOR_INDEX[key];
  if (fixed !== undefined) return PALETTE[fixed];
  let hash = 0;
  for (let index = 0; index < key.length; index += 1) {
    hash = (hash * 31 + key.charCodeAt(index)) >>> 0;
  }
  return PALETTE[hash % PALETTE.length];
}
