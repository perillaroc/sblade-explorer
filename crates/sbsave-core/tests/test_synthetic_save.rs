//! End-to-end tests over the shared synthetic save (roadmap task 1.1).
//!
//! These run without a real save: the fixture is written byte by byte by
//! `sbsave_core::fixture`, then loaded through the same discovery → parse →
//! analyze → report JSON chain the CLI and desktop app use.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use sbsave_core::analyze::analyze;
use sbsave_core::catalog::load_catalog;
use sbsave_core::fixture::{QuestMapShape, SyntheticSave, SAVE_FILE_NAME, STEAM_ID};
use sbsave_core::report::analysis_to_dict;
use sbsave_core::savegame::{discover_saves, load_save};
use serde_json::Value;

struct ScratchDir {
    path: PathBuf,
}

impl ScratchDir {
    fn new(name: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let unique = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "sbsave-fixture-{}-{name}-{unique}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create scratch dir");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn fixture_save() -> (ScratchDir, PathBuf) {
    let dir = ScratchDir::new("save");
    let path = SyntheticSave::default()
        .write_to(dir.path())
        .expect("write fixture");
    (dir, path)
}

fn missing_item<'a>(report: &'a Value, category: &str, id: &str) -> &'a Value {
    report["categories"]
        .as_array()
        .expect("categories")
        .iter()
        .find(|entry| entry["key"] == category)
        .unwrap_or_else(|| panic!("category {category} missing"))["missing"]
        .as_array()
        .expect("missing list")
        .iter()
        .find(|item| item["id"] == id)
        .unwrap_or_else(|| panic!("item {id} is not missing"))
}

#[test]
fn fixture_parses_every_covered_field() {
    let (_dir, path) = fixture_save();
    let save = load_save(&path, None).expect("load fixture");

    assert_eq!(save.slot, 0);
    assert_eq!(save.steam_id.as_deref(), Some(STEAM_ID));
    assert_eq!(
        save.gvas.header.save_game_class_name,
        "/Script/SB.SBSaveGame"
    );

    for alias in ["Can_001", "BS_Green_Suit", "Gear_Fixture_Unmapped_001"] {
        assert!(
            save.obtained_items.contains(alias),
            "item alias {alias} should be parsed"
        );
    }
    assert!(save.achievements.contains_key("Ach_Album_Unlock_ThornHead"));
    assert_eq!(save.achievements.len(), 5);
    assert!(save.derived_aliases.contains("Can_002"));
    assert_eq!(save.ng_plus_count(), 1);
    assert_eq!(save.difficulty(), 2);
    assert_eq!(save.play_time_seconds(), 7200);
    assert_eq!(
        save.string_counters.get("MapName").map(String::as_str),
        Some("E04")
    );
    assert!(save.shop_purchases.contains_key("Shop_Kaya"));
    assert_eq!(save.friendships.get("Lily"), Some(&100));
    assert!(save.quests.completed.contains("Quest_Epic_08"));
    assert!(save.quests.failed.contains("CQ_Xion10_GetFirstCan"));
    assert!(save.quests.active.contains("Quest_Epic_Hidden"));
    assert!(!save.quests.has_started("Quest_Epic_12"));
    assert_eq!(save.current_map(), Some("E04"));
    assert_eq!(save.current_epic_quest(), Some("Quest_Epic_11"));
}

#[test]
fn current_story_hides_when_data_is_missing() {
    let dir = ScratchDir::new("no-story");
    let save = SyntheticSave {
        map_name: String::new(),
        active_quests: Vec::new(),
        ..SyntheticSave::default()
    };
    let path = save.write_to(dir.path()).expect("write fixture");
    let loaded = load_save(&path, None).expect("load fixture");
    assert_eq!(loaded.current_map(), None);
    assert_eq!(loaded.current_epic_quest(), None);
}

#[test]
fn missing_or_malformed_quest_map_yields_empty_state() {
    for shape in [QuestMapShape::Missing, QuestMapShape::Malformed] {
        let dir = ScratchDir::new(match shape {
            QuestMapShape::Missing => "quests-missing",
            QuestMapShape::Malformed => "quests-malformed",
            QuestMapShape::Normal => unreachable!(),
        });
        let save = SyntheticSave {
            quest_map: shape,
            ..SyntheticSave::default()
        };
        let path = save.write_to(dir.path()).expect("write fixture");
        let loaded = load_save(&path, None).expect("load fixture");
        assert!(loaded.quests.is_empty(), "{shape:?}");
        // Other fields must still parse.
        assert!(loaded.obtained_items.contains("Can_001"));
        assert_eq!(
            loaded.string_counters.get("MapName").map(String::as_str),
            Some("E04")
        );
    }
}

#[test]
fn discovery_loads_fixture_from_save_dirs() {
    let (dir, path) = fixture_save();
    let joined = std::env::join_paths([dir.path()]).expect("join dirs");
    // This test binary is dedicated to the fixture, so mutating the process
    // environment cannot race with other discovery tests.
    std::env::set_var("SBSAVE_SAVE_DIRS", joined);
    let slots = discover_saves(None);
    std::env::remove_var("SBSAVE_SAVE_DIRS");

    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].path, path);
    assert_eq!(slots[0].slot, 0);
    assert_eq!(slots[0].steam_id.as_deref(), Some(STEAM_ID));
    assert_eq!(slots[0].label(), format!("StellarBladeSave00 ({STEAM_ID})"));

    let loaded = load_save(&slots[0].path, slots[0].steam_id.clone()).expect("load discovered");
    assert_eq!(loaded.path, path);
}

#[test]
fn report_json_summarises_fixture() {
    let (_dir, path) = fixture_save();
    let save = load_save(&path, None).expect("load fixture");
    let catalog = load_catalog(None).expect("catalog");
    let analysis = analyze(&save, &catalog, None);
    let report = analysis_to_dict(&analysis, true);

    // Catalog sentinels plus the fixture's own progress.
    assert_eq!(report["summary"]["catalog_total"], 808);
    assert_eq!(report["summary"]["album_total"], 122);
    assert_eq!(report["summary"]["catalog_obtained"], 10);
    assert_eq!(report["summary"]["album_obtained"], 1);
    assert_eq!(report["summary"]["missing_total"], 798);
    assert_eq!(report["summary"]["percent"], 1.24);
    // 10 direct aliases + 3 derived from achievements.
    assert_eq!(report["summary"]["obtained_aliases"], 13);
    assert_eq!(report["summary"]["unmapped_aliases"], 3);
    // Quest_Epic_08 completed (Xion locked) + Quest_Epic_Hidden active (Eidos 9 open).
    assert_eq!(report["summary"]["missed_count"], 154);
    assert_eq!(report["summary"]["missable_open_count"], 29);
    assert_eq!(report["categories"].as_array().map(Vec::len), Some(16));

    let save_json = &report["save"];
    assert_eq!(save_json["steam_id"], STEAM_ID);
    assert_eq!(save_json["slot"], 0);
    assert_eq!(save_json["playthrough"], "NG+1");
    assert_eq!(save_json["ng_plus_count"], 1);
    assert_eq!(save_json["difficulty"], 2);
    assert_eq!(save_json["play_time_seconds"], 7200);

    // Blocking reasons: missable, NG++ and DLC gating respectively.
    assert_eq!(
        missing_item(&report, "cans", "Can_009")["reason"],
        "已错过（本存档无法再获取）"
    );
    assert_eq!(
        missing_item(&report, "cans", "Can_009")["missable_state"],
        "missed"
    );
    assert_eq!(missing_item(&report, "cans", "Can_009")["missable"], true);
    assert_eq!(
        missing_item(&report, "nano_suits", "BS_05_E")["reason"],
        "需要三周目(NG++)"
    );
    assert_eq!(
        missing_item(&report, "nano_suits", "BS_Nikke_06")["reason"],
        "NIKKE DLC限定"
    );
    assert_eq!(
        missing_item(&report, "records", "Item_Records_DED40_Memory_01")["missable_state"],
        "open"
    );

    let unmapped: Vec<&str> = report["unmapped_obtained"]
        .as_array()
        .expect("unmapped list")
        .iter()
        .map(|alias| alias.as_str().expect("alias string"))
        .collect();
    assert_eq!(
        unmapped,
        vec![
            "Fixture_Unmapped_Alias",
            "Gear_Fixture_Achievement",
            "Gear_Fixture_Unmapped_001",
        ]
    );
}

#[test]
fn quest_state_is_readable_from_the_fixture() {
    // 1.2 adds the parser; until then the fixture must still expose the three
    // quest lists in the GVAS tree so the parser has realistic input.
    let (_dir, path) = fixture_save();
    let save = load_save(&path, None).expect("load fixture");
    let quest_map = save.gvas.get("DataMap_SBQuest").expect("quest map");
    let sbsave_core::gvas::Value::Map(map) = &quest_map.value else {
        panic!("DataMap_SBQuest should be a map");
    };
    let quest = map
        .get(&sbsave_core::gvas::Value::Str("Quest".to_string()))
        .expect("Quest entry");
    let sbsave_core::gvas::Value::Struct(structure) = quest else {
        panic!("Quest entry should be a struct");
    };

    let aliases = |field: &str| -> Vec<String> {
        let sbsave_core::gvas::Value::Array(array) =
            &structure.get(field).expect("quest list").value
        else {
            panic!("{field} should be an array");
        };
        array
            .elements
            .iter()
            .map(|element| {
                let sbsave_core::gvas::Value::Struct(record) = element else {
                    panic!("quest record should be a struct");
                };
                match &record.get("QuestAlias").expect("QuestAlias").value {
                    sbsave_core::gvas::Value::Str(alias) => alias.clone(),
                    other => panic!("unexpected QuestAlias value: {other:?}"),
                }
            })
            .collect()
    };

    assert_eq!(
        aliases("CompleteQuestList"),
        ["Quest_Epic_08", "CQ_SD10_RR"]
    );
    assert_eq!(aliases("FailedQuestList"), ["CQ_Xion10_GetFirstCan"]);
    assert_eq!(
        aliases("ProgressQuestList"),
        [
            "Quest_Epic_11",
            "Quest_Epic_Hidden",
            "CQ_Public_ForceGreenDot"
        ]
    );
    assert_eq!(
        save.string_counters.get("MapName").map(String::as_str),
        Some("E04")
    );
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some(SAVE_FILE_NAME)
    );
}
