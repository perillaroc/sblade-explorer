use std::collections::HashMap;

use sbsave_core::catalog::load_catalog;

#[test]
fn bundled_catalog_loads() {
    let catalog = load_catalog(None).expect("catalog");
    assert!(catalog.version >= 1);
    assert!(catalog.items.len() > 700);
    for key in [
        "nano_suits",
        "cans",
        "records",
        "camps",
        "hair",
        "fish",
        "design_patterns",
    ] {
        assert!(catalog.categories.contains_key(key), "{key}");
        assert!(!catalog.by_category(key).is_empty(), "{key}");
    }
}

#[test]
fn aliases_are_unique() {
    let catalog = load_catalog(None).expect("catalog");
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();
    for item in &catalog.items {
        for alias in item.satisfy_aliases() {
            if let Some(previous) = seen.get(alias) {
                duplicates.push(format!("{alias} {previous} {}", item.id));
            }
            seen.insert(alias.to_string(), item.id.clone());
        }
    }
    assert!(
        duplicates.is_empty(),
        "{:?}",
        &duplicates[..duplicates.len().min(10)]
    );
}

#[test]
fn key_alias_mappings() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();
    assert_eq!(index["BS_Raven"].name, "渡鸦装");
    assert_eq!(index["BS_102"].name, "活肤紧身服");
    assert_eq!(index["Can_007"].category, "cans");
    assert_eq!(index["Fish_Salmon"].name, "鲑鱼");
    assert_eq!(index["Earring_001"].name, "绯红泪珠");
    assert_eq!(index["Hair_000"].name, "星球空降马尾");
    assert_eq!(index["DesignPattern_BS_01"].category, "design_patterns");
}

#[test]
fn game_data_names_applied() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();
    assert_eq!(index["Can_007"].name, "妙之跃");
    assert_eq!(index["BS_Nikke_01"].name, "浪游剑客服");
    assert_eq!(index["Hair_Nikke_01"].name, "月下美人");
    assert_eq!(index["DesignPattern_BS_01"].name, "设计图：镂空服");
    assert_eq!(
        index["Item_Records_DED10_Memory_09"].name,
        "《塑料之心：第3卷》"
    );
    assert_eq!(index["Item_Records_DED10_Memory_09"].confidence, "high");
    assert_eq!(
        index["ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"].name,
        "隐秘之路"
    );
}

#[test]
fn catalog_counts() {
    let catalog = load_catalog(None).expect("catalog");
    assert_eq!(catalog.by_category("nano_suits").len(), 126);
    assert_eq!(catalog.by_category("cans").len(), 49);
    assert_eq!(catalog.by_category("camps").len(), 89);
    let low_confidence = catalog
        .items
        .iter()
        .filter(|item| item.confidence != "high")
        .count();
    assert_eq!(low_confidence, 69);
}

#[test]
fn ng_plus_metadata_present() {
    let catalog = load_catalog(None).expect("catalog");
    let ng_items: Vec<_> = catalog
        .items
        .iter()
        .filter(|item| item.ng_plus > 0)
        .collect();
    assert!(ng_items.len() > 30);
    assert!(ng_items.iter().any(|item| item.ng_plus == 2));
}

#[test]
fn chinese_translations_cover_guide_text() {
    let catalog = load_catalog(None).expect("catalog");
    let with_obtain: Vec<_> = catalog
        .items
        .iter()
        .filter(|item| item.obtain.is_some())
        .collect();
    assert_eq!(with_obtain.len(), 353);
    let missing: Vec<&str> = with_obtain
        .iter()
        .filter(|item| item.obtain_zh.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(missing.is_empty(), "{missing:?}");

    let index = catalog.alias_index();
    assert_eq!(index["BS_09_2"].area_zh.as_deref(), Some("埃多斯7号"));
    assert_eq!(index["BS_09_2"].location_zh.as_deref(), Some("淹水商业区"));
    assert!(index["BS_09_2"]
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("淹水商业区")));
    assert!(index["Hair_006"]
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.starts_with("完成支线任务《第一位顾客》")));

    let construction: Vec<_> = catalog
        .items
        .iter()
        .filter(|item| item.location_zh.as_deref() == Some("施工区"))
        .collect();
    assert!(construction.iter().any(|item| item
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("施工区东侧"))));
}
