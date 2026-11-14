//! Deterministic synthetic GVAS saves for tests.
//!
//! The bytes are built property by property (the same approach as
//! `tests/test_gvas.rs`) so the core, CLI and desktop suites can exercise the
//! full pipeline — discovery, parsing, analysis and report serialization —
//! without a real save. Nothing here is used by the shipped application; the
//! module only exists so tests share one fixture instead of duplicating
//! byte-level constructors.
//!
//! The default fixture covers everything the save-aware roadmap needs:
//! item aliases, achievements (including an album entry), NG+ counters, shop
//! purchases and friendships, unmapped aliases, `DataMap_SBQuest` lists and
//! `DataMap_FString.MapName`.

use std::io;
use std::path::{Path, PathBuf};

/// Steam id used for the fixture save directory name.
pub const STEAM_ID: &str = "76561198000000000";
/// File name the game uses for the main slot.
pub const SAVE_FILE_NAME: &str = "StellarBladeSave00.sav";
const SAVE_GAME_CLASS: &str = "/Script/SB.SBSaveGame";
const ENGINE_BRANCH: &str = "++UE4+Release-4.26";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticAchievement {
    pub alias: String,
    pub progress: i32,
    pub completed: bool,
    pub received_reward: bool,
}

impl SyntheticAchievement {
    pub fn new(alias: &str, progress: i32, completed: bool) -> Self {
        Self {
            alias: alias.to_string(),
            progress,
            completed,
            received_reward: completed,
        }
    }
}

/// Shape of the `DataMap_SBQuest` property in the fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuestMapShape {
    /// A well formed map with the three quest lists.
    #[default]
    Normal,
    /// No `DataMap_SBQuest` property at all (older saves).
    Missing,
    /// Present but not a map of structs (unexpected type).
    Malformed,
}

/// One synthetic save. The default value is the shared baseline fixture; tests
/// that need a different quest state clone it and edit the relevant vector.
#[derive(Debug, Clone)]
pub struct SyntheticSave {
    pub obtained_items: Vec<String>,
    pub achievements: Vec<SyntheticAchievement>,
    pub completed_quests: Vec<String>,
    pub failed_quests: Vec<String>,
    pub active_quests: Vec<String>,
    pub quest_map: QuestMapShape,
    pub map_name: String,
    pub ng_plus_count: i32,
    pub difficulty: i32,
    pub play_time_seconds: i32,
}

impl Default for SyntheticSave {
    fn default() -> Self {
        Self {
            obtained_items: vec![
                "Can_001",
                "BS_Green_Suit",
                "Hair_000",
                "Earring_001",
                "DroneSeal_001",
                "AdamCostume_001",
                "LilyCostume_001",
                "Item_Records_Xion_Memory_01",
                // Deliberately not in the catalog: exercises `unmapped_aliases`.
                "Gear_Fixture_Unmapped_001",
                "Fixture_Unmapped_Alias",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
            achievements: vec![
                // Album entry: satisfied through the achievement alias itself.
                SyntheticAchievement::new("Ach_Album_Unlock_ThornHead", 1, true),
                // Derived alias `Can_002`.
                SyntheticAchievement::new("Acquire_Item_Can_002", 1, true),
                // Derived alias `Item_Records_Xion_Memory_03`.
                SyntheticAchievement::new(
                    "Acquire_Item_Item_Records_Xion_Memory_03_Maintain",
                    1,
                    true,
                ),
                // Derived alias `Gear_Fixture_Achievement` is unmapped too.
                SyntheticAchievement::new("Acquire_Item_Gear_Fixture_Achievement", 3, false),
                // Non-item achievement: no derived alias.
                SyntheticAchievement::new("Trophy_KillCharacter_AllNative", 63, false),
            ],
            completed_quests: vec!["Quest_Epic_08".to_string(), "CQ_SD10_RR".to_string()],
            failed_quests: vec!["CQ_Xion10_GetFirstCan".to_string()],
            active_quests: vec![
                "Quest_Epic_11".to_string(),
                "Quest_Epic_Hidden".to_string(),
                "CQ_Public_ForceGreenDot".to_string(),
            ],
            quest_map: QuestMapShape::Normal,
            map_name: "E04".to_string(),
            ng_plus_count: 1,
            difficulty: 2,
            play_time_seconds: 7200,
        }
    }
}

impl SyntheticSave {
    /// Serializes the save to GVAS bytes.
    pub fn bytes(&self) -> Vec<u8> {
        let obtained: Vec<&str> = self.obtained_items.iter().map(String::as_str).collect();
        let item_struct = struct_body(&[set_prop("ItemOtaineSet", "NameProperty", &obtained)]);
        let item_map = map_prop(
            "DataMap_SBItem",
            "NameProperty",
            "StructProperty",
            &[(fstr("Item"), item_struct)],
        );

        let quest_struct = struct_body(&[
            quest_list("CompleteQuestList", &self.completed_quests, 3),
            quest_list("FailedQuestList", &self.failed_quests, 2),
            quest_list("ProgressQuestList", &self.active_quests, 1),
        ]);
        let quest_map = match self.quest_map {
            QuestMapShape::Normal => Some(map_prop(
                "DataMap_SBQuest",
                "NameProperty",
                "StructProperty",
                &[(fstr("Quest"), quest_struct)],
            )),
            QuestMapShape::Missing => None,
            QuestMapShape::Malformed => Some(str_prop("DataMap_SBQuest", "not-a-map")),
        };

        let string_map = map_prop(
            "DataMap_FString",
            "NameProperty",
            "StrProperty",
            &[(fstr("MapName"), fstr(&self.map_name))],
        );

        let int_entries: Vec<(Vec<u8>, Vec<u8>)> = [
            ("Version", 1),
            ("GameDifficulty", self.difficulty),
            ("NewGamePlusPlayCount", self.ng_plus_count),
            ("PlayTime", self.play_time_seconds),
        ]
        .into_iter()
        .map(|(key, value)| (fstr(key), value.to_le_bytes().to_vec()))
        .collect();
        let int_map = map_prop("DataMap_int32", "NameProperty", "IntProperty", &int_entries);

        let achievement_elements: Vec<Vec<u8>> = self
            .achievements
            .iter()
            .map(|achievement| {
                struct_body(&[
                    str_prop("AchievementAlias", &achievement.alias),
                    int_prop("ProgressValue", achievement.progress),
                    bool_prop("bCompleted", achievement.completed),
                    bool_prop("bRecievedReward", achievement.received_reward),
                ])
            })
            .collect();
        let achievements = struct_prop(
            "SBAchievement",
            "SBAchievementData",
            &struct_body(&[struct_array_prop(
                "AchievementList",
                "SBAchievementRecord",
                &achievement_elements,
            )]),
        );

        let purchase_items = vec![
            struct_body(&[
                str_prop("ItemAlias", "SQ_Item_Mindmap_Part1"),
                int_prop("PurchaseCount", 1),
            ]),
            struct_body(&[
                str_prop("ItemAlias", "Gear_ShieldIgnorePercentage_Uncommon"),
                int_prop("PurchaseCount", 1),
            ]),
        ];
        let shop = struct_prop(
            "SBShopHistory",
            "SBShopHistoryData",
            &struct_body(&[
                map_prop(
                    "Purchase",
                    "NameProperty",
                    "StructProperty",
                    &[(
                        fstr("Shop_Kaya"),
                        struct_body(&[struct_array_prop(
                            "Array",
                            "SBShopPurchase",
                            &purchase_items,
                        )]),
                    )],
                ),
                map_prop(
                    "FriendShip",
                    "NameProperty",
                    "IntProperty",
                    &[(fstr("Lily"), 100i32.to_le_bytes().to_vec())],
                ),
            ]),
        );

        let mut payload = header();
        let mut properties = Vec::new();
        if let Some(quest_map) = quest_map {
            properties.push(quest_map);
        }
        properties.extend([item_map, string_map, int_map, achievements, shop]);
        // Unknown properties must be ignored by the parser (new game fields).
        properties.push(str_prop("UnknownFixtureProperty", "ignored"));
        for property in properties {
            payload.extend_from_slice(&property);
        }
        payload.extend_from_slice(&fstr("None"));
        payload.extend_from_slice(&[0u8; 4]);
        payload
    }

    /// Writes `<directory>/<steam id>/StellarBladeSave00.sav` and returns the
    /// file path, which is what `discover_saves` expects.
    pub fn write_to(&self, directory: &Path) -> io::Result<PathBuf> {
        let save_dir = directory.join(STEAM_ID);
        std::fs::create_dir_all(&save_dir)?;
        let path = save_dir.join(SAVE_FILE_NAME);
        std::fs::write(&path, self.bytes())?;
        Ok(path)
    }
}

/// Bytes of the default synthetic save.
pub fn synthetic_save_bytes() -> Vec<u8> {
    SyntheticSave::default().bytes()
}

/// Writes the default synthetic save into `directory`.
pub fn write_synthetic_save(directory: &Path) -> io::Result<PathBuf> {
    SyntheticSave::default().write_to(directory)
}

// -- Byte-level GVAS construction -------------------------------------------

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
    data.extend_from_slice(&fstr(ENGINE_BRANCH));
    data.extend_from_slice(&3u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&fstr(SAVE_GAME_CLASS));
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

fn struct_body(properties: &[Vec<u8>]) -> Vec<u8> {
    let mut out: Vec<u8> = properties.concat();
    out.extend_from_slice(&fstr("None"));
    out
}

fn int_prop(name: &str, value: i32) -> Vec<u8> {
    tag(name, "IntProperty", &value.to_le_bytes(), b"")
}

fn str_prop(name: &str, value: &str) -> Vec<u8> {
    tag(name, "StrProperty", &fstr(value), b"")
}

fn bool_prop(name: &str, value: bool) -> Vec<u8> {
    tag(name, "BoolProperty", &[u8::from(value)], b"")
}

fn byte_prop(name: &str, value: u8) -> Vec<u8> {
    tag(name, "ByteProperty", &[value], &fstr("None"))
}

fn struct_prop(name: &str, struct_name: &str, body: &[u8]) -> Vec<u8> {
    let mut extras = fstr(struct_name);
    extras.extend_from_slice(&[0u8; 16]);
    tag(name, "StructProperty", body, &extras)
}

/// Set of `NameProperty` values. The fixture never writes an empty set because
/// the parser reads the first `int32` as an optional removed-count prefix.
fn set_prop(name: &str, inner_type: &str, values: &[&str]) -> Vec<u8> {
    assert!(!values.is_empty(), "synthetic sets must not be empty");
    let mut value = (values.len() as i32).to_le_bytes().to_vec();
    for entry in values {
        value.extend_from_slice(&fstr(entry));
    }
    tag(name, "SetProperty", &value, &fstr(inner_type))
}

/// Array of structs: count, one element tag, then the struct bodies back to back.
fn struct_array_prop(name: &str, element_struct: &str, elements: &[Vec<u8>]) -> Vec<u8> {
    let bodies: Vec<u8> = elements.concat();
    let mut value = (elements.len() as i32).to_le_bytes().to_vec();
    value.extend_from_slice(&fstr(name));
    value.extend_from_slice(&fstr("StructProperty"));
    value.extend_from_slice(&(bodies.len() as u64).to_le_bytes());
    value.extend_from_slice(&fstr(element_struct));
    value.extend_from_slice(&[0u8; 16]);
    value.push(0);
    value.extend_from_slice(&bodies);
    tag(name, "ArrayProperty", &value, &fstr("StructProperty"))
}

fn map_prop(
    name: &str,
    key_type: &str,
    value_type: &str,
    entries: &[(Vec<u8>, Vec<u8>)],
) -> Vec<u8> {
    let mut value = 0i32.to_le_bytes().to_vec();
    value.extend_from_slice(&(entries.len() as i32).to_le_bytes());
    for (key, entry) in entries {
        value.extend_from_slice(key);
        value.extend_from_slice(entry);
    }
    let mut extras = fstr(key_type);
    extras.extend_from_slice(&fstr(value_type));
    tag(name, "MapProperty", &value, &extras)
}

fn quest_list(name: &str, aliases: &[String], state: u8) -> Vec<u8> {
    let elements: Vec<Vec<u8>> = aliases
        .iter()
        .map(|alias| {
            struct_body(&[
                str_prop("QuestAlias", alias),
                byte_prop("QuestState", state),
                // Fields a newer game build may add: the parser must skip them.
                int_prop("UnknownQuestField", 0),
            ])
        })
        .collect();
    struct_array_prop(name, "SBQuestData", &elements)
}
