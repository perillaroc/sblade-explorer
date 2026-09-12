"""Tests for the GVAS reader using synthetic archives."""

from __future__ import annotations

import struct

import pytest

from sbsave.gvas import (
    ArrayValue,
    GvasError,
    MapValue,
    Property,
    StructValue,
    read_gvas_bytes,
)


def _fstr(value: str) -> bytes:
    raw = value.encode("utf-8") + b"\x00"
    return struct.pack("<i", len(raw)) + raw


def _header() -> bytes:
    data = b"GVAS"
    data += struct.pack("<I", 2)  # save game version
    data += struct.pack("<I", 522)  # package file version
    data += struct.pack("<HHH", 4, 26, 2)  # engine version
    data += struct.pack("<I", 0)  # changelist
    data += _fstr("++UE4+Release-4.26")
    data += struct.pack("<I", 3)  # custom version format
    data += struct.pack("<I", 0)  # custom version count
    data += _fstr("/Script/SB.SBSaveGame")
    return data


def _tag(name: str, type_name: str, value: bytes, extras: bytes = b"") -> bytes:
    return _fstr(name) + _fstr(type_name) + struct.pack("<Q", len(value)) + extras + b"\x00" + value


def test_reads_int64_property():
    payload = _header() + _tag("TestInt", "IntProperty", struct.pack("<i", 42)) + _fstr("None") + b"\x00\x00\x00\x00"
    gvas = read_gvas_bytes(payload)
    prop = gvas.get("TestInt")
    assert prop is not None
    assert prop.type == "IntProperty"
    assert prop.value == 42
    assert gvas.header.save_game_class_name == "/Script/SB.SBSaveGame"


def test_reads_evas_wrapper():
    payload = b"EVAS\x01\x00\x00\x00" + _header() + _tag("Name", "StrProperty", _fstr("Eve")) + _fstr("None")
    gvas = read_gvas_bytes(payload)
    assert gvas.header.has_evas_prefix is True
    assert gvas.get("Name").value == "Eve"


def test_reads_bool_and_string():
    payload = (
        _header()
        + _tag("Flag", "BoolProperty", b"\x01")
        + _tag("Label", "StrProperty", _fstr("hello"))
        + _fstr("None")
    )
    gvas = read_gvas_bytes(payload)
    assert gvas.get("Flag").value is True
    assert gvas.get("Label").value == "hello"


def test_reads_struct_field():
    body = _tag("Inner", "IntProperty", struct.pack("<i", 7)) + _fstr("None")
    extras = _fstr("TestStruct") + b"\x00" * 16
    payload = _header() + _tag("Root", "StructProperty", body, extras) + _fstr("None")
    gvas = read_gvas_bytes(payload)
    root = gvas.get("Root")
    assert isinstance(root.value, StructValue)
    assert root.value.get("Inner").value == 7


def test_reads_struct_array():
    body0 = _tag("Alias", "NameProperty", _fstr("A")) + _fstr("None")
    body1 = _tag("Alias", "NameProperty", _fstr("B")) + _fstr("None")
    element_tag = (
        _fstr("Items")
        + _fstr("StructProperty")
        + struct.pack("<Q", len(body0) + len(body1))
        + _fstr("TestStruct")
        + b"\x00" * 16
        + b"\x00"
    )
    value = struct.pack("<i", 2) + element_tag + body0 + body1
    extras = _fstr("StructProperty")
    payload = _header() + _tag("Items", "ArrayProperty", value, extras) + _fstr("None")
    gvas = read_gvas_bytes(payload)
    array = gvas.get("Items").value
    assert isinstance(array, ArrayValue)
    assert [entry.get("Alias").value for entry in array.elements] == ["A", "B"]


def test_reads_name_map():
    entries = _fstr("Key") + _tag("Value", "IntProperty", struct.pack("<i", 5)) + _fstr("None")
    value = struct.pack("<i", 0) + struct.pack("<i", 1) + entries
    extras = _fstr("NameProperty") + _fstr("StructProperty")
    payload = _header() + _tag("DataMap", "MapProperty", value, extras) + _fstr("None")
    gvas = read_gvas_bytes(payload)
    mapping = gvas.get("DataMap").value
    assert isinstance(mapping, MapValue)
    assert mapping.get("Key").get("Value").value == 5


def test_text_property_falls_back_to_raw():
    payload = _header() + _tag("Rich", "TextProperty", b"\x00" * 4 + _fstr("x")) + _fstr("None")
    gvas = read_gvas_bytes(payload)
    prop = gvas.get("Rich")
    assert prop.type == "TextProperty"


def test_rejects_non_gvas():
    with pytest.raises(GvasError):
        read_gvas_bytes(b"NOPE" + b"\x00" * 32)


def test_property_dataclass_helpers():
    prop = Property(name="A", type="IntProperty", value=1)
    assert prop.field("A") is None
