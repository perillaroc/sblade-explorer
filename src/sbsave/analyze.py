"""Compare the catalog against a save to find what is still missing."""

from __future__ import annotations

from dataclasses import dataclass, field

from .catalog import Catalog, CatalogItem, Category
from .savegame import SaveData


@dataclass
class ItemStatus:
    item: CatalogItem
    obtained: bool
    reason: str | None = None

    @property
    def flags(self) -> list[str]:
        flags: list[str] = []
        if self.item.dlc:
            flags.append(self.item.dlc_label or "DLC")
        if self.item.ng_plus:
            flags.append(self.item.ng_plus_label)
        if self.item.missable:
            flags.append("可错过")
        if self.item.confidence != "high":
            flags.append("映射待确认")
        return flags


@dataclass
class CategoryResult:
    category: Category
    total: int
    obtained_count: int
    missing: list[ItemStatus] = field(default_factory=list)
    obtained_items: list[CatalogItem] = field(default_factory=list)
    extra_obtained: list[str] = field(default_factory=list)
    blocked_count: int = 0
    statuses: list[ItemStatus] = field(default_factory=list)

    @property
    def percent(self) -> float:
        return (self.obtained_count / self.total * 100) if self.total else 0.0


@dataclass
class Analysis:
    save: SaveData
    categories: list[CategoryResult]
    unmapped_obtained: list[str] = field(default_factory=list)
    catalog_total: int = 0
    catalog_obtained: int = 0

    @property
    def missing_total(self) -> int:
        return sum(len(cat.missing) for cat in self.categories)

    @property
    def percent(self) -> float:
        return (self.catalog_obtained / self.catalog_total * 100) if self.catalog_total else 0.0


def _blocked_reason(item: CatalogItem, save: SaveData) -> str | None:
    if item.ng_plus > save.ng_plus_count:
        return f"需要{item.ng_plus_label}"
    if item.dlc:
        return f"{item.dlc_label}限定"
    if item.missable:
        return "可错过(注意节点)"
    return None


def _prefix(alias: str) -> str:
    if "_" in alias:
        head = alias.split("_", 1)[0]
        if head and not head.isdigit():
            return head
    return alias


def analyze(save: SaveData, catalog: Catalog, categories: list[str] | None = None) -> Analysis:
    obtained = save.all_obtained
    alias_index = catalog.alias_index()

    results: list[CategoryResult] = []
    for category in catalog.category_list():
        if categories and category.key not in categories:
            continue
        items = catalog.by_category(category.key)
        if not items:
            continue
        missing: list[ItemStatus] = []
        obtained_items: list[CatalogItem] = []
        statuses: list[ItemStatus] = []
        blocked = 0
        for item in items:
            is_obtained = any(alias in obtained for alias in item.satisfy_aliases)
            status = ItemStatus(item=item, obtained=is_obtained)
            statuses.append(status)
            if is_obtained:
                obtained_items.append(item)
            else:
                status.reason = _blocked_reason(item, save)
                if status.reason:
                    blocked += 1
                missing.append(status)
        extra = sorted(
            alias
            for alias in obtained
            if alias_index.get(alias) is None and _category_matches_alias(category, alias)
        )
        results.append(
            CategoryResult(
                category=category,
                total=len(items),
                obtained_count=len(obtained_items),
                missing=missing,
                obtained_items=obtained_items,
                extra_obtained=extra,
                blocked_count=blocked,
                statuses=statuses,
            )
        )

    unmapped = sorted(alias for alias in obtained if alias not in alias_index)
    return Analysis(
        save=save,
        categories=results,
        unmapped_obtained=unmapped,
        catalog_total=sum(cat.total for cat in results),
        catalog_obtained=sum(cat.obtained_count for cat in results),
    )


_CATEGORY_PREFIXES: dict[str, tuple[str, ...]] = {
    "nano_suits": ("BS_", "Nanosuit", "NanoSuit"),
    "cans": ("Can_",),
    "records": ("Item_Records_",),
    "passcodes": ("Item_Records_",),
    "camps": ("ChangeState_ZoneEnv_",),
    "hair": ("Hair_",),
    "glasses": ("FaceAccessory_",),
    "earrings": ("Earring_",),
    "drone_seals": ("DroneSeal_",),
    "design_patterns": ("DesignPattern_",),
    "adam_costumes": ("AdamCostume_",),
    "lily_costumes": ("LilyCostume_",),
    "fish": ("Fish_",),
    "gear": ("Gear_",),
}


def _category_matches_alias(category: Category, alias: str) -> bool:
    prefixes = _CATEGORY_PREFIXES.get(category.key)
    if prefixes is None:
        return False
    return alias.startswith(prefixes)


def unmapped_summary(aliases: list[str]) -> list[tuple[str, int]]:
    counts: dict[str, int] = {}
    for alias in aliases:
        counts[_prefix(alias)] = counts.get(_prefix(alias), 0) + 1
    return sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))
