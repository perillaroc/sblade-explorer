use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::i18n::{Locale, Messages};
use crate::paths;

const CATALOG_JSON: &str = include_str!("../../../data/catalog.json");

pub fn ng_plus_label(ng_plus: i64, locale: Locale) -> String {
    Messages::new(locale).ng_plus_label(ng_plus)
}

pub fn dlc_label(dlc: &str, locale: Locale) -> String {
    Messages::new(locale).dlc_label(dlc)
}

#[derive(Debug, Clone)]
pub struct Category {
    pub key: String,
    pub name: String,
    pub name_en: Option<String>,
    pub order: i64,
    pub section: String,
    pub aliases: Vec<String>,
}

impl Category {
    /// Display name in the requested UI locale, falling back to Chinese.
    pub fn localized_name(&self, locale: Locale) -> &str {
        match locale {
            Locale::En => self.name_en.as_deref().unwrap_or(&self.name),
            Locale::Zh => &self.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuideLink {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Guides {
    pub web: Option<GuideLink>,
    pub video: Option<GuideLink>,
}

/// Official local name for a region code or quest alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedName {
    pub name: String,
    pub name_zh: String,
}

/// Machine readable condition behind a `missable` entry. Each list holds quest
/// aliases from `DataMap_SBQuest`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MissableRule {
    /// The item can no longer be obtained once any of these quests completed.
    pub void_after_quests: Vec<String>,
    /// The item only becomes obtainable after one of these quests started.
    pub requires_quests: Vec<String>,
    /// The item is lost when one of these quests appears in the failed list.
    pub void_on_failed_quests: Vec<String>,
}

impl MissableRule {
    pub fn is_empty(&self) -> bool {
        self.void_after_quests.is_empty()
            && self.requires_quests.is_empty()
            && self.void_on_failed_quests.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct CatalogItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub aliases: Vec<String>,
    pub area: Option<String>,
    pub area_zh: Option<String>,
    pub location: Option<String>,
    pub location_zh: Option<String>,
    pub obtain: Option<String>,
    pub obtain_zh: Option<String>,
    pub ng_plus: i64,
    pub dlc: Option<String>,
    pub missable: bool,
    pub missable_rule: Option<MissableRule>,
    pub note: Option<String>,
    pub confidence: String,
    pub source: Option<String>,
    pub name_en: Option<String>,
    pub record_type: Option<String>,
    pub record_type_zh: Option<String>,
    pub record_type_en: Option<String>,
    pub order: i64,
    pub guides: Option<Guides>,
    pub desc_zh: Option<String>,
    pub desc_en: Option<String>,
}

impl CatalogItem {
    pub fn satisfy_aliases(&self) -> Vec<&str> {
        if self.aliases.is_empty() {
            vec![self.id.as_str()]
        } else {
            self.aliases.iter().map(String::as_str).collect()
        }
    }

    pub fn ng_plus_label(&self, locale: Locale) -> String {
        ng_plus_label(self.ng_plus, locale)
    }

    pub fn dlc_label(&self, locale: Locale) -> Option<String> {
        self.dlc.as_deref().map(|dlc| dlc_label(dlc, locale))
    }

    /// Record type display label for the requested locale, falling back to
    /// Chinese and then to the raw record type key.
    pub fn record_type_label(&self, locale: Locale) -> Option<&str> {
        match locale {
            Locale::En => self
                .record_type_en
                .as_deref()
                .or(self.record_type_zh.as_deref())
                .or(self.record_type.as_deref()),
            Locale::Zh => self
                .record_type_zh
                .as_deref()
                .or(self.record_type.as_deref()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub version: i64,
    pub categories: HashMap<String, Category>,
    pub items: Vec<CatalogItem>,
    /// `DataMap_FString.MapName` code -> region name.
    pub map_names: HashMap<String, LocalizedName>,
    /// Quest alias -> `Quest_*_Title` text.
    pub quest_titles: HashMap<String, LocalizedName>,
}

impl Catalog {
    pub fn by_category(&self, key: &str) -> Vec<&CatalogItem> {
        self.items
            .iter()
            .filter(|item| item.category == key)
            .collect()
    }

    pub fn alias_index(&self) -> HashMap<&str, &CatalogItem> {
        let mut index: HashMap<&str, &CatalogItem> = HashMap::new();
        for item in &self.items {
            for alias in item.satisfy_aliases() {
                index.entry(alias).or_insert(item);
            }
        }
        index
    }

    pub fn category_list(&self) -> Vec<&Category> {
        let mut categories: Vec<&Category> = self.categories.values().collect();
        categories.sort_by(|left, right| {
            left.order
                .cmp(&right.order)
                .then_with(|| left.name.cmp(&right.name))
        });
        categories
    }

    pub fn map_name(&self, code: &str) -> Option<&LocalizedName> {
        self.map_names.get(code)
    }

    pub fn quest_title(&self, alias: &str) -> Option<&LocalizedName> {
        self.quest_titles.get(alias)
    }
}

#[derive(Debug)]
pub enum CatalogError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CatalogError::Io(error) => write!(formatter, "{error}"),
            CatalogError::Json(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for CatalogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CatalogError::Io(error) => Some(error),
            CatalogError::Json(error) => Some(error),
        }
    }
}

impl From<io::Error> for CatalogError {
    fn from(error: io::Error) -> Self {
        CatalogError::Io(error)
    }
}

impl From<serde_json::Error> for CatalogError {
    fn from(error: serde_json::Error) -> Self {
        CatalogError::Json(error)
    }
}

fn json_str(raw: &JsonValue, key: &str) -> Option<String> {
    raw.get(key).and_then(JsonValue::as_str).map(str::to_string)
}

fn parse_guide_link(raw: &JsonValue) -> Option<GuideLink> {
    let title = json_str(raw, "title")?;
    let url = json_str(raw, "url")?;
    if title.is_empty() || url.is_empty() {
        return None;
    }
    Some(GuideLink { title, url })
}

fn parse_guides(raw: &JsonValue) -> Option<Guides> {
    let guides = Guides {
        web: raw.get("web").and_then(parse_guide_link),
        video: raw.get("video").and_then(parse_guide_link),
    };
    if guides.web.is_none() && guides.video.is_none() {
        return None;
    }
    Some(guides)
}

fn parse_string_list(raw: Option<&JsonValue>) -> Vec<String> {
    raw.and_then(JsonValue::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(JsonValue::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn parse_missable_rule(raw: &JsonValue) -> Option<MissableRule> {
    let rule = MissableRule {
        void_after_quests: parse_string_list(raw.get("void_after_quests")),
        requires_quests: parse_string_list(raw.get("requires_quests")),
        void_on_failed_quests: parse_string_list(raw.get("void_on_failed_quests")),
    };
    if rule.is_empty() {
        return None;
    }
    Some(rule)
}

fn parse_localized_names(raw: Option<&JsonValue>) -> HashMap<String, LocalizedName> {
    let mut names = HashMap::new();
    let Some(entries) = raw.and_then(JsonValue::as_object) else {
        return names;
    };
    for (key, value) in entries {
        let Some(name) = json_str(value, "name").filter(|name| !name.is_empty()) else {
            continue;
        };
        let name_zh = json_str(value, "name_zh")
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| name.clone());
        names.insert(key.clone(), LocalizedName { name, name_zh });
    }
    names
}

fn parse_item(raw: &JsonValue) -> CatalogItem {
    let id = json_str(raw, "id").unwrap_or_default();
    let name = json_str(raw, "name")
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| id.clone());
    let mut aliases = Vec::new();
    match raw.get("aliases") {
        Some(JsonValue::Array(values)) => aliases.extend(
            values
                .iter()
                .filter_map(JsonValue::as_str)
                .map(str::to_string),
        ),
        Some(JsonValue::String(value)) => aliases.push(value.clone()),
        _ => {}
    }
    CatalogItem {
        id,
        name,
        category: json_str(raw, "category").unwrap_or_default(),
        aliases,
        area: json_str(raw, "area"),
        area_zh: json_str(raw, "area_zh"),
        location: json_str(raw, "location"),
        location_zh: json_str(raw, "location_zh"),
        obtain: json_str(raw, "obtain"),
        obtain_zh: json_str(raw, "obtain_zh"),
        ng_plus: raw.get("ng_plus").and_then(JsonValue::as_i64).unwrap_or(0),
        dlc: json_str(raw, "dlc"),
        missable: raw
            .get("missable")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false),
        missable_rule: raw.get("missable_rule").and_then(parse_missable_rule),
        note: json_str(raw, "note"),
        confidence: json_str(raw, "confidence").unwrap_or_else(|| "high".to_string()),
        source: json_str(raw, "source"),
        name_en: json_str(raw, "name_en"),
        record_type: json_str(raw, "record_type"),
        record_type_zh: json_str(raw, "record_type_zh"),
        record_type_en: json_str(raw, "record_type_en"),
        order: raw.get("order").and_then(JsonValue::as_i64).unwrap_or(0),
        guides: raw.get("guides").and_then(parse_guides),
        desc_zh: json_str(raw, "desc_zh"),
        desc_en: json_str(raw, "desc_en"),
    }
}

fn parse_catalog(payload: &JsonValue) -> Catalog {
    let mut categories: HashMap<String, Category> = HashMap::new();
    if let Some(list) = payload.get("categories").and_then(JsonValue::as_array) {
        for raw in list {
            let key = json_str(raw, "key").unwrap_or_default();
            let name = json_str(raw, "name")
                .filter(|name| !name.is_empty())
                .unwrap_or_else(|| key.clone());
            let order = raw.get("order").and_then(JsonValue::as_i64).unwrap_or(100);
            let section = json_str(raw, "section").unwrap_or_else(|| "collection".to_string());
            let name_en = json_str(raw, "name_en");
            categories.insert(
                key.clone(),
                Category {
                    key,
                    name,
                    name_en,
                    order,
                    section,
                    aliases: Vec::new(),
                },
            );
        }
    }
    let items: Vec<CatalogItem> = payload
        .get("items")
        .and_then(JsonValue::as_array)
        .map(|list| list.iter().map(parse_item).collect())
        .unwrap_or_default();
    for item in &items {
        categories
            .entry(item.category.clone())
            .or_insert_with(|| Category {
                key: item.category.clone(),
                name: item.category.clone(),
                name_en: None,
                order: 100,
                section: "collection".to_string(),
                aliases: Vec::new(),
            });
    }
    Catalog {
        version: payload
            .get("version")
            .and_then(JsonValue::as_i64)
            .unwrap_or(1),
        categories,
        items,
        map_names: parse_localized_names(payload.get("map_names")),
        quest_titles: parse_localized_names(payload.get("quest_titles")),
    }
}

fn read_json(path: &Path) -> Result<JsonValue, CatalogError> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn user_catalog_path() -> PathBuf {
    paths::local_app_data()
        .join("sbsave")
        .join("catalog.user.json")
}

pub fn load_catalog(extra_paths: Option<&[PathBuf]>) -> Result<Catalog, CatalogError> {
    let mut catalog = parse_catalog(&serde_json::from_str(CATALOG_JSON)?);

    let user_path = user_catalog_path();
    if user_path.is_file() {
        merge(&mut catalog, &read_json(&user_path)?);
    }
    if let Some(paths) = extra_paths {
        for path in paths {
            if path.is_file() {
                merge(&mut catalog, &read_json(path)?);
            }
        }
    }
    Ok(catalog)
}

fn merge(catalog: &mut Catalog, payload: &JsonValue) {
    if let Some(list) = payload.get("categories").and_then(JsonValue::as_array) {
        for raw in list {
            let key = json_str(raw, "key").unwrap_or_default();
            let name = json_str(raw, "name")
                .filter(|name| !name.is_empty())
                .unwrap_or_else(|| key.clone());
            let order = raw.get("order").and_then(JsonValue::as_i64).unwrap_or(100);
            let name_en = json_str(raw, "name_en");
            match catalog.categories.get_mut(&key) {
                Some(existing) => {
                    if raw.get("name").is_some() {
                        existing.name = name;
                    }
                    if name_en.is_some() {
                        existing.name_en = name_en.clone();
                    }
                }
                None => {
                    catalog.categories.insert(
                        key.clone(),
                        Category {
                            key,
                            name,
                            name_en,
                            order,
                            section: json_str(raw, "section")
                                .unwrap_or_else(|| "collection".to_string()),
                            aliases: Vec::new(),
                        },
                    );
                }
            }
        }
    }
    if let Some(list) = payload.get("items").and_then(JsonValue::as_array) {
        for raw in list {
            let item = parse_item(raw);
            let category = item.category.clone();
            if let Some(index) = catalog
                .items
                .iter()
                .position(|existing| existing.id == item.id)
            {
                catalog.items[index] = item;
            } else {
                catalog.items.push(item);
            }
            catalog
                .categories
                .entry(category.clone())
                .or_insert_with(|| Category {
                    key: category.clone(),
                    name: category,
                    name_en: None,
                    order: 100,
                    section: "collection".to_string(),
                    aliases: Vec::new(),
                });
        }
    }
}
