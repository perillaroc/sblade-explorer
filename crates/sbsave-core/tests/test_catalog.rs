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
fn record_types_cover_records() {
    let catalog = load_catalog(None).expect("catalog");
    let records = catalog.by_category("records");
    assert_eq!(records.len(), 310);
    let missing: Vec<&str> = records
        .iter()
        .filter(|item| item.record_type.is_none() || item.record_type_zh.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(missing.is_empty(), "{missing:?}");

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for item in &records {
        let key = item.record_type.as_deref().unwrap_or("?");
        *counts.entry(key).or_default() += 1;
    }
    assert_eq!(counts["memorystick"], 187);
    assert_eq!(counts["document_series"], 27);
    assert_eq!(counts["document_promotions"], 27);
    assert_eq!(counts["document_messages"], 15);
    assert_eq!(counts["document_journal"], 12);
    assert_eq!(counts["document_log_data"], 11);
    assert_eq!(counts["document_books"], 10);
    assert_eq!(counts["document_information"], 10);
    assert_eq!(counts["document_prayers"], 7);
    assert_eq!(counts["document_announcements"], 4);

    let other_with_type = catalog
        .items
        .iter()
        .filter(|item| item.category != "records" && item.record_type.is_some())
        .count();
    assert_eq!(other_with_type, 0);
}

#[test]
fn record_type_samples_and_variant_inheritance() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();
    let type_of = |id: &str| {
        (
            index[id].record_type.as_deref(),
            index[id].record_type_zh.as_deref(),
        )
    };
    assert_eq!(
        type_of("Item_Records_DED10_Memory_09"),
        (Some("document_series"), Some("文档·系列"))
    );
    assert_eq!(
        type_of("Item_Records_Xion_Memory_15"),
        (Some("document_journal"), Some("文档·日志"))
    );
    assert_eq!(
        type_of("Item_Records_WLB_Memory_52"),
        (Some("document_messages"), Some("文档·消息"))
    );
    assert_eq!(
        type_of("Item_Records_DED20_Memory_01"),
        (Some("memorystick"), Some("记忆棒"))
    );
    assert_eq!(
        type_of("Item_Records_Day1_Memory_09_1"),
        (Some("document_promotions"), Some("文档·宣传"))
    );
    assert_eq!(
        type_of("Item_Records_ME01_Memory_01_2"),
        (Some("memorystick"), Some("记忆棒"))
    );
    assert_eq!(
        type_of("Item_Records_WLA_Memory_05"),
        (Some("memorystick"), Some("记忆棒"))
    );
}

#[test]
fn memorysticks_follow_in_game_menu_order() {
    let catalog = load_catalog(None).expect("catalog");
    let memories: Vec<_> = catalog
        .items
        .iter()
        .filter(|item| item.record_type.as_deref() == Some("memorystick"))
        .collect();
    assert_eq!(memories.len(), 187);
    let mut orders: Vec<i64> = memories.iter().map(|item| item.order).collect();
    assert!(orders.iter().all(|order| *order > 0));
    orders.sort_unstable();
    orders.dedup();
    assert_eq!(orders, (1..=186).collect::<Vec<_>>());

    let index = catalog.alias_index();
    let first = index["Item_Records_DED10_Memory_11"];
    assert_eq!(first.area.as_deref(), Some("Eidos 7"));
    assert_eq!(first.area_zh.as_deref(), Some("埃多斯7号"));
    assert_eq!(first.order, 1);
    assert_eq!(
        index["Item_Records_DED40_Memory_10"].area_zh.as_deref(),
        Some("埃多斯9号")
    );
    assert_eq!(
        index["Item_Records_WLB_Memory_64"].area_zh.as_deref(),
        Some("大沙漠")
    );
    assert_eq!(
        index["Item_Records_Quest_Request_006_02"]
            .area_zh
            .as_deref(),
        Some("废土")
    );
    assert_eq!(
        index["Item_Records_Xion_Memory_32"].area_zh.as_deref(),
        Some("希雍")
    );
    assert_eq!(
        index["Item_Records_ME01_Memory_01_2"].order,
        index["Item_Records_ME01_Memory_01"].order
    );
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
