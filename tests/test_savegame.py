"""Integration tests against a real save when one is available on this machine."""

from __future__ import annotations

import pytest

from sbsave.savegame import SaveData, derive_aliases, discover_saves, load_save


@pytest.fixture(scope="module")
def save() -> SaveData:
    slots = discover_saves()
    if not slots:
        pytest.skip("no Stellar Blade save found on this machine")
    return load_save(slots[0].path, steam_id=slots[0].steam_id)


def test_save_parses(save: SaveData):
    assert save.gvas.header.save_game_class_name == "/Script/SB.SBSaveGame"
    assert save.gvas.header.engine.major == 4
    assert len(save.gvas.properties) >= 20


def test_obtained_items(save: SaveData):
    assert len(save.obtained_items) > 200
    assert all(isinstance(alias, str) and alias for alias in save.obtained_items)


def test_counters_and_achievements(save: SaveData):
    assert "NewGamePlusPlayCount" in save.counters
    assert save.play_time_seconds >= 0
    assert len(save.achievements) > 100


def test_derived_aliases_covers_camps(save: SaveData):
    camps = {alias for alias in save.derived_aliases if alias.endswith("_Camp")}
    assert camps, "camps should be derived from achievement records"
    assert camps <= save.all_obtained


def test_derive_aliases_rules():
    assert derive_aliases("Acquire_Item_BS_01") == {"BS_01"}
    assert derive_aliases("Ach_Album_Unlock_Item_Records_Xion_Memory_01_RealEnding") == {
        "Item_Records_Xion_Memory_01"
    }
    assert derive_aliases("ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp") == {
        "ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"
    }
    assert derive_aliases("Kill_Character_WLA_MiteA") == set()
    assert derive_aliases("Acquire_Item_Can_048_NotMaintain") == {"Can_048"}
