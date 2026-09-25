use std::collections::HashMap;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::paths;

const CATALOG_JSON: &str = include_str!("../../../data/catalog.json");

pub fn ng_plus_label(ng_plus: i64) -> String {
    match ng_plus {
        0 => "首周目".to_string(),
        1 => "二周目(NG+)".to_string(),
        2 => "三周目(NG++)".to_string(),
        other => format!("NG+{other}"),
    }
}

pub fn dlc_label(dlc: &str) -> String {
    match dlc {
        "nier" => "尼尔 DLC",
        "nikke" => "NIKKE DLC",
        "deluxe" => "豪华版",
        "preorder" => "预购特典",
        "summer" => "夏日更新",
        other => other,
    }
    .to_string()
}

#[derive(Debug, Clone)]
pub struct Category {
    pub key: String,
    pub name: String,
    pub order: i64,
    pub section: String,
    pub aliases: Vec<String>,
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
    pub note: Option<String>,
    pub confidence: String,
    pub source: Option<String>,
    pub name_en: Option<String>,
    pub record_type: Option<String>,
    pub record_type_zh: Option<String>,
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

    pub fn ng_plus_label(&self) -> String {
        ng_plus_label(self.ng_plus)
    }

    pub fn dlc_label(&self) -> Option<String> {
        self.dlc.as_deref().map(dlc_label)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub version: i64,
    pub categories: HashMap<String, Category>,
    pub items: Vec<CatalogItem>,
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
        note: json_str(raw, "note"),
        confidence: json_str(raw, "confidence").unwrap_or_else(|| "high".to_string()),
        source: json_str(raw, "source"),
        name_en: json_str(raw, "name_en"),
        record_type: json_str(raw, "record_type"),
        record_type_zh: json_str(raw, "record_type_zh"),
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
            categories.insert(
                key.clone(),
                Category {
                    key,
                    name,
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
            match catalog.categories.get_mut(&key) {
                Some(existing) => {
                    if raw.get("name").is_some() {
                        existing.name = name;
                    }
                }
                None => {
                    catalog.categories.insert(
                        key.clone(),
                        Category {
                            key,
                            name,
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
                    order: 100,
                    section: "collection".to_string(),
                    aliases: Vec::new(),
                });
        }
    }
}
