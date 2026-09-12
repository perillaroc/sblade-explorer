"""High level access to a Stellar Blade save file."""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from pathlib import Path

from .gvas import (
    ArrayValue,
    GvasError,
    GvasFile,
    MapValue,
    Property,
    SetValue,
    StructValue,
    load_gvas,
)


class SaveError(Exception):
    """Raised when a save file does not look like a Stellar Blade save."""


@dataclass
class SaveSlot:
    path: Path
    steam_id: str | None
    slot: int
    mtime: float

    @property
    def label(self) -> str:
        sid = self.steam_id or "unknown"
        return f"StellarBladeSave{self.slot:02d} ({sid})"


@dataclass
class AchievementRecord:
    alias: str
    progress: int = 0
    completed: bool = False
    received_reward: bool = False
    raw: dict[str, object] = field(default_factory=dict)


@dataclass
class ShopEntry:
    alias: str
    fields: dict[str, object] = field(default_factory=dict)


@dataclass
class SaveData:
    path: Path
    steam_id: str | None
    slot: int
    gvas: GvasFile
    obtained_items: set[str] = field(default_factory=set)
    derived_aliases: set[str] = field(default_factory=set)
    counters: dict[str, int] = field(default_factory=dict)
    string_counters: dict[str, str] = field(default_factory=dict)
    achievements: dict[str, AchievementRecord] = field(default_factory=dict)
    shop_purchases: dict[str, ShopEntry] = field(default_factory=dict)
    friendships: dict[str, int] = field(default_factory=dict)

    # ---- convenience accessors -------------------------------------------------

    @property
    def all_obtained(self) -> set[str]:
        """Item aliases plus aliases derived from achievement progress."""
        return self.obtained_items | self.derived_aliases

    @property
    def ng_plus_count(self) -> int:
        return self.counters.get("NewGamePlusPlayCount", 0)

    @property
    def difficulty(self) -> int:
        return self.counters.get("GameDifficulty", 1)

    @property
    def play_time_seconds(self) -> int:
        return self.counters.get("PlayTime", 0)

    @property
    def save_version(self) -> int:
        return self.counters.get("Version", 1)

    @property
    def playthrough_label(self) -> str:
        if self.ng_plus_count <= 0:
            return "一周目"
        return f"NG+{self.ng_plus_count}"

    @property
    def play_time_label(self) -> str:
        seconds = self.play_time_seconds
        hours, remainder = divmod(seconds, 3600)
        minutes = remainder // 60
        return f"{hours}小时{minutes:02d}分"

    @property
    def difficulty_label(self) -> str:
        return {0: "简单", 1: "普通", 2: "困难"}.get(self.difficulty, f"未知({self.difficulty})")

    def has_item(self, alias: str) -> bool:
        return alias in self.all_obtained


_ACHIEVEMENT_PREFIXES = ("Ach_Album_Unlock_", "Acquire_Item_")
_ACHIEVEMENT_SUFFIXES = (
    "_NotMaintainNewGamePlus",
    "_NotMaintain",
    "_RealEnding",
    "_Maintain",
    "_Popup",
    "_Used",
)
_ITEM_ALIAS_PREFIXES = (
    "BS_",
    "Can_",
    "Hair_",
    "Earring_",
    "FaceAccessory_",
    "DroneSeal_",
    "DesignPattern_",
    "Fish_",
    "AdamCostume_",
    "LilyCostume_",
    "Gear_",
    "Item_Records_",
    "ChangeState_ZoneEnv_",
)


def derive_aliases(alias: str) -> set[str]:
    """Translate achievement aliases into the item aliases they imply.

    Camps are only tracked through ``ChangeState_ZoneEnv_..._Camp`` records,
    and records may be tracked either as items or as
    ``Ach_Album_Unlock_Item_Records_...`` achievements depending on the save,
    so both need normalising.
    """
    base = alias
    for prefix in _ACHIEVEMENT_PREFIXES:
        if base.startswith(prefix):
            base = base[len(prefix) :]
            break
    changed = True
    while changed:
        changed = False
        for suffix in _ACHIEVEMENT_SUFFIXES:
            if base.endswith(suffix):
                base = base[: -len(suffix)]
                changed = True
    if not base or not base.startswith(_ITEM_ALIAS_PREFIXES):
        return set()
    if base.startswith("ChangeState_ZoneEnv_") and not base.endswith("_Camp"):
        return set()
    return {base}

    def achieved(self, alias: str) -> bool:
        record = self.achievements.get(alias)
        return bool(record and (record.completed or record.progress > 0))


def _appdata_paths() -> list[Path]:
    home = Path(os.environ.get("USERPROFILE", Path.home()))
    local = Path(os.environ.get("LOCALAPPDATA", home / "AppData" / "Local"))
    return [
        local / "SB" / "Saved" / "SaveGames",
        home / "Documents" / "StellarBlade",
        local / "SB_Demo" / "Saved" / "SaveGames",
    ]


def _slot_from_name(name: str) -> int | None:
    stem = Path(name).stem
    for prefix in ("StellarBladeSaveDemo", "StellarBladeSave"):
        if stem.startswith(prefix):
            suffix = stem[len(prefix) :]
            if suffix.isdigit():
                return int(suffix)
    return None


def discover_saves(extra_dirs: list[Path] | None = None) -> list[SaveSlot]:
    """Find saves in the default Stellar Blade locations."""
    dirs = list(extra_dirs or []) + _appdata_paths()
    found: dict[Path, SaveSlot] = {}
    for directory in dirs:
        if not directory.is_dir():
            continue
        for path in directory.rglob("StellarBladeSave*.sav"):
            if "Backup" in path.parts:
                continue
            slot = _slot_from_name(path.name)
            if slot is None:
                continue
            steam_id = path.parent.name if path.parent.name.isdigit() else None
            try:
                mtime = path.stat().st_mtime
            except OSError:
                continue
            found[path] = SaveSlot(path=path, steam_id=steam_id, slot=slot, mtime=mtime)
    return sorted(found.values(), key=lambda s: s.mtime, reverse=True)


def _entry_struct(value: object) -> StructValue:
    if not isinstance(value, StructValue):
        raise SaveError(f"expected struct, got {type(value).__name__}")
    return value


def _require(prop: Property | None, name: str) -> Property:
    if prop is None:
        raise SaveError(f"save is missing expected data: {name}")
    return prop


def extract(path: Path, gvas: GvasFile, steam_id: str | None = None) -> SaveData:
    obtained: set[str] = set()
    counters: dict[str, int] = {}
    string_counters: dict[str, str] = {}
    achievements: dict[str, AchievementRecord] = {}
    purchases: dict[str, ShopEntry] = {}
    friendships: dict[str, int] = {}

    item_map = _require(gvas.get("DataMap_SBItem"), "DataMap_SBItem")
    if isinstance(item_map.value, MapValue):
        item_entry = item_map.value.get("Item")
        if item_entry is not None:
            item_struct = _entry_struct(item_entry)
            obtained_prop = item_struct.get("ItemOtaineSet")
            if obtained_prop is not None and isinstance(obtained_prop.value, SetValue):
                obtained = {str(alias) for alias in obtained_prop.value.elements}

    int_map = gvas.get("DataMap_int32")
    if int_map is not None and isinstance(int_map.value, MapValue):
        for key, value in int_map.value.entries:
            if isinstance(value, int):
                counters[str(key)] = value

    str_map = gvas.get("DataMap_FString")
    if str_map is not None and isinstance(str_map.value, MapValue):
        for key, value in str_map.value.entries:
            if isinstance(value, str):
                string_counters[str(key)] = value

    achievement_prop = gvas.get("SBAchievement")
    if achievement_prop is not None and isinstance(achievement_prop.value, StructValue):
        listing = achievement_prop.value.get("AchievementList")
        if listing is not None and isinstance(listing.value, ArrayValue):
            for element in listing.value.elements:
                if not isinstance(element, StructValue):
                    continue
                alias_prop = element.get("AchievementAlias")
                if alias_prop is None or not isinstance(alias_prop.value, str):
                    continue
                alias = alias_prop.value
                record = AchievementRecord(alias=alias)
                for field_name, prop in element.fields.items():
                    if field_name == "AchievementAlias":
                        continue
                    record.raw[field_name] = prop.value
                    if field_name == "ProgressValue" and isinstance(prop.value, int):
                        record.progress = prop.value
                    elif field_name == "bCompleted":
                        record.completed = bool(prop.value)
                    elif field_name == "bRecievedReward":
                        record.received_reward = bool(prop.value)
                achievements[alias] = record

    shop_prop = gvas.get("SBShopHistory")
    if shop_prop is not None and isinstance(shop_prop.value, StructValue):
        purchase_prop = shop_prop.value.get("Purchase")
        if purchase_prop is not None and isinstance(purchase_prop.value, MapValue):
            for key, value in purchase_prop.value.entries:
                fields: dict[str, object] = {}
                if isinstance(value, StructValue):
                    fields = {name: prop.value for name, prop in value.fields.items()}
                purchases[str(key)] = ShopEntry(alias=str(key), fields=fields)
        friendship_prop = shop_prop.value.get("FriendShip")
        if friendship_prop is not None and isinstance(friendship_prop.value, MapValue):
            for key, value in friendship_prop.value.entries:
                if isinstance(value, int):
                    friendships[str(key)] = value

    derived: set[str] = set()
    for alias in achievements:
        derived |= derive_aliases(alias)

    save = SaveData(
        path=path,
        steam_id=steam_id,
        slot=_slot_from_name(path.name) or 0,
        gvas=gvas,
        obtained_items=obtained,
        derived_aliases=derived,
        counters=counters,
        string_counters=string_counters,
        achievements=achievements,
        shop_purchases=purchases,
        friendships=friendships,
    )
    if not obtained and gvas.header.save_game_class_name != "/Script/SB.SBSaveGame":
        raise SaveError("unsupported save game class")
    return save


def load_save(path: str | Path, steam_id: str | None = None) -> SaveData:
    path = Path(path)
    try:
        gvas = load_gvas(path)
    except GvasError as exc:
        raise SaveError(str(exc)) from exc
    if gvas.header.save_game_class_name != "/Script/SB.SBSaveGame":
        raise SaveError(f"not a Stellar Blade save: {gvas.header.save_game_class_name}")
    if steam_id is None and path.parent.name.isdigit():
        steam_id = path.parent.name
    return extract(path, gvas, steam_id=steam_id)


def pick_default_save(saves: list[SaveSlot] | None = None) -> SaveSlot:
    saves = saves if saves is not None else discover_saves()
    if not saves:
        raise SaveError("no Stellar Blade saves found")
    # Prefer the main slot (00), then the most recently written.
    main = [s for s in saves if s.slot == 0]
    return (main or saves)[0]
