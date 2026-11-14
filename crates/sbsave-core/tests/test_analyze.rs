use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use sbsave_core::analyze::{
    analyze, analyze_with_dlcs, normalize_owned_dlcs, Analysis, BlockReason, ItemStatus,
    MissableState,
};
use sbsave_core::catalog::load_catalog;
use sbsave_core::gvas::{EngineVersion, GvasFile, GvasHeader};
use sbsave_core::i18n::Locale;
use sbsave_core::savegame::{AchievementRecord, QuestState, SaveData};

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
        quests: QuestState::default(),
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
    // 希雍城区（Can_003/009/011/019/027/036/039/045）与任务解锁区域（Can_025）的罐子标记易错过
    let mut blocked: Vec<&str> = cans
        .missing
        .iter()
        .filter(|status| status.reason.is_some())
        .map(|status| status.item.id.as_str())
        .collect();
    blocked.sort_unstable();
    assert_eq!(
        blocked,
        [
            "Can_003", "Can_009", "Can_011", "Can_019", "Can_025", "Can_027", "Can_036", "Can_039",
            "Can_045"
        ]
    );
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
    assert_eq!(
        missing.reason_label(Locale::Zh).as_deref(),
        Some("需要二周目(NG+)")
    );
    assert_eq!(
        missing.reason_label(Locale::En).as_deref(),
        Some("Requires NG+")
    );

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
fn missable_reason() {
    let catalog = load_catalog(None).expect("catalog");
    let missable_item = catalog
        .items
        .iter()
        .find(|item| item.missable && item.category == "records")
        .expect("missable record");

    let save = make_save(&[], &[], 0);
    let filter = vec!["records".to_string()];
    let result = analyze(&save, &catalog, Some(filter.as_slice()));
    let missing = result.categories[0]
        .missing
        .iter()
        .find(|status| status.item.id == missable_item.id)
        .expect("missable missing");
    assert_eq!(
        missing.reason_label(Locale::Zh).as_deref(),
        Some("可错过(注意节点)")
    );
    assert_eq!(
        missing.reason_label(Locale::En).as_deref(),
        Some("Missable (mind the checkpoint)")
    );
    let flags = missing.flags(Locale::Zh);
    assert!(flags.iter().any(|flag| flag == "可错过"), "{flags:?}");
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
    assert_eq!(result.catalog_total, 808);
    assert_eq!(result.catalog_obtained, 0);
    assert_eq!(result.missing_total(), 808);
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

#[test]
fn growth_items_count_separately_from_collection_progress() {
    let catalog = load_catalog(None).expect("catalog");
    // Mk2 is an upgrade of the same gear entry: it counts as obtained.
    let save = make_save(&["Gear_AdditiveFixedDamage_Common_MK2"], &[], 0);
    let result = analyze(&save, &catalog, None);

    let gear = result
        .categories
        .iter()
        .find(|category| category.category.key == "gear")
        .expect("gear");
    assert_eq!(gear.total, 77);
    assert_eq!(gear.obtained_count, 1);
    assert_eq!(gear.missing.len(), 76);

    // Collection rate and missing totals stay on the 13 collection categories.
    assert_eq!(result.catalog_total, 808);
    assert_eq!(result.catalog_obtained, 0);
    assert_eq!(result.missing_total(), 808);
    assert_eq!(result.growth_total, 89);
    assert_eq!(result.growth_obtained, 1);
    assert_eq!(result.growth_missing_total(), 88);
    assert!((result.growth_percent() - 100.0 / 89.0).abs() < 1e-9);
    assert!(result.unmapped_obtained.is_empty());
}

#[test]
fn exospine_enhance_state_counts_as_obtained() {
    let catalog = load_catalog(None).expect("catalog");
    // `PT_Combo` comes from ItemEnhanceState.ExospineEnhanceStateMap; the
    // catalog entry also accepts the NG+ Mk2 alias.
    let save = make_save(&["PT_Combo", "PT_Survive_MK2"], &[], 0);
    let result = analyze(&save, &catalog, None);

    let exospines = result
        .categories
        .iter()
        .find(|category| category.category.key == "exospines")
        .expect("exospines");
    assert_eq!(exospines.total, 12);
    assert_eq!(exospines.obtained_count, 2);
    assert_eq!(exospines.missing.len(), 10);
    assert!(result.unmapped_obtained.is_empty());
}

fn with_quests(
    mut save: SaveData,
    completed: &[&str],
    failed: &[&str],
    active: &[&str],
) -> SaveData {
    let set = |aliases: &[&str]| aliases.iter().map(|alias| alias.to_string()).collect();
    save.quests = QuestState {
        completed: set(completed),
        failed: set(failed),
        active: set(active),
    };
    save
}

fn status_for<'a>(result: &'a Analysis<'_>, id: &str) -> &'a ItemStatus<'a> {
    result
        .categories
        .iter()
        .flat_map(|category| &category.statuses)
        .find(|status| status.item.id == id)
        .unwrap_or_else(|| panic!("status for {id}"))
}

#[test]
fn xion_items_are_missed_after_epic_08() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_quests(make_save(&[], &[], 0), &["Quest_Epic_08"], &[], &[]);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "Can_009");

    assert_eq!(status.missable_state, MissableState::Missed);
    assert_eq!(status.reason, Some(BlockReason::Missed));
    assert_eq!(
        status.reason_label(Locale::Zh).as_deref(),
        Some("已错过（本存档无法再获取）")
    );
    assert_eq!(
        status.reason_label(Locale::En).as_deref(),
        Some("Missed (unobtainable in this save)")
    );
    assert!(status.flags(Locale::Zh).iter().any(|flag| flag == "已错过"));
    assert!(result.missed_count() > 0);
}

#[test]
fn xion_items_stay_open_before_epic_08() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_quests(make_save(&[], &[], 0), &["Quest_Epic_07"], &[], &[]);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "Can_009");

    assert_eq!(status.missable_state, MissableState::Open);
    assert_eq!(status.reason, Some(BlockReason::Missable));
    assert_eq!(
        status.reason_label(Locale::Zh).as_deref(),
        Some("可错过（仍可获得）")
    );
    assert_eq!(result.missed_count(), 0);
    assert!(result.missable_open_count() > 0);
}

#[test]
fn eidos9_items_open_after_hidden_quest_started() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_quests(make_save(&[], &[], 0), &[], &[], &["Quest_Epic_Hidden"]);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "Item_Records_DED40_Memory_01");

    assert_eq!(status.missable_state, MissableState::Open);
    assert_eq!(status.reason, Some(BlockReason::Missable));
    assert!(result.missable_open_count() > 0);
}

#[test]
fn eidos9_items_stay_unknown_before_hidden_quest() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_quests(make_save(&[], &[], 0), &["Quest_Epic_08"], &[], &[]);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "Item_Records_DED40_Memory_01");

    assert_eq!(status.missable_state, MissableState::Unknown);
    assert_eq!(
        status.reason_label(Locale::Zh).as_deref(),
        Some("可错过(注意节点)")
    );
}

#[test]
fn entries_without_rules_fall_back_to_unknown() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_quests(make_save(&[], &[], 0), &["Quest_Epic_08"], &[], &[]);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "BS_18");

    assert!(status.item.missable);
    assert_eq!(status.missable_state, MissableState::Unknown);
    assert_eq!(status.reason, Some(BlockReason::Missable));
    assert!(status.flags(Locale::Zh).iter().any(|flag| flag == "可错过"));
}

#[test]
fn empty_quest_state_never_reports_missed() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let result = analyze(&save, &catalog, None);
    let status = status_for(&result, "Can_009");

    assert_eq!(status.missable_state, MissableState::Unknown);
    assert_eq!(result.missed_count(), 0);
}

fn with_story(mut save: SaveData, map: Option<&str>, active: &[&str]) -> SaveData {
    if let Some(map) = map {
        save.string_counters
            .insert("MapName".to_string(), map.to_string());
    }
    save.quests = QuestState {
        completed: HashSet::new(),
        failed: HashSet::new(),
        active: active.iter().map(|alias| alias.to_string()).collect(),
    };
    save
}

#[test]
fn analysis_resolves_current_area_and_main_quest() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_story(make_save(&[], &[], 0), Some("E04"), &["Quest_Epic_11"]);
    let result = analyze(&save, &catalog, None);

    assert_eq!(result.story.map_code, Some("E04"));
    assert_eq!(
        result.story.map.map(|name| name.name.as_str()),
        Some("Xion")
    );
    assert_eq!(
        result.story.map.map(|name| name.name_zh.as_str()),
        Some("希雍")
    );
    assert_eq!(result.story.epic_quest, Some("Quest_Epic_11"));
    assert_eq!(
        result
            .story
            .epic_quest_title
            .map(|title| title.name.as_str()),
        Some("Burning Xion")
    );
}

#[test]
fn analysis_falls_back_to_raw_codes_when_names_are_unknown() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_story(make_save(&[], &[], 0), Some("E99"), &["Quest_Epic_99"]);
    let result = analyze(&save, &catalog, None);

    assert_eq!(result.story.map_code, Some("E99"));
    assert!(result.story.map.is_none());
    assert_eq!(result.story.epic_quest, Some("Quest_Epic_99"));
    assert!(result.story.epic_quest_title.is_none());
}

#[test]
fn analysis_hides_story_when_save_has_no_map_or_epic_quest() {
    let catalog = load_catalog(None).expect("catalog");
    let save = with_story(make_save(&[], &[], 0), None, &["CQ_Public_ForceGreenDot"]);
    let result = analyze(&save, &catalog, None);

    assert!(result.story.map_code.is_none());
    assert!(result.story.map.is_none());
    assert!(result.story.epic_quest.is_none());
    assert!(result.story.epic_quest_title.is_none());
}

#[test]
fn owned_dlc_is_not_reported_as_dlc_blocked() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);

    let unowned = analyze(&save, &catalog, None);
    let nikke = status_for(&unowned, "BS_Nikke_06");
    assert_eq!(nikke.reason, Some(BlockReason::Dlc("nikke".to_string())));
    assert!(!nikke.dlc_owned);

    let owned = vec!["nikke".to_string()];
    let result = analyze_with_dlcs(&save, &catalog, None, &owned);
    assert_eq!(result.owned_dlcs, vec!["nikke".to_string()]);

    let nikke = status_for(&result, "BS_Nikke_06");
    assert!(nikke.dlc_owned);
    assert_eq!(nikke.reason, None);
    assert!(nikke
        .flags(Locale::Zh)
        .iter()
        .any(|flag| flag.contains("已拥有")));

    let nier = status_for(&result, "BS_Nier_01");
    assert!(!nier.dlc_owned);
    assert_eq!(nier.reason, Some(BlockReason::Dlc("nier".to_string())));

    let nikke_items = result
        .categories
        .iter()
        .flat_map(|category| &category.statuses)
        .filter(|status| status.item.dlc.as_deref() == Some("nikke"))
        .count();
    assert_eq!(nikke_items, 18);
    let blocked_nikke = result
        .categories
        .iter()
        .flat_map(|category| &category.statuses)
        .filter(|status| {
            status.item.dlc.as_deref() == Some("nikke")
                && matches!(status.reason, Some(BlockReason::Dlc(_)))
        })
        .count();
    assert_eq!(blocked_nikke, 0);
}

#[test]
fn owned_dlc_normalization_filters_unknown_and_duplicates() {
    assert_eq!(
        normalize_owned_dlcs(["NIKKE", "nier", "nope", "nier"]),
        vec!["nier".to_string(), "nikke".to_string()]
    );
    assert!(normalize_owned_dlcs(Vec::<String>::new()).is_empty());
}
