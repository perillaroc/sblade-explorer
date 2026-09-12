"""Catalog data integrity tests."""

from __future__ import annotations

from sbsave.catalog import load_catalog


def test_bundled_catalog_loads():
    catalog = load_catalog()
    assert catalog.version >= 1
    assert len(catalog.items) > 700
    for key in ("nano_suits", "cans", "records", "camps", "hair", "fish", "design_patterns"):
        assert key in catalog.categories, key
        assert catalog.by_category(key), key


def test_aliases_are_unique():
    catalog = load_catalog()
    seen: dict[str, str] = {}
    duplicates = []
    for item in catalog.items:
        for alias in item.satisfy_aliases:
            if alias in seen:
                duplicates.append((alias, seen[alias], item.id))
            seen[alias] = item.id
    assert not duplicates, duplicates[:10]


def test_key_alias_mappings():
    index = load_catalog().alias_index()
    assert index["BS_Raven"].name == "渡鸦装"
    assert index["BS_102"].name == "活肤紧身服"
    assert index["Can_007"].category == "cans"
    assert index["Fish_Salmon"].name == "鲑鱼"
    assert index["Earring_001"].name == "绯红泪珠"
    assert index["Hair_000"].name == "星球空降马尾"
    assert index["DesignPattern_BS_01"].category == "design_patterns"


def test_catalog_counts():
    catalog = load_catalog()
    assert len(catalog.by_category("nano_suits")) == 126
    assert len(catalog.by_category("cans")) == 49
    assert len(catalog.by_category("camps")) == 89


def test_ng_plus_metadata_present():
    catalog = load_catalog()
    ng_items = [item for item in catalog.items if item.ng_plus > 0]
    assert len(ng_items) > 30
    assert any(item.ng_plus == 2 for item in ng_items)
