"""UE4 GVAS save file reader tailored to Stellar Blade (SB).

Full-game saves are an 8 byte ``EVAS`` wrapper followed by a UE4.26 GVAS
archive; demo and settings saves are plain GVAS.  This module only reads.

Layout notes (verified against retail v1.4.x saves):

* FString: ``int32`` length, includes the terminating NUL.  Negative lengths
  are UTF-16LE.
* Property tag: name (FString), type (FString), size (``uint64``), type
  specific fields, a single byte "has property guid" flag, then the value.
  Type specific fields ordered as they appear:

  - ``StructProperty``: struct name (FString) + 16 byte struct guid
  - ``ArrayProperty`` / ``SetProperty``: inner type (FString)
  - ``MapProperty``: key type (FString) + value type (FString)
  - ``ByteProperty``: enum name (FString)
  - ``EnumProperty``: enum name (FString) + inner type (FString)

* ``BoolProperty`` stores its value in the tag (1 byte) and has size 0.
* Arrays of structs are ``int32 count`` followed by a single property tag
  header and then ``count`` tagged struct bodies back to back.
* Sets and maps are prefixed with an empty FString before the element count.
* Any struct/map/array that cannot be parsed as tagged data falls back to raw
  bytes, and the reader always resynchronises on ``tag_start + size``.
"""

from __future__ import annotations

import struct
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

EVAS_MAGIC = b"EVAS"
GVAS_MAGIC = b"GVAS"

SIMPLE_FIXED: dict[str, tuple[str, int]] = {
    "Int8Property": ("b", 1),
    "Int16Property": ("h", 2),
    "IntProperty": ("i", 4),
    "Int64Property": ("q", 8),
    "UInt16Property": ("H", 2),
    "UInt32Property": ("I", 4),
    "UInt64Property": ("Q", 8),
    "FloatProperty": ("f", 4),
    "DoubleProperty": ("d", 8),
}

STRING_TYPES = {"StrProperty", "NameProperty", "EnumProperty"}

CONTAINER_TYPES = {
    "StructProperty",
    "ArrayProperty",
    "SetProperty",
    "MapProperty",
}

TAG_TYPES = (
    set(SIMPLE_FIXED)
    | STRING_TYPES
    | CONTAINER_TYPES
    | {"BoolProperty", "ByteProperty", "TextProperty"}
)


class GvasError(Exception):
    """Raised when a save file cannot be parsed."""


class RawValue:
    """Unparsed bytes (unknown struct layout or unsupported type)."""

    __slots__ = ("data",)

    def __init__(self, data: bytes) -> None:
        self.data = data

    def __repr__(self) -> str:  # pragma: no cover - debug helper
        return f"RawValue({len(self.data)} bytes)"


@dataclass
class StructValue:
    fields: dict[str, Property] = field(default_factory=dict)

    def get(self, name: str) -> Property | None:
        return self.fields.get(name)

    def __contains__(self, name: str) -> bool:
        return name in self.fields


@dataclass
class ArrayValue:
    elements: list[Any] = field(default_factory=list)


@dataclass
class SetValue:
    elements: list[Any] = field(default_factory=list)


@dataclass
class MapValue:
    entries: list[tuple[Any, Any]] = field(default_factory=list)

    def get(self, key: Any, default: Any = None) -> Any:
        for k, v in self.entries:
            if k == key:
                return v
        return default

    def keys(self) -> list[Any]:
        return [k for k, _ in self.entries]


@dataclass
class Property:
    name: str
    type: str
    value: Any
    size: int = 0
    struct_name: str | None = None
    enum_name: str | None = None
    inner_type: str | None = None
    key_type: str | None = None
    value_type: str | None = None

    def as_struct(self) -> StructValue:
        if not isinstance(self.value, StructValue):
            raise GvasError(f"property {self.name!r} is not a parsed struct")
        return self.value

    def field(self, name: str) -> Property | None:
        if isinstance(self.value, StructValue):
            return self.value.fields.get(name)
        return None


@dataclass
class EngineVersion:
    major: int
    minor: int
    patch: int
    changelist: int
    branch: str

    def __str__(self) -> str:
        return f"{self.major}.{self.minor}.{self.patch}"


@dataclass
class GvasHeader:
    save_game_version: int
    package_file_version: int
    engine: EngineVersion
    custom_version_format: int
    custom_versions: list[tuple[str, int]]
    save_game_class_name: str
    has_evas_prefix: bool = False


@dataclass
class GvasFile:
    header: GvasHeader
    properties: dict[str, Property]
    footer: bytes = b""

    def get(self, name: str) -> Property | None:
        return self.properties.get(name)


class _Reader:
    __slots__ = ("data", "pos")

    def __init__(self, data: bytes, pos: int = 0) -> None:
        self.data = data
        self.pos = pos

    def remaining(self) -> int:
        return len(self.data) - self.pos

    def ensure(self, count: int) -> None:
        if count < 0 or self.pos + count > len(self.data):
            raise GvasError(f"unexpected end of file at offset {self.pos} (need {count} bytes)")

    def read(self, count: int) -> bytes:
        self.ensure(count)
        value = self.data[self.pos : self.pos + count]
        self.pos += count
        return value

    def u8(self) -> int:
        return self.read(1)[0]

    def i16(self) -> int:
        return struct.unpack("<h", self.read(2))[0]

    def u16(self) -> int:
        return struct.unpack("<H", self.read(2))[0]

    def i32(self) -> int:
        return struct.unpack("<i", self.read(4))[0]

    def u32(self) -> int:
        return struct.unpack("<I", self.read(4))[0]

    def i64(self) -> int:
        return struct.unpack("<q", self.read(8))[0]

    def u64(self) -> int:
        return struct.unpack("<Q", self.read(8))[0]

    def fstring(self) -> str:
        length = self.i32()
        if length == 0:
            return ""
        if length > 0:
            if length > self.remaining():
                raise GvasError(f"invalid string length {length} at offset {self.pos}")
            raw = self.read(length)
            if raw.endswith(b"\x00"):
                raw = raw[:-1]
            return raw.decode("utf-8", "replace")
        length = -length
        if 2 * length > self.remaining():
            raise GvasError(f"invalid wide string length {length} at offset {self.pos}")
        raw = self.read(2 * length)
        if raw.endswith(b"\x00\x00"):
            raw = raw[:-2]
        return raw.decode("utf-16-le", "replace")


def _is_reasonable_name(name: str) -> bool:
    if not name or len(name) > 512:
        return False
    return all(32 <= ord(ch) <= 126 for ch in name)


def _read_scalar(reader: _Reader, type_name: str) -> Any:
    if type_name in SIMPLE_FIXED:
        fmt, _size = SIMPLE_FIXED[type_name]
        return struct.unpack("<" + fmt, reader.read(_size))[0]
    if type_name in STRING_TYPES:
        return reader.fstring()
    if type_name == "BoolProperty":
        return reader.u8() != 0
    if type_name == "ByteProperty":
        return reader.u8()
    raise GvasError(f"unsupported scalar type {type_name!r}")


@dataclass
class _TagHeader:
    name: str
    type: str
    size: int
    struct_name: str | None = None
    enum_name: str | None = None
    inner_type: str | None = None
    key_type: str | None = None
    value_type: str | None = None
    value_start: int = 0


def _read_tag_header(reader: _Reader) -> _TagHeader | None:
    name = reader.fstring()
    if name in ("None", ""):
        return None
    type_name = reader.fstring()
    size = reader.u64()

    struct_name: str | None = None
    enum_name: str | None = None
    inner_type: str | None = None
    key_type: str | None = None
    value_type: str | None = None

    if type_name == "StructProperty":
        struct_name = reader.fstring()
        reader.read(16)  # struct guid
    elif type_name in ("ArrayProperty", "SetProperty"):
        inner_type = reader.fstring()
    elif type_name == "MapProperty":
        key_type = reader.fstring()
        value_type = reader.fstring()
    elif type_name == "ByteProperty":
        enum_name = reader.fstring()
    elif type_name == "EnumProperty":
        enum_name = reader.fstring()
        inner_type = reader.fstring()

    reader.u8()  # has property guid
    return _TagHeader(
        name=name,
        type=type_name,
        size=size,
        struct_name=struct_name,
        enum_name=enum_name,
        inner_type=inner_type,
        key_type=key_type,
        value_type=value_type,
        value_start=reader.pos,
    )


def _read_struct_body(reader: _Reader) -> StructValue:
    fields: dict[str, Property] = {}
    while True:
        prop = _read_property(reader)
        if prop is None:
            return StructValue(fields)
        fields[prop.name] = prop


def _try_struct(reader: _Reader, size: int) -> Any:
    start = reader.pos
    try:
        body = _read_struct_body(reader)
        if reader.pos != start + size:
            raise GvasError("struct body size mismatch")
        return body
    except GvasError:
        reader.pos = start
        return RawValue(reader.read(size))


def _try_array(reader: _Reader, size: int, inner_type: str | None) -> Any:
    start = reader.pos
    try:
        count = reader.i32()
        if count < 0 or count > 5_000_000:
            raise GvasError(f"bad array count {count}")
        if inner_type == "StructProperty":
            header = _read_tag_header(reader)
            if header is None or header.type != "StructProperty":
                raise GvasError("missing array element tag")
            elements: list[Any] = []
            for _ in range(count):
                element_start = reader.pos
                body = _read_struct_body(reader)
                elements.append(body)
                if reader.pos == element_start:
                    raise GvasError("empty array element")
            value: Any = ArrayValue(elements)
        else:
            elements = [_read_scalar(reader, inner_type or "") for _ in range(count)]
            value = ArrayValue(elements)
        if reader.pos != start + size:
            raise GvasError("array body size mismatch")
        return value
    except GvasError:
        reader.pos = start
        return RawValue(reader.read(size))


def _try_set(reader: _Reader, size: int, inner_type: str | None) -> Any:
    start = reader.pos

    def parse_elements() -> list[Any]:
        count = reader.i32()
        if count < 0 or count > 5_000_000:
            raise GvasError(f"bad set count {count}")
        if inner_type == "StructProperty":
            elements = []
            for _ in range(count):
                element_start = reader.pos
                body = _read_struct_body(reader)
                elements.append(body)
                if reader.pos == element_start:
                    raise GvasError("empty set element")
            return elements
        return [_read_scalar(reader, inner_type or "") for _ in range(count)]

    try:
        prefix = reader.i32()
        elements: list[Any]
        if prefix == 0:
            elements = parse_elements()
        else:
            reader.pos = start
            elements = parse_elements()
        if reader.pos != start + size:
            raise GvasError("set body size mismatch")
        return SetValue(elements)
    except GvasError:
        reader.pos = start
        return RawValue(reader.read(size))


def _read_value_for_type(
    reader: _Reader,
    type_name: str,
    struct_name: str | None = None,
    inner_type: str | None = None,
    key_type: str | None = None,
    value_type: str | None = None,
) -> Any:
    if type_name in SIMPLE_FIXED or type_name in ("StrProperty", "NameProperty", "EnumProperty"):
        return _read_scalar(reader, type_name)
    if type_name == "BoolProperty":
        return reader.u8() != 0
    if type_name == "ByteProperty":
        return reader.u8()
    if type_name == "StructProperty":
        body = _read_struct_body(reader)
        return body
    if type_name == "ArrayProperty":
        count = reader.i32()
        if count < 0 or count > 5_000_000:
            raise GvasError(f"bad array count {count}")
        if inner_type == "StructProperty":
            header = _read_tag_header(reader)
            if header is None or header.type != "StructProperty":
                raise GvasError("missing array element tag")
            return ArrayValue([_read_struct_body(reader) for _ in range(count)])
        return ArrayValue([_read_scalar(reader, inner_type or "") for _ in range(count)])
    if type_name == "SetProperty":
        count = reader.i32()
        if count < 0 or count > 5_000_000:
            raise GvasError(f"bad set count {count}")
        if inner_type == "StructProperty":
            return SetValue([_read_struct_body(reader) for _ in range(count)])
        return SetValue([_read_scalar(reader, inner_type or "") for _ in range(count)])
    if type_name == "MapProperty":
        count = reader.i32()
        if count < 0 or count > 5_000_000:
            raise GvasError(f"bad map count {count}")
        entries: list[tuple[Any, Any]] = []
        for _ in range(count):
            key = _read_value_for_type(reader, key_type or "NameProperty")
            value = _read_value_for_type(reader, value_type or "StrProperty")
            entries.append((key, value))
        return MapValue(entries)
    raise GvasError(f"unsupported value type {type_name!r}")


def _try_map(reader: _Reader, size: int, key_type: str | None, value_type: str | None) -> Any:
    start = reader.pos
    try:
        prefix = reader.i32()
        if prefix != 0:
            reader.pos = start
        value = _read_value_for_type(reader, "MapProperty", key_type=key_type, value_type=value_type)
        if reader.pos != start + size:
            raise GvasError("map body size mismatch")
        return value
    except GvasError:
        reader.pos = start
        return RawValue(reader.read(size))


def _read_property(reader: _Reader) -> Property | None:
    header = _read_tag_header(reader)
    if header is None:
        return None

    type_name = header.type
    value_start = header.value_start
    size = header.size

    if type_name == "BoolProperty":
        value: Any = reader.u8() != 0
        size = reader.pos - value_start
    elif type_name == "StructProperty":
        value = _try_struct(reader, size)
    elif type_name == "ArrayProperty":
        value = _try_array(reader, size, header.inner_type)
    elif type_name == "SetProperty":
        value = _try_set(reader, size, header.inner_type)
    elif type_name == "MapProperty":
        value = _try_map(reader, size, header.key_type, header.value_type)
    elif type_name == "ByteProperty" and size != 1:
        value = reader.fstring()
    elif type_name == "TextProperty":
        value = RawValue(reader.read(size))
    elif type_name in TAG_TYPES:
        value = _read_scalar(reader, type_name)
    else:
        value = RawValue(reader.read(size))

    reader.pos = value_start + size
    return Property(
        name=header.name,
        type=type_name,
        value=value,
        size=size,
        struct_name=header.struct_name,
        enum_name=header.enum_name,
        inner_type=header.inner_type,
        key_type=header.key_type,
        value_type=header.value_type,
    )


def read_gvas_bytes(data: bytes) -> GvasFile:
    """Parse a GVAS archive (optionally wrapped in the EVAS container)."""
    has_evas = data[:4] == EVAS_MAGIC
    base = 8 if has_evas else 0
    if data[base : base + 4] != GVAS_MAGIC:
        raise GvasError("file does not start with a GVAS archive")

    reader = _Reader(data, base + 4)
    save_game_version = reader.u32()
    package_file_version = reader.u32()
    if save_game_version >= 3:  # UE5 archives carry a second package version
        reader.u32()
    major = reader.u16()
    minor = reader.u16()
    patch = reader.u16()
    changelist = reader.u32()
    branch = reader.fstring()
    custom_version_format = reader.u32()
    custom_version_count = reader.u32()
    custom_versions: list[tuple[str, int]] = []
    for _ in range(custom_version_count):
        guid = reader.read(16)
        version = reader.i32()
        custom_versions.append((guid.hex(), version))
    save_game_class_name = reader.fstring()

    properties: dict[str, Property] = {}
    while True:
        prop = _read_property(reader)
        if prop is None:
            break
        properties[prop.name] = prop

    footer = reader.read(reader.remaining())
    header = GvasHeader(
        save_game_version=save_game_version,
        package_file_version=package_file_version,
        engine=EngineVersion(major, minor, patch, changelist, branch),
        custom_version_format=custom_version_format,
        custom_versions=custom_versions,
        save_game_class_name=save_game_class_name,
        has_evas_prefix=has_evas,
    )
    return GvasFile(header=header, properties=properties, footer=footer)


def load_gvas(path: str | Path) -> GvasFile:
    raw = Path(path).read_bytes()
    return read_gvas_bytes(raw)


def to_jsonable(value: Any, max_raw: int = 64) -> Any:
    """Convert parsed values into JSON friendly structures."""
    if isinstance(value, RawValue):
        head = value.data[:max_raw].hex()
        suffix = "" if len(value.data) <= max_raw else f"...(+{len(value.data) - max_raw} bytes)"
        return f"<raw:{head}{suffix}>"
    if isinstance(value, StructValue):
        return {name: to_jsonable(prop, max_raw) for name, prop in value.fields.items()}
    if isinstance(value, ArrayValue) or isinstance(value, SetValue):
        return [to_jsonable(item, max_raw) for item in value.elements]
    if isinstance(value, MapValue):
        return [[to_jsonable(k, max_raw), to_jsonable(v, max_raw)] for k, v in value.entries]
    if isinstance(value, Property):
        return to_jsonable(value.value, max_raw)
    return value
