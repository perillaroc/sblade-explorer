use std::time::UNIX_EPOCH;

use sbsave_core::analyze::analyze;
use sbsave_core::catalog::load_catalog;
use sbsave_core::report::{analysis_to_dict, write_json, write_markdown};
use sbsave_core::savegame::{discover_saves, load_save, pick_default_save, SaveData};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveSlotInfo {
    path: String,
    steam_id: Option<String>,
    slot: u32,
    mtime_ms: u64,
    size: u64,
    label: String,
}

fn slot_info(slot: &sbsave_core::savegame::SaveSlot) -> SaveSlotInfo {
    let size = std::fs::metadata(&slot.path)
        .map(|meta| meta.len())
        .unwrap_or(0);
    let mtime_ms = slot
        .mtime
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0);
    SaveSlotInfo {
        path: slot.path.to_string_lossy().into_owned(),
        steam_id: slot.steam_id.clone(),
        slot: slot.slot,
        mtime_ms,
        size,
        label: slot.label(),
    }
}

#[tauri::command]
fn list_saves() -> Vec<SaveSlotInfo> {
    discover_saves(None).iter().map(slot_info).collect()
}

fn load_for(path: Option<String>, slot: Option<u32>) -> Result<SaveData, String> {
    if let Some(path) = path {
        return load_save(&path, None).map_err(|error| error.to_string());
    }
    let mut slots = discover_saves(None);
    if let Some(slot) = slot {
        slots.retain(|candidate| candidate.slot == slot);
    }
    let chosen = pick_default_save(Some(slots)).map_err(|error| error.to_string())?;
    load_save(&chosen.path, chosen.steam_id).map_err(|error| error.to_string())
}

#[tauri::command]
fn analyze_save(
    path: Option<String>,
    slot: Option<u32>,
    categories: Option<Vec<String>>,
) -> Result<serde_json::Value, String> {
    let save = load_for(path, slot)?;
    let catalog = load_catalog(None).map_err(|error| error.to_string())?;
    let analysis = analyze(&save, &catalog, categories.as_deref());
    Ok(analysis_to_dict(&analysis, true))
}

#[tauri::command]
fn export_report(
    path: Option<String>,
    slot: Option<u32>,
    categories: Option<Vec<String>>,
    lang: String,
    format: String,
    out_path: String,
) -> Result<String, String> {
    let save = load_for(path, slot)?;
    let catalog = load_catalog(None).map_err(|error| error.to_string())?;
    let analysis = analyze(&save, &catalog, categories.as_deref());
    match format.as_str() {
        "json" => write_json(&analysis, &out_path, true),
        "markdown" => write_markdown(&analysis, &out_path, &lang),
        other => return Err(format!("未知导出格式: {other}")),
    }
    .map_err(|error| error.to_string())?;
    Ok(out_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_saves,
            analyze_save,
            export_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_save_path() -> Option<String> {
        let mut slots = discover_saves(None);
        if slots.is_empty() {
            return None;
        }
        Some(slots.remove(0).path.to_string_lossy().into_owned())
    }

    #[test]
    fn list_saves_returns_slot_info() {
        for slot in list_saves() {
            assert!(!slot.path.is_empty());
            assert!(!slot.label.is_empty());
        }
    }

    #[test]
    fn analyze_save_produces_contract() {
        let Some(path) = first_save_path() else {
            return;
        };
        let value = analyze_save(Some(path), None, None).expect("analyze");
        assert_eq!(value["summary"]["catalog_total"].as_u64(), Some(810));
        assert!(value["categories"]
            .as_array()
            .is_some_and(|categories| categories.len() == 13));
    }

    #[test]
    fn export_report_writes_markdown() {
        let Some(path) = first_save_path() else {
            return;
        };
        let target = std::env::temp_dir().join("sbsave-ui-export-test.md");
        let target_text = target.to_string_lossy().into_owned();
        let written = export_report(
            Some(path),
            None,
            None,
            "zh".to_string(),
            "markdown".to_string(),
            target_text,
        )
        .expect("export");
        let text = std::fs::read_to_string(&written).expect("read export");
        assert!(text.contains("# 剑星存档分析"));
        let _ = std::fs::remove_file(&written);
    }
}
