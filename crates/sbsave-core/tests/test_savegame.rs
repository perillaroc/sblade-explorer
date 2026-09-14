use std::collections::BTreeMap;

use sbsave_core::savegame::{derive_aliases, discover_saves, load_save, SaveData};
use serde_json::json;

fn real_save() -> Option<SaveData> {
    let mut slots = discover_saves(None);
    if slots.is_empty() {
        return None;
    }
    let first = slots.remove(0);
    Some(load_save(&first.path, first.steam_id.clone()).expect("load save"))
}

#[test]
fn save_parses() {
    let Some(save) = real_save() else {
        return;
    };
    assert_eq!(
        save.gvas.header.save_game_class_name,
        "/Script/SB.SBSaveGame"
    );
    assert_eq!(save.gvas.header.engine.major, 4);
    assert!(save.gvas.properties.len() >= 20);
}

#[test]
fn obtained_items() {
    let Some(save) = real_save() else {
        return;
    };
    assert!(save.obtained_items.len() > 200);
    assert!(save.obtained_items.iter().all(|alias| !alias.is_empty()));
}

#[test]
fn counters_and_achievements() {
    let Some(save) = real_save() else {
        return;
    };
    assert!(save.counters.contains_key("NewGamePlusPlayCount"));
    assert!(save.play_time_seconds() >= 0);
    assert!(save.achievements.len() > 100);
}

#[test]
fn derived_aliases_covers_camps() {
    let Some(save) = real_save() else {
        return;
    };
    let camps: Vec<&String> = save
        .derived_aliases
        .iter()
        .filter(|alias| alias.ends_with("_Camp"))
        .collect();
    assert!(
        !camps.is_empty(),
        "camps should be derived from achievement records"
    );
    let all = save.all_obtained();
    assert!(camps.iter().all(|camp| all.contains(*camp)));
}

#[test]
fn derive_aliases_rules() {
    assert_eq!(
        derive_aliases("Acquire_Item_BS_01"),
        Some("BS_01".to_string())
    );
    assert_eq!(
        derive_aliases("Ach_Album_Unlock_Item_Records_Xion_Memory_01_RealEnding"),
        Some("Item_Records_Xion_Memory_01".to_string())
    );
    assert_eq!(
        derive_aliases("ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"),
        Some("ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp".to_string())
    );
    assert_eq!(derive_aliases("Kill_Character_WLA_MiteA"), None);
    assert_eq!(
        derive_aliases("Acquire_Item_Can_048_NotMaintain"),
        Some("Can_048".to_string())
    );
}

#[test]
#[ignore = "requires SBSAVE_SUMMARY_OUT; uses SBSAVE_SAVE or a locally discovered save"]
fn dumps_real_save_summary() {
    let Ok(out) = std::env::var("SBSAVE_SUMMARY_OUT") else {
        return;
    };
    let save = match std::env::var("SBSAVE_SAVE") {
        Ok(path) => load_save(path, None).expect("load save"),
        Err(_) => match real_save() {
            Some(save) => save,
            None => return,
        },
    };

    let mut obtained: Vec<&String> = save.obtained_items.iter().collect();
    obtained.sort();
    let mut derived: Vec<&String> = save.derived_aliases.iter().collect();
    derived.sort();
    let counters: BTreeMap<&String, i64> = save.counters.iter().map(|(k, v)| (k, *v)).collect();
    let string_counters: BTreeMap<&String, &String> = save.string_counters.iter().collect();
    let friendships: BTreeMap<&String, i64> =
        save.friendships.iter().map(|(k, v)| (k, *v)).collect();
    let achievements: BTreeMap<&String, serde_json::Value> = save
        .achievements
        .iter()
        .map(|(alias, record)| {
            let mut raw: Vec<&String> = record.raw.iter().map(|(name, _)| name).collect();
            raw.sort();
            (
                alias,
                json!({
                    "completed": record.completed,
                    "progress": record.progress,
                    "raw_fields": raw,
                    "received_reward": record.received_reward,
                }),
            )
        })
        .collect();
    let shop_purchases: BTreeMap<&String, Vec<&String>> = save
        .shop_purchases
        .iter()
        .map(|(alias, entry)| {
            let mut fields: Vec<&String> = entry.fields.iter().map(|(name, _)| name).collect();
            fields.sort();
            (alias, fields)
        })
        .collect();

    let payload = json!({
        "achievements": achievements,
        "class": save.gvas.header.save_game_class_name,
        "counters": counters,
        "derived_aliases": derived,
        "difficulty": save.difficulty(),
        "friendships": friendships,
        "ng_plus": save.ng_plus_count(),
        "obtained_items": obtained,
        "play_time": save.play_time_seconds(),
        "shop_purchases": shop_purchases,
        "string_counters": string_counters,
    });

    let text = serde_json::to_string_pretty(&payload).expect("json");
    std::fs::write(&out, text).expect("write summary");
}
