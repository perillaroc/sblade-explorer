use std::time::UNIX_EPOCH;

use sbsave_core::analyze::analyze;
use sbsave_core::catalog::{load_catalog, GuideLink};
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

#[tauri::command]
fn guide_links() -> Result<serde_json::Value, String> {
    let catalog = load_catalog(None).map_err(|error| error.to_string())?;
    let mut links = serde_json::Map::new();
    for item in &catalog.items {
        let Some(guides) = &item.guides else { continue };
        let mut entry = serde_json::Map::new();
        if let Some(web) = &guides.web {
            entry.insert("web".to_string(), guide_link_to_dict(web));
        }
        if let Some(video) = &guides.video {
            entry.insert("video".to_string(), guide_link_to_dict(video));
        }
        links.insert(item.id.clone(), serde_json::Value::Object(entry));
    }
    Ok(serde_json::Value::Object(links))
}

fn guide_link_to_dict(link: &GuideLink) -> serde_json::Value {
    serde_json::json!({
        "title": link.title,
        "url": link.url,
    })
}

/// A browser that can be selected in the settings dialog as a link target.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BrowserInfo {
    id: String,
    name: String,
    path: String,
}

#[cfg(windows)]
mod browsers {
    use std::path::Path;

    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    use super::BrowserInfo;

    const CLIENTS_KEY: &str = r"SOFTWARE\Clients\StartMenuInternet";

    /// Lists the browsers registered as `StartMenuInternet` clients (Edge, Chrome, Firefox, ...).
    /// Registry entries whose executable is missing are skipped so the dialog never offers a
    /// browser that cannot be launched.
    pub fn detect() -> Vec<BrowserInfo> {
        let mut found: Vec<BrowserInfo> = Vec::new();
        for hive in [
            RegKey::predef(HKEY_LOCAL_MACHINE),
            RegKey::predef(HKEY_CURRENT_USER),
        ] {
            let Ok(clients) = hive.open_subkey(CLIENTS_KEY) else {
                continue;
            };
            for key_name in clients.enum_keys().flatten() {
                let Ok(client) = clients.open_subkey(&key_name) else {
                    continue;
                };
                let name: String = client.get_value("").unwrap_or_else(|_| key_name.clone());
                let Ok(command) = client.open_subkey(r"shell\open\command") else {
                    continue;
                };
                let Ok(command_text) = command.get_value::<String, _>("") else {
                    continue;
                };
                let Some(path) = parse_executable(&command_text) else {
                    continue;
                };
                if !Path::new(&path).is_file() {
                    continue;
                }
                if found
                    .iter()
                    .any(|browser| browser.path.eq_ignore_ascii_case(&path))
                {
                    continue;
                }
                found.push(BrowserInfo {
                    id: key_name.to_lowercase(),
                    name,
                    path,
                });
            }
        }
        found.sort_by_key(|browser| browser.name.to_lowercase());
        found
    }

    /// Extracts the executable from a `shell\open\command` value such as
    /// `"C:\Program Files\Google\Chrome\Application\chrome.exe" -- "%1"`.
    fn parse_executable(command: &str) -> Option<String> {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return None;
        }
        if let Some(rest) = trimmed.strip_prefix('"') {
            let end = rest.find('"')?;
            let path = &rest[..end];
            return (!path.is_empty()).then(|| path.to_string());
        }
        trimmed.split_whitespace().next().map(str::to_string)
    }

    #[cfg(test)]
    mod tests {
        use super::{detect, parse_executable};

        #[test]
        fn parse_executable_strips_arguments_and_quotes() {
            assert_eq!(
                parse_executable(
                    r#""C:\Program Files\Google\Chrome\Application\chrome.exe" -- "%1""#
                )
                .as_deref(),
                Some(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            );
            assert_eq!(
                parse_executable(r"C:\browsers\firefox.exe %1").as_deref(),
                Some(r"C:\browsers\firefox.exe")
            );
            assert_eq!(parse_executable("   "), None);
            assert_eq!(parse_executable("\"unterminated"), None);
        }

        #[test]
        fn detect_returns_existing_browser_executables() {
            for browser in detect() {
                assert!(!browser.id.is_empty());
                assert!(!browser.name.is_empty());
                assert!(
                    std::path::Path::new(&browser.path).is_file(),
                    "浏览器可执行文件不存在: {}",
                    browser.path
                );
            }
        }
    }
}

/// Lists the browsers registered on this machine for the settings dialog.
/// Non-Windows platforms have no such registry list; the dialog then only
/// offers the system default browser and a manually picked executable.
#[tauri::command]
fn list_browsers() -> Vec<BrowserInfo> {
    #[cfg(windows)]
    {
        browsers::detect()
    }
    #[cfg(not(windows))]
    {
        Vec::new()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_saves,
            analyze_save,
            export_report,
            guide_links,
            list_browsers
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
        assert_eq!(value["summary"]["album_total"].as_u64(), Some(122));
        assert!(value["categories"]
            .as_array()
            .is_some_and(|categories| categories.len() == 15));
    }

    #[test]
    fn guide_links_returns_chinese_guides() {
        let links = guide_links().expect("guide links");
        let links = links.as_object().expect("object");
        assert_eq!(links.len(), 808);
        let can = links.get("Can_001").expect("can guide");
        assert!(can["web"]["url"]
            .as_str()
            .is_some_and(|url| url.starts_with("https://www.gamersky.com/")));
        assert!(can["video"]["url"]
            .as_str()
            .is_some_and(|url| url.starts_with("https://www.bilibili.com/")));
        let camp = links
            .get("ChangeState_ZoneEnv_AYL_01_EnvS_005_Camp")
            .expect("camp guide");
        assert!(camp["web"]["title"]
            .as_str()
            .is_some_and(|title| !title.is_empty()));
        assert!(camp.get("video").is_none());
    }

    #[test]
    fn guide_urls_are_allowed_by_capability_scope() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let capability: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(manifest_dir.join("capabilities/default.json"))
                .expect("read capability"),
        )
        .expect("parse capability");
        let mut patterns = Vec::new();
        for permission in capability["permissions"].as_array().expect("permissions") {
            if permission["identifier"] == "opener:allow-open-url" {
                for entry in permission["allow"].as_array().expect("allow") {
                    // `app: true` lets the settings dialog open whitelisted links with a
                    // browser chosen by the user instead of the system default.
                    assert_eq!(
                        entry["app"].as_bool(),
                        Some(true),
                        "opener 链接条目必须允许指定浏览器打开: {entry}"
                    );
                    patterns.push(
                        glob::Pattern::new(entry["url"].as_str().expect("scope url"))
                            .expect("scope pattern"),
                    );
                }
            }
        }
        assert!(!patterns.is_empty(), "能力文件缺少 opener 链接白名单");

        let links = guide_links().expect("guide links");
        for (id, guides) in links.as_object().expect("object") {
            for key in ["web", "video"] {
                if let Some(url) = guides.get(key).and_then(|link| link["url"].as_str()) {
                    assert!(
                        patterns.iter().any(|pattern| pattern.matches(url)),
                        "攻略链接不在能力白名单内: {id} {key} {url}"
                    );
                }
            }
        }
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
