//! Catalog builder: port of the former Python `tools/build_catalog.py`.
//!
//! Inputs: `data/raw/api/*.json`, `data/raw/api/i18n/`, `data/raw/crosswalk.json`,
//! `data/raw/universe/aliases.json`, `data/raw/game/name_map.json` and
//! `data/raw/memorystick_order.json`.
//! Output: `data/catalog.json` (byte-identical to the Python generator).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::jsonio;

const GENERATED_BY: &str = "crates/sbsave-tools (catalog build)";

const SOURCES: [&str; 6] = [
    "https://stellarbladeguide.com (collectibles, cosmetics, locations, cycles)",
    "https://github.com/wuxiao00j/stellar-blade-macos-save-editor (Simplified Chinese names)",
    "https://github.com/lecher-wang/Stellar-Blade-100-completion-save-file (alias universe)",
    "Stellar Blade game tables + zh-Hans/en Game.locres (mined names, data/raw/game/name_map.json)",
    "data/raw/api/i18n (hand maintained Chinese translations of guide region/location/obtain text)",
    "https://mapgenie.io + https://www.gamersky.com (in-game Data Bank order of memorysticks)",
];

const CATEGORIES: [(&str, &str, i64); 13] = [
    ("nano_suits", "纳米战衣", 10),
    ("cans", "罐子", 20),
    ("records", "记录(文档/记忆棒)", 30),
    ("passcodes", "密码", 40),
    ("camps", "营地", 50),
    ("hair", "发型", 60),
    ("glasses", "眼镜/面饰", 70),
    ("earrings", "耳饰", 80),
    ("drone_seals", "无人机外观", 90),
    ("adam_costumes", "亚当服装", 100),
    ("lily_costumes", "莉莉服装", 110),
    ("design_patterns", "设计图案", 120),
    ("fish", "鱼类", 130),
];

// In-game Data Bank record types (order follows the game's Records menu).
const RECORD_TYPES: [(&str, &str); 10] = [
    ("memorystick", "记忆棒"),
    ("document_log_data", "文档·日志数据"),
    ("document_journal", "文档·日志"),
    ("document_messages", "文档·消息"),
    ("document_announcements", "文档·公告"),
    ("document_series", "文档·系列"),
    ("document_books", "文档·书籍"),
    ("document_information", "文档·信息"),
    ("document_promotions", "文档·宣传"),
    ("document_prayers", "文档·祈祷"),
];

const FISH_SKIP: [&str; 14] = [
    "Fish_01",
    "Fish_02",
    "Fish_03",
    "Fish_04",
    "Fish_05",
    "Fish_06",
    "Fish_07",
    "Fish_08",
    "Fish_09",
    "Fish_10",
    "Fish_Box4",
    "Fish_Slice_Bait",
    "Fish_GoldFish",
    "Fish_Mahimahi",
];

fn game_note(category: &str) -> Option<&'static str> {
    match category {
        "records" => Some("名称来自游戏数据库（zh-Hans 本地化）"),
        "passcodes" => Some("名称来自游戏数据库（zh-Hans 本地化）"),
        "camps" => Some("名称来自游戏营地数据（zh-Hans 本地化）"),
        "design_patterns" => Some("图案名称来自游戏数据（zh-Hans 本地化）"),
        _ => None,
    }
}

#[derive(Deserialize)]
struct Crosswalk {
    site_base_url: String,
    nano_suits: Vec<CrosswalkSuit>,
    appearance: Vec<CrosswalkAppearance>,
    fish_zh: HashMap<String, String>,
    zone_zh: HashMap<String, String>,
    #[serde(default)]
    record_type_overrides: HashMap<String, i64>,
}

#[derive(Deserialize)]
struct CrosswalkSuit {
    site_id: i64,
    aliases: Vec<String>,
    zh: String,
    confidence: String,
}

#[derive(Deserialize)]
struct CrosswalkAppearance {
    site_id: i64,
    aliases: Vec<String>,
    zh: String,
    category: String,
    confidence: String,
}

#[derive(Deserialize)]
struct MemorystickOrder {
    #[allow(dead_code)]
    source: String,
    regions: Vec<MemorystickRegion>,
}

#[derive(Deserialize)]
struct MemorystickRegion {
    name: String,
    items: Vec<String>,
}

#[derive(Debug)]
struct SiteItem {
    title: String,
    cycle: String,
    level: String,
    location: String,
    description: String,
    source_file: String,
    order: i64,
    types: Vec<String>,
    subtype: Option<String>,
}

#[derive(Serialize)]
struct CatalogItem {
    id: String,
    name: String,
    name_en: Option<String>,
    category: String,
    aliases: Vec<String>,
    area: Option<String>,
    location: Option<String>,
    obtain: Option<String>,
    ng_plus: i64,
    dlc: Option<String>,
    missable: bool,
    note: Option<String>,
    confidence: String,
    source: Option<String>,
    area_zh: Option<String>,
    location_zh: Option<String>,
    obtain_zh: Option<String>,
    record_type: Option<String>,
    record_type_zh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
}

#[derive(Serialize)]
struct CategoryEntry {
    key: &'static str,
    name: &'static str,
    order: i64,
}

#[derive(Serialize)]
struct CatalogPayload {
    version: i64,
    generated_by: &'static str,
    sources: &'static [&'static str],
    categories: Vec<CategoryEntry>,
    items: Vec<CatalogItem>,
}

struct I18n {
    levels: HashMap<String, String>,
    locations: HashMap<String, String>,
    obtain: HashMap<String, String>,
}

pub struct BuildOutput {
    pub bytes: Vec<u8>,
    pub item_count: usize,
    pub alias_count: usize,
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("读取 {} 失败: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("解析 {} 失败: {error}", path.display()))
}

fn string_field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn truthy_string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn load_site_index(api_dir: &Path) -> Result<BTreeMap<i64, SiteItem>, String> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(api_dir)
        .map_err(|error| format!("读取 {} 失败: {error}", api_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    paths.sort();

    let mut index: BTreeMap<i64, SiteItem> = BTreeMap::new();
    let mut order = 0i64;
    for path in paths {
        let payload = read_json(&path)?;
        let source_file = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let levels = payload
            .as_array()
            .ok_or_else(|| format!("{} 不是 API 响应数组", path.display()))?;
        for level in levels {
            let level_name = level
                .get("level_name")
                .and_then(Value::as_str)
                .filter(|text| !text.is_empty())
                .unwrap_or_default()
                .to_string();
            let locations = level
                .get("locations")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for location in &locations {
                let location_name = location
                    .get("location_name")
                    .and_then(Value::as_str)
                    .filter(|text| !text.is_empty())
                    .unwrap_or_default()
                    .to_string();
                let collectibles = location
                    .get("collectibles")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                for item in &collectibles {
                    order += 1;
                    let id = item
                        .get("id")
                        .and_then(|value| {
                            value
                                .as_i64()
                                .or_else(|| value.as_f64().map(|number| number as i64))
                                .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
                        })
                        .ok_or_else(|| format!("{} 中存在非法条目 id", path.display()))?;
                    let cycle =
                        truthy_string_field(item, "cycle").unwrap_or_else(|| "Base".to_string());
                    let description = item
                        .get("description")
                        .and_then(|value| value.get("content"))
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    let types = item
                        .get("types")
                        .and_then(Value::as_array)
                        .map(|values| {
                            values
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_string)
                                .collect()
                        })
                        .unwrap_or_default();
                    index.insert(
                        id,
                        SiteItem {
                            title: string_field(item, "title"),
                            cycle,
                            level: level_name.clone(),
                            location: location_name.clone(),
                            description,
                            source_file: source_file.clone(),
                            order,
                            types,
                            subtype: truthy_string_field(item, "subtype"),
                        },
                    );
                }
            }
        }
    }
    Ok(index)
}

fn dlc_from_aliases(aliases: &[String], cycle: &str) -> Option<String> {
    if aliases.iter().any(|alias| alias.contains("Nier")) {
        return Some("nier".to_string());
    }
    if aliases.iter().any(|alias| alias.contains("Nikke")) {
        return Some("nikke".to_string());
    }
    if cycle == "DLC" {
        return Some("dlc".to_string());
    }
    None
}

fn cycle_to_ng_plus(cycle: &str) -> i64 {
    match cycle {
        "NG+" => 1,
        "NG++" => 2,
        _ => 0,
    }
}

#[allow(clippy::too_many_arguments)]
fn make_item(
    item_id: String,
    name: String,
    category: &str,
    aliases: Vec<String>,
    site: Option<&SiteItem>,
    confidence: &str,
    name_en: Option<String>,
    note: Option<String>,
    source: Option<String>,
    base_url: &str,
) -> CatalogItem {
    let cycle = site.map(|item| item.cycle.as_str()).unwrap_or("Base");
    let source = match site {
        Some(item) => {
            let stem = if item.source_file.len() >= 5 {
                &item.source_file[..item.source_file.len() - 5]
            } else {
                ""
            };
            Some(format!("{}/api/{}", base_url, stem.replace("__", "/")))
        }
        None => source,
    };
    let dlc = dlc_from_aliases(&aliases, cycle);
    CatalogItem {
        id: item_id,
        name,
        name_en: name_en
            .filter(|text| !text.is_empty())
            .or_else(|| site.map(|item| item.title.clone())),
        category: category.to_string(),
        aliases,
        area: site.map(|item| item.level.clone()),
        location: site.map(|item| item.location.clone()),
        obtain: site.map(|item| item.description.clone()),
        ng_plus: cycle_to_ng_plus(cycle),
        dlc,
        missable: false,
        note,
        confidence: confidence.to_string(),
        source,
        area_zh: None,
        location_zh: None,
        obtain_zh: None,
        record_type: None,
        record_type_zh: None,
        order: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn plain_item(
    id: String,
    name: String,
    name_en: String,
    category: &str,
    aliases: Vec<String>,
    area: Option<String>,
    note: &str,
    confidence: &str,
    ng_plus: i64,
    dlc: Option<String>,
) -> CatalogItem {
    CatalogItem {
        id,
        name,
        name_en: Some(name_en),
        category: category.to_string(),
        aliases,
        area,
        location: None,
        obtain: None,
        ng_plus,
        dlc,
        missable: false,
        note: Some(note.to_string()),
        confidence: confidence.to_string(),
        source: None,
        area_zh: None,
        location_zh: None,
        obtain_zh: None,
        record_type: None,
        record_type_zh: None,
        order: None,
    }
}

fn zone_label(zone: &str, zone_zh: &HashMap<String, String>) -> String {
    if let Some(label) = zone_zh.get(zone) {
        return label.clone();
    }
    let parts: Vec<&str> = zone.split('_').collect();
    if parts.len() >= 2 {
        let last = parts[parts.len() - 1];
        if !last.is_empty() && last.chars().all(|character| character.is_ascii_digit()) {
            let base = parts[..parts.len() - 1].join("_");
            return zone_zh
                .get(&base)
                .cloned()
                .unwrap_or_else(|| zone.to_string());
        }
    }
    zone.to_string()
}

fn build_nano_suits(
    site: &BTreeMap<i64, SiteItem>,
    crosswalk: &Crosswalk,
) -> Result<Vec<CatalogItem>, String> {
    let mut items: Vec<CatalogItem> = Vec::new();
    let mut covered: BTreeSet<i64> = BTreeSet::new();
    for suit in &crosswalk.nano_suits {
        let info = site
            .get(&suit.site_id)
            .ok_or_else(|| format!("nano suit site id {} not found", suit.site_id))?;
        covered.insert(suit.site_id);
        let item_id = suit
            .aliases
            .first()
            .cloned()
            .ok_or_else(|| format!("nano suit site id {} 缺少别名", suit.site_id))?;
        items.push(make_item(
            item_id,
            suit.zh.clone(),
            "nano_suits",
            suit.aliases.clone(),
            Some(info),
            &suit.confidence,
            None,
            None,
            None,
            &crosswalk.site_base_url,
        ));
    }
    let missing: Vec<i64> = site
        .iter()
        .filter(|(id, item)| {
            item.source_file == "cosmetics__nano-suits.json" && !covered.contains(id)
        })
        .map(|(id, _)| *id)
        .collect();
    for site_id in missing {
        let info = &site[&site_id];
        let note = if info.title == "Skin Suit" {
            "默认外观，无存档物品 ID"
        } else {
            "未建立别名映射"
        };
        items.push(make_item(
            format!("site_{site_id}"),
            info.title.clone(),
            "nano_suits",
            Vec::new(),
            Some(info),
            "none",
            None,
            Some(note.to_string()),
            None,
            &crosswalk.site_base_url,
        ));
    }
    Ok(items)
}

fn build_appearance(
    site: &BTreeMap<i64, SiteItem>,
    crosswalk: &Crosswalk,
) -> Result<Vec<CatalogItem>, String> {
    let mut items: Vec<CatalogItem> = Vec::new();
    for appearance in &crosswalk.appearance {
        let info = site
            .get(&appearance.site_id)
            .ok_or_else(|| format!("appearance site id {} not found", appearance.site_id))?;
        let item_id = appearance
            .aliases
            .first()
            .cloned()
            .ok_or_else(|| format!("appearance site id {} 缺少别名", appearance.site_id))?;
        items.push(make_item(
            item_id,
            appearance.zh.clone(),
            &appearance.category,
            appearance.aliases.clone(),
            Some(info),
            &appearance.confidence,
            None,
            None,
            None,
            &crosswalk.site_base_url,
        ));
    }
    Ok(items)
}

fn fold_diacritic(character: char) -> char {
    match character {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' => 'a',
        'ç' | 'ć' | 'č' => 'c',
        'è' | 'é' | 'ê' | 'ë' | 'ē' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ñ' | 'ń' => 'n',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ō' => 'o',
        'ù' | 'ú' | 'û' | 'ü' | 'ū' => 'u',
        'ý' | 'ÿ' => 'y',
        'š' => 's',
        'ž' => 'z',
        _ => character,
    }
}

fn normalize_name(text: &str) -> String {
    text.chars()
        .map(fold_diacritic)
        .collect::<String>()
        .to_lowercase()
        .replace('\u{2019}', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn record_type_from_site(types: &[String], subtype: Option<&str>) -> Option<&'static str> {
    if types.iter().any(|value| value == "Memorystick") {
        return Some("memorystick");
    }
    if !types.iter().any(|value| value == "Document") {
        return None;
    }
    match subtype? {
        "Series" => Some("document_series"),
        "Promotions" => Some("document_promotions"),
        "Messages" => Some("document_messages"),
        "Journal" => Some("document_journal"),
        "Log Data" => Some("document_log_data"),
        "Books" => Some("document_books"),
        "Information" => Some("document_information"),
        "Prayers" => Some("document_prayers"),
        "Announcements" => Some("document_announcements"),
        _ => None,
    }
}

fn record_type_label(key: &str) -> Option<&'static str> {
    RECORD_TYPES
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .map(|(_, label)| *label)
}

/// Normalizes a record title for matching: strips localization markup, folds
/// diacritics, lowercases and reduces punctuation/whitespace to single spaces.
fn fold_record_title(text: &str) -> String {
    let mut stripped = String::new();
    let mut in_tag = false;
    for character in text.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => stripped.push(fold_diacritic(character)),
            _ => {}
        }
    }
    let mut normalized = String::new();
    let mut pending_space = false;
    for character in stripped.to_lowercase().chars() {
        let character = match character {
            '\u{2019}' | '\u{2018}' => '\'',
            other => other,
        };
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
            pending_space = false;
        } else if !pending_space && !normalized.is_empty() {
            normalized.push(' ');
            pending_space = true;
        }
    }
    normalized.trim_end().replace("centre", "center")
}

fn singularize(text: &str) -> String {
    text.split_whitespace()
        .map(|token| {
            if token.len() > 3 && token.ends_with('s') {
                &token[..token.len() - 1]
            } else {
                token
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Loose variant of [`fold_record_title`] that additionally drops the
/// "Information:" prefix, plural word endings and a trailing article number
/// "1" (guide/game naming differences).
fn loose_record_title(text: &str) -> String {
    let folded = fold_record_title(text);
    let without_information = folded.strip_prefix("information ").unwrap_or(&folded);
    let singular = singularize(without_information);
    singular.strip_suffix(" 1").unwrap_or(&singular).to_string()
}

fn apply_guide_obtain(item: &mut CatalogItem, info: &SiteItem) {
    if item.obtain.is_none() && !info.description.is_empty() {
        item.obtain = Some(info.description.clone());
    }
}

fn apply_record_types(
    items: &mut [CatalogItem],
    site: &BTreeMap<i64, SiteItem>,
    crosswalk: &Crosswalk,
) -> Result<(), String> {
    let overrides = &crosswalk.record_type_overrides;
    let mut by_fold: HashMap<String, Vec<i64>> = HashMap::new();
    let mut by_loose: HashMap<String, Vec<i64>> = HashMap::new();
    for (site_id, info) in site {
        if record_type_from_site(&info.types, info.subtype.as_deref()).is_none() {
            continue;
        }
        by_fold
            .entry(fold_record_title(&info.title))
            .or_default()
            .push(*site_id);
        by_loose
            .entry(loose_record_title(&info.title))
            .or_default()
            .push(*site_id);
    }
    for site_ids in by_fold.values_mut().chain(by_loose.values_mut()) {
        site_ids.sort_unstable();
    }

    let mut used: BTreeSet<i64> = BTreeSet::new();
    for item in items.iter_mut() {
        if item.category != "records" {
            continue;
        }
        let Some(site_id) = overrides.get(&item.id) else {
            continue;
        };
        let Some(info) = site.get(site_id) else {
            return Err(format!(
                "记录类型覆盖 {} 指向不存在的攻略条目 {site_id}",
                item.id
            ));
        };
        let Some(key) = record_type_from_site(&info.types, info.subtype.as_deref()) else {
            return Err(format!(
                "记录类型覆盖 {} 的攻略条目 {site_id} 缺少类型: {}",
                item.id, info.title
            ));
        };
        if !used.insert(*site_id) {
            return Err(format!("记录类型覆盖条目重复: {site_id}"));
        }
        item.record_type = Some(key.to_string());
        item.record_type_zh = record_type_label(key).map(str::to_string);
        apply_guide_obtain(item, info);
    }
    let missing_overrides: Vec<&str> = overrides
        .keys()
        .filter(|alias| {
            !items
                .iter()
                .any(|item| item.id == **alias && item.category == "records")
        })
        .map(String::as_str)
        .collect();
    if !missing_overrides.is_empty() {
        return Err(format!(
            "记录类型覆盖未命中目录条目: {}",
            missing_overrides.join(", ")
        ));
    }

    for item in items.iter_mut() {
        if item.category != "records" || item.record_type.is_some() {
            continue;
        }
        let Some(name) = item
            .name_en
            .as_deref()
            .filter(|name| !name.is_empty() && *name != item.id)
        else {
            continue;
        };
        let mut candidates = by_fold
            .get(&fold_record_title(name))
            .cloned()
            .unwrap_or_default();
        if candidates.iter().all(|site_id| used.contains(site_id)) {
            candidates = by_loose
                .get(&loose_record_title(name))
                .cloned()
                .unwrap_or_default();
        }
        let Some(site_id) = candidates
            .into_iter()
            .find(|site_id| !used.contains(site_id))
        else {
            continue;
        };
        let Some(info) = site.get(&site_id) else {
            continue;
        };
        let Some(key) = record_type_from_site(&info.types, info.subtype.as_deref()) else {
            continue;
        };
        used.insert(site_id);
        item.record_type = Some(key.to_string());
        item.record_type_zh = record_type_label(key).map(str::to_string);
        apply_guide_obtain(item, info);
    }

    let inherited: HashMap<String, (String, Option<String>)> = items
        .iter()
        .filter_map(|item| {
            item.record_type
                .as_ref()
                .map(|key| (item.id.clone(), (key.clone(), item.obtain.clone())))
        })
        .collect();
    for item in items.iter_mut() {
        if item.category != "records" || item.record_type.is_some() {
            continue;
        }
        let base = item
            .id
            .rsplit_once('_')
            .filter(|(_, suffix)| {
                !suffix.is_empty() && suffix.chars().all(|value| value.is_ascii_digit())
            })
            .map(|(prefix, _)| prefix);
        let Some((key, obtain)) = base.and_then(|base| inherited.get(base)) else {
            continue;
        };
        item.record_type = Some(key.clone());
        item.record_type_zh = record_type_label(key).map(str::to_string);
        if item.obtain.is_none() {
            item.obtain = obtain.clone();
        }
    }

    let missing: Vec<&str> = items
        .iter()
        .filter(|item| item.category == "records" && item.record_type.is_none())
        .map(|item| item.id.as_str())
        .collect();
    if !missing.is_empty() {
        return Err(format!("以下记录条目缺少类型映射: {}", missing.join(", ")));
    }
    Ok(())
}

/// Matches passcode collectibles to guide entries by their in-game code token
/// (the last component of the guide title, e.g. `μηλαμη`) and copies the guide
/// obtain text. A passcode may share its guide entry with a memorystick
/// (dual-type collectibles), so no cross-category bookkeeping is needed.
fn apply_passcode_obtain(
    items: &mut [CatalogItem],
    site: &BTreeMap<i64, SiteItem>,
) -> Result<(), String> {
    let mut by_token: HashMap<String, Vec<i64>> = HashMap::new();
    for (site_id, info) in site {
        if !info.types.iter().any(|value| value == "Passcode") {
            continue;
        }
        let token = info
            .title
            .rsplit('/')
            .next()
            .unwrap_or(&info.title)
            .trim()
            .to_string();
        by_token.entry(token).or_default().push(*site_id);
    }
    for site_ids in by_token.values_mut() {
        site_ids.sort_unstable();
    }

    for item in items.iter_mut() {
        if item.category != "passcodes" || item.obtain.is_some() {
            continue;
        }
        let Some(token) = item
            .name_en
            .as_deref()
            .map(str::trim)
            .filter(|token| !token.is_empty() && *token != item.id)
        else {
            continue;
        };
        let Some(site_id) = by_token.get(token).and_then(|site_ids| site_ids.first()) else {
            continue;
        };
        let Some(info) = site.get(site_id) else {
            continue;
        };
        apply_guide_obtain(item, info);
    }

    let missing: Vec<&str> = items
        .iter()
        .filter(|item| item.category == "passcodes" && item.obtain.is_none())
        .map(|item| item.id.as_str())
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "以下密码条目未匹配到攻略条目: {}",
            missing.join(", ")
        ));
    }
    Ok(())
}

/// Applies the in-game Data Bank order of memorysticks: every memorystick gets
/// the game region as `area` and a continuous menu `order` across regions.
/// Version variants (`..._01_2`) inherit the order of their base entry.
fn apply_memorystick_order(
    items: &mut [CatalogItem],
    order: &MemorystickOrder,
) -> Result<(), String> {
    let mut by_id: HashMap<&str, (&str, i64)> = HashMap::new();
    let mut sequence = 0i64;
    for region in &order.regions {
        for id in &region.items {
            sequence += 1;
            if by_id
                .insert(id.as_str(), (region.name.as_str(), sequence))
                .is_some()
            {
                return Err(format!("记忆棒选单顺序重复: {id}"));
            }
        }
    }
    for id in by_id.keys() {
        let known = items
            .iter()
            .any(|item| item.id == *id && item.record_type.as_deref() == Some("memorystick"));
        if !known {
            return Err(format!("记忆棒选单顺序指向不存在的条目: {id}"));
        }
    }
    for item in items.iter_mut() {
        if item.record_type.as_deref() != Some("memorystick") {
            continue;
        }
        let entry = by_id.get(item.id.as_str()).copied().or_else(|| {
            item.id
                .rsplit_once('_')
                .filter(|(_, suffix)| {
                    !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
                })
                .and_then(|(base, _)| by_id.get(base).copied())
        });
        let Some((region, number)) = entry else {
            return Err(format!("记忆棒缺少选单顺序: {}", item.id));
        };
        item.area = Some(region.to_string());
        item.order = Some(number);
    }
    Ok(())
}

// Guide title typos kept on purpose by stellarbladeguide.com.
const CROSSWALK_TITLE_EXCEPTIONS: [i64; 1] = [1043];

fn crosswalk_title_matches(title: &str, game_name: &str) -> bool {
    let title = normalize_name(title).replace("demin", "denim");
    let game_name = normalize_name(game_name).replace("demin", "denim");
    if title == game_name {
        return true;
    }
    [" earrings", " outfit", " glasses"].iter().any(|suffix| {
        title
            .strip_suffix(suffix)
            .is_some_and(|base| base == game_name)
            || game_name
                .strip_suffix(suffix)
                .is_some_and(|base| base == title)
    })
}

fn validate_crosswalk_titles(
    site: &BTreeMap<i64, SiteItem>,
    crosswalk: &Crosswalk,
    game_names: &Value,
) -> Result<(), String> {
    let Some(names) = game_names.get("items").and_then(Value::as_object) else {
        return Ok(());
    };
    let entries = crosswalk
        .nano_suits
        .iter()
        .map(|suit| (suit.site_id, &suit.aliases, "纳米战衣"))
        .chain(
            crosswalk
                .appearance
                .iter()
                .map(|appearance| (appearance.site_id, &appearance.aliases, "外观")),
        );
    for (site_id, aliases, label) in entries {
        if CROSSWALK_TITLE_EXCEPTIONS.contains(&site_id) {
            continue;
        }
        let Some(info) = site.get(&site_id) else {
            return Err(format!("{label} site id {site_id} not found"));
        };
        let resolved: BTreeSet<&str> = aliases
            .iter()
            .filter_map(|alias| names.get(alias))
            .filter_map(|entry| entry.get("en"))
            .filter_map(Value::as_str)
            .collect();
        if resolved.len() != 1 {
            continue;
        }
        let Some(game_name) = resolved.iter().next() else {
            continue;
        };
        if !crosswalk_title_matches(&info.title, game_name) {
            return Err(format!(
                "{label} site id {site_id} 名称不匹配: 攻略「{}」 vs 游戏「{game_name}」（请核对 data/raw/crosswalk.json）",
                info.title
            ));
        }
    }
    Ok(())
}

fn build_cans(
    site: &BTreeMap<i64, SiteItem>,
    game_names: &Value,
    crosswalk: &Crosswalk,
) -> Result<Vec<CatalogItem>, String> {
    let items_map = game_names
        .get("items")
        .and_then(Value::as_object)
        .ok_or_else(|| "name_map.json 缺少 items 映射".to_string())?;
    let mut alias_by_title: BTreeMap<String, String> = BTreeMap::new();
    for (alias, entry) in items_map {
        if !alias.starts_with("Can_") {
            continue;
        }
        let Some(name_en) = entry.get("en").and_then(Value::as_str) else {
            continue;
        };
        let title = normalize_name(name_en);
        if let Some(previous) = alias_by_title.insert(title.clone(), alias.clone()) {
            return Err(format!(
                "游戏罐头名称重复: {name_en}（{previous} / {alias}）"
            ));
        }
    }

    let mut can_ids: Vec<i64> = site
        .iter()
        .filter(|(_, item)| item.source_file == "collectibles__cans.json")
        .map(|(id, _)| *id)
        .collect();
    can_ids.sort_by_key(|id| site[id].order);

    let mut items: Vec<CatalogItem> = Vec::new();
    let mut matched: BTreeSet<String> = BTreeSet::new();
    for site_id in can_ids {
        let info = &site[&site_id];
        let title = normalize_name(&info.title);
        let Some(alias) = alias_by_title.get(&title).cloned() else {
            return Err(format!(
                "攻略罐头条目未匹配到游戏名称: {}（site id {site_id}）",
                info.title
            ));
        };
        if !matched.insert(alias.clone()) {
            return Err(format!("攻略罐头条目重复匹配: {}（{alias}）", info.title));
        }
        items.push(make_item(
            alias.clone(),
            info.title.clone(),
            "cans",
            vec![alias],
            Some(info),
            "medium",
            None,
            Some("编号按游戏数据名称与攻略条目匹配".to_string()),
            None,
            &crosswalk.site_base_url,
        ));
    }

    let unmatched: Vec<&str> = alias_by_title
        .values()
        .filter(|alias| !matched.contains(*alias))
        .map(String::as_str)
        .collect();
    if !unmatched.is_empty() {
        return Err(format!(
            "以下游戏罐头未在攻略条目中找到: {}",
            unmatched.join(", ")
        ));
    }
    Ok(items)
}

fn normalize_record_aliases(universe: &Value, suffix_re: &Regex) -> BTreeSet<String> {
    let mut aliases: BTreeSet<String> = BTreeSet::new();
    if let Some(records) = universe.get("records").and_then(Value::as_array) {
        for alias in records {
            if let Some(alias) = alias.as_str() {
                aliases.insert(suffix_re.replace(alias, "").into_owned());
            }
        }
    }
    aliases
}

fn build_records(universe: &Value, crosswalk: &Crosswalk) -> Result<Vec<CatalogItem>, String> {
    let zone_re = Regex::new(r"^Item_Records_([A-Za-z0-9]+?)_(Memory|Passcode)_(\d+)(.*)$")
        .expect("records regex");
    let suffix_re = Regex::new(r"_(Maintain|Used|Popup)$").expect("suffix regex");
    let mut items: Vec<CatalogItem> = Vec::new();
    for alias in normalize_record_aliases(universe, &suffix_re) {
        let Some(captures) = zone_re.captures(&alias) else {
            items.push(plain_item(
                alias.clone(),
                alias.replace("Item_Records_", "记录："),
                alias.clone(),
                "records",
                vec![alias.clone()],
                None,
                "任务/活动记录，名称来自内部 ID",
                "low",
                0,
                None,
            ));
            continue;
        };
        let zone = captures[1].to_string();
        let kind = captures[2].to_string();
        let number: i64 = captures[3]
            .parse()
            .map_err(|_| format!("记录编号非法: {alias}"))?;
        let suffix = captures[4].to_string();
        let kind_zh = if kind == "Memory" { "记录" } else { "密码" };
        let category = if kind == "Memory" {
            "records"
        } else {
            "passcodes"
        };
        let suffix_zh = match suffix.as_str() {
            "_1" => "（版本2）",
            "_2" => "（版本3）",
            _ => "",
        };
        let label = zone_label(&zone, &crosswalk.zone_zh);
        let name = format!("{label} {kind_zh} {number:02}{suffix_zh}");
        items.push(plain_item(
            alias.clone(),
            name,
            alias.clone(),
            category,
            vec![alias],
            Some(label),
            "按内部 ID 生成名称；文档/记忆棒共享该别名族，位置请参考攻略同区域列表",
            "low",
            0,
            None,
        ));
    }
    Ok(items)
}

fn build_fish(universe: &Value, crosswalk: &Crosswalk) -> Vec<CatalogItem> {
    let mut items: Vec<CatalogItem> = Vec::new();
    let Some(aliases) = universe.get("fish").and_then(Value::as_array) else {
        return items;
    };
    for alias in aliases {
        let Some(alias) = alias.as_str() else {
            continue;
        };
        if FISH_SKIP.contains(&alias) {
            continue;
        }
        let name = crosswalk
            .fish_zh
            .get(alias)
            .cloned()
            .unwrap_or_else(|| alias.replace("Fish_", ""));
        let dlc = if alias.contains("Nikke") {
            Some("nikke".to_string())
        } else {
            None
        };
        items.push(plain_item(
            alias.to_string(),
            name,
            alias.replace("Fish_", ""),
            "fish",
            vec![alias.to_string()],
            None,
            "钓鱼图鉴；位置参考各钓鱼点攻略",
            "high",
            0,
            dlc,
        ));
    }
    items
}

fn build_design_patterns(
    site: &BTreeMap<i64, SiteItem>,
    universe: &Value,
    crosswalk: &Crosswalk,
) -> Vec<CatalogItem> {
    let pattern_universe: BTreeSet<String> = universe
        .get("design_patterns")
        .and_then(Value::as_array)
        .map(|patterns| {
            patterns
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    let mut items: Vec<CatalogItem> = Vec::new();
    let mut covered: BTreeSet<String> = BTreeSet::new();
    for suit in &crosswalk.nano_suits {
        let candidates: Vec<String> = suit
            .aliases
            .iter()
            .map(|alias| format!("DesignPattern_{alias}"))
            .filter(|candidate| pattern_universe.contains(candidate))
            .collect();
        if candidates.is_empty() {
            continue;
        }
        covered.extend(candidates.iter().cloned());
        let info = site.get(&suit.site_id);
        items.push(make_item(
            candidates[0].clone(),
            format!("设计图案：{}", suit.zh),
            "design_patterns",
            candidates,
            info,
            "medium",
            None,
            Some("图案与战衣同名（存档内部 ID 推导）".to_string()),
            None,
            &crosswalk.site_base_url,
        ));
    }
    for alias in pattern_universe.difference(&covered) {
        items.push(plain_item(
            alias.clone(),
            alias.replace("DesignPattern_", "设计图案："),
            alias.clone(),
            "design_patterns",
            vec![alias.clone()],
            None,
            "未建立战衣映射",
            "low",
            0,
            None,
        ));
    }
    items
}

fn build_camps(universe: &Value, crosswalk: &Crosswalk) -> Vec<CatalogItem> {
    let camp_re = Regex::new(r"^ChangeState_ZoneEnv_(.+?)_EnvS_(\d+)_Camp$").expect("camp regex");
    let mut items: Vec<CatalogItem> = Vec::new();
    let Some(aliases) = universe.get("camps").and_then(Value::as_array) else {
        return items;
    };
    for alias in aliases {
        let Some(alias) = alias.as_str() else {
            continue;
        };
        let (zone, number) = match camp_re.captures(alias) {
            Some(captures) => (
                captures[1].to_string(),
                captures[2].parse::<i64>().unwrap_or_default(),
            ),
            None => ("?".to_string(), 0),
        };
        let label = zone_label(&zone, &crosswalk.zone_zh);
        items.push(plain_item(
            alias.to_string(),
            format!("{label} 营地 #{number}"),
            alias.to_string(),
            "camps",
            vec![alias.to_string()],
            Some(label),
            "按内部 ID 生成名称；位置请参考攻略同区域营地列表",
            "low",
            0,
            None,
        ));
    }
    items
}

fn load_string_map(path: &Path) -> Result<HashMap<String, String>, String> {
    let payload = read_json(path)?;
    let mut map = HashMap::new();
    if let Some(object) = payload.as_object() {
        for (key, value) in object {
            if let Some(text) = value.as_str() {
                map.insert(key.clone(), text.to_string());
            }
        }
    }
    Ok(map)
}

fn load_i18n(i18n_dir: &Path, site: &BTreeMap<i64, SiteItem>) -> Result<I18n, String> {
    let levels = load_string_map(&i18n_dir.join("levels.json"))?;
    let locations = load_string_map(&i18n_dir.join("locations.json"))?;
    let mut paths: Vec<PathBuf> = std::fs::read_dir(i18n_dir)
        .map_err(|error| format!("读取 {} 失败: {error}", i18n_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("obtain_") && name.ends_with(".json"))
        })
        .collect();
    paths.sort();
    let mut obtain: HashMap<String, String> = HashMap::new();
    for path in paths {
        let payload = read_json(&path)?;
        let object = payload
            .as_object()
            .ok_or_else(|| format!("{} 不是对象", path.display()))?;
        for (site_id, text) in object {
            let id: i64 = site_id
                .parse()
                .map_err(|_| format!("{} 中的站点 id 非法: {site_id}", path.display()))?;
            let Some(info) = site.get(&id) else {
                continue;
            };
            if info.description.is_empty() {
                continue;
            }
            if let Some(text) = text.as_str() {
                obtain.insert(info.description.clone(), text.to_string());
            }
        }
    }
    Ok(I18n {
        levels,
        locations,
        obtain,
    })
}

fn apply_game_names(items: &mut [CatalogItem], game_names: &Value) {
    let names = game_names.get("items").and_then(Value::as_object);
    let camps = game_names.get("camps").and_then(Value::as_object);
    for item in items.iter_mut() {
        let camp_entry = if item.category == "camps" {
            camps.and_then(|map| map.get(&item.id))
        } else {
            None
        };
        let entry = camp_entry
            .filter(|value| !value.as_object().is_some_and(Map::is_empty))
            .or_else(|| names.and_then(|map| map.get(&item.id)));
        let Some(entry) = entry else {
            continue;
        };
        let Some(zh) = entry
            .get("zh")
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
        else {
            continue;
        };
        item.name = zh.to_string();
        if let Some(en) = entry
            .get("en")
            .and_then(Value::as_str)
            .filter(|text| !text.is_empty())
        {
            item.name_en = Some(en.to_string());
        }
        item.confidence = "high".to_string();
        item.source = Some("game".to_string());
        if let Some(note) = game_note(&item.category) {
            item.note = Some(note.to_string());
        } else if item.category == "cans" {
            item.note = None;
        }
    }
}

fn apply_i18n(items: &mut [CatalogItem], i18n: &I18n) {
    for item in items.iter_mut() {
        item.area_zh = item
            .area
            .as_ref()
            .and_then(|key| i18n.levels.get(key))
            .cloned();
        item.location_zh = item
            .location
            .as_ref()
            .and_then(|key| i18n.locations.get(key))
            .cloned();
        item.obtain_zh = item
            .obtain
            .as_ref()
            .and_then(|key| i18n.obtain.get(key))
            .cloned();
    }
}

pub fn build_catalog_bytes(root: &Path) -> Result<BuildOutput, String> {
    let api_dir = root.join("data/raw/api");
    let site = load_site_index(&api_dir)?;
    let universe = read_json(&root.join("data/raw/universe/aliases.json"))?;
    let crosswalk_path = root.join("data/raw/crosswalk.json");
    let crosswalk: Crosswalk = serde_json::from_str(
        &std::fs::read_to_string(&crosswalk_path)
            .map_err(|error| format!("读取 {} 失败: {error}", crosswalk_path.display()))?,
    )
    .map_err(|error| format!("解析 {} 失败: {error}", crosswalk_path.display()))?;
    let game_names_path = root.join("data/raw/game/name_map.json");
    let game_names = if game_names_path.is_file() {
        read_json(&game_names_path)?
    } else {
        Value::Object(Map::new())
    };
    let memorystick_order_path = root.join("data/raw/memorystick_order.json");
    let memorystick_order: MemorystickOrder = serde_json::from_str(
        &std::fs::read_to_string(&memorystick_order_path)
            .map_err(|error| format!("读取 {} 失败: {error}", memorystick_order_path.display()))?,
    )
    .map_err(|error| format!("解析 {} 失败: {error}", memorystick_order_path.display()))?;
    let i18n = load_i18n(&api_dir.join("i18n"), &site)?;
    validate_crosswalk_titles(&site, &crosswalk, &game_names)?;

    let mut items: Vec<CatalogItem> = Vec::new();
    items.extend(build_nano_suits(&site, &crosswalk)?);
    items.extend(build_cans(&site, &game_names, &crosswalk)?);
    items.extend(build_records(&universe, &crosswalk)?);
    items.extend(build_fish(&universe, &crosswalk));
    items.extend(build_camps(&universe, &crosswalk));
    items.extend(build_appearance(&site, &crosswalk)?);
    items.extend(build_design_patterns(&site, &universe, &crosswalk));
    apply_game_names(&mut items, &game_names);
    apply_record_types(&mut items, &site, &crosswalk)?;
    apply_passcode_obtain(&mut items, &site)?;
    apply_memorystick_order(&mut items, &memorystick_order)?;
    apply_i18n(&mut items, &i18n);

    let mut seen: HashMap<String, String> = HashMap::new();
    for item in &items {
        for alias in &item.aliases {
            if let Some(previous) = seen.insert(alias.clone(), item.id.clone()) {
                return Err(format!(
                    "duplicate alias {alias}: {previous} vs {}",
                    item.id
                ));
            }
        }
    }

    let payload = CatalogPayload {
        version: 1,
        generated_by: GENERATED_BY,
        sources: &SOURCES,
        categories: CATEGORIES
            .iter()
            .map(|(key, name, order)| CategoryEntry {
                key,
                name,
                order: *order,
            })
            .collect(),
        items,
    };
    let bytes = jsonio::to_bytes(&payload).map_err(|error| format!("序列化目录库失败: {error}"))?;
    Ok(BuildOutput {
        bytes,
        item_count: payload.items.len(),
        alias_count: seen.len(),
    })
}

pub fn run(root: &Path) -> Result<(), String> {
    let output = build_catalog_bytes(root)?;
    let out_path = root.join("data/catalog.json");
    std::fs::write(&out_path, &output.bytes)
        .map_err(|error| format!("写入 {} 失败: {error}", out_path.display()))?;
    let shown = out_path.strip_prefix(root).unwrap_or(&out_path);
    println!(
        "wrote {} with {} items, {} aliases",
        shown.display(),
        output.item_count,
        output.alias_count
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        apply_memorystick_order, crosswalk_title_matches, fold_record_title, loose_record_title,
        normalize_name, plain_item, record_type_from_site, zone_label, MemorystickOrder,
        MemorystickRegion,
    };

    #[test]
    fn record_type_from_site_splits_documents_and_memorysticks() {
        let doc = vec!["Document".to_string()];
        let stick = vec!["Memorystick".to_string()];
        let dual = vec!["Passcode".to_string(), "Memorystick".to_string()];
        assert_eq!(
            record_type_from_site(&doc, Some("Log Data")),
            Some("document_log_data")
        );
        assert_eq!(record_type_from_site(&stick, None), Some("memorystick"));
        assert_eq!(record_type_from_site(&dual, None), Some("memorystick"));
        assert_eq!(record_type_from_site(&doc, Some("Unknown")), None);
        assert_eq!(record_type_from_site(&["Passcode".to_string()], None), None);
    }

    #[test]
    fn fold_record_title_normalizes_guide_and_game_spellings() {
        assert_eq!(
            fold_record_title("Funding Announcement: Colony-bound Rocket"),
            "funding announcement colony bound rocket"
        );
        assert_eq!(
            fold_record_title("<StoryTitle_1>: : : WARNING : : :</>"),
            "warning"
        );
        assert_eq!(
            fold_record_title("Welcome to the Raphael Space Centre!"),
            "welcome to the raphael space center"
        );
    }

    #[test]
    fn loose_record_title_drops_prefix_plurals_and_article_number() {
        assert_eq!(
            loose_record_title("Information: Service Drones"),
            "service drone"
        );
        assert_eq!(
            loose_record_title("Athena 82's Orders"),
            "athena 82 s order"
        );
        assert_eq!(loose_record_title("Tattered Report 1"), "tattered report");
        assert_eq!(
            loose_record_title("Quarantine Failure?"),
            "quarantine failure"
        );
    }

    #[test]
    fn normalize_name_folds_accents_and_quotes() {
        assert_eq!(normalize_name("Cryo Café Original"), "cryo cafe original");
        assert_eq!(
            normalize_name("Johnson\u{2019}s  Highball"),
            "johnson's highball"
        );
    }

    #[test]
    fn crosswalk_title_matches_tolerates_known_variants() {
        assert!(crosswalk_title_matches(
            "Crimson Tear Earrings",
            "Crimson Tear"
        ));
        assert!(crosswalk_title_matches(
            "Wandering Swordfighter",
            "Wandering Swordfighter Outfit"
        ));
        assert!(crosswalk_title_matches("Cat's Eye Glasses", "Cat's Eye"));
        assert!(crosswalk_title_matches(
            "FourSeconds Destroyed Demin",
            "FourSeconds Destroyed Denim"
        ));
        assert!(!crosswalk_title_matches("Office Style", "Crew Style"));
        assert!(!crosswalk_title_matches("Missing Link", "Never Look Back"));
    }

    #[test]
    fn zone_label_uses_table_then_strips_trailing_number() {
        let mut zones = HashMap::new();
        zones.insert("WLA".to_string(), "废土A".to_string());
        zones.insert("WLA_2".to_string(), "废土A-2".to_string());
        zones.insert("DED10".to_string(), "大沙漠1".to_string());
        assert_eq!(zone_label("WLA", &zones), "废土A");
        assert_eq!(zone_label("WLA_2", &zones), "废土A-2");
        assert_eq!(zone_label("DED10", &zones), "大沙漠1");
        assert_eq!(zone_label("Unknown_3", &zones), "Unknown_3");
        assert_eq!(zone_label("Unknown", &zones), "Unknown");
        assert_eq!(zone_label("ME_01", &zones), "ME_01");
    }

    fn memorystick(id: &str) -> super::CatalogItem {
        let mut item = plain_item(
            id.to_string(),
            id.to_string(),
            id.to_string(),
            "records",
            vec![id.to_string()],
            None,
            "test",
            "high",
            0,
            None,
        );
        item.record_type = Some("memorystick".to_string());
        item
    }

    fn order_regions(regions: &[(&str, &[&str])]) -> MemorystickOrder {
        MemorystickOrder {
            source: "test".to_string(),
            regions: regions
                .iter()
                .map(|(name, items)| MemorystickRegion {
                    name: name.to_string(),
                    items: items.iter().map(|id| id.to_string()).collect(),
                })
                .collect(),
        }
    }

    #[test]
    fn memorystick_order_applies_region_and_sequence_with_variant_inheritance() {
        let mut items = vec![
            memorystick("Item_Records_ME01_Memory_01"),
            memorystick("Item_Records_ME01_Memory_01_2"),
            memorystick("Item_Records_DED10_Memory_11"),
        ];
        let order = order_regions(&[
            ("Eidos 7", &["Item_Records_DED10_Memory_11"]),
            ("Matrix 11", &["Item_Records_ME01_Memory_01"]),
        ]);
        apply_memorystick_order(&mut items, &order).expect("order");
        assert_eq!(items[2].area.as_deref(), Some("Eidos 7"));
        assert_eq!(items[2].order, Some(1));
        assert_eq!(items[0].area.as_deref(), Some("Matrix 11"));
        assert_eq!(items[0].order, Some(2));
        assert_eq!(items[1].area.as_deref(), Some("Matrix 11"));
        assert_eq!(items[1].order, Some(2));
    }

    #[test]
    fn memorystick_order_rejects_unknown_and_unmapped_items() {
        let order = order_regions(&[("Eidos 7", &["Item_Records_DED10_Memory_11"])]);
        let mut mapped = vec![memorystick("Item_Records_DED10_Memory_11")];
        assert!(apply_memorystick_order(&mut mapped, &order).is_ok());

        let mut missing = vec![memorystick("Item_Records_WLA_Memory_01")];
        assert!(apply_memorystick_order(&mut missing, &order).is_err());

        let mut ghost = vec![memorystick("Item_Records_DED10_Memory_11")];
        let ghost_order = order_regions(&[("Wasteland", &["Item_Records_WLA_Memory_99"])]);
        assert!(apply_memorystick_order(&mut ghost, &ghost_order).is_err());
    }
}
