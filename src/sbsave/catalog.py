"""Collectible catalog: bundled data + user overrides."""

from __future__ import annotations

import json
import os
from collections.abc import Iterable
from dataclasses import dataclass, field
from importlib import resources
from pathlib import Path
from typing import Any

CATALOG_RESOURCE = "catalog.json"

NG_PLUS_LABELS = {0: "首周目", 1: "二周目(NG+)", 2: "三周目(NG++)"}

DLC_LABELS = {
    "nier": "尼尔 DLC",
    "nikke": "NIKKE DLC",
    "deluxe": "豪华版",
    "preorder": "预购特典",
    "summer": "夏日更新",
}


@dataclass
class Category:
    key: str
    name: str
    order: int = 100
    aliases: list[str] = field(default_factory=list)


@dataclass
class CatalogItem:
    id: str
    name: str
    category: str
    aliases: list[str] = field(default_factory=list)
    area: str | None = None
    location: str | None = None
    obtain: str | None = None
    ng_plus: int = 0
    dlc: str | None = None
    missable: bool = False
    note: str | None = None
    confidence: str = "high"
    source: str | None = None
    name_en: str | None = None

    @property
    def satisfy_aliases(self) -> list[str]:
        return self.aliases or [self.id]

    @property
    def ng_plus_label(self) -> str:
        return NG_PLUS_LABELS.get(self.ng_plus, f"NG+{self.ng_plus}")

    @property
    def dlc_label(self) -> str | None:
        return DLC_LABELS.get(self.dlc or "", self.dlc)


@dataclass
class Catalog:
    version: int
    categories: dict[str, Category]
    items: list[CatalogItem]

    def by_category(self, key: str) -> list[CatalogItem]:
        return [item for item in self.items if item.category == key]

    def alias_index(self) -> dict[str, CatalogItem]:
        index: dict[str, CatalogItem] = {}
        for item in self.items:
            for alias in item.satisfy_aliases:
                index.setdefault(alias, item)
        return index

    def category_list(self) -> list[Category]:
        return sorted(self.categories.values(), key=lambda c: (c.order, c.name))


def _parse_item(raw: dict[str, Any]) -> CatalogItem:
    aliases = raw.get("aliases") or []
    if isinstance(aliases, str):
        aliases = [aliases]
    return CatalogItem(
        id=str(raw["id"]),
        name=str(raw.get("name") or raw["id"]),
        category=str(raw["category"]),
        aliases=[str(a) for a in aliases],
        area=raw.get("area"),
        location=raw.get("location"),
        obtain=raw.get("obtain"),
        ng_plus=int(raw.get("ng_plus") or 0),
        dlc=raw.get("dlc"),
        missable=bool(raw.get("missable")),
        note=raw.get("note"),
        confidence=str(raw.get("confidence") or "high"),
        source=raw.get("source"),
        name_en=raw.get("name_en"),
    )


def _parse_catalog(payload: dict[str, Any]) -> Catalog:
    categories: dict[str, Category] = {}
    for raw in payload.get("categories") or []:
        category = Category(
            key=str(raw["key"]),
            name=str(raw.get("name") or raw["key"]),
            order=int(raw.get("order") or 100),
        )
        categories[category.key] = category
    items = [_parse_item(raw) for raw in payload.get("items") or []]
    for item in items:
        categories.setdefault(item.category, Category(key=item.category, name=item.category))
    return Catalog(version=int(payload.get("version") or 1), categories=categories, items=items)


def _read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def user_catalog_path() -> Path:
    local = Path(os.environ.get("LOCALAPPDATA", Path.home() / "AppData" / "Local"))
    return local / "sbsave" / "catalog.user.json"


def load_catalog(extra_paths: Iterable[Path] | None = None) -> Catalog:
    """Load the bundled catalog and merge optional user override files."""
    bundled = json.loads(resources.files("sbsave.data").joinpath(CATALOG_RESOURCE).read_text(encoding="utf-8"))
    catalog = _parse_catalog(bundled)

    paths: list[Path] = []
    default_user = user_catalog_path()
    if default_user.is_file():
        paths.append(default_user)
    paths.extend(Path(p) for p in (extra_paths or []) if Path(p).is_file())

    for path in paths:
        _merge(catalog, _read_json(path))
    return catalog


def _merge(catalog: Catalog, payload: dict[str, Any]) -> None:
    for raw in payload.get("categories") or []:
        key = str(raw["key"])
        existing = catalog.categories.get(key)
        if existing is None:
            catalog.categories[key] = Category(
                key=key, name=str(raw.get("name") or key), order=int(raw.get("order") or 100)
            )
        elif raw.get("name"):
            existing.name = str(raw["name"])
    existing_ids = {item.id: item for item in catalog.items}
    for raw in payload.get("items") or []:
        item = _parse_item(raw)
        if item.id in existing_ids:
            index = catalog.items.index(existing_ids[item.id])
            catalog.items[index] = item
        else:
            catalog.items.append(item)
        catalog.categories.setdefault(item.category, Category(key=item.category, name=item.category))
