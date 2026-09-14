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


def test_game_data_names_applied():
    index = load_catalog().alias_index()
    assert index["Can_007"].name == "妙之跃"
    assert index["BS_Nikke_01"].name == "浪游剑客服"
    assert index["Hair_Nikke_01"].name == "月下美人"
    assert index["DesignPattern_BS_01"].name == "设计图：镂空服"
    assert index["Item_Records_DED10_Memory_09"].name == "《塑料之心：第3卷》"
    assert index["Item_Records_DED10_Memory_09"].confidence == "high"
    assert index["ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"].name == "隐秘之路"


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


def test_chinese_translations_cover_guide_text():
    catalog = load_catalog()
    with_obtain = [item for item in catalog.items if item.obtain]
    assert len(with_obtain) == 353
    missing = [item.id for item in with_obtain if not item.obtain_zh]
    assert not missing, missing[:10]
    index = catalog.alias_index()
    assert index["BS_09_2"].area_zh == "埃多斯7号"
    assert index["BS_09_2"].location_zh == "淹水商业区"
    assert "淹水商业区" in index["BS_09_2"].obtain_zh
    assert index["Hair_006"].obtain_zh.startswith("完成支线任务《第一位顾客》")
    construction = [item for item in catalog.items if item.location_zh == "施工区"]
    assert any("施工区东侧" in (item.obtain_zh or "") for item in construction)
