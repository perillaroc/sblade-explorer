"""Mine internal ID -> localized name mappings from Stellar Blade game data.

The save file only stores internal aliases (e.g. ``Can_007``,
``Item_Records_DED10_Memory_09``).  This tool reads exported game tables and the
Simplified Chinese / English ``Game.locres`` to build a name map that
``build_catalog.py`` consumes.

Game files are never committed.  Either point ``--dump`` at a directory with the
required exports (see ``data/raw/README.md``) or pass ``--game``/``--tools`` so
the tool drives ``cue4parse`` and ``repak`` itself.

Run with:  uv run python tools/mine_game_names.py --dump path/to/dump
Output:    data/raw/game/name_map.json
"""

from __future__ import annotations

import argparse
import json
import re
import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UNIVERSE = ROOT / "data" / "raw" / "universe" / "aliases.json"
DEFAULT_OUT = ROOT / "data" / "raw" / "game" / "name_map.json"

LANG_FILES = {"zh": "Game.zh-Hans.locres", "en": "Game.en.locres"}
LOCRES_PATH = "SB/Content/Localization/Game/{culture}/Game.locres"
LOCRES_CULTURES = {"zh": "zh-Hans", "en": "en"}
TABLE_PACKAGES = (
    "*/ItemTable.uasset",
    "*/ZoneCampTable.uasset",
)
_SUFFIX_RE = re.compile(r"_(Maintain|Used|Popup)$")
_ENV_RE = re.compile(r"(.+?)_(\d+)_EnvS_(\d+)$")

_LOCRES_MAGIC = struct.pack("<IIII", 0x7574140E, 0xFC034A67, 0x9D90154A, 0x1B7F37C3)


class _Reader:
    def __init__(self, data: bytes) -> None:
        self.data = data
        self.pos = 0

    def u8(self) -> int:
        value = self.data[self.pos]
        self.pos += 1
        return value

    def i32(self) -> int:
        value = struct.unpack_from("<i", self.data, self.pos)[0]
        self.pos += 4
        return value

    def u32(self) -> int:
        value = struct.unpack_from("<I", self.data, self.pos)[0]
        self.pos += 4
        return value

    def i64(self) -> int:
        value = struct.unpack_from("<q", self.data, self.pos)[0]
        self.pos += 8
        return value

    def fstring(self) -> str:
        length = self.i32()
        if length == 0:
            return ""
        if length > 0:
            raw = self.data[self.pos : self.pos + length]
            self.pos += length
            return raw.split(b"\x00", 1)[0].decode("utf-8", errors="replace")
        length = -length
        raw = self.data[self.pos : self.pos + length * 2]
        self.pos += length * 2
        return raw.decode("utf-16le", errors="replace").split("\x00", 1)[0]


def read_locres(path: Path) -> dict[str, dict[str, str]]:
    """Parse an Unreal Engine 4 ``.locres`` file into namespace -> key -> text."""
    reader = _Reader(path.read_bytes())
    if reader.data[:16] == _LOCRES_MAGIC:
        reader.pos = 16
        version = reader.u8()
    else:
        version = 0
    strings: list[tuple[str, int]] = []
    if version >= 2:
        offset = reader.i64()
        if offset != -1:
            saved = reader.pos
            reader.pos = offset
            for _ in range(reader.i32()):
                text = reader.fstring()
                refs = reader.i32() if version >= 3 else -1
                strings.append((text, refs))
            reader.pos = saved
    if version >= 3:
        reader.pos += 4  # total entry count
    namespaces: dict[str, dict[str, str]] = {}
    for _ in range(reader.u32()):
        if version >= 3:
            reader.u32()  # namespace hash
        namespace = reader.fstring()
        entries: dict[str, str] = {}
        for _ in range(reader.u32()):
            if version >= 3:
                reader.u32()  # key hash
            key = reader.fstring()
            reader.u32()  # source string hash
            index = reader.i32()
            if 0 <= index < len(strings):
                entries[key] = strings[index][0]
            if version > 3:
                reader.pos += 4  # Stellar Blade extra field
        namespaces[namespace] = entries
    return namespaces


def flatten(namespaces: dict[str, dict[str, str]]) -> dict[str, str]:
    flat: dict[str, str] = {}
    for entries in namespaces.values():
        for key, text in entries.items():
            flat.setdefault(key.lower(), text)
    return flat


def load_rows(path: Path) -> dict[str, dict]:
    payload = json.loads(path.read_text(encoding="utf-8-sig"))
    if isinstance(payload, list):
        for export in payload:
            if export.get("Type") == "DataTable" and export.get("Rows"):
                return export["Rows"]
    raise SystemExit(f"{path} is not a CUE4Parse DataTable export")


def export_with_cue4parse(game: Path, tools: Path, dump: Path, mappings: Path) -> None:
    cue4parse = tools / "cue4parse.exe"
    if not cue4parse.exists():
        raise SystemExit(f"cue4parse.exe not found in {tools}")
    args = [
        str(cue4parse),
        "-i",
        str(game),
        "-g",
        "GAME_StellarBlade",
        "-f",
        "json",
        "-o",
        str(dump),
        "-y",
        "-m",
        str(mappings),
    ]
    for pattern in TABLE_PACKAGES:
        args += ["-p", pattern]
    subprocess.run(args, check=True)


def unpack_locres(game: Path, tools: Path, dump: Path) -> None:
    repak = tools / "repak.exe"
    if not repak.exists():
        raise SystemExit(f"repak.exe not found in {tools}")
    pak = game / "SB" / "Content" / "Paks" / "pakchunk0-WindowsNoEditor.pak"
    args = [str(repak), "unpack", str(pak), "-o", str(dump), "-q"]
    for culture in LOCRES_CULTURES.values():
        args += ["-i", LOCRES_PATH.format(culture=culture)]
    subprocess.run(args, check=True)


def find_locres(dump: Path, culture: str) -> Path:
    direct = dump / f"Game.{culture}.locres"
    if direct.exists():
        return direct
    matches = sorted(dump.rglob(f"*{culture}*/Game.locres"))
    if not matches:
        raise SystemExit(f"Game.locres for {culture} not found under {dump}")
    return matches[0]


def build_items(item_rows: dict[str, dict], loc: dict[str, dict[str, str]], universe: dict) -> dict[str, dict]:
    zh = flatten(loc["zh"])
    en = flatten(loc["en"])
    names: dict[str, dict] = {}
    for alias, row in item_rows.items():
        key = (row.get("Name") or "").strip()
        if not key:
            continue
        text = zh.get(key.lower())
        if not text:
            continue
        names[alias] = {"zh": text, "en": en.get(key.lower()) or ""}
    for raw in universe.get("records", []):
        alias = _SUFFIX_RE.sub("", raw)
        title_key = alias.replace("Item_", "") + "_Title"
        text = zh.get(title_key.lower())
        if not text:
            continue
        names[alias] = {"zh": text, "en": en.get(title_key.lower()) or ""}
    return names


def build_camps(camp_rows: dict[str, dict], loc: dict[str, dict[str, str]]) -> dict[str, dict]:
    zh = flatten(loc["zh"])
    en = flatten(loc["en"])
    camps: dict[str, dict] = {}
    for row in camp_rows.values():
        name_key = (row.get("CampName") or "").strip()
        text = zh.get(name_key.lower())
        if not name_key or not text:
            continue
        entry = {"zh": text, "en": en.get(name_key.lower()) or ""}
        for value in row.values():
            if not isinstance(value, str) or "EnvS" not in value:
                continue
            match = _ENV_RE.match(value.replace("ChangeState_ZoneEnv_", "").removesuffix("_Camp"))
            if not match:
                continue
            alias = f"ChangeState_ZoneEnv_{match.group(1)}_{match.group(2)}_EnvS_{match.group(3)}_Camp"
            camps.setdefault(alias, entry)
    return camps


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dump", type=Path, help="directory with CUE4Parse JSON exports and Game.locres files")
    parser.add_argument("--game", type=Path, help="Stellar Blade install directory (enables auto extraction)")
    parser.add_argument("--tools", type=Path, help="directory with cue4parse.exe, repak.exe and the mappings file")
    parser.add_argument("--mappings", type=Path, help="UAssetGUI/CUE4Parse .usmap file")
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT, help=f"output path (default: {DEFAULT_OUT})")
    args = parser.parse_args()

    if not args.dump and not args.game:
        parser.error("pass --dump or --game")
    dump = args.dump or Path("data/raw/game/dump")
    if args.game:
        tools = args.tools or dump
        mappings = args.mappings or tools / "Mappings.usmap"
        print(f"extracting game tables from {args.game} with {tools} ...", file=sys.stderr)
        export_with_cue4parse(args.game, tools, dump, mappings)
        unpack_locres(args.game, tools, dump)

    loc = {lang: read_locres(find_locres(dump, culture)) for lang, culture in LOCRES_CULTURES.items()}
    item_table = next(dump.rglob("ItemTable.json"), None)
    camp_table = next(dump.rglob("ZoneCampTable.json"), None)
    if item_table is None or camp_table is None:
        raise SystemExit("ItemTable.json or ZoneCampTable.json not found in dump")
    universe = json.loads(UNIVERSE.read_text(encoding="utf-8"))
    items = build_items(load_rows(item_table), loc, universe)
    camps = build_camps(load_rows(camp_table), loc)
    payload = {
        "version": 1,
        "generated_by": "tools/mine_game_names.py",
        "source": "Stellar Blade game tables (ItemTable, ZoneCampTable) + zh-Hans/en Game.locres",
        "items": dict(sorted(items.items())),
        "camps": dict(sorted(camps.items())),
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(payload, ensure_ascii=False, indent=1), encoding="utf-8")
    try:
        shown = args.out.relative_to(ROOT)
    except ValueError:
        shown = args.out
    print(f"wrote {shown}: {len(items)} item names, {len(camps)} camps", file=sys.stderr)


if __name__ == "__main__":
    main()
