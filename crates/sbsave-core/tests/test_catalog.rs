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
fn bayonetta_collab_entries_are_mapped() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();
    assert_eq!(index["BS_Bayonetta_01"].name, "时装子弹（蓓优妮塔）");
    assert_eq!(
        index["BS_Bayonetta_01"].name_en.as_deref(),
        Some("Couture Bullet (Bayonetta)")
    );
    assert_eq!(index["BS_Bayonetta_03"].name, "时装子弹（贞德）");
    assert_eq!(index["FaceAccessory_Bayonetta_002"].name, "修女的眼镜");
    assert_eq!(index["Earring_Bayonetta_003"].name, "贞德的耳环");
    assert_eq!(index["Earring_Bayonetta_003"].category, "earrings");
    assert_eq!(index["BS_Bayonetta_02"].confidence, "high");
    assert_eq!(
        index["BS_Bayonetta_02"].area.as_deref(),
        Some("Bayonetta Collab")
    );
}

#[test]
fn eidos9_camps_use_confirmed_rows() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();
    let parking = index["ChangeState_ZoneEnv_DED40_EnvS_039_Camp"];
    assert_eq!(parking.name, "中央立体停车场");
    assert_eq!(parking.name_en.as_deref(), Some("Central Parking Tower"));
    assert_eq!(parking.confidence, "high");
    assert_eq!(
        index["ChangeState_ZoneEnv_DED40_EnvS_054_Camp"]
            .name_en
            .as_deref(),
        Some("Atelier")
    );
    assert!(catalog
        .by_category("camps")
        .iter()
        .all(|item| item.confidence == "high"));
}

#[test]
fn gear_categories_are_growth_only() {
    let catalog = load_catalog(None).expect("catalog");
    assert_eq!(catalog.categories["gear"].section, "growth");
    let gear = catalog.by_category("gear");
    assert_eq!(gear.len(), 77);
    assert!(gear.iter().all(|item| item.confidence == "high"));
    assert!(gear.iter().all(|item| !item.missable));

    let index = catalog.alias_index();
    let fixed = index["Gear_AdditiveFixedDamage_Common"];
    assert_eq!(fixed.name, "固定伤害组件");
    assert_eq!(fixed.name_en.as_deref(), Some("Fixed Damage Gear"));
    assert_eq!(
        fixed.aliases,
        vec![
            "Gear_AdditiveFixedDamage_Common".to_string(),
            "Gear_AdditiveFixedDamage_Common_MK2".to_string()
        ]
    );
    assert_eq!(fixed.area.as_deref(), Some("Eidos 7"));
    assert_eq!(fixed.location.as_deref(), Some("Parking Tower"));
    // Upgraded (Mk2) copies count as the same gear entry.
    assert_eq!(index["Gear_AdditiveFixedDamage_Common_MK2"].id, fixed.id);
    // The guide has no Common Burst Expansion pickup, so it is not an entry.
    assert!(!index.contains_key("Gear_MaxBurstGauge_Common"));
}

#[test]
fn exospines_are_matched_by_game_name() {
    let catalog = load_catalog(None).expect("catalog");
    assert_eq!(catalog.categories["exospines"].section, "growth");
    let exospines = catalog.by_category("exospines");
    assert_eq!(exospines.len(), 12);
    assert!(exospines.iter().all(|item| !item.missable));

    let index = catalog.alias_index();
    let combo = index["PT_Combo"];
    assert_eq!(combo.name, "“连锁反应”型辅助脊椎");
    assert_eq!(combo.name_en.as_deref(), Some("Chain-Type Exospine"));
    assert_eq!(
        combo.aliases,
        vec!["PT_Combo".to_string(), "PT_Combo_MK2".to_string()]
    );
    // Guide spelling "Judgement" is normalized to the game's "Judgment".
    assert_eq!(
        index["PT_Tachy"].name_en.as_deref(),
        Some("Judgment-Type Exospine")
    );
}

#[test]
fn catalog_counts() {
    let catalog = load_catalog(None).expect("catalog");
    assert_eq!(catalog.items.len(), 1019);
    assert_eq!(catalog.categories.len(), 17);
    assert_eq!(catalog.by_category("nano_suits").len(), 130);
    assert_eq!(catalog.by_category("cans").len(), 49);
    assert_eq!(catalog.by_category("camps").len(), 89);
    assert_eq!(catalog.by_category("naytiba").len(), 67);
    assert_eq!(catalog.by_category("characters").len(), 55);
    let low_confidence = catalog
        .items
        .iter()
        .filter(|item| item.confidence != "high")
        .count();
    // 埃多斯9号 6 座营地已用存档坐标确认行号，不再有低置信度条目
    assert_eq!(low_confidence, 0);
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
    // 与游戏 AlbumTable 的记录数一致（记忆棒 186 + 文档 114）
    assert_eq!(records.len(), 300);
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
    assert_eq!(counts["memorystick"], 186);
    assert_eq!(counts["document_series"], 27);
    assert_eq!(counts["document_promotions"], 18);
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
fn record_type_samples_and_variant_aliases() {
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
        type_of("Item_Records_WLA_Memory_05"),
        (Some("memorystick"), Some("记忆棒"))
    );

    // 重复获取的副本别名并入基础条目（游戏数据库只有一条记录），不再单列；
    // 副本别名仍参与已获得判定。
    let variant = &index["Item_Records_Day1_Memory_09_1"];
    assert_eq!(variant.id, "Item_Records_Day1_Memory_09");
    assert_eq!(variant.record_type.as_deref(), Some("document_promotions"));
    assert!(variant
        .aliases
        .iter()
        .any(|alias| alias == "Item_Records_Day1_Memory_09_1"));
    assert!(!catalog
        .items
        .iter()
        .any(|item| item.id == "Item_Records_Day1_Memory_09_1"));
    let memory = &index["Item_Records_ME01_Memory_01_2"];
    assert_eq!(memory.id, "Item_Records_ME01_Memory_01");
    assert_eq!(memory.record_type.as_deref(), Some("memorystick"));
    assert!(memory
        .aliases
        .iter()
        .any(|alias| alias == "Item_Records_ME01_Memory_01_2"));
}

#[test]
fn memorysticks_follow_in_game_menu_order() {
    let catalog = load_catalog(None).expect("catalog");
    let memories: Vec<_> = catalog
        .items
        .iter()
        .filter(|item| item.record_type.as_deref() == Some("memorystick"))
        .collect();
    assert_eq!(memories.len(), 186);
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
    assert_eq!(with_web, 887);
    assert_eq!(with_video, 694);
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
    // 808 收集品 + 77 装备 + 12 辅助脊椎（成长区块）均带获取文案与中文翻译
    assert_eq!(with_obtain.len(), 897);
    let missing: Vec<&str> = with_obtain
        .iter()
        .filter(|item| item.obtain_zh.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(missing.is_empty(), "{missing:?}");

    // 仅图鉴以官方说明代替获取方式，其余收集品全部有获取文案
    let collection_missing: Vec<&str> = catalog
        .items
        .iter()
        .filter(|item| item.category != "naytiba" && item.category != "characters")
        .filter(|item| item.obtain.is_none() || item.obtain_zh.is_none())
        .map(|item| item.id.as_str())
        .collect();
    assert!(collection_missing.is_empty(), "{collection_missing:?}");

    let camps = catalog.by_category("camps");
    assert!(camps
        .iter()
        .all(|item| item.obtain.is_some() && item.obtain_zh.is_some()));
    let fish = catalog.by_category("fish");
    assert!(fish
        .iter()
        .all(|item| item.obtain.is_some() && item.obtain_zh.is_some()));

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

    // 营地获取文案来自站点 description（英文原文 + obtain_camps.json 翻译）
    let camp = &index["ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"];
    assert!(camp
        .obtain
        .as_deref()
        .is_some_and(|text| text.contains("Solar Tower")));
    assert!(camp
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("太阳塔")));
    // 未收录进站点快照的营地使用 crosswalk 手工文案
    assert!(index["ChangeState_ZoneEnv_WLB_20_EnvS_025_Camp"]
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("11号矩阵")));

    // 鱼类获取文案来自 fish_obtain.json（钓点 + 鱼饵）
    let fish = &index["Fish_Dunkleosteus"];
    assert!(fish
        .obtain_zh
        .as_deref()
        .is_some_and(|text| text.contains("钓点") && text.contains("神奇鱼饵")));
    assert!(fish
        .obtain
        .as_deref()
        .is_some_and(|text| text.contains("Great Desert (Oasis)")));

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

#[test]
fn missable_entries_cover_locked_regions() {
    let catalog = load_catalog(None).expect("catalog");
    let missable = catalog.items.iter().filter(|item| item.missable).count();
    // 希雍 156 + 埃多斯9号 28 + 支线奖励/任务区域（去重后）
    assert_eq!(missable, 211);

    let index = catalog.alias_index();
    // 完成卢瓦底层后希雍城区锁定
    assert!(index["Item_Records_Xion_Memory_01"].missable);
    assert!(index["BS_01"].missable);
    // 埃多斯9号：百合进度 100% 才开放的可选区域
    assert!(index["ChangeState_ZoneEnv_DED40_EnvS_092_Camp"].missable);
    assert!(index["Item_Records_DED40_Memory_01"].missable);
    // 支线奖励：完成《第一位顾客》后在理发店解锁的发型
    assert!(index["Hair_006"].missable);
    // 常规区域与最终区域不标记
    assert!(!index["Item_Records_WLA_Memory_01"].missable);
    assert!(!index["ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"].missable);
    // 图鉴不参与易错过标记
    for category in ["naytiba", "characters"] {
        assert!(catalog
            .by_category(category)
            .iter()
            .all(|item| !item.missable));
    }
}

#[test]
fn missable_rules_are_expanded_from_raw_data() {
    let catalog = load_catalog(None).expect("catalog");
    let index = catalog.alias_index();

    let xion = index["Can_009"].missable_rule.as_ref().expect("xion rule");
    assert_eq!(xion.void_after_quests, vec!["Quest_Epic_08".to_string()]);
    assert!(xion.requires_quests.is_empty());
    assert!(xion.void_on_failed_quests.is_empty());

    let eidos = index["Item_Records_DED40_Memory_01"]
        .missable_rule
        .as_ref()
        .expect("eidos rule");
    assert_eq!(eidos.requires_quests, vec!["Quest_Epic_Hidden".to_string()]);

    // quest_rewards is not enabled yet: statically missable, no rule.
    assert!(index["BS_18"].missable);
    assert!(index["BS_18"].missable_rule.is_none());

    let with_rule = catalog
        .items
        .iter()
        .filter(|item| item.missable_rule.is_some())
        .count();
    assert_eq!(with_rule, 190);
}

#[test]
fn map_and_quest_names_are_embedded() {
    let catalog = load_catalog(None).expect("catalog");

    let xion = catalog.map_name("E04").expect("E04");
    assert_eq!(xion.name, "Xion");
    assert_eq!(xion.name_zh, "希雍");
    assert_eq!(catalog.map_name("F02").expect("F02").name, "Eidos 7");
    assert_eq!(catalog.map_name("C05").expect("C05").name, "Nest");
    assert!(catalog.map_name("E99").is_none());
    assert_eq!(catalog.map_names.len(), 12);

    let burning = catalog.quest_title("Quest_Epic_11").expect("Epic 11");
    assert_eq!(burning.name, "Burning Xion");
    assert_eq!(burning.name_zh, "燃烧的希雍");
    assert!(catalog.quest_title("Quest_Nope").is_none());
    assert_eq!(catalog.quest_titles.len(), 89);
}
