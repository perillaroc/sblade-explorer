use std::fmt;
use std::path::Path;

pub const EVAS_MAGIC: &[u8; 4] = b"EVAS";
pub const GVAS_MAGIC: &[u8; 4] = b"GVAS";

const MAX_ELEMENTS: i32 = 5_000_000;

#[derive(Debug)]
pub enum GvasError {
    Parse(String),
    Io(std::io::Error),
}

impl fmt::Display for GvasError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GvasError::Parse(message) => formatter.write_str(message),
            GvasError::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for GvasError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GvasError::Parse(_) => None,
            GvasError::Io(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for GvasError {
    fn from(error: std::io::Error) -> Self {
        GvasError::Io(error)
    }
}

fn parse_error<T>(message: impl Into<String>) -> Result<T, GvasError> {
    Err(GvasError::Parse(message.into()))
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawValue {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StructValue {
    pub fields: Vec<(String, Property)>,
}

impl StructValue {
    pub fn get(&self, name: &str) -> Option<&Property> {
        self.fields
            .iter()
            .find(|(field_name, _)| field_name == name)
            .map(|(_, property)| property)
    }

    pub fn insert(&mut self, property: Property) {
        let name = property.name.clone();
        if let Some(slot) = self
            .fields
            .iter_mut()
            .find(|(field_name, _)| *field_name == name)
        {
            slot.1 = property;
        } else {
            self.fields.push((name, property));
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArrayValue {
    pub elements: Vec<Value>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetValue {
    pub elements: Vec<Value>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapValue {
    pub entries: Vec<(Value, Value)>,
}

impl MapValue {
    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.entries
            .iter()
            .find(|(entry_key, _)| entry_key == key)
            .map(|(_, value)| value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Raw(RawValue),
    Struct(StructValue),
    Array(ArrayValue),
    Set(SetValue),
    Map(MapValue),
    Bool(bool),
    Byte(u8),
    Int8(i8),
    Int16(i16),
    Int(i32),
    Int64(i64),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float(f32),
    Double(f64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub prop_type: String,
    pub value: Value,
    pub size: u64,
    pub struct_name: Option<String>,
    pub enum_name: Option<String>,
    pub inner_type: Option<String>,
    pub key_type: Option<String>,
    pub value_type: Option<String>,
}

impl Default for Property {
    fn default() -> Self {
        Self {
            name: String::new(),
            prop_type: String::new(),
            value: Value::Byte(0),
            size: 0,
            struct_name: None,
            enum_name: None,
            inner_type: None,
            key_type: None,
            value_type: None,
        }
    }
}

impl Property {
    pub fn as_struct(&self) -> Result<&StructValue, GvasError> {
        match &self.value {
            Value::Struct(value) => Ok(value),
            _ => parse_error(format!("property {:?} is not a parsed struct", self.name)),
        }
    }

    pub fn field(&self, name: &str) -> Option<&Property> {
        match &self.value {
            Value::Struct(value) => value.get(name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngineVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub changelist: u32,
    pub branch: String,
}

impl fmt::Display for EngineVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GvasHeader {
    pub save_game_version: u32,
    pub package_file_version: u32,
    pub engine: EngineVersion,
    pub custom_version_format: u32,
    pub custom_versions: Vec<(String, i32)>,
    pub save_game_class_name: String,
    pub has_evas_prefix: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GvasFile {
    pub header: GvasHeader,
    pub properties: Vec<(String, Property)>,
    pub footer: Vec<u8>,
}

impl GvasFile {
    pub fn get(&self, name: &str) -> Option<&Property> {
        self.properties
            .iter()
            .find(|(property_name, _)| property_name == name)
            .map(|(_, property)| property)
    }
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn ensure(&self, count: usize) -> Result<(), GvasError> {
        let in_bounds = self
            .pos
            .checked_add(count)
            .is_some_and(|end| end <= self.data.len());
        if !in_bounds {
            return parse_error(format!(
                "unexpected end of file at offset {} (need {count} bytes)",
                self.pos
            ));
        }
        Ok(())
    }

    fn read(&mut self, count: usize) -> Result<&'a [u8], GvasError> {
        self.ensure(count)?;
        let value = &self.data[self.pos..self.pos + count];
        self.pos += count;
        Ok(value)
    }

    fn u8(&mut self) -> Result<u8, GvasError> {
        Ok(self.read(1)?[0])
    }

    fn i16(&mut self) -> Result<i16, GvasError> {
        Ok(i16::from_le_bytes(self.read(2)?.try_into().unwrap()))
    }

    fn u16(&mut self) -> Result<u16, GvasError> {
        Ok(u16::from_le_bytes(self.read(2)?.try_into().unwrap()))
    }

    fn i32(&mut self) -> Result<i32, GvasError> {
        Ok(i32::from_le_bytes(self.read(4)?.try_into().unwrap()))
    }

    fn u32(&mut self) -> Result<u32, GvasError> {
        Ok(u32::from_le_bytes(self.read(4)?.try_into().unwrap()))
    }

    fn i64(&mut self) -> Result<i64, GvasError> {
        Ok(i64::from_le_bytes(self.read(8)?.try_into().unwrap()))
    }

    fn u64(&mut self) -> Result<u64, GvasError> {
        Ok(u64::from_le_bytes(self.read(8)?.try_into().unwrap()))
    }

    fn f32(&mut self) -> Result<f32, GvasError> {
        Ok(f32::from_le_bytes(self.read(4)?.try_into().unwrap()))
    }

    fn f64(&mut self) -> Result<f64, GvasError> {
        Ok(f64::from_le_bytes(self.read(8)?.try_into().unwrap()))
    }

    fn fstring(&mut self) -> Result<String, GvasError> {
        let length = self.i32()?;
        if length == 0 {
            return Ok(String::new());
        }
        if length > 0 {
            let length = length as usize;
            if length > self.remaining() {
                return parse_error(format!(
                    "invalid string length {length} at offset {}",
                    self.pos
                ));
            }
            let mut raw = self.read(length)?;
            if raw.last() == Some(&0) {
                raw = &raw[..raw.len() - 1];
            }
            return Ok(String::from_utf8_lossy(raw).into_owned());
        }
        let length = length.unsigned_abs() as usize;
        if 2 * length > self.remaining() {
            return parse_error(format!(
                "invalid wide string length {length} at offset {}",
                self.pos
            ));
        }
        let raw = self.read(2 * length)?;
        let raw = if raw.ends_with(&[0, 0]) {
            &raw[..raw.len() - 2]
        } else {
            raw
        };
        let units: Vec<u16> = raw
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect();
        Ok(String::from_utf16_lossy(&units))
    }
}

fn read_scalar(reader: &mut Reader<'_>, type_name: &str) -> Result<Value, GvasError> {
    match type_name {
        "Int8Property" => Ok(Value::Int8(reader.u8()? as i8)),
        "Int16Property" => Ok(Value::Int16(reader.i16()?)),
        "IntProperty" => Ok(Value::Int(reader.i32()?)),
        "Int64Property" => Ok(Value::Int64(reader.i64()?)),
        "UInt16Property" => Ok(Value::UInt16(reader.u16()?)),
        "UInt32Property" => Ok(Value::UInt32(reader.u32()?)),
        "UInt64Property" => Ok(Value::UInt64(reader.u64()?)),
        "FloatProperty" => Ok(Value::Float(reader.f32()?)),
        "DoubleProperty" => Ok(Value::Double(reader.f64()?)),
        "StrProperty" | "NameProperty" | "EnumProperty" => Ok(Value::Str(reader.fstring()?)),
        "BoolProperty" => Ok(Value::Bool(reader.u8()? != 0)),
        "ByteProperty" => Ok(Value::Byte(reader.u8()?)),
        other => parse_error(format!("unsupported scalar type {other:?}")),
    }
}

fn is_tag_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "Int8Property"
            | "Int16Property"
            | "IntProperty"
            | "Int64Property"
            | "UInt16Property"
            | "UInt32Property"
            | "UInt64Property"
            | "FloatProperty"
            | "DoubleProperty"
            | "StrProperty"
            | "NameProperty"
            | "EnumProperty"
            | "BoolProperty"
            | "ByteProperty"
            | "TextProperty"
            | "StructProperty"
            | "ArrayProperty"
            | "SetProperty"
            | "MapProperty"
    )
}

struct TagHeader {
    name: String,
    type_name: String,
    size: u64,
    struct_name: Option<String>,
    enum_name: Option<String>,
    inner_type: Option<String>,
    key_type: Option<String>,
    value_type: Option<String>,
    value_start: usize,
}

fn read_tag_header(reader: &mut Reader<'_>) -> Result<Option<TagHeader>, GvasError> {
    let name = reader.fstring()?;
    if name.is_empty() || name == "None" {
        return Ok(None);
    }
    let type_name = reader.fstring()?;
    let size = reader.u64()?;

    let mut struct_name = None;
    let mut enum_name = None;
    let mut inner_type = None;
    let mut key_type = None;
    let mut value_type = None;

    match type_name.as_str() {
        "StructProperty" => {
            struct_name = Some(reader.fstring()?);
            reader.read(16)?;
        }
        "ArrayProperty" | "SetProperty" => {
            inner_type = Some(reader.fstring()?);
        }
        "MapProperty" => {
            key_type = Some(reader.fstring()?);
            value_type = Some(reader.fstring()?);
        }
        "ByteProperty" => {
            enum_name = Some(reader.fstring()?);
        }
        "EnumProperty" => {
            enum_name = Some(reader.fstring()?);
            inner_type = Some(reader.fstring()?);
        }
        _ => {}
    }

    reader.u8()?;
    Ok(Some(TagHeader {
        name,
        type_name,
        size,
        struct_name,
        enum_name,
        inner_type,
        key_type,
        value_type,
        value_start: reader.pos,
    }))
}

fn read_struct_body(reader: &mut Reader<'_>) -> Result<StructValue, GvasError> {
    let mut fields = StructValue::default();
    while let Some(property) = read_property(reader)? {
        fields.insert(property);
    }
    Ok(fields)
}

fn try_struct(reader: &mut Reader<'_>, size: u64) -> Result<Value, GvasError> {
    let start = reader.pos;
    let end = start.saturating_add(size as usize);
    if let Ok(body) = parse_struct_body(reader, end) {
        return Ok(Value::Struct(body));
    }
    reader.pos = start;
    Ok(Value::Raw(RawValue {
        data: reader.read(size as usize)?.to_vec(),
    }))
}

fn parse_struct_body(reader: &mut Reader<'_>, end: usize) -> Result<StructValue, GvasError> {
    let body = read_struct_body(reader)?;
    if reader.pos != end {
        return parse_error("struct body size mismatch");
    }
    Ok(body)
}

fn try_array(
    reader: &mut Reader<'_>,
    size: u64,
    inner_type: Option<&str>,
) -> Result<Value, GvasError> {
    let start = reader.pos;
    let end = start.saturating_add(size as usize);
    if let Ok(value) = parse_array_body(reader, end, inner_type) {
        return Ok(value);
    }
    reader.pos = start;
    Ok(Value::Raw(RawValue {
        data: reader.read(size as usize)?.to_vec(),
    }))
}

fn parse_array_body(
    reader: &mut Reader<'_>,
    end: usize,
    inner_type: Option<&str>,
) -> Result<Value, GvasError> {
    let count = reader.i32()?;
    if !(0..=MAX_ELEMENTS).contains(&count) {
        return parse_error(format!("bad array count {count}"));
    }
    let count = count as usize;
    let mut elements = Vec::with_capacity(count.min(1024));
    if inner_type == Some("StructProperty") {
        match read_tag_header(reader)? {
            Some(header) if header.type_name == "StructProperty" => {}
            _ => return parse_error("missing array element tag"),
        }
        for _ in 0..count {
            let element_start = reader.pos;
            let body = read_struct_body(reader)?;
            if reader.pos == element_start {
                return parse_error("empty array element");
            }
            elements.push(Value::Struct(body));
        }
    } else {
        let element_type = inner_type.unwrap_or("");
        for _ in 0..count {
            elements.push(read_scalar(reader, element_type)?);
        }
    }
    if reader.pos != end {
        return parse_error("array body size mismatch");
    }
    Ok(Value::Array(ArrayValue { elements }))
}

fn try_set(
    reader: &mut Reader<'_>,
    size: u64,
    inner_type: Option<&str>,
) -> Result<Value, GvasError> {
    let start = reader.pos;
    let end = start.saturating_add(size as usize);
    if let Ok(value) = parse_set_body(reader, start, end, inner_type) {
        return Ok(value);
    }
    reader.pos = start;
    Ok(Value::Raw(RawValue {
        data: reader.read(size as usize)?.to_vec(),
    }))
}

fn parse_set_body(
    reader: &mut Reader<'_>,
    start: usize,
    end: usize,
    inner_type: Option<&str>,
) -> Result<Value, GvasError> {
    let prefix = reader.i32()?;
    if prefix != 0 {
        reader.pos = start;
    }
    let count = reader.i32()?;
    if !(0..=MAX_ELEMENTS).contains(&count) {
        return parse_error(format!("bad set count {count}"));
    }
    let count = count as usize;
    let mut elements = Vec::with_capacity(count.min(1024));
    if inner_type == Some("StructProperty") {
        for _ in 0..count {
            let element_start = reader.pos;
            let body = read_struct_body(reader)?;
            if reader.pos == element_start {
                return parse_error("empty set element");
            }
            elements.push(Value::Struct(body));
        }
    } else {
        let element_type = inner_type.unwrap_or("");
        for _ in 0..count {
            elements.push(read_scalar(reader, element_type)?);
        }
    }
    if reader.pos != end {
        return parse_error("set body size mismatch");
    }
    Ok(Value::Set(SetValue { elements }))
}

fn try_map(
    reader: &mut Reader<'_>,
    size: u64,
    key_type: Option<&str>,
    value_type: Option<&str>,
) -> Result<Value, GvasError> {
    let start = reader.pos;
    let end = start.saturating_add(size as usize);
    if let Ok(value) = parse_map_body(reader, start, end, key_type, value_type) {
        return Ok(value);
    }
    reader.pos = start;
    Ok(Value::Raw(RawValue {
        data: reader.read(size as usize)?.to_vec(),
    }))
}

fn parse_map_body(
    reader: &mut Reader<'_>,
    start: usize,
    end: usize,
    key_type: Option<&str>,
    value_type: Option<&str>,
) -> Result<Value, GvasError> {
    let prefix = reader.i32()?;
    if prefix != 0 {
        reader.pos = start;
    }
    let value = read_value_for_type(reader, "MapProperty", None, key_type, value_type)?;
    if reader.pos != end {
        return parse_error("map body size mismatch");
    }
    Ok(value)
}

fn read_value_for_type(
    reader: &mut Reader<'_>,
    type_name: &str,
    inner_type: Option<&str>,
    key_type: Option<&str>,
    value_type: Option<&str>,
) -> Result<Value, GvasError> {
    match type_name {
        "BoolProperty" => Ok(Value::Bool(reader.u8()? != 0)),
        "ByteProperty" => Ok(Value::Byte(reader.u8()?)),
        "StructProperty" => Ok(Value::Struct(read_struct_body(reader)?)),
        "ArrayProperty" => {
            let count = reader.i32()?;
            if !(0..=MAX_ELEMENTS).contains(&count) {
                return parse_error(format!("bad array count {count}"));
            }
            let count = count as usize;
            let mut elements = Vec::with_capacity(count.min(1024));
            if inner_type == Some("StructProperty") {
                match read_tag_header(reader)? {
                    Some(header) if header.type_name == "StructProperty" => {}
                    _ => return parse_error("missing array element tag"),
                }
                for _ in 0..count {
                    elements.push(Value::Struct(read_struct_body(reader)?));
                }
            } else {
                let element_type = inner_type.unwrap_or("");
                for _ in 0..count {
                    elements.push(read_scalar(reader, element_type)?);
                }
            }
            Ok(Value::Array(ArrayValue { elements }))
        }
        "SetProperty" => {
            let count = reader.i32()?;
            if !(0..=MAX_ELEMENTS).contains(&count) {
                return parse_error(format!("bad set count {count}"));
            }
            let count = count as usize;
            let mut elements = Vec::with_capacity(count.min(1024));
            if inner_type == Some("StructProperty") {
                for _ in 0..count {
                    elements.push(Value::Struct(read_struct_body(reader)?));
                }
            } else {
                let element_type = inner_type.unwrap_or("");
                for _ in 0..count {
                    elements.push(read_scalar(reader, element_type)?);
                }
            }
            Ok(Value::Set(SetValue { elements }))
        }
        "MapProperty" => {
            let count = reader.i32()?;
            if !(0..=MAX_ELEMENTS).contains(&count) {
                return parse_error(format!("bad map count {count}"));
            }
            let mut entries = Vec::with_capacity((count as usize).min(1024));
            for _ in 0..count {
                let key = read_value_for_type(
                    reader,
                    key_type.unwrap_or("NameProperty"),
                    None,
                    None,
                    None,
                )?;
                let value = read_value_for_type(
                    reader,
                    value_type.unwrap_or("StrProperty"),
                    None,
                    None,
                    None,
                )?;
                entries.push((key, value));
            }
            Ok(Value::Map(MapValue { entries }))
        }
        other if is_tag_type(other) => read_scalar(reader, other),
        other => parse_error(format!("unsupported value type {other:?}")),
    }
}

fn read_property(reader: &mut Reader<'_>) -> Result<Option<Property>, GvasError> {
    let Some(header) = read_tag_header(reader)? else {
        return Ok(None);
    };

    let value_start = header.value_start;
    let mut size = header.size;

    let value = match header.type_name.as_str() {
        "BoolProperty" => {
            let value = Value::Bool(reader.u8()? != 0);
            size = (reader.pos - value_start) as u64;
            value
        }
        "StructProperty" => try_struct(reader, size)?,
        "ArrayProperty" => try_array(reader, size, header.inner_type.as_deref())?,
        "SetProperty" => try_set(reader, size, header.inner_type.as_deref())?,
        "MapProperty" => try_map(
            reader,
            size,
            header.key_type.as_deref(),
            header.value_type.as_deref(),
        )?,
        "ByteProperty" if size != 1 => Value::Str(reader.fstring()?),
        "TextProperty" => Value::Raw(RawValue {
            data: reader.read(size as usize)?.to_vec(),
        }),
        other if is_tag_type(other) => read_scalar(reader, other)?,
        _ => Value::Raw(RawValue {
            data: reader.read(size as usize)?.to_vec(),
        }),
    };

    let next = usize::try_from(size)
        .ok()
        .and_then(|size| value_start.checked_add(size))
        .unwrap_or(usize::MAX);
    reader.pos = next;

    Ok(Some(Property {
        name: header.name,
        prop_type: header.type_name,
        value,
        size,
        struct_name: header.struct_name,
        enum_name: header.enum_name,
        inner_type: header.inner_type,
        key_type: header.key_type,
        value_type: header.value_type,
    }))
}

fn insert_named(properties: &mut Vec<(String, Property)>, property: Property) {
    let name = property.name.clone();
    if let Some(slot) = properties
        .iter_mut()
        .find(|(property_name, _)| *property_name == name)
    {
        slot.1 = property;
    } else {
        properties.push((name, property));
    }
}

pub fn read_gvas_bytes(data: &[u8]) -> Result<GvasFile, GvasError> {
    let has_evas = data.len() >= 4 && &data[..4] == EVAS_MAGIC;
    let base = if has_evas { 8 } else { 0 };
    if data.get(base..base + 4) != Some(&GVAS_MAGIC[..]) {
        return parse_error("file does not start with a GVAS archive");
    }

    let mut reader = Reader::new(data, base + 4);
    let save_game_version = reader.u32()?;
    let package_file_version = reader.u32()?;
    if save_game_version >= 3 {
        reader.u32()?;
    }
    let major = reader.u16()?;
    let minor = reader.u16()?;
    let patch = reader.u16()?;
    let changelist = reader.u32()?;
    let branch = reader.fstring()?;
    let custom_version_format = reader.u32()?;
    let custom_version_count = reader.u32()?;
    let mut custom_versions: Vec<(String, i32)> = Vec::new();
    for _ in 0..custom_version_count {
        let guid = reader.read(16)?;
        let version = reader.i32()?;
        let hex: String = guid.iter().map(|byte| format!("{byte:02x}")).collect();
        custom_versions.push((hex, version));
    }
    let save_game_class_name = reader.fstring()?;

    let mut properties: Vec<(String, Property)> = Vec::new();
    while let Some(property) = read_property(&mut reader)? {
        insert_named(&mut properties, property);
    }

    let remaining = reader.remaining();
    let footer = reader.read(remaining)?.to_vec();

    let header = GvasHeader {
        save_game_version,
        package_file_version,
        engine: EngineVersion {
            major,
            minor,
            patch,
            changelist,
            branch,
        },
        custom_version_format,
        custom_versions,
        save_game_class_name,
        has_evas_prefix: has_evas,
    };
    Ok(GvasFile {
        header,
        properties,
        footer,
    })
}

pub fn load_gvas(path: impl AsRef<Path>) -> Result<GvasFile, GvasError> {
    let data = std::fs::read(path)?;
    read_gvas_bytes(&data)
}

fn format_raw(data: &[u8], max_raw: usize) -> String {
    let head_len = data.len().min(max_raw);
    let head: String = data[..head_len]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let suffix = if data.len() > max_raw {
        format!("...(+{} bytes)", data.len() - max_raw)
    } else {
        String::new()
    };
    format!("<raw:{head}{suffix}>")
}

pub fn to_jsonable(value: &Value, max_raw: usize) -> serde_json::Value {
    use serde_json::Value as Json;

    match value {
        Value::Raw(raw) => Json::String(format_raw(&raw.data, max_raw)),
        Value::Struct(structure) => {
            let mut object = serde_json::Map::new();
            for (name, property) in &structure.fields {
                object.insert(name.clone(), to_jsonable(&property.value, max_raw));
            }
            Json::Object(object)
        }
        Value::Array(array) => Json::Array(
            array
                .elements
                .iter()
                .map(|item| to_jsonable(item, max_raw))
                .collect(),
        ),
        Value::Set(set) => Json::Array(
            set.elements
                .iter()
                .map(|item| to_jsonable(item, max_raw))
                .collect(),
        ),
        Value::Map(map) => Json::Array(
            map.entries
                .iter()
                .map(|(key, value)| {
                    Json::Array(vec![to_jsonable(key, max_raw), to_jsonable(value, max_raw)])
                })
                .collect(),
        ),
        Value::Bool(value) => Json::Bool(*value),
        Value::Byte(value) => Json::from(*value),
        Value::Int8(value) => Json::from(*value),
        Value::Int16(value) => Json::from(*value),
        Value::Int(value) => Json::from(*value),
        Value::Int64(value) => Json::from(*value),
        Value::UInt16(value) => Json::from(*value),
        Value::UInt32(value) => Json::from(*value),
        Value::UInt64(value) => Json::from(*value),
        Value::Float(value) => serde_json::Number::from_f64(*value as f64)
            .map(Json::Number)
            .unwrap_or(Json::Null),
        Value::Double(value) => serde_json::Number::from_f64(*value)
            .map(Json::Number)
            .unwrap_or(Json::Null),
        Value::Str(value) => Json::String(value.clone()),
    }
}
