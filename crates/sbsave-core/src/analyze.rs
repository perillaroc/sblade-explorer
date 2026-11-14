use std::collections::{HashMap, HashSet};

use crate::catalog::{Catalog, CatalogItem, Category, LocalizedName};
use crate::i18n::{Locale, Messages};
use crate::savegame::SaveData;

/// DLC keys the user can mark as owned. The save carries no reliable
/// entitlement field, so ownership is supplied by settings / CLI.
pub const KNOWN_DLCS: [&str; 2] = ["nier", "nikke"];

/// Lowercases, filters to known DLC keys and deduplicates.
pub fn normalize_owned_dlcs<I, S>(values: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut owned: Vec<String> = values
        .into_iter()
        .map(|value| value.as_ref().trim().to_ascii_lowercase())
        .filter(|value| KNOWN_DLCS.contains(&value.as_str()))
        .collect();
    owned.sort();
    owned.dedup();
    owned
}

/// Why a missing item cannot be obtained in the current playthrough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockReason {
    NgPlus(i64),
    Dlc(String),
    Missable,
    /// The save's quest state proves the item is no longer obtainable.
    Missed,
}

/// Save-aware state of a statically missable entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MissableState {
    /// The rule is satisfied now, so the item can still be collected.
    Open,
    /// A quest in the save's completed/failed lists closed the window.
    Missed,
    /// No rule, no quest data, or an incomplete rule: never report missed.
    #[default]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ItemStatus<'a> {
    pub item: &'a CatalogItem,
    pub obtained: bool,
    pub reason: Option<BlockReason>,
    pub missable_state: MissableState,
    /// The item belongs to a DLC the user marked as owned.
    pub dlc_owned: bool,
}

impl ItemStatus<'_> {
    pub fn flags(&self, locale: Locale) -> Vec<String> {
        let messages = Messages::new(locale);
        let mut flags = Vec::new();
        if let Some(dlc) = self.item.dlc.as_deref() {
            flags.push(if self.dlc_owned {
                messages.flag_dlc_owned(dlc)
            } else {
                self.item.dlc_label(locale).unwrap_or_default()
            });
        }
        if self.item.ng_plus > 0 {
            flags.push(self.item.ng_plus_label(locale));
        }
        if self.item.missable {
            flags.push(if self.missable_state == MissableState::Missed {
                messages.flag_missed().to_string()
            } else {
                messages.flag_missable().to_string()
            });
        }
        if self.item.confidence != "high" {
            flags.push(messages.flag_unconfirmed().to_string());
        }
        flags
    }

    /// Localized explanation of why the item is missing.
    pub fn reason_label(&self, locale: Locale) -> Option<String> {
        let messages = Messages::new(locale);
        self.reason.as_ref().map(|reason| match reason {
            BlockReason::NgPlus(ng_plus) => messages.reason_ng_plus(*ng_plus),
            BlockReason::Dlc(dlc) => messages.reason_dlc(dlc),
            BlockReason::Missable => {
                if self.missable_state == MissableState::Open {
                    messages.reason_missable_open().to_string()
                } else {
                    messages.reason_missable().to_string()
                }
            }
            BlockReason::Missed => messages.reason_missed().to_string(),
        })
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

    /// Missing entries proven unobtainable in this save.
    pub fn missed_count(&self) -> usize {
        self.statuses
            .iter()
            .filter(|status| !status.obtained && status.missable_state == MissableState::Missed)
            .count()
    }

    /// Missing entries that are still obtainable in this save.
    pub fn missable_open_count(&self) -> usize {
        self.statuses
            .iter()
            .filter(|status| !status.obtained && status.missable_state == MissableState::Open)
            .count()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentStory<'a> {
    /// Raw `MapName` code from the save (e.g. `E04`), when present.
    pub map_code: Option<&'a str>,
    /// Resolved region name; `None` when the code is unknown.
    pub map: Option<&'a LocalizedName>,
    /// Active `Quest_Epic_*` alias, when present.
    pub epic_quest: Option<&'a str>,
    /// Resolved quest title; `None` when the title is not in the data.
    pub epic_quest_title: Option<&'a LocalizedName>,
}

#[derive(Debug)]
pub struct Analysis<'a> {
    pub save: &'a SaveData,
    pub categories: Vec<CategoryResult<'a>>,
    pub unmapped_obtained: Vec<String>,
    pub catalog_total: usize,
    pub catalog_obtained: usize,
    pub album_total: usize,
    pub album_obtained: usize,
    pub story: CurrentStory<'a>,
    /// DLC keys supplied by settings / CLI, not read from the save.
    pub owned_dlcs: Vec<String>,
}

fn is_album(category: &Category) -> bool {
    category.section == "album"
}

impl Analysis<'_> {
    /// Missing entries of the collectible categories (excludes the album).
    pub fn missing_total(&self) -> usize {
        self.categories
            .iter()
            .filter(|result| !is_album(result.category))
            .map(|result| result.missing.len())
            .sum()
    }

    /// Missing entries of the album categories (图鉴).
    pub fn album_missing_total(&self) -> usize {
        self.categories
            .iter()
            .filter(|result| is_album(result.category))
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

    pub fn album_percent(&self) -> f64 {
        if self.album_total == 0 {
            0.0
        } else {
            self.album_obtained as f64 / self.album_total as f64 * 100.0
        }
    }

    /// Missing entries proven unobtainable in this save.
    pub fn missed_count(&self) -> usize {
        self.categories
            .iter()
            .map(CategoryResult::missed_count)
            .sum()
    }

    /// Missing entries that are still obtainable in this save.
    pub fn missable_open_count(&self) -> usize {
        self.categories
            .iter()
            .map(CategoryResult::missable_open_count)
            .sum()
    }
}

/// Evaluates a `missable_rule` against the save's quest lists.
///
/// Unknown beats missed: when the save has no quest data, or a rule lacks a
/// condition we can evaluate, the entry falls back to the legacy static text.
fn missable_state(item: &CatalogItem, save: &SaveData) -> MissableState {
    let Some(rule) = item.missable_rule.as_ref() else {
        return MissableState::Unknown;
    };
    if save.quests.is_empty() {
        return MissableState::Unknown;
    }
    if rule
        .void_after_quests
        .iter()
        .any(|quest| save.quests.completed.contains(quest))
    {
        return MissableState::Missed;
    }
    if rule
        .void_on_failed_quests
        .iter()
        .any(|quest| save.quests.failed.contains(quest))
    {
        return MissableState::Missed;
    }
    if !rule.requires_quests.is_empty()
        && !rule
            .requires_quests
            .iter()
            .any(|quest| save.quests.has_started(quest))
    {
        // The area has not opened yet; without the final node rule we cannot
        // tell whether the entry has already been lost.
        return MissableState::Unknown;
    }
    MissableState::Open
}

fn evaluate(
    item: &CatalogItem,
    save: &SaveData,
    dlc_owned: bool,
) -> (Option<BlockReason>, MissableState) {
    if item.ng_plus > save.ng_plus_count() {
        return (
            Some(BlockReason::NgPlus(item.ng_plus)),
            MissableState::Unknown,
        );
    }
    if let Some(dlc) = item.dlc.as_deref() {
        if !dlc_owned {
            return (
                Some(BlockReason::Dlc(dlc.to_string())),
                MissableState::Unknown,
            );
        }
    }
    if item.missable {
        let state = missable_state(item, save);
        let reason = if state == MissableState::Missed {
            BlockReason::Missed
        } else {
            BlockReason::Missable
        };
        return (Some(reason), state);
    }
    (None, MissableState::Unknown)
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
    analyze_with_dlcs(save, catalog, categories, &[])
}

/// Same as [`analyze`], with DLC ownership supplied by settings / CLI.
pub fn analyze_with_dlcs<'a>(
    save: &'a SaveData,
    catalog: &'a Catalog,
    categories: Option<&[String]>,
    owned_dlcs: &[String],
) -> Analysis<'a> {
    let owned = normalize_owned_dlcs(owned_dlcs.iter());
    let owned_set: HashSet<&str> = owned.iter().map(String::as_str).collect();
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
                .any(|alias| obtained.contains(*alias) || save.achievements.contains_key(*alias));
            let dlc_owned = item
                .dlc
                .as_deref()
                .is_some_and(|dlc| owned_set.contains(dlc));
            let (reason, missable_state) = if is_obtained {
                (None, MissableState::Unknown)
            } else {
                evaluate(item, save, dlc_owned)
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
                missable_state,
                dlc_owned,
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

    let catalog_total = results
        .iter()
        .filter(|result| !is_album(result.category))
        .map(|result| result.total)
        .sum();
    let catalog_obtained = results
        .iter()
        .filter(|result| !is_album(result.category))
        .map(|result| result.obtained_count)
        .sum();
    let album_total = results
        .iter()
        .filter(|result| is_album(result.category))
        .map(|result| result.total)
        .sum();
    let album_obtained = results
        .iter()
        .filter(|result| is_album(result.category))
        .map(|result| result.obtained_count)
        .sum();
    let map_code = save.current_map();
    let epic_quest = save.current_epic_quest();
    let story = CurrentStory {
        map_code,
        map: map_code.and_then(|code| catalog.map_name(code)),
        epic_quest,
        epic_quest_title: epic_quest.and_then(|alias| catalog.quest_title(alias)),
    };
    Analysis {
        save,
        categories: results,
        unmapped_obtained,
        catalog_total,
        catalog_obtained,
        album_total,
        album_obtained,
        story,
        owned_dlcs: owned,
    }
}
