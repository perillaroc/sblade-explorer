"""Build src/sbsave/data/catalog.json from raw guides + crosswalk data.

Run with:  uv run python tools/build_catalog.py

Inputs:
  data/raw/api/*.json         stellarbladeguide.com API responses (committed)
  data/raw/universe/aliases.json  alias universes extracted from a 100% save
  tools/crosswalk_data.py     hand curated alias <-> guide item mappings

Output:
  src/sbsave/data/catalog.json
"""

from __future__ import annotations

import json
import re
from pathlib import Path

import crosswalk_data as cw

ROOT = Path(__file__).resolve().parent.parent
API_DIR = ROOT / "data" / "raw" / "api"
UNIVERSE = ROOT / "data" / "raw" / "universe" / "aliases.json"
OUTPUT = ROOT / "src" / "sbsave" / "data" / "catalog.json"

CATEGORIES = [
    ("nano_suits", "纳米战衣", 10),
    ("cans", "罐子", 20),
    ("records", "记录(文档/记忆棒)", 30),
    ("passcodes", "密码", 40),
    ("camps", "营地", 50),
    ("hair", "发型", 60),
    ("glasses", "眼镜/面饰", 70),
    ("earrings", "耳饰", 80),
    ("drone_seals", "无人机外观", 90),
    ("adam_costumes", "亚当服装", 100),
    ("lily_costumes", "莉莉服装", 110),
    ("design_patterns", "设计图案", 120),
    ("fish", "鱼类", 130),
]

CYCLE_TO_NG_PLUS = {"Base": 0, "NG+": 1, "NG++": 2, "DLC": 0}


def load_site_index() -> dict[int, dict]:
    index: dict[int, dict] = {}
    order = 0
    for path in sorted(API_DIR.glob("*.json")):
        payload = json.loads(path.read_text(encoding="utf-8"))
        for level in payload:
            for location in level.get("locations", []):
                for item in location.get("collectibles", []):
                    order += 1
                    index[int(item["id"])] = {
                        "title": item.get("title") or "",
                        "cycle": item.get("cycle") or "Base",
                        "level": level.get("level_name") or "",
                        "location": location.get("location_name") or "",
                        "description": (item.get("description") or {}).get("content") or "",
                        "types": item.get("types") or [],
                        "source_file": path.name,
                        "order": order,
                    }
    return index


def dlc_from_aliases(aliases: list[str], cycle: str) -> str | None:
    if any("Nier" in alias for alias in aliases):
        return "nier"
    if any("Nikke" in alias for alias in aliases):
        return "nikke"
    if cycle == "DLC":
        return "dlc"
    return None


def make_item(
    *,
    item_id: str,
    name: str,
    category: str,
    aliases: list[str],
    site: dict | None,
    confidence: str = "high",
    name_en: str | None = None,
    note: str | None = None,
    source: str | None = None,
) -> dict:
    cycle = site.get("cycle", "Base") if site else "Base"
    return {
        "id": item_id,
        "name": name,
        "name_en": name_en or (site.get("title") if site else None),
        "category": category,
        "aliases": aliases,
        "area": site.get("level") if site else None,
        "location": site.get("location") if site else None,
        "obtain": site.get("description") if site else None,
        "ng_plus": CYCLE_TO_NG_PLUS.get(cycle, 0),
        "dlc": dlc_from_aliases(aliases, cycle),
        "missable": False,
        "note": note,
        "confidence": confidence,
        "source": f"{cw.SITE_BASE_URL}/api/{site['source_file'][:-5].replace('__', '/')}" if site else source,
    }


def build_nano_suits(site: dict[int, dict]) -> list[dict]:
    items: list[dict] = []
    covered: set[int] = set()
    for site_id, aliases, zh, confidence in cw.NANO_SUITS:
        info = site.get(site_id)
        if info is None:
            raise SystemExit(f"nano suit site id {site_id} not found")
        covered.add(site_id)
        items.append(
            make_item(
                item_id=aliases[0],
                name=zh,
                category="nano_suits",
                aliases=aliases,
                site=info,
                confidence=confidence,
            )
        )
    missing = {i for i in site if site[i]["source_file"] == "cosmetics__nano-suits.json"} - covered
    for site_id in sorted(missing):
        title = site[site_id]["title"]
        note = "默认外观，无存档物品 ID" if title == "Skin Suit" else "未建立别名映射"
        items.append(
            make_item(
                item_id=f"site_{site_id}",
                name=title,
                category="nano_suits",
                aliases=[],
                site=site[site_id],
                confidence="none",
                note=note,
            )
        )
    return items


def build_appearance(site: dict[int, dict]) -> list[dict]:
    items: list[dict] = []
    covered: set[int] = set()
    for site_id, aliases, zh, category, confidence in cw.APPEARANCE:
        info = site.get(site_id)
        if info is None:
            raise SystemExit(f"appearance site id {site_id} not found")
        covered.add(site_id)
        items.append(
            make_item(
                item_id=aliases[0],
                name=zh,
                category=category,
                aliases=aliases,
                site=info,
                confidence=confidence,
            )
        )
    return items


def build_cans(site: dict[int, dict]) -> list[dict]:
    can_ids = sorted(
        (i for i in site if site[i]["source_file"] == "collectibles__cans.json"),
        key=lambda i: site[i]["order"],
    )
    items: list[dict] = []
    for number, site_id in enumerate(can_ids, start=1):
        info = site[site_id]
        items.append(
            make_item(
                item_id=f"Can_{number:03d}",
                name=info["title"],
                category="cans",
                aliases=[f"Can_{number:03d}"],
                site=info,
                confidence="medium",
                note="编号按攻略顺序推定，可用游戏内罐子图鉴核对",
            )
        )
    return items


_ZONE_RE = re.compile(r"Item_Records_([A-Za-z0-9]+?)_(Memory|Passcode)_(\d+)(.*)$")
_SUFFIX_RE = re.compile(r"_(Maintain|Used|Popup)$")


def zone_label(zone: str) -> str:
    label = cw.ZONE_ZH.get(zone)
    if label:
        return label
    parts = zone.split("_")
    if len(parts) >= 2 and parts[-1].isdigit():
        base = "_".join(parts[:-1])
        return cw.ZONE_ZH.get(base, zone)
    return zone


def normalize_record_aliases(universe: dict) -> list[str]:
    aliases: set[str] = set()
    for alias in universe.get("records", []):
        aliases.add(_SUFFIX_RE.sub("", alias))
    return sorted(aliases)


def build_records(universe: dict) -> list[dict]:
    items: list[dict] = []
    for alias in normalize_record_aliases(universe):
        match = _ZONE_RE.match(alias)
        if not match:
            items.append(
                {
                    "id": alias,
                    "name": alias.replace("Item_Records_", "记录："),
                    "name_en": alias,
                    "category": "records",
                    "aliases": [alias],
                    "area": None,
                    "location": None,
                    "obtain": None,
                    "ng_plus": 0,
                    "dlc": None,
                    "missable": False,
                    "note": "任务/活动记录，名称来自内部 ID",
                    "confidence": "low",
                    "source": None,
                }
            )
            continue
        zone, kind, number, suffix = match.groups()
        kind_zh = "记录" if kind == "Memory" else "密码"
        category = "records" if kind == "Memory" else "passcodes"
        suffix_zh = ""
        if suffix == "_1":
            suffix_zh = "（版本2）"
        elif suffix == "_2":
            suffix_zh = "（版本3）"
        name = f"{zone_label(zone)} {kind_zh} {int(number):02d}{suffix_zh}"
        items.append(
            {
                "id": alias,
                "name": name,
                "name_en": alias,
                "category": category,
                "aliases": [alias],
                "area": zone_label(zone),
                "location": None,
                "obtain": None,
                "ng_plus": 0,
                "dlc": None,
                "missable": False,
                "note": "按内部 ID 生成名称；文档/记忆棒共享该别名族，位置请参考攻略同区域列表",
                "confidence": "low",
                "source": None,
            }
        )
    return items


def build_fish(universe: dict) -> list[dict]:
    skip = {"Fish_01", "Fish_02", "Fish_03", "Fish_04", "Fish_05", "Fish_06",
            "Fish_07", "Fish_08", "Fish_09", "Fish_10", "Fish_Box4", "Fish_Slice_Bait",
            "Fish_GoldFish", "Fish_Mahimahi"}
    items: list[dict] = []
    for alias in universe.get("fish", []):
        if alias in skip:
            continue
        name = cw.FISH_ZH.get(alias, alias.replace("Fish_", ""))
        dlc = "nikke" if "Nikke" in alias else None
        items.append(
            {
                "id": alias,
                "name": name,
                "name_en": alias.replace("Fish_", ""),
                "category": "fish",
                "aliases": [alias],
                "area": None,
                "location": None,
                "obtain": None,
                "ng_plus": 0,
                "dlc": dlc,
                "missable": False,
                "note": "钓鱼图鉴；位置参考各钓鱼点攻略",
                "confidence": "high",
                "source": None,
            }
        )
    return items


def build_design_patterns(site: dict[int, dict], universe: dict) -> list[dict]:
    pattern_universe = set(universe.get("design_patterns", []))
    items: list[dict] = []
    covered: set[str] = set()
    for site_id, aliases, zh, _confidence in cw.NANO_SUITS:
        candidates = []
        for alias in aliases:
            candidate = f"DesignPattern_{alias}"
            if candidate in pattern_universe:
                candidates.append(candidate)
        if not candidates:
            continue
        info = site.get(site_id)
        covered.update(candidates)
        items.append(
            make_item(
                item_id=candidates[0],
                name=f"设计图案：{zh}",
                category="design_patterns",
                aliases=candidates,
                site=info,
                confidence="medium",
                note="图案与战衣同名（存档内部 ID 推导）",
            )
        )
    for alias in sorted(pattern_universe - covered):
        items.append(
            {
                "id": alias,
                "name": alias.replace("DesignPattern_", "设计图案："),
                "name_en": alias,
                "category": "design_patterns",
                "aliases": [alias],
                "area": None,
                "location": None,
                "obtain": None,
                "ng_plus": 0,
                "dlc": None,
                "missable": False,
                "note": "未建立战衣映射",
                "confidence": "low",
                "source": None,
            }
        )
    return items


_CAMP_RE = re.compile(r"ChangeState_ZoneEnv_(.+?)_EnvS_(\d+)_Camp")


def build_camps(universe: dict) -> list[dict]:
    items: list[dict] = []
    for alias in universe.get("camps", []):
        match = _CAMP_RE.match(alias)
        zone = match.group(1) if match else "?"
        number = int(match.group(2)) if match else 0
        items.append(
            {
                "id": alias,
                "name": f"{zone_label(zone)} 营地 #{number}",
                "name_en": alias,
                "category": "camps",
                "aliases": [alias],
                "area": zone_label(zone),
                "location": None,
                "obtain": None,
                "ng_plus": 0,
                "dlc": None,
                "missable": False,
                "note": "按内部 ID 生成名称；位置请参考攻略同区域营地列表",
                "confidence": "low",
                "source": None,
            }
        )
    return items


def build_gear(universe: dict) -> list[dict]:
    items: list[dict] = []
    for alias in universe.get("gear", []):
        name = alias.replace("Gear_", "").replace("_", " ")
        items.append(
            {
                "id": alias,
                "name": name,
                "name_en": alias,
                "category": "gear",
                "aliases": [alias],
                "area": None,
                "location": None,
                "obtain": None,
                "ng_plus": 1 if alias.endswith("_MK2") else 0,
                "dlc": None,
                "missable": False,
                "note": "名称来自内部 ID；MK2 为多周目版本",
                "confidence": "low",
                "source": None,
            }
        )
    return items


def main() -> None:
    site = load_site_index()
    universe = json.loads(UNIVERSE.read_text(encoding="utf-8"))

    items: list[dict] = []
    items += build_nano_suits(site)
    items += build_cans(site)
    items += build_records(universe)
    items += build_fish(universe)
    items += build_camps(universe)
    items += build_appearance(site)
    items += build_design_patterns(site, universe)
    # Gear is deliberately left out of v1: it is equipment rather than a
    # collectible, and the alias names carry no localised text.  The alias
    # universe is kept in data/raw/universe/aliases.json for future use.

    # Validate alias uniqueness.
    seen: dict[str, str] = {}
    for item in items:
        for alias in item["aliases"]:
            if alias in seen:
                raise SystemExit(f"duplicate alias {alias}: {seen[alias]} vs {item['id']}")
            seen[alias] = item["id"]

    catalog = {
        "version": 1,
        "generated_by": "tools/build_catalog.py",
        "sources": [
            "https://stellarbladeguide.com (collectibles, cosmetics, locations, cycles)",
            "https://github.com/wuxiao00j/stellar-blade-macos-save-editor (Simplified Chinese names)",
            "https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file (alias universe)",
        ],
        "categories": [{"key": k, "name": n, "order": o} for k, n, o in CATEGORIES],
        "items": items,
    }
    OUTPUT.write_text(json.dumps(catalog, ensure_ascii=False, indent=1), encoding="utf-8")
    print(f"wrote {OUTPUT.relative_to(ROOT)} with {len(items)} items, {len(seen)} aliases")


if __name__ == "__main__":
    main()
