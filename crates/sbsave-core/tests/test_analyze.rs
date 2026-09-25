use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use sbsave_core::analyze::analyze;
use sbsave_core::catalog::load_catalog;
use sbsave_core::gvas::{EngineVersion, GvasFile, GvasHeader};
use sbsave_core::savegame::{AchievementRecord, SaveData};

fn make_header() -> GvasHeader {
    GvasHeader {
        save_game_version: 2,
        package_file_version: 522,
        engine: EngineVersion {
            major: 4,
            minor: 26,
            patch: 2,
            changelist: 0,
            branch: "++UE4+Release-4.26".to_string(),
        },
        custom_version_format: 3,
        custom_versions: Vec::new(),
        save_game_class_name: "/Script/SB.SBSaveGame".to_string(),
        has_evas_prefix: false,
    }
}

fn make_save(obtained: &[&str], derived: &[&str], ng_plus: i64) -> SaveData {
    SaveData {
        path: PathBuf::from("StellarBladeSave00.sav"),
        steam_id: Some("76561198000000000".to_string()),
        slot: 0,
        gvas: GvasFile {
            header: make_header(),
            properties: Vec::new(),
            footer: Vec::new(),
        },
        obtained_items: obtained
            .iter()
            .map(|alias| alias.to_string())
            .collect::<HashSet<_>>(),
        derived_aliases: derived.iter().map(|alias| alias.to_string()).collect(),
        counters: HashMap::from([("NewGamePlusPlayCount".to_string(), ng_plus)]),
        string_counters: HashMap::new(),
        achievements: HashMap::new(),
        shop_purchases: HashMap::new(),
        friendships: HashMap::new(),
    }
}

fn with_achievements(mut save: SaveData, aliases: &[&str]) -> SaveData {
    for alias in aliases {
        save.achievements.insert(
            alias.to_string(),
            AchievementRecord {
                alias: alias.to_string(),
                ..AchievementRecord::default()
            },
        );
    }
    save
}

#[test]
fn missing_detection() {
    let save = make_save(&["Can_001", "Can_002"], &[], 0);
    let catalog = load_catalog(None).expect("catalog");
    let result = analyze(&save, &catalog, None);
    let cans = result
        .categories
        .iter()
        .find(|category| category.category.key == "cans")
        .expect("cans");
    assert_eq!(cans.obtained_count, 2);
    assert_eq!(cans.missing.len(), 47);
    assert!(cans.missing.iter().all(|status| status.reason.is_none()));
}

#[test]
fn ng_plus_reason() {
    let catalog = load_catalog(None).expect("catalog");
    let ng_item = catalog
        .items
        .iter()
        .find(|item| item.ng_plus == 1 && item.category == "nano_suits")
        .expect("ng item");

    let save = make_save(&[], &[], 0);
    let filter = vec!["nano_suits".to_string()];
    let result = analyze(&save, &catalog, Some(filter.as_slice()));
    let missing = result.categories[0]
        .missing
        .iter()
        .find(|status| status.item.id == ng_item.id)
        .expect("ng missing");
    assert_eq!(missing.reason.as_deref(), Some("需要二周目(NG+)"));

    let save_ng = make_save(&[], &[], 1);
    let result_ng = analyze(&save_ng, &catalog, Some(filter.as_slice()));
    let missing_ng = result_ng.categories[0]
        .missing
        .iter()
        .find(|status| status.item.id == ng_item.id)
        .expect("ng missing");
    assert!(missing_ng.reason.is_none());
}

#[test]
fn derived_aliases_count_as_obtained() {
    let catalog = load_catalog(None).expect("catalog");
    let camp = catalog
        .items
        .iter()
        .find(|item| item.category == "camps")
        .expect("camp");
    let derived: Vec<&str> = camp.satisfy_aliases();

    let save = make_save(&[], &derived, 0);
    let filter = vec!["camps".to_string()];
    let result = analyze(&save, &catalog, Some(filter.as_slice()));
    assert_eq!(result.categories[0].obtained_count, 1);
}

#[test]
fn extra_obtained_aliases() {
    let save = make_save(&["Can_001", "Can_999"], &[], 0);
    let catalog = load_catalog(None).expect("catalog");
    let filter = vec!["cans".to_string()];
    let result = analyze(&save, &catalog, Some(filter.as_slice()));
    assert_eq!(
        result.categories[0].extra_obtained,
        vec!["Can_999".to_string()]
    );
}

#[test]
fn unmapped_aliases() {
    let save = make_save(&["Weird_Alias_123"], &[], 0);
    let catalog = load_catalog(None).expect("catalog");
    let filter = vec!["cans".to_string()];
    let result = analyze(&save, &catalog, Some(filter.as_slice()));
    assert_eq!(
        result.unmapped_obtained,
        vec!["Weird_Alias_123".to_string()]
    );
}

#[test]
fn album_achievements_count_as_obtained() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_achievements(
        make_save(&[], &[], 0),
        &["Ach_Album_Unlock_ThornHead", "Ach_Album_Unlock_Adam_3"],
    );
    let result = analyze(&save, &catalog, None);

    let naytiba = result
        .categories
        .iter()
        .find(|category| category.category.key == "naytiba")
        .expect("naytiba");
    assert_eq!(naytiba.total, 67);
    assert_eq!(naytiba.obtained_count, 1);
    assert_eq!(naytiba.missing.len(), 66);

    let characters = result
        .categories
        .iter()
        .find(|category| category.category.key == "characters")
        .expect("characters");
    assert_eq!(characters.total, 55);
    assert_eq!(characters.obtained_count, 1);

    // 图鉴不计入目录进度
    assert_eq!(result.catalog_total, 810);
    assert_eq!(result.catalog_obtained, 0);
    assert_eq!(result.missing_total(), 810);
    assert_eq!(result.album_total, 122);
    assert_eq!(result.album_obtained, 2);
    assert_eq!(result.album_missing_total(), 120);
    assert!(result.unmapped_obtained.is_empty());

    let filter = vec!["naytiba".to_string()];
    let album_only = analyze(&save, &catalog, Some(filter.as_slice()));
    assert_eq!(album_only.catalog_total, 0);
    assert_eq!(album_only.album_total, 67);
    assert_eq!(album_only.album_obtained, 1);
}
