"""Shared fixtures for tests."""

from __future__ import annotations

from pathlib import Path

import pytest

from sbsave.gvas import EngineVersion, GvasFile, GvasHeader
from sbsave.savegame import SaveData


def _header() -> GvasHeader:
    return GvasHeader(
        save_game_version=2,
        package_file_version=522,
        engine=EngineVersion(4, 26, 2, 0, "++UE4+Release-4.26"),
        custom_version_format=3,
        custom_versions=[],
        save_game_class_name="/Script/SB.SBSaveGame",
    )


@pytest.fixture()
def make_save():
    def factory(
        obtained: set[str] | None = None,
        derived: set[str] | None = None,
        ng_plus: int = 0,
    ) -> SaveData:
        return SaveData(
            path=Path("StellarBladeSave00.sav"),
            steam_id="76561198000000000",
            slot=0,
            gvas=GvasFile(header=_header(), properties={}, footer=b""),
            obtained_items=set(obtained or set()),
            derived_aliases=set(derived or set()),
            counters={"NewGamePlusPlayCount": ng_plus},
        )

    return factory
