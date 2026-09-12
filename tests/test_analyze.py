"""Tests for the catalog diff / NG+ logic."""

from __future__ import annotations

from sbsave.analyze import analyze
from sbsave.catalog import load_catalog


def test_missing_detection(make_save):
    save = make_save(obtained={"Can_001", "Can_002"})
    catalog = load_catalog()
    result = analyze(save, catalog)
    cans = next(cat for cat in result.categories if cat.category.key == "cans")
    assert cans.obtained_count == 2
    assert len(cans.missing) == 47
    assert all(status.reason is None for status in cans.missing)


def test_ng_plus_reason(make_save):
    catalog = load_catalog()
    ng_item = next(item for item in catalog.items if item.ng_plus == 1 and item.category == "nano_suits")
    save = make_save(ng_plus=0)
    result = analyze(save, catalog, categories=["nano_suits"])
    missing = {status.item.id: status for status in result.categories[0].missing}
    assert missing[ng_item.id].reason == "需要二周目(NG+)"

    save_ng = make_save(ng_plus=1)
    result_ng = analyze(save_ng, catalog, categories=["nano_suits"])
    missing_ng = {status.item.id: status for status in result_ng.categories[0].missing}
    assert missing_ng[ng_item.id].reason is None


def test_derived_aliases_count_as_obtained(make_save):
    camp = next(item for item in load_catalog().items if item.category == "camps")
    save = make_save(derived=set(camp.satisfy_aliases))
    result = analyze(save, load_catalog(), categories=["camps"])
    assert result.categories[0].obtained_count == 1


def test_extra_obtained_aliases(make_save):
    catalog = load_catalog()
    save = make_save(obtained={"Can_001", "Can_999"})
    result = analyze(save, catalog, categories=["cans"])
    assert result.categories[0].extra_obtained == ["Can_999"]


def test_unmapped_aliases(make_save):
    save = make_save(obtained={"Weird_Alias_123"})
    result = analyze(save, load_catalog(), categories=["cans"])
    assert result.unmapped_obtained == ["Weird_Alias_123"]
