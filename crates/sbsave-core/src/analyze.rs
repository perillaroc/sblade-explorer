use std::collections::HashMap;

use crate::catalog::{Catalog, CatalogItem, Category};
use crate::savegame::SaveData;

#[derive(Debug, Clone)]
pub struct ItemStatus<'a> {
    pub item: &'a CatalogItem,
    pub obtained: bool,
    pub reason: Option<String>,
}

impl ItemStatus<'_> {
    pub fn flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        if self.item.dlc.is_some() {
            flags.push(self.item.dlc_label().unwrap_or_default());
        }
        if self.item.ng_plus > 0 {
            flags.push(self.item.ng_plus_label());
        }
        if self.item.missable {
            flags.push("可错过".to_string());
        }
        if self.item.confidence != "high" {
            flags.push("映射待确认".to_string());
        }
        flags
    }
}

#[derive(Debug)]
pub struct CategoryResult<'a> {
    pub category: &'a Category,
    pub total: usize,
    pub obtained_count: usize,
    pub missing: Vec<ItemStatus<'a>>,
    pub obtained_items: Vec<&'a CatalogItem>,
    pub extra_obtained: Vec<String>,
    pub blocked_count: usize,
    pub statuses: Vec<ItemStatus<'a>>,
}

impl CategoryResult<'_> {
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.obtained_count as f64 / self.total as f64 * 100.0
        }
    }
}

#[derive(Debug)]
pub struct Analysis<'a> {
    pub save: &'a SaveData,
    pub categories: Vec<CategoryResult<'a>>,
    pub unmapped_obtained: Vec<String>,
    pub catalog_total: usize,
    pub catalog_obtained: usize,
}

impl Analysis<'_> {
    pub fn missing_total(&self) -> usize {
        self.categories
            .iter()
            .map(|result| result.missing.len())
            .sum()
    }

    pub fn percent(&self) -> f64 {
        if self.catalog_total == 0 {
            0.0
        } else {
            self.catalog_obtained as f64 / self.catalog_total as f64 * 100.0
        }
    }
}

fn blocked_reason(item: &CatalogItem, save: &SaveData) -> Option<String> {
    if item.ng_plus > save.ng_plus_count() {
        return Some(format!("需要{}", item.ng_plus_label()));
    }
    if item.dlc.is_some() {
        return Some(format!("{}限定", item.dlc_label().unwrap_or_default()));
    }
    if item.missable {
        return Some("可错过(注意节点)".to_string());
    }
    None
}

fn alias_prefix(alias: &str) -> &str {
    if let Some((head, _)) = alias.split_once('_') {
        if !head.is_empty() && !head.chars().all(|ch| ch.is_ascii_digit()) {
            return head;
        }
    }
    alias
}

pub fn unmapped_summary(aliases: &[String]) -> Vec<(String, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for alias in aliases {
        *counts.entry(alias_prefix(alias)).or_default() += 1;
    }
    let mut entries: Vec<(String, usize)> = counts
        .into_iter()
        .map(|(prefix, count)| (prefix.to_string(), count))
        .collect();
    entries.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    entries
}

fn category_prefixes(key: &str) -> Option<&'static [&'static str]> {
    match key {
        "nano_suits" => Some(&["BS_", "Nanosuit", "NanoSuit"]),
        "cans" => Some(&["Can_"]),
        "records" => Some(&["Item_Records_"]),
        "passcodes" => Some(&["Item_Records_"]),
        "camps" => Some(&["ChangeState_ZoneEnv_"]),
        "hair" => Some(&["Hair_"]),
        "glasses" => Some(&["FaceAccessory_"]),
        "earrings" => Some(&["Earring_"]),
        "drone_seals" => Some(&["DroneSeal_"]),
        "design_patterns" => Some(&["DesignPattern_"]),
        "adam_costumes" => Some(&["AdamCostume_"]),
        "lily_costumes" => Some(&["LilyCostume_"]),
        "fish" => Some(&["Fish_"]),
        "gear" => Some(&["Gear_"]),
        _ => None,
    }
}

fn category_matches_alias(category: &Category, alias: &str) -> bool {
    match category_prefixes(&category.key) {
        Some(prefixes) => prefixes.iter().any(|prefix| alias.starts_with(prefix)),
        None => false,
    }
}

pub fn analyze<'a>(
    save: &'a SaveData,
    catalog: &'a Catalog,
    categories: Option<&[String]>,
) -> Analysis<'a> {
    let obtained = save.all_obtained();
    let alias_index = catalog.alias_index();

    let mut results: Vec<CategoryResult<'a>> = Vec::new();
    for category in catalog.category_list() {
        if categories.is_some_and(|filter| !filter.iter().any(|key| key == &category.key)) {
            continue;
        }
        let items = catalog.by_category(&category.key);
        if items.is_empty() {
            continue;
        }
        let total = items.len();
        let mut missing: Vec<ItemStatus<'a>> = Vec::new();
        let mut obtained_items: Vec<&'a CatalogItem> = Vec::new();
        let mut statuses: Vec<ItemStatus<'a>> = Vec::new();
        let mut blocked = 0usize;
        for item in items {
            let is_obtained = item
                .satisfy_aliases()
                .iter()
                .any(|alias| obtained.contains(*alias));
            let reason = if is_obtained {
                None
            } else {
                blocked_reason(item, save)
            };
            if reason.is_some() {
                blocked += 1;
            }
            if is_obtained {
                obtained_items.push(item);
            }
            let status = ItemStatus {
                item,
                obtained: is_obtained,
                reason,
            };
            if !is_obtained {
                missing.push(status.clone());
            }
            statuses.push(status);
        }
        let mut extra_obtained: Vec<String> = obtained
            .iter()
            .filter(|alias| {
                !alias_index.contains_key(alias.as_str()) && category_matches_alias(category, alias)
            })
            .cloned()
            .collect();
        extra_obtained.sort();
        results.push(CategoryResult {
            category,
            total,
            obtained_count: obtained_items.len(),
            missing,
            obtained_items,
            extra_obtained,
            blocked_count: blocked,
            statuses,
        });
    }

    let mut unmapped_obtained: Vec<String> = obtained
        .iter()
        .filter(|alias| !alias_index.contains_key(alias.as_str()))
        .cloned()
        .collect();
    unmapped_obtained.sort();

    let catalog_total = results.iter().map(|result| result.total).sum();
    let catalog_obtained = results.iter().map(|result| result.obtained_count).sum();
    Analysis {
        save,
        categories: results,
        unmapped_obtained,
        catalog_total,
        catalog_obtained,
    }
}
