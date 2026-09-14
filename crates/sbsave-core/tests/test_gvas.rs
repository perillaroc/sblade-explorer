use sbsave_core::gvas::{read_gvas_bytes, to_jsonable, Property, StructValue, Value};

fn fstr(value: &str) -> Vec<u8> {
    let mut raw = value.as_bytes().to_vec();
    raw.push(0);
    let mut out = (raw.len() as i32).to_le_bytes().to_vec();
    out.extend_from_slice(&raw);
    out
}

fn header() -> Vec<u8> {
    let mut data = b"GVAS".to_vec();
    data.extend_from_slice(&2u32.to_le_bytes());
    data.extend_from_slice(&522u32.to_le_bytes());
    data.extend_from_slice(&4u16.to_le_bytes());
    data.extend_from_slice(&26u16.to_le_bytes());
    data.extend_from_slice(&2u16.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&fstr("++UE4+Release-4.26"));
    data.extend_from_slice(&3u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&fstr("/Script/SB.SBSaveGame"));
    data
}

fn tag(name: &str, type_name: &str, value: &[u8], extras: &[u8]) -> Vec<u8> {
    let mut out = fstr(name);
    out.extend_from_slice(&fstr(type_name));
    out.extend_from_slice(&(value.len() as u64).to_le_bytes());
    out.extend_from_slice(extras);
    out.push(0);
    out.extend_from_slice(value);
    out
}

#[test]
fn reads_int64_property() {
    let mut payload = header();
    payload.extend_from_slice(&tag("TestInt", "IntProperty", &42i32.to_le_bytes(), b""));
    payload.extend_from_slice(&fstr("None"));
    payload.extend_from_slice(&[0, 0, 0, 0]);

    let gvas = read_gvas_bytes(&payload).expect("parse");
    let property = gvas.get("TestInt").expect("property");
    assert_eq!(property.prop_type, "IntProperty");
    assert_eq!(property.value, Value::Int(42));
    assert_eq!(gvas.header.save_game_class_name, "/Script/SB.SBSaveGame");
}

#[test]
fn reads_evas_wrapper() {
    let mut payload = b"EVAS\x01\x00\x00\x00".to_vec();
    payload.extend_from_slice(&header());
    payload.extend_from_slice(&tag("Name", "StrProperty", &fstr("Eve"), b""));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    assert!(gvas.header.has_evas_prefix);
    assert_eq!(
        gvas.get("Name").expect("property").value,
        Value::Str("Eve".into())
    );
}

#[test]
fn reads_bool_and_string() {
    let mut payload = header();
    payload.extend_from_slice(&tag("Flag", "BoolProperty", b"\x01", b""));
    payload.extend_from_slice(&tag("Label", "StrProperty", &fstr("hello"), b""));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    assert_eq!(gvas.get("Flag").expect("flag").value, Value::Bool(true));
    assert_eq!(
        gvas.get("Label").expect("label").value,
        Value::Str("hello".into())
    );
}

#[test]
fn reads_struct_field() {
    let mut body = tag("Inner", "IntProperty", &7i32.to_le_bytes(), b"");
    body.extend_from_slice(&fstr("None"));
    let mut extras = fstr("TestStruct");
    extras.extend_from_slice(&[0u8; 16]);

    let mut payload = header();
    payload.extend_from_slice(&tag("Root", "StructProperty", &body, &extras));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    let root = gvas.get("Root").expect("root");
    let structure: &StructValue = root.as_struct().expect("struct");
    assert_eq!(structure.get("Inner").expect("inner").value, Value::Int(7));
}

#[test]
fn reads_struct_array() {
    let mut body0 = tag("Alias", "NameProperty", &fstr("A"), b"");
    body0.extend_from_slice(&fstr("None"));
    let mut body1 = tag("Alias", "NameProperty", &fstr("B"), b"");
    body1.extend_from_slice(&fstr("None"));

    let mut element_tag = fstr("Items");
    element_tag.extend_from_slice(&fstr("StructProperty"));
    element_tag.extend_from_slice(&((body0.len() + body1.len()) as u64).to_le_bytes());
    element_tag.extend_from_slice(&fstr("TestStruct"));
    element_tag.extend_from_slice(&[0u8; 16]);
    element_tag.push(0);

    let mut value = 2i32.to_le_bytes().to_vec();
    value.extend_from_slice(&element_tag);
    value.extend_from_slice(&body0);
    value.extend_from_slice(&body1);

    let mut payload = header();
    payload.extend_from_slice(&tag(
        "Items",
        "ArrayProperty",
        &value,
        &fstr("StructProperty"),
    ));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    let items = match &gvas.get("Items").expect("items").value {
        Value::Array(array) => &array.elements,
        other => panic!("expected array, got {other:?}"),
    };
    let aliases: Vec<Value> = items
        .iter()
        .map(|element| match element {
            Value::Struct(structure) => structure.get("Alias").expect("alias").value.clone(),
            other => panic!("expected struct element, got {other:?}"),
        })
        .collect();
    assert_eq!(
        aliases,
        vec![Value::Str("A".into()), Value::Str("B".into())]
    );
}

#[test]
fn reads_name_map() {
    let mut entries = fstr("Key");
    entries.extend_from_slice(&tag("Value", "IntProperty", &5i32.to_le_bytes(), b""));
    entries.extend_from_slice(&fstr("None"));

    let mut value = 0i32.to_le_bytes().to_vec();
    value.extend_from_slice(&1i32.to_le_bytes());
    value.extend_from_slice(&entries);

    let mut extras = fstr("NameProperty");
    extras.extend_from_slice(&fstr("StructProperty"));

    let mut payload = header();
    payload.extend_from_slice(&tag("DataMap", "MapProperty", &value, &extras));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    let mapping = match &gvas.get("DataMap").expect("map").value {
        Value::Map(map) => map,
        other => panic!("expected map, got {other:?}"),
    };
    let entry = mapping.get(&Value::Str("Key".into())).expect("entry");
    let structure = match entry {
        Value::Struct(structure) => structure,
        other => panic!("expected struct entry, got {other:?}"),
    };
    assert_eq!(structure.get("Value").expect("value").value, Value::Int(5));
}

#[test]
fn text_property_falls_back_to_raw() {
    let mut value = vec![0u8; 4];
    value.extend_from_slice(&fstr("x"));

    let mut payload = header();
    payload.extend_from_slice(&tag("Rich", "TextProperty", &value, b""));
    payload.extend_from_slice(&fstr("None"));

    let gvas = read_gvas_bytes(&payload).expect("parse");
    let property = gvas.get("Rich").expect("property");
    assert_eq!(property.prop_type, "TextProperty");
    assert!(matches!(property.value, Value::Raw(_)));
}

#[test]
fn rejects_non_gvas() {
    let mut payload = b"NOPE".to_vec();
    payload.extend_from_slice(&[0u8; 32]);
    assert!(read_gvas_bytes(&payload).is_err());
}

#[test]
fn property_dataclass_helpers() {
    let property = Property {
        name: "A".into(),
        prop_type: "IntProperty".into(),
        value: Value::Int(1),
        ..Default::default()
    };
    assert!(property.field("A").is_none());
}

#[test]
#[ignore = "requires a real save: set SBSAVE_SAVE"]
fn parses_real_save() {
    let Ok(path) = std::env::var("SBSAVE_SAVE") else {
        return;
    };
    let data = std::fs::read(&path).expect("read save");
    let gvas = read_gvas_bytes(&data).expect("parse save");
    assert_eq!(gvas.header.save_game_class_name, "/Script/SB.SBSaveGame");
    assert!(!gvas.properties.is_empty());

    if let Ok(out) = std::env::var("SBSAVE_TREE_OUT") {
        let tree: serde_json::Map<String, serde_json::Value> = gvas
            .properties
            .iter()
            .map(|(name, property)| (name.clone(), to_jsonable(&property.value, 64)))
            .collect();
        let text = serde_json::to_string_pretty(&serde_json::Value::Object(tree)).expect("json");
        std::fs::write(&out, text).expect("write tree");
    }
}
