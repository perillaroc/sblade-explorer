use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::analyze::{Analysis, CategoryResult, ItemStatus};
use crate::catalog::CatalogItem;
use crate::savegame::SaveData;

pub const LANGUAGES: [&str; 3] = ["zh", "en", "both"];

const MATRIX_LEGEND: &str = "✅ 已获得 · ❌ 未获得 · 🔒 需更高周目 · 🎁 DLC/特典 · ➖ 默认外观";
const MATRIX_COLUMNS: [&str; 4] = ["首周目", "二周目(NG+)", "三周目(NG++)", "DLC/特典"];
const MATRIX_CATEGORIES: [&str; 6] = [
    "nano_suits",
    "earrings",
    "glasses",
    "drone_seals",
    "adam_costumes",
    "lily_costumes",
];

fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|text| !text.is_empty())
}

fn localized(zh: Option<&str>, en: Option<&str>, lang: &str) -> String {
    if lang == "en" {
        return non_empty(en)
            .or_else(|| non_empty(zh))
            .unwrap_or("")
            .to_string();
    }
    if lang == "both" {
        if let (Some(zh), Some(en)) = (non_empty(zh), non_empty(en)) {
            if zh != en {
                return format!("{zh}（{en}）");
            }
        }
    }
    non_empty(zh)
        .or_else(|| non_empty(en))
        .unwrap_or("")
        .to_string()
}

fn display_name(item: &CatalogItem, lang: &str) -> String {
    if lang == "en" {
        return non_empty(item.name_en.as_deref())
            .unwrap_or(&item.name)
            .to_string();
    }
    if lang == "both" {
        if let Some(en) = non_empty(item.name_en.as_deref()) {
            if en != item.name {
                return format!("{}（{en}）", item.name);
            }
        }
    }
    item.name.clone()
}

fn location_label(status: &ItemStatus, lang: &str) -> String {
    let item = status.item;
    let parts = [
        localized(item.area_zh.as_deref(), item.area.as_deref(), lang),
        localized(item.location_zh.as_deref(), item.location.as_deref(), lang),
    ];
    parts
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" · ")
}

fn obtain_label(status: &ItemStatus, lang: &str) -> String {
    let item = status.item;
    let mut parts: Vec<String> = Vec::new();
    let obtain = localized(item.obtain_zh.as_deref(), item.obtain.as_deref(), lang);
    if !obtain.is_empty() {
        parts.push(obtain);
    }
    if let Some(note) = non_empty(item.note.as_deref()) {
        parts.push(note.to_string());
    }
    if let Some(reason) = &status.reason {
        parts.push(reason.clone());
    }
    parts.join(" | ")
}

fn item_emoji(status: &ItemStatus, save: &SaveData) -> &'static str {
    if status.obtained {
        return "✅";
    }
    let item = status.item;
    if item.dlc.is_some() {
        return "🎁";
    }
    if item.ng_plus > save.ng_plus_count() {
        return "🔒";
    }
    if item.aliases.is_empty() {
        return "➖";
    }
    "❌"
}

fn area_order(area: &str) -> i32 {
    match area {
        "Default" => 0,
        "Eidos 7" => 1,
        "Xion" => 2,
        "Wasteland" => 3,
        "Matrix 11" => 5,
        "Great Desert" => 6,
        "Abyss Levoire" => 7,
        "Eidos 9" => 8,
        "Spire 4" => 9,
        "Boss Challenge" => 11,
        _ => 99,
    }
}

type AreaMatrix = Vec<(String, Vec<(String, Vec<String>)>)>;
type RawAreaEntry = (String, String, Vec<(String, Vec<String>)>);

fn item_matrix(statuses: &[ItemStatus], save: &SaveData, lang: &str) -> AreaMatrix {
    let mut labels: HashMap<String, String> = HashMap::new();
    let mut raw_areas: Vec<String> = Vec::new();
    let mut area_index: HashMap<String, usize> = HashMap::new();
    let mut location_index: Vec<HashMap<String, usize>> = Vec::new();
    let mut areas: Vec<Vec<(String, Vec<String>)>> = Vec::new();

    for status in statuses {
        let item = status.item;
        let area = item.area.clone().unwrap_or_else(|| "未分类".to_string());
        let location = item
            .location
            .clone()
            .unwrap_or_else(|| "未分类".to_string());

        labels.entry(area.clone()).or_insert_with(|| {
            let label = localized(item.area_zh.as_deref(), item.area.as_deref(), lang);
            if label.is_empty() {
                "未分类".to_string()
            } else {
                label
            }
        });
        labels.entry(location.clone()).or_insert_with(|| {
            let label = localized(item.location_zh.as_deref(), item.location.as_deref(), lang);
            if label.is_empty() {
                "未分类".to_string()
            } else {
                label
            }
        });

        let area_slot = *area_index.entry(area.clone()).or_insert_with(|| {
            raw_areas.push(area.clone());
            areas.push(Vec::new());
            location_index.push(HashMap::new());
            areas.len() - 1
        });
        let location_slot = *location_index[area_slot]
            .entry(location.clone())
            .or_insert_with(|| {
                areas[area_slot]
                    .push((location.clone(), vec![String::new(); MATRIX_COLUMNS.len()]));
                areas[area_slot].len() - 1
            });

        let column = if item.dlc.is_some() {
            3
        } else {
            item.ng_plus.clamp(0, 2) as usize
        };
        let entry = format!("{} {}", item_emoji(status, save), display_name(item, lang));
        let cell = &mut areas[area_slot][location_slot].1[column];
        if cell.is_empty() {
            *cell = entry;
        } else {
            cell.push('\n');
            cell.push_str(&entry);
        }
    }

    let mut collected: Vec<RawAreaEntry> = raw_areas
        .iter()
        .enumerate()
        .map(|(index, raw_area)| {
            let label = labels
                .get(raw_area)
                .cloned()
                .unwrap_or_else(|| "未分类".to_string());
            let locations = areas[index]
                .iter()
                .map(|(raw_location, cells)| {
                    let location = labels
                        .get(raw_location)
                        .cloned()
                        .unwrap_or_else(|| "未分类".to_string());
                    (location, cells.clone())
                })
                .collect();
            (raw_area.clone(), label, locations)
        })
        .collect();
    collected.sort_by(|left, right| {
        area_order(&left.0)
            .cmp(&area_order(&right.0))
            .then_with(|| left.0.cmp(&right.0))
    });
    collected
        .into_iter()
        .map(|(_, label, locations)| (label, locations))
        .collect()
}

fn used_matrix_columns(locations: &[(String, Vec<String>)]) -> Vec<usize> {
    (0..MATRIX_COLUMNS.len())
        .filter(|index| locations.iter().any(|(_, cells)| !cells[*index].is_empty()))
        .collect()
}

fn is_matrix_category(key: &str) -> bool {
    MATRIX_CATEGORIES.contains(&key)
}

fn is_album_category(result: &CategoryResult) -> bool {
    result.category.section == "album"
}

fn print_item_matrix(result: &CategoryResult, save: &SaveData, lang: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{} 已获得 {}/{} · 图例：{}\n",
        result.category.name, result.obtained_count, result.total, MATRIX_LEGEND
    ));
    for (area, locations) in item_matrix(&result.statuses, save, lang) {
        let used = used_matrix_columns(&locations);
        out.push_str(&format!("{area}\n"));
        let headers: Vec<&str> = used.iter().map(|index| MATRIX_COLUMNS[*index]).collect();
        out.push_str(&format!("地点 | {}\n", headers.join(" | ")));
        for (location, cells) in locations {
            let values: Vec<String> = used
                .iter()
                .map(|index| {
                    if cells[*index].is_empty() {
                        "—".to_string()
                    } else {
                        cells[*index].replace('\n', " ")
                    }
                })
                .collect();
            out.push_str(&format!("{location} | {}\n", values.join(" | ")));
        }
        out.push('\n');
    }
    out
}

fn item_markdown(result: &CategoryResult, save: &SaveData, lang: &str) -> Vec<String> {
    let mut lines = vec![
        format!("## {}获取一览", result.category.name),
        String::new(),
    ];
    lines.push(format!(
        "已获得 {}/{} · 图例：{}",
        result.obtained_count, result.total, MATRIX_LEGEND
    ));
    for (area, locations) in item_matrix(&result.statuses, save, lang) {
        let used = used_matrix_columns(&locations);
        lines.push(String::new());
        lines.push(format!("### {area}"));
        lines.push(String::new());
        let mut headers = vec!["地点".to_string()];
        headers.extend(used.iter().map(|index| MATRIX_COLUMNS[*index].to_string()));
        lines.push(format!("| {} |", headers.join(" | ")));
        lines.push(format!(
            "| --- | {} |",
            used.iter().map(|_| "---").collect::<Vec<_>>().join(" | ")
        ));
        for (location, cells) in locations {
            let values: Vec<String> = used
                .iter()
                .map(|index| {
                    if cells[*index].is_empty() {
                        "—".to_string()
                    } else {
                        cells[*index].replace('\n', "<br>")
                    }
                })
                .map(|value| value.replace('|', "\\|"))
                .collect();
            lines.push(format!("| {location} | {} |", values.join(" | ")));
        }
    }
    lines
}

pub fn print_report(analysis: &Analysis, show_all: bool, lang: &str) -> String {
    let save = analysis.save;
    let mut out = String::new();
    out.push_str(&format!(
        "剑星存档分析 — {}\n",
        save.path.file_name().map_or_else(
            || save.path.display().to_string(),
            |name| name.to_string_lossy().into_owned()
        )
    ));
    out.push_str(&format!(
        "SteamID: {} | 周目: {} (NG+{}) | 难度: {} | 游玩时间: {}\n",
        save.steam_id.as_deref().unwrap_or("未知"),
        save.playthrough_label(),
        save.ng_plus_count(),
        save.difficulty_label(),
        save.play_time_label()
    ));
    out.push_str(&format!(
        "已获得物品别名: {} | 目录进度: {}/{} ({:.1}%) | 未收集: {}\n",
        save.all_obtained().len(),
        analysis.catalog_obtained,
        analysis.catalog_total,
        analysis.percent(),
        analysis.missing_total()
    ));
    out.push_str(&format!(
        "图鉴进度: {}/{} ({:.1}%) | 未收集: {}（不计入目录进度）\n",
        analysis.album_obtained,
        analysis.album_total,
        analysis.album_percent(),
        analysis.album_missing_total()
    ));
    out.push('\n');

    out.push_str("分类汇总\n");
    out.push_str("分类 | 进度 | 缺失 | 其中需多周目/DLC | 未映射别名\n");
    for result in &analysis.categories {
        if is_album_category(result) {
            continue;
        }
        out.push_str(&format!(
            "{} | {}/{} ({:.0}%) | {} | {} | {}\n",
            result.category.name,
            result.obtained_count,
            result.total,
            result.percent(),
            result.missing.len(),
            result.blocked_count,
            result.extra_obtained.len()
        ));
    }

    out.push_str("\n图鉴汇总（不计入目录进度）\n");
    out.push_str("图鉴 | 进度 | 缺失\n");
    for result in &analysis.categories {
        if !is_album_category(result) {
            continue;
        }
        out.push_str(&format!(
            "{} | {}/{} ({:.0}%) | {}\n",
            result.category.name,
            result.obtained_count,
            result.total,
            result.percent(),
            result.missing.len()
        ));
    }

    for result in &analysis.categories {
        if !is_matrix_category(&result.category.key) {
            continue;
        }
        out.push('\n');
        out.push_str(&print_item_matrix(result, save, lang));
    }

    for result in &analysis.categories {
        if is_matrix_category(&result.category.key) || result.missing.is_empty() {
            continue;
        }
        out.push('\n');
        out.push_str(&format!(
            "{} {}/{} · 缺 {}\n",
            result.category.name,
            result.obtained_count,
            result.total,
            result.missing.len()
        ));
        for status in &result.missing {
            let flags = status.flags();
            let flag_text = if flags.is_empty() {
                String::new()
            } else {
                format!(" [{}]", flags.join("/"))
            };
            let location = location_label(status, lang);
            let obtain = obtain_label(status, lang);
            let mut line = format!("  {} {}", status.item.id, display_name(status.item, lang));
            if !location.is_empty() {
                line.push_str(&format!("  {location}"));
            }
            if !obtain.is_empty() {
                line.push_str(&format!("  ({obtain})"));
            }
            line.push_str(&flag_text);
            out.push_str(&line);
            out.push('\n');
        }
    }

    if show_all {
        for result in &analysis.categories {
            if is_matrix_category(&result.category.key) || result.obtained_items.is_empty() {
                continue;
            }
            out.push('\n');
            out.push_str(&format!("{} 已收集:\n", result.category.name));
            for item in &result.obtained_items {
                out.push_str(&format!("  ✓ {} {}\n", item.id, display_name(item, lang)));
            }
        }
    }

    if !analysis.unmapped_obtained.is_empty() {
        out.push('\n');
        out.push_str(&format!(
            "有 {} 个已获得别名不在目录库中（不影响已收集判定，可补充到用户覆盖文件）\n",
            analysis.unmapped_obtained.len()
        ));
        for (prefix, count) in crate::analyze::unmapped_summary(&analysis.unmapped_obtained)
            .into_iter()
            .take(15)
        {
            out.push_str(&format!("  {prefix}: {count}\n"));
        }
    }
    out
}

fn round2(value: f64) -> f64 {
    format!("{value:.2}").parse().unwrap_or(value)
}

fn optional_string(value: Option<&str>) -> Value {
    match value {
        Some(text) => Value::String(text.to_string()),
        None => Value::Null,
    }
}

fn catalog_item_to_dict(item: &CatalogItem) -> Value {
    let mut object = Map::new();
    object.insert("id".to_string(), Value::String(item.id.clone()));
    object.insert("name".to_string(), Value::String(item.name.clone()));
    object.insert("category".to_string(), Value::String(item.category.clone()));
    object.insert(
        "aliases".to_string(),
        Value::Array(item.aliases.iter().cloned().map(Value::String).collect()),
    );
    object.insert("area".to_string(), optional_string(item.area.as_deref()));
    object.insert(
        "area_zh".to_string(),
        optional_string(item.area_zh.as_deref()),
    );
    object.insert(
        "location".to_string(),
        optional_string(item.location.as_deref()),
    );
    object.insert(
        "location_zh".to_string(),
        optional_string(item.location_zh.as_deref()),
    );
    object.insert(
        "obtain".to_string(),
        optional_string(item.obtain.as_deref()),
    );
    object.insert(
        "obtain_zh".to_string(),
        optional_string(item.obtain_zh.as_deref()),
    );
    object.insert("ng_plus".to_string(), Value::from(item.ng_plus));
    object.insert("dlc".to_string(), optional_string(item.dlc.as_deref()));
    object.insert("missable".to_string(), Value::Bool(item.missable));
    object.insert("note".to_string(), optional_string(item.note.as_deref()));
    object.insert(
        "confidence".to_string(),
        Value::String(item.confidence.clone()),
    );
    object.insert(
        "source".to_string(),
        optional_string(item.source.as_deref()),
    );
    object.insert(
        "name_en".to_string(),
        optional_string(item.name_en.as_deref()),
    );
    object.insert(
        "record_type".to_string(),
        optional_string(item.record_type.as_deref()),
    );
    object.insert(
        "record_type_zh".to_string(),
        optional_string(item.record_type_zh.as_deref()),
    );
    object.insert(
        "desc_zh".to_string(),
        optional_string(item.desc_zh.as_deref()),
    );
    object.insert(
        "desc_en".to_string(),
        optional_string(item.desc_en.as_deref()),
    );
    object.insert("order".to_string(), Value::from(item.order));
    Value::Object(object)
}

fn item_to_dict(status: &ItemStatus) -> Value {
    let item = status.item;
    let mut object = Map::new();
    object.insert("id".to_string(), Value::String(item.id.clone()));
    object.insert("name".to_string(), Value::String(item.name.clone()));
    object.insert(
        "name_en".to_string(),
        optional_string(item.name_en.as_deref()),
    );
    object.insert(
        "aliases".to_string(),
        Value::Array(
            item.satisfy_aliases()
                .into_iter()
                .map(|alias| Value::String(alias.to_string()))
                .collect(),
        ),
    );
    object.insert("area".to_string(), optional_string(item.area.as_deref()));
    object.insert(
        "area_zh".to_string(),
        optional_string(item.area_zh.as_deref()),
    );
    object.insert(
        "location".to_string(),
        optional_string(item.location.as_deref()),
    );
    object.insert(
        "location_zh".to_string(),
        optional_string(item.location_zh.as_deref()),
    );
    object.insert(
        "obtain".to_string(),
        optional_string(item.obtain.as_deref()),
    );
    object.insert(
        "obtain_zh".to_string(),
        optional_string(item.obtain_zh.as_deref()),
    );
    object.insert("ng_plus".to_string(), Value::from(item.ng_plus));
    object.insert("dlc".to_string(), optional_string(item.dlc.as_deref()));
    object.insert("missable".to_string(), Value::Bool(item.missable));
    object.insert("note".to_string(), optional_string(item.note.as_deref()));
    object.insert(
        "confidence".to_string(),
        Value::String(item.confidence.clone()),
    );
    object.insert(
        "reason".to_string(),
        match &status.reason {
            Some(reason) => Value::String(reason.clone()),
            None => Value::Null,
        },
    );
    object.insert(
        "flags".to_string(),
        Value::Array(status.flags().into_iter().map(Value::String).collect()),
    );
    object.insert(
        "record_type".to_string(),
        optional_string(item.record_type.as_deref()),
    );
    object.insert(
        "record_type_zh".to_string(),
        optional_string(item.record_type_zh.as_deref()),
    );
    object.insert(
        "desc_zh".to_string(),
        optional_string(item.desc_zh.as_deref()),
    );
    object.insert(
        "desc_en".to_string(),
        optional_string(item.desc_en.as_deref()),
    );
    object.insert("order".to_string(), Value::from(item.order));
    Value::Object(object)
}

fn category_to_dict(result: &CategoryResult, include_obtained: bool) -> Value {
    let mut object = Map::new();
    object.insert(
        "key".to_string(),
        Value::String(result.category.key.clone()),
    );
    object.insert(
        "name".to_string(),
        Value::String(result.category.name.clone()),
    );
    object.insert(
        "section".to_string(),
        Value::String(result.category.section.clone()),
    );
    object.insert("total".to_string(), Value::from(result.total));
    object.insert("obtained".to_string(), Value::from(result.obtained_count));
    object.insert(
        "missing".to_string(),
        Value::Array(result.missing.iter().map(item_to_dict).collect()),
    );
    object.insert(
        "extra_obtained_aliases".to_string(),
        Value::Array(
            result
                .extra_obtained
                .iter()
                .cloned()
                .map(Value::String)
                .collect(),
        ),
    );
    if include_obtained {
        object.insert(
            "obtained_items".to_string(),
            Value::Array(
                result
                    .obtained_items
                    .iter()
                    .map(|item| catalog_item_to_dict(item))
                    .collect(),
            ),
        );
    }
    Value::Object(object)
}

pub fn analysis_to_dict(analysis: &Analysis, include_obtained: bool) -> Value {
    let save = analysis.save;

    let mut save_object = Map::new();
    save_object.insert(
        "path".to_string(),
        Value::String(save.path.to_string_lossy().into_owned()),
    );
    save_object.insert(
        "steam_id".to_string(),
        match &save.steam_id {
            Some(steam_id) => Value::String(steam_id.clone()),
            None => Value::Null,
        },
    );
    save_object.insert("slot".to_string(), Value::from(save.slot));
    save_object.insert(
        "playthrough".to_string(),
        Value::String(save.playthrough_label()),
    );
    save_object.insert(
        "ng_plus_count".to_string(),
        Value::from(save.ng_plus_count()),
    );
    save_object.insert("difficulty".to_string(), Value::from(save.difficulty()));
    save_object.insert(
        "play_time_seconds".to_string(),
        Value::from(save.play_time_seconds()),
    );

    let mut summary = Map::new();
    summary.insert(
        "catalog_total".to_string(),
        Value::from(analysis.catalog_total),
    );
    summary.insert(
        "catalog_obtained".to_string(),
        Value::from(analysis.catalog_obtained),
    );
    summary.insert(
        "missing_total".to_string(),
        Value::from(analysis.missing_total()),
    );
    summary.insert(
        "percent".to_string(),
        Value::from(round2(analysis.percent())),
    );
    summary.insert("album_total".to_string(), Value::from(analysis.album_total));
    summary.insert(
        "album_obtained".to_string(),
        Value::from(analysis.album_obtained),
    );
    summary.insert(
        "album_missing_total".to_string(),
        Value::from(analysis.album_missing_total()),
    );
    summary.insert(
        "album_percent".to_string(),
        Value::from(round2(analysis.album_percent())),
    );
    summary.insert(
        "obtained_aliases".to_string(),
        Value::from(save.all_obtained().len()),
    );
    summary.insert(
        "unmapped_aliases".to_string(),
        Value::from(analysis.unmapped_obtained.len()),
    );

    let mut payload = Map::new();
    payload.insert("save".to_string(), Value::Object(save_object));
    payload.insert("summary".to_string(), Value::Object(summary));
    payload.insert(
        "categories".to_string(),
        Value::Array(
            analysis
                .categories
                .iter()
                .map(|result| category_to_dict(result, include_obtained))
                .collect(),
        ),
    );
    payload.insert(
        "unmapped_obtained".to_string(),
        Value::Array(
            analysis
                .unmapped_obtained
                .iter()
                .cloned()
                .map(Value::String)
                .collect(),
        ),
    );
    Value::Object(payload)
}

pub fn write_json(
    analysis: &Analysis,
    path: impl AsRef<std::path::Path>,
    include_obtained: bool,
) -> Result<(), std::io::Error> {
    let payload = analysis_to_dict(analysis, include_obtained);
    let text = serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "{}".to_string());
    std::fs::write(path, text)
}

pub fn render_markdown(analysis: &Analysis, lang: &str) -> String {
    let save = analysis.save;
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "# 剑星存档分析 — {}",
        save.path.file_name().map_or_else(
            || save.path.display().to_string(),
            |name| name.to_string_lossy().into_owned()
        )
    ));
    lines.push(String::new());
    lines.push(format!(
        "- SteamID: {}\n- 周目: {} (NG+{})\n- 难度: {}\n- 游玩时间: {}\n- 目录进度: {}/{} ({:.1}%)\n- 图鉴进度: {}/{} ({:.1}%)\n- 未收集: {}（图鉴 {}）",
        save.steam_id.as_deref().unwrap_or("未知"),
        save.playthrough_label(),
        save.ng_plus_count(),
        save.difficulty_label(),
        save.play_time_label(),
        analysis.catalog_obtained,
        analysis.catalog_total,
        analysis.percent(),
        analysis.album_obtained,
        analysis.album_total,
        analysis.album_percent(),
        analysis.missing_total(),
        analysis.album_missing_total()
    ));
    lines.push(String::new());
    lines.push("## 分类汇总".to_string());
    lines.push(String::new());
    lines.push("| 分类 | 进度 | 缺失 | 需多周目/DLC |".to_string());
    lines.push("| --- | ---: | ---: | ---: |".to_string());
    for result in &analysis.categories {
        if is_album_category(result) {
            continue;
        }
        lines.push(format!(
            "| {} | {}/{} ({:.0}%) | {} | {} |",
            result.category.name,
            result.obtained_count,
            result.total,
            result.percent(),
            result.missing.len(),
            result.blocked_count
        ));
    }
    lines.push(String::new());
    lines.push("## 图鉴汇总（不计入目录进度）".to_string());
    lines.push(String::new());
    lines.push("| 图鉴 | 进度 | 缺失 |".to_string());
    lines.push("| --- | ---: | ---: |".to_string());
    for result in &analysis.categories {
        if !is_album_category(result) {
            continue;
        }
        lines.push(format!(
            "| {} | {}/{} ({:.0}%) | {} |",
            result.category.name,
            result.obtained_count,
            result.total,
            result.percent(),
            result.missing.len()
        ));
    }
    for result in &analysis.categories {
        if !is_matrix_category(&result.category.key) {
            continue;
        }
        lines.push(String::new());
        lines.extend(item_markdown(result, save, lang));
    }
    lines.push(String::new());
    lines.push("## 未收集清单".to_string());
    for result in &analysis.categories {
        if is_matrix_category(&result.category.key) || result.missing.is_empty() {
            continue;
        }
        lines.push(String::new());
        lines.push(format!(
            "### {} ({}/{})",
            result.category.name, result.obtained_count, result.total
        ));
        lines.push(String::new());
        for status in &result.missing {
            let item = status.item;
            let mut details: Vec<String> = Vec::new();
            let location = location_label(status, lang);
            if !location.is_empty() {
                details.push(location);
            }
            let obtain = localized(item.obtain_zh.as_deref(), item.obtain.as_deref(), lang);
            if !obtain.is_empty() {
                details.push(obtain);
            }
            if let Some(note) = non_empty(item.note.as_deref()) {
                details.push(note.to_string());
            }
            if let Some(reason) = &status.reason {
                details.push(reason.clone());
            }
            let flags = status.flags();
            let suffix = if flags.is_empty() {
                String::new()
            } else {
                format!(" `{}`", flags.join("/"))
            };
            let mut line = format!("- **{}** (`{}`)", display_name(item, lang), item.id);
            if !details.is_empty() {
                line.push_str(" — ");
                line.push_str(&details.join("；"));
            }
            line.push_str(&suffix);
            lines.push(line);
        }
    }
    if !analysis.unmapped_obtained.is_empty() {
        lines.push(String::new());
        lines.push(format!(
            "## 未映射别名 ({})",
            analysis.unmapped_obtained.len()
        ));
        lines.push(String::new());
        lines.push(
            analysis
                .unmapped_obtained
                .iter()
                .map(|alias| format!("`{alias}`"))
                .collect::<Vec<_>>()
                .join("，"),
        );
    }
    lines.push(String::new());
    lines.join("\n")
}

pub fn write_markdown(
    analysis: &Analysis,
    path: impl AsRef<std::path::Path>,
    lang: &str,
) -> Result<(), std::io::Error> {
    std::fs::write(path, render_markdown(analysis, lang))
}
