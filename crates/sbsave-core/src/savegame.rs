use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::gvas::{GvasError, GvasFile, Property, Value};
use crate::paths;

pub const SAVE_GAME_CLASS: &str = "/Script/SB.SBSaveGame";

const ACHIEVEMENT_PREFIXES: [&str; 2] = ["Ach_Album_Unlock_", "Acquire_Item_"];
const ACHIEVEMENT_SUFFIXES: [&str; 6] = [
    "_NotMaintainNewGamePlus",
    "_NotMaintain",
    "_RealEnding",
    "_Maintain",
    "_Popup",
    "_Used",
];
const ITEM_ALIAS_PREFIXES: [&str; 13] = [
    "BS_",
    "Can_",
    "Hair_",
    "Earring_",
    "FaceAccessory_",
    "DroneSeal_",
    "DesignPattern_",
    "Fish_",
    "AdamCostume_",
    "LilyCostume_",
    "Gear_",
    "Item_Records_",
    "ChangeState_ZoneEnv_",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveError(pub String);

impl fmt::Display for SaveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for SaveError {}

impl From<GvasError> for SaveError {
    fn from(error: GvasError) -> Self {
        Self(error.to_string())
    }
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct SaveSlot {
    pub path: PathBuf,
    pub steam_id: Option<String>,
    pub slot: u32,
    pub mtime: SystemTime,
}

impl SaveSlot {
    pub fn label(&self) -> String {
        format!(
            "StellarBladeSave{:02} ({})",
            self.slot,
            self.steam_id.as_deref().unwrap_or("unknown")
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct AchievementRecord {
    pub alias: String,
    pub progress: i64,
    pub completed: bool,
    pub received_reward: bool,
    pub raw: Vec<(String, Value)>,
}

#[derive(Debug, Clone, Default)]
pub struct ShopEntry {
    pub alias: String,
    pub fields: Vec<(String, Value)>,
}

#[derive(Debug)]
pub struct SaveData {
    pub path: PathBuf,
    pub steam_id: Option<String>,
    pub slot: u32,
    pub gvas: GvasFile,
    pub obtained_items: HashSet<String>,
    pub derived_aliases: HashSet<String>,
    pub counters: HashMap<String, i64>,
    pub string_counters: HashMap<String, String>,
    pub achievements: HashMap<String, AchievementRecord>,
    pub shop_purchases: HashMap<String, ShopEntry>,
    pub friendships: HashMap<String, i64>,
}

impl SaveData {
    pub fn all_obtained(&self) -> HashSet<String> {
        self.obtained_items
            .union(&self.derived_aliases)
            .cloned()
            .collect()
    }

    pub fn ng_plus_count(&self) -> i64 {
        self.counters
            .get("NewGamePlusPlayCount")
            .copied()
            .unwrap_or(0)
    }

    pub fn difficulty(&self) -> i64 {
        self.counters.get("GameDifficulty").copied().unwrap_or(1)
    }

    pub fn play_time_seconds(&self) -> i64 {
        self.counters.get("PlayTime").copied().unwrap_or(0)
    }

    pub fn save_version(&self) -> i64 {
        self.counters.get("Version").copied().unwrap_or(1)
    }

    pub fn playthrough_label(&self) -> String {
        let count = self.ng_plus_count();
        if count <= 0 {
            "一周目".to_string()
        } else {
            format!("NG+{count}")
        }
    }

    pub fn play_time_label(&self) -> String {
        let seconds = self.play_time_seconds();
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{hours}小时{minutes:02}分")
    }

    pub fn difficulty_label(&self) -> String {
        match self.difficulty() {
            0 => "简单".to_string(),
            1 => "普通".to_string(),
            2 => "困难".to_string(),
            other => format!("未知({other})"),
        }
    }

    pub fn has_item(&self, alias: &str) -> bool {
        self.obtained_items.contains(alias) || self.derived_aliases.contains(alias)
    }
}

pub fn derive_aliases(alias: &str) -> Option<String> {
    let mut base = alias.to_string();
    for prefix in ACHIEVEMENT_PREFIXES {
        if let Some(rest) = base.strip_prefix(prefix) {
            base = rest.to_string();
            break;
        }
    }
    loop {
        let mut changed = false;
        for suffix in ACHIEVEMENT_SUFFIXES {
            if base.ends_with(suffix) {
                let length = base.len() - suffix.len();
                base.truncate(length);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    if base.is_empty()
        || !ITEM_ALIAS_PREFIXES
            .iter()
            .any(|prefix| base.starts_with(prefix))
    {
        return None;
    }
    if base.starts_with("ChangeState_ZoneEnv_") && !base.ends_with("_Camp") {
        return None;
    }
    Some(base)
}

fn appdata_paths() -> Vec<PathBuf> {
    let home = paths::home_dir();
    let local = paths::local_app_data();
    vec![
        local.join("SB").join("Saved").join("SaveGames"),
        home.join("Documents").join("StellarBlade"),
        local.join("SB_Demo").join("Saved").join("SaveGames"),
    ]
}

fn slot_from_name(name: &str) -> Option<u32> {
    let stem = Path::new(name).file_stem()?.to_str()?;
    for prefix in ["StellarBladeSaveDemo", "StellarBladeSave"] {
        if let Some(suffix) = stem.strip_prefix(prefix) {
            if !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()) {
                return suffix.parse().ok();
            }
        }
    }
    None
}

fn is_steam_id(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|ch| ch.is_ascii_digit())
}

pub fn discover_saves(extra_dirs: Option<&[PathBuf]>) -> Vec<SaveSlot> {
    let mut dirs: Vec<PathBuf> = extra_dirs.map(<[PathBuf]>::to_vec).unwrap_or_default();
    dirs.extend(appdata_paths());

    let mut found: Vec<SaveSlot> = Vec::new();
    for directory in dirs {
        if !directory.is_dir() {
            continue;
        }
        for entry in walkdir::WalkDir::new(&directory)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !name.starts_with("StellarBladeSave") || !name.ends_with(".sav") {
                continue;
            }
            if path
                .components()
                .any(|component| component.as_os_str() == "Backup")
            {
                continue;
            }
            let Some(slot) = slot_from_name(name) else {
                continue;
            };
            let steam_id = path
                .parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .filter(|name| is_steam_id(name))
                .map(str::to_string);
            let Ok(metadata) = path.metadata() else {
                continue;
            };
            let Ok(mtime) = metadata.modified() else {
                continue;
            };
            found.push(SaveSlot {
                path: path.to_path_buf(),
                steam_id,
                slot,
                mtime,
            });
        }
    }
    found.sort_by(|left, right| right.mtime.cmp(&left.mtime));
    found
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Raw(_) => "RawValue",
        Value::Struct(_) => "StructValue",
        Value::Array(_) => "ArrayValue",
        Value::Set(_) => "SetValue",
        Value::Map(_) => "MapValue",
        Value::Bool(_) => "bool",
        Value::Byte(_) | Value::Int8(_) | Value::Int16(_) | Value::Int(_) | Value::Int64(_) => {
            "int"
        }
        Value::UInt16(_) | Value::UInt32(_) | Value::UInt64(_) => "int",
        Value::Float(_) | Value::Double(_) => "float",
        Value::Str(_) => "str",
    }
}

fn as_struct_value(value: &Value) -> Result<&crate::gvas::StructValue, SaveError> {
    match value {
        Value::Struct(structure) => Ok(structure),
        other => Err(SaveError(format!(
            "expected struct, got {}",
            value_type_name(other)
        ))),
    }
}

fn require<'a>(property: Option<&'a Property>, name: &str) -> Result<&'a Property, SaveError> {
    property.ok_or_else(|| SaveError(format!("save is missing expected data: {name}")))
}

fn int_value(value: &Value) -> Option<i64> {
    match value {
        Value::Bool(value) => Some(i64::from(*value)),
        Value::Byte(value) => Some(i64::from(*value)),
        Value::Int8(value) => Some(i64::from(*value)),
        Value::Int16(value) => Some(i64::from(*value)),
        Value::Int(value) => Some(i64::from(*value)),
        Value::Int64(value) => Some(*value),
        Value::UInt16(value) => Some(i64::from(*value)),
        Value::UInt32(value) => Some(i64::from(*value)),
        Value::UInt64(value) => i64::try_from(*value).ok(),
        _ => None,
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(value) => *value,
        Value::Byte(value) => *value != 0,
        Value::Int8(value) => *value != 0,
        Value::Int16(value) => *value != 0,
        Value::Int(value) => *value != 0,
        Value::Int64(value) => *value != 0,
        Value::UInt16(value) => *value != 0,
        Value::UInt32(value) => *value != 0,
        Value::UInt64(value) => *value != 0,
        Value::Float(value) => *value != 0.0,
        Value::Double(value) => *value != 0.0,
        Value::Str(value) => !value.is_empty(),
        Value::Raw(value) => !value.data.is_empty(),
        Value::Struct(_) | Value::Array(_) | Value::Set(_) | Value::Map(_) => true,
    }
}

fn text_value(value: &Value) -> String {
    match value {
        Value::Str(value) => value.clone(),
        Value::Bool(value) => {
            if *value {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        other => match int_value(other) {
            Some(number) => number.to_string(),
            None => format!("{other:?}"),
        },
    }
}

pub fn extract(
    path: PathBuf,
    gvas: GvasFile,
    steam_id: Option<String>,
) -> Result<SaveData, SaveError> {
    let mut obtained: HashSet<String> = HashSet::new();
    let mut counters: HashMap<String, i64> = HashMap::new();
    let mut string_counters: HashMap<String, String> = HashMap::new();
    let mut achievements: HashMap<String, AchievementRecord> = HashMap::new();
    let mut purchases: HashMap<String, ShopEntry> = HashMap::new();
    let mut friendships: HashMap<String, i64> = HashMap::new();

    let item_map = require(gvas.get("DataMap_SBItem"), "DataMap_SBItem")?;
    if let Value::Map(map) = &item_map.value {
        if let Some(entry) = map.get(&Value::Str("Item".to_string())) {
            let structure = as_struct_value(entry)?;
            if let Some(obtained_prop) = structure.get("ItemOtaineSet") {
                if let Value::Set(set) = &obtained_prop.value {
                    for element in &set.elements {
                        obtained.insert(text_value(element));
                    }
                }
            }
        }
    }

    if let Some(int_map) = gvas.get("DataMap_int32") {
        if let Value::Map(map) = &int_map.value {
            for (key, value) in &map.entries {
                if let Some(number) = int_value(value) {
                    counters.insert(text_value(key), number);
                }
            }
        }
    }

    if let Some(string_map) = gvas.get("DataMap_FString") {
        if let Value::Map(map) = &string_map.value {
            for (key, value) in &map.entries {
                if let Value::Str(text) = value {
                    string_counters.insert(text_value(key), text.clone());
                }
            }
        }
    }

    if let Some(achievement_prop) = gvas.get("SBAchievement") {
        if let Value::Struct(structure) = &achievement_prop.value {
            if let Some(listing) = structure.get("AchievementList") {
                if let Value::Array(array) = &listing.value {
                    for element in &array.elements {
                        let Value::Struct(fields) = element else {
                            continue;
                        };
                        let Some(alias_prop) = fields.get("AchievementAlias") else {
                            continue;
                        };
                        let Value::Str(alias) = &alias_prop.value else {
                            continue;
                        };
                        let mut record = AchievementRecord {
                            alias: alias.clone(),
                            ..AchievementRecord::default()
                        };
                        for (field_name, property) in &fields.fields {
                            if field_name == "AchievementAlias" {
                                continue;
                            }
                            record
                                .raw
                                .push((field_name.clone(), property.value.clone()));
                            match field_name.as_str() {
                                "ProgressValue" => {
                                    if let Some(number) = int_value(&property.value) {
                                        record.progress = number;
                                    }
                                }
                                "bCompleted" => record.completed = is_truthy(&property.value),
                                "bRecievedReward" => {
                                    record.received_reward = is_truthy(&property.value);
                                }
                                _ => {}
                            }
                        }
                        achievements.insert(alias.clone(), record);
                    }
                }
            }
        }
    }

    if let Some(shop_prop) = gvas.get("SBShopHistory") {
        if let Value::Struct(structure) = &shop_prop.value {
            if let Some(purchase_prop) = structure.get("Purchase") {
                if let Value::Map(map) = &purchase_prop.value {
                    for (key, value) in &map.entries {
                        let alias = text_value(key);
                        let fields = match value {
                            Value::Struct(fields) => fields
                                .fields
                                .iter()
                                .map(|(name, property)| (name.clone(), property.value.clone()))
                                .collect(),
                            _ => Vec::new(),
                        };
                        purchases.insert(alias.clone(), ShopEntry { alias, fields });
                    }
                }
            }
            if let Some(friendship_prop) = structure.get("FriendShip") {
                if let Value::Map(map) = &friendship_prop.value {
                    for (key, value) in &map.entries {
                        if let Some(number) = int_value(value) {
                            friendships.insert(text_value(key), number);
                        }
                    }
                }
            }
        }
    }

    let mut derived: HashSet<String> = HashSet::new();
    for alias in achievements.keys() {
        if let Some(normalised) = derive_aliases(alias) {
            derived.insert(normalised);
        }
    }

    if obtained.is_empty() && gvas.header.save_game_class_name != SAVE_GAME_CLASS {
        return Err(SaveError("unsupported save game class".to_string()));
    }

    let slot = path
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(slot_from_name)
        .unwrap_or(0);

    Ok(SaveData {
        path,
        steam_id,
        slot,
        gvas,
        obtained_items: obtained,
        derived_aliases: derived,
        counters,
        string_counters,
        achievements,
        shop_purchases: purchases,
        friendships,
    })
}

pub fn load_save(path: impl AsRef<Path>, steam_id: Option<String>) -> Result<SaveData, SaveError> {
    let path = path.as_ref().to_path_buf();
    let gvas = crate::gvas::load_gvas(&path)?;
    if gvas.header.save_game_class_name != SAVE_GAME_CLASS {
        return Err(SaveError(format!(
            "not a Stellar Blade save: {}",
            gvas.header.save_game_class_name
        )));
    }
    let steam_id = steam_id.or_else(|| {
        path.parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .filter(|name| is_steam_id(name))
            .map(str::to_string)
    });
    extract(path, gvas, steam_id)
}

pub fn pick_default_save(saves: Option<Vec<SaveSlot>>) -> Result<SaveSlot, SaveError> {
    let saves = saves.unwrap_or_else(|| discover_saves(None));
    if saves.is_empty() {
        return Err(SaveError("no Stellar Blade saves found".to_string()));
    }
    let main = saves.iter().find(|slot| slot.slot == 0);
    Ok(main.unwrap_or(&saves[0]).clone())
}
