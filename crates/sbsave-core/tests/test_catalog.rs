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
        "naytiba",
        "characters",
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
    assert_eq!(catalog.items.len(), 932);
    assert_eq!(catalog.categories.len(), 15);
    assert_eq!(catalog.by_category("nano_suits").len(), 126);
    assert_eq!(catalog.by_category("cans").len(), 49);
    assert_eq!(catalog.by_category("camps").len(), 89);
    assert_eq!(catalog.by_category("naytiba").len(), 67);
    assert_eq!(catalog.by_category("characters").len(), 55);
    let low_confidence = catalog
        .items
        .iter()
        .filter(|item| item.confidence != "high")
        .count();
    assert_eq!(low_confidence, 69);
}

#[test]
fn album_categories_cover_game_album() {
    let catalog = load_catalog(None).expect("catalog");
    assert_eq!(catalog.categories["naytiba"].section, "album");
    assert_eq!(catalog.categories["characters"].section, "album");
    assert_eq!(catalog.categories["cans"].section, "collection");

    let index = catalog.alias_index();
    let thorn = &index["Ach_Album_Unlock_ThornHead"];
    assert_eq!(thorn.id, "Album_ThornHead");
    assert_eq!(thorn.name, "棘蛇兽");
    assert_eq!(thorn.name_en.as_deref(), Some("Thornhead"));
    assert_eq!(thorn.category, "naytiba");
    assert_eq!(thorn.area_zh.as_deref(), Some("孽奇拔小兵"));
    assert!(thorn
        .desc_zh
        .as_deref()
        .is_some_and(|text| text.contains("生态情报")));
    assert_eq!(thorn.order, 1);

    let adam = &index["Ach_Album_Unlock_Adam_3"];
    assert_eq!(adam.name, "艾德姆（资料 3/5）");
    assert_eq!(adam.name_en.as_deref(), Some("Adam (Entry 3/5)"));
    assert_eq!(adam.category, "characters");
    assert_eq!(adam.area_zh.as_deref(), Some("角色"));
    assert!(adam
        .desc_zh
        .as_deref()
        .is_some_and(|text| text.contains("轨道电梯")));

    let nikke = &index["Ach_Album_Unlock_Scarlet_1"];
    assert_eq!(
        nikke.area_zh.as_deref(),
        Some("《剑星》X《胜利女神：妮姬》")
    );

    let mut group_counts: HashMap<&str, usize> = HashMap::new();
    for item in catalog.by_category("naytiba") {
        *group_counts
            .entry(item.area_zh.as_deref().unwrap_or("?"))
            .or_default() += 1;
    }
    assert_eq!(group_counts["孽奇拔小兵"], 12);
    assert_eq!(group_counts["孽奇拔战士"], 36);
    assert_eq!(group_counts["精锐孽奇拔"], 10);
    assert_eq!(group_counts["阿尔法孽奇拔"], 6);
    assert_eq!(group_counts["上古孽奇拔"], 3);

    // 每个角色页都有页序，页数与游戏图鉴一致（Adam 5 页、迅驰 3 页、母主领域 1 页）
    let names: Vec<&str> = catalog
        .by_category("characters")
        .iter()
        .map(|item| item.name.as_str())
        .collect();
    assert!(names.contains(&"艾德姆（资料 5/5）"));
    assert!(names.contains(&"迅驰（资料 3/3）"));
    assert!(names.contains(&"母主领域（资料 1/1）"));
    assert!(names.iter().all(|name| name.contains("（资料 ")));
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
fn document_locations_follow_guide() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();

    let memo = &index["Item_Records_DED10_Memory_07"];
    assert_eq!(memo.area.as_deref(), Some("Eidos 7"));
    assert_eq!(memo.area_zh.as_deref(), Some("埃多斯7号"));
    assert_eq!(memo.location.as_deref(), Some("Parking Tower"));
    assert_eq!(memo.location_zh.as_deref(), Some("停车塔"));

    let journal = &index["Item_Records_DED20_Memory_13"];
    assert_eq!(journal.area_zh.as_deref(), Some("埃多斯7号"));
    assert_eq!(journal.location_zh.as_deref(), Some("淹水商业区"));

    assert_eq!(
        index["Item_Records_DED40_Memory_12"].area_zh.as_deref(),
        Some("埃多斯9号")
    );
    assert_eq!(
        index["Item_Records_DED40_Memory_12"].location_zh.as_deref(),
        Some("工坊")
    );
    assert_eq!(
        index["Item_Records_Day1_Memory_01"].area_zh.as_deref(),
        Some("希雍")
    );
    assert_eq!(
        index["Item_Records_Day1_Memory_01"].location_zh.as_deref(),
        Some("希雍城")
    );
    assert_eq!(
        index["Item_Records_ETC_Memory_04"].area_zh.as_deref(),
        Some("废土")
    );
    assert_eq!(
        index["Item_Records_ETC_Memory_04"].location_zh.as_deref(),
        Some("大峡谷")
    );
    assert_eq!(
        index["Item_Records_SE06_Memory_10"].area_zh.as_deref(),
        Some("尖塔4")
    );
    assert_eq!(
        index["Item_Records_SE06_Memory_10"].location_zh.as_deref(),
        Some("拉斐尔太空中心")
    );
    assert_eq!(
        index["Item_Records_WLB_Memory_42"].area_zh.as_deref(),
        Some("大沙漠")
    );
    assert_eq!(
        index["Item_Records_WLB_Memory_42"].location_zh.as_deref(),
        Some("大沙漠中部")
    );

    assert_eq!(
        index["Item_Records_Day1_Memory_09_1"].area,
        index["Item_Records_Day1_Memory_09"].area
    );
    assert_eq!(
        index["Item_Records_Day1_Memory_09_1"].location,
        index["Item_Records_Day1_Memory_09"].location
    );

    let stick = &index["Item_Records_DED10_Memory_11"];
    assert_eq!(stick.area.as_deref(), Some("Eidos 7"));
    assert_eq!(stick.location, None);

    let passcode = &index["Item_Records_DED10_Passcode_01"];
    assert_eq!(passcode.area.as_deref(), Some("Eidos 7"));
    assert_eq!(passcode.area_zh.as_deref(), Some("埃多斯7号"));
    assert_eq!(passcode.location.as_deref(), Some("Silent Street"));
    assert_eq!(passcode.location_zh.as_deref(), Some("寂静街"));

    let documents_missing: Vec<&str> = catalog
        .by_category("records")
        .iter()
        .filter(|item| {
            item.record_type
                .as_deref()
                .is_some_and(|key| key.starts_with("document_"))
        })
        .filter(|item| item.area.is_none() || item.location.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(documents_missing.is_empty(), "{documents_missing:?}");

    let passcodes_missing: Vec<&str> = catalog
        .by_category("passcodes")
        .iter()
        .filter(|item| item.area.is_none() || item.location.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(passcodes_missing.is_empty(), "{passcodes_missing:?}");
}

#[test]
fn guide_links_cover_catalog() {
    let catalog = load_catalog(None).expect("catalog");
    let mut with_web = 0;
    let mut with_video = 0;
    let mut missing: Vec<&str> = Vec::new();
    for item in &catalog.items {
        if item.category == "naytiba" || item.category == "characters" {
            assert!(item.guides.is_none(), "图鉴条目不应有攻略链接: {}", item.id);
            continue;
        }
        let Some(guides) = &item.guides else {
            missing.push(item.id.as_str());
            continue;
        };
        if guides.web.is_some() {
            with_web += 1;
        }
        if guides.video.is_some() {
            with_video += 1;
        }
        for link in [guides.web.as_ref(), guides.video.as_ref()]
            .into_iter()
            .flatten()
        {
            assert!(link.url.starts_with("https://"), "{}", link.url);
            assert!(!link.title.is_empty(), "{}", link.url);
        }
    }
    // 仅默认外观类条目没有可链接的攻略（发型：默认马尾与首领挑战奖励）
    assert_eq!(missing, ["Hair_000", "Hair_Nikke_01"]);
    assert_eq!(with_web, 804);
    assert_eq!(with_video, 696);
    let index = catalog.alias_index();
    let can = index["Can_011"].guides.as_ref().expect("can guides");
    assert_eq!(
        can.web.as_ref().map(|link| link.url.as_str()),
        Some("https://www.gamersky.com/handbook/202404/1737659_2.shtml")
    );
    assert!(can
        .video
        .as_ref()
        .is_some_and(|link| link.url.ends_with("?p=11")));

    let nikke_fish = index["Fish_Nikke_Poli"].guides.as_ref().expect("fish");
    assert!(nikke_fish
        .video
        .as_ref()
        .is_some_and(|link| link.url.ends_with("?p=27")));

    let memory = index["Item_Records_WLB_Memory_64"]
        .guides
        .as_ref()
        .and_then(|guides| guides.video.as_ref())
        .expect("memorystick video");
    assert!(memory.url.contains("BV1kr421L74V"), "{}", memory.url);

    let camp = index["ChangeState_ZoneEnv_AYL_01_EnvS_005_Camp"]
        .guides
        .as_ref()
        .expect("camp guides");
    assert!(camp.web.is_some());
    assert!(camp.video.is_none());

    // 纳米战衣与设计图逐件指向游民星空女主服装图鉴的具体分页
    let suit = index["BS_41"].guides.as_ref().expect("suit guides");
    assert_eq!(
        suit.web.as_ref().map(|link| link.url.as_str()),
        Some("https://www.gamersky.com/handbook/202404/1737082_20.shtml")
    );
    let pattern = index["DesignPattern_BS_41"]
        .guides
        .as_ref()
        .expect("pattern guides");
    assert_eq!(
        pattern.web.as_ref().map(|link| link.url.as_str()),
        suit.web.as_ref().map(|link| link.url.as_str())
    );
    // 战衣视频逐套对应 B 站「126 套纳米服全收集」分 P
    let suit_video = suit.video.as_ref().expect("suit video");
    assert!(
        suit_video.url.contains("BV1Wfj1ztEuj"),
        "{}",
        suit_video.url
    );
    assert!(suit_video.url.ends_with("?p=71"), "{}", suit_video.url);
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
    assert_eq!(with_obtain.len(), 686);
    let missing: Vec<&str> = with_obtain
        .iter()
        .filter(|item| item.obtain_zh.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(missing.is_empty(), "{missing:?}");

    let records = catalog.by_category("records");
    assert_eq!(
        records.iter().filter(|item| item.obtain.is_some()).count(),
        records.len()
    );
    let passcodes = catalog.by_category("passcodes");
    assert_eq!(
        passcodes
            .iter()
            .filter(|item| item.obtain.is_some())
            .count(),
        passcodes.len()
    );

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

    assert!(index["Item_Records_DED10_Memory_09"]
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("图书馆")));
    assert!(index["Item_Records_ATL_Passcode_01"]
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("蜂巢")));
    assert_eq!(
        index["Item_Records_ME01_Memory_01_2"].obtain,
        index["Item_Records_ME01_Memory_01"].obtain
    );
    assert_eq!(
        index["Item_Records_ME01_Memory_01_2"].obtain_zh,
        index["Item_Records_ME01_Memory_01"].obtain_zh
    );

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
