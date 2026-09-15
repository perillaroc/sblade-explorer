//! Game name miner: port of the former Python `tools/mine_game_names.py`.
//!
//! Reads exported game tables and `Game.{zh-Hans,en}.locres` to build
//! `data/raw/game/name_map.json`.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::jsonio;
use crate::locres;

const GENERATED_BY: &str = "crates/sbsave-tools (mine-names)";
const SOURCE: &str =
    "Stellar Blade game tables (ItemTable, ZoneCampTable) + zh-Hans/en Game.locres";
const LOCRES_CULTURES: [(&str, &str); 2] = [("zh", "zh-Hans"), ("en", "en")];
const LOCRES_PATH: &str = "SB/Content/Localization/Game/{culture}/Game.locres";
const TABLE_PACKAGES: [&str; 2] = ["*/ItemTable.uasset", "*/ZoneCampTable.uasset"];

pub struct Options {
    pub dump: Option<PathBuf>,
    pub game: Option<PathBuf>,
    pub tools: Option<PathBuf>,
    pub mappings: Option<PathBuf>,
    pub out: Option<PathBuf>,
}

#[derive(Serialize, Clone)]
pub struct NameEntry {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize)]
struct NameMapPayload {
    version: i64,
    generated_by: &'static str,
    source: &'static str,
    items: BTreeMap<String, NameEntry>,
    camps: BTreeMap<String, NameEntry>,
}

pub fn run(options: &Options) -> Result<(), String> {
    let root = crate::repo_root();
    let dump = options
        .dump
        .clone()
        .unwrap_or_else(|| PathBuf::from("data/raw/game/dump"));
    if let Some(game) = &options.game {
        let tools = options.tools.clone().unwrap_or_else(|| dump.clone());
        let mappings = options
            .mappings
            .clone()
            .unwrap_or_else(|| tools.join("Mappings.usmap"));
        eprintln!(
            "extracting game tables from {} with {} ...",
            game.display(),
            tools.display()
        );
        export_with_cue4parse(game, &tools, &dump, &mappings)?;
        unpack_locres(game, &tools, &dump)?;
    }

    let mut localisations: HashMap<&str, locres::Namespaces> = HashMap::new();
    for (language, culture) in LOCRES_CULTURES {
        localisations.insert(
            language,
            locres::read_locres(&find_locres(&dump, culture)?)?,
        );
    }
    let zh = locres::flatten(&localisations["zh"]);
    let en = locres::flatten(&localisations["en"]);

    let item_table = find_first(&dump, "ItemTable.json");
    let camp_table = find_first(&dump, "ZoneCampTable.json");
    let (Some(item_table), Some(camp_table)) = (item_table, camp_table) else {
        return Err("ItemTable.json or ZoneCampTable.json not found in dump".to_string());
    };

    let universe_path = root.join("data/raw/universe/aliases.json");
    let universe: Value = serde_json::from_str(
        &std::fs::read_to_string(&universe_path)
            .map_err(|error| format!("读取 {} 失败: {error}", universe_path.display()))?,
    )
    .map_err(|error| format!("解析 {} 失败: {error}", universe_path.display()))?;

    let items = build_items(&load_rows(&item_table)?, &zh, &en, &universe);
    let camps = build_camps(&load_rows(&camp_table)?, &zh, &en);

    let payload = NameMapPayload {
        version: 1,
        generated_by: GENERATED_BY,
        source: SOURCE,
        items,
        camps,
    };
    let bytes =
        jsonio::to_bytes(&payload).map_err(|error| format!("序列化名称映射失败: {error}"))?;

    let out = options
        .out
        .clone()
        .unwrap_or_else(|| root.join("data/raw/game/name_map.json"));
    if let Some(parent) = out.parent().filter(|parent| !parent.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("创建 {} 失败: {error}", parent.display()))?;
    }
    std::fs::write(&out, &bytes)
        .map_err(|error| format!("写入 {} 失败: {error}", out.display()))?;
    let shown = out.strip_prefix(&root).unwrap_or(&out);
    eprintln!(
        "wrote {}: {} item names, {} camps",
        shown.display(),
        payload.items.len(),
        payload.camps.len()
    );
    Ok(())
}

fn export_with_cue4parse(
    game: &Path,
    tools: &Path,
    dump: &Path,
    mappings: &Path,
) -> Result<(), String> {
    let cue4parse = tools.join("cue4parse.exe");
    if !cue4parse.exists() {
        return Err(format!("cue4parse.exe not found in {}", tools.display()));
    }
    let mut command = Command::new(&cue4parse);
    command
        .arg("-i")
        .arg(game)
        .arg("-g")
        .arg("GAME_StellarBlade")
        .arg("-f")
        .arg("json")
        .arg("-o")
        .arg(dump)
        .arg("-y")
        .arg("-m")
        .arg(mappings);
    for pattern in TABLE_PACKAGES {
        command.arg("-p").arg(pattern);
    }
    let status = command
        .status()
        .map_err(|error| format!("运行 cue4parse 失败: {error}"))?;
    if !status.success() {
        return Err(format!("cue4parse 退出码 {status}"));
    }
    Ok(())
}

fn unpack_locres(game: &Path, tools: &Path, dump: &Path) -> Result<(), String> {
    let repak = tools.join("repak.exe");
    if !repak.exists() {
        return Err(format!("repak.exe not found in {}", tools.display()));
    }
    let pak = game
        .join("SB")
        .join("Content")
        .join("Paks")
        .join("pakchunk0-WindowsNoEditor.pak");
    let mut command = Command::new(&repak);
    command
        .arg("unpack")
        .arg(&pak)
        .arg("-o")
        .arg(dump)
        .arg("-q");
    for (_, culture) in LOCRES_CULTURES {
        command
            .arg("-i")
            .arg(LOCRES_PATH.replace("{culture}", culture));
    }
    let status = command
        .status()
        .map_err(|error| format!("运行 repak 失败: {error}"))?;
    if !status.success() {
        return Err(format!("repak 退出码 {status}"));
    }
    Ok(())
}

fn find_locres(dump: &Path, culture: &str) -> Result<PathBuf, String> {
    let direct = dump.join(format!("Game.{culture}.locres"));
    if direct.exists() {
        return Ok(direct);
    }
    let mut matches: Vec<PathBuf> = Vec::new();
    for entry in walkdir::WalkDir::new(dump)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() || entry.file_name() != "Game.locres" {
            continue;
        }
        let parent_matches = entry
            .path()
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains(culture));
        if parent_matches {
            matches.push(entry.into_path());
        }
    }
    matches.sort();
    matches.into_iter().next().ok_or_else(|| {
        format!(
            "Game.locres for {culture} not found under {}",
            dump.display()
        )
    })
}

fn find_first(root: &Path, file_name: &str) -> Option<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .find(|entry| entry.file_type().is_file() && entry.file_name() == file_name)
        .map(|entry| entry.into_path())
}

fn load_rows(path: &Path) -> Result<Map<String, Value>, String> {
    let bytes =
        std::fs::read(path).map_err(|error| format!("读取 {} 失败: {error}", path.display()))?;
    let bytes = bytes
        .strip_prefix(&[0xEF, 0xBB, 0xBF][..])
        .unwrap_or(&bytes);
    let payload: Value = serde_json::from_slice(bytes)
        .map_err(|error| format!("解析 {} 失败: {error}", path.display()))?;
    if let Value::Array(exports) = &payload {
        for export in exports {
            if export.get("Type").and_then(Value::as_str) != Some("DataTable") {
                continue;
            }
            if let Some(object) = export.get("Rows").and_then(Value::as_object) {
                if !object.is_empty() {
                    return Ok(object.clone());
                }
            }
        }
    }
    Err(format!(
        "{} is not a CUE4Parse DataTable export",
        path.display()
    ))
}

fn build_items(
    item_rows: &Map<String, Value>,
    zh: &HashMap<String, String>,
    en: &HashMap<String, String>,
    universe: &Value,
) -> BTreeMap<String, NameEntry> {
    let mut names: BTreeMap<String, NameEntry> = BTreeMap::new();
    for (alias, row) in item_rows {
        let key = row
            .get("Name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if key.is_empty() {
            continue;
        }
        let lower = key.to_lowercase();
        let Some(text) = zh.get(&lower) else {
            continue;
        };
        names.insert(
            alias.clone(),
            NameEntry {
                zh: text.clone(),
                en: en.get(&lower).cloned().unwrap_or_default(),
            },
        );
    }
    let suffix_re = Regex::new(r"_(Maintain|Used|Popup)$").expect("suffix regex");
    if let Some(records) = universe.get("records").and_then(Value::as_array) {
        for raw in records {
            let Some(raw) = raw.as_str() else {
                continue;
            };
            let alias = suffix_re.replace(raw, "").into_owned();
            let title_key = format!("{}_Title", alias.replace("Item_", ""));
            let lower = title_key.to_lowercase();
            let Some(text) = zh.get(&lower) else {
                continue;
            };
            names.insert(
                alias,
                NameEntry {
                    zh: text.clone(),
                    en: en.get(&lower).cloned().unwrap_or_default(),
                },
            );
        }
    }
    names
}

fn build_camps(
    camp_rows: &Map<String, Value>,
    zh: &HashMap<String, String>,
    en: &HashMap<String, String>,
) -> BTreeMap<String, NameEntry> {
    let env_re = Regex::new(r"^(.+?)_(\d+)_EnvS_(\d+)$").expect("env regex");
    let mut camps: BTreeMap<String, NameEntry> = BTreeMap::new();
    for row in camp_rows.values() {
        let Some(row) = row.as_object() else {
            continue;
        };
        let name_key = row
            .get("CampName")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim()
            .to_string();
        if name_key.is_empty() {
            continue;
        }
        let lower = name_key.to_lowercase();
        let Some(text) = zh.get(&lower) else {
            continue;
        };
        let entry = NameEntry {
            zh: text.clone(),
            en: en.get(&lower).cloned().unwrap_or_default(),
        };
        for value in row.values() {
            let Some(value) = value.as_str() else {
                continue;
            };
            if !value.contains("EnvS") {
                continue;
            }
            let stripped = value.replace("ChangeState_ZoneEnv_", "");
            let stripped = stripped.strip_suffix("_Camp").unwrap_or(&stripped);
            let Some(captures) = env_re.captures(stripped) else {
                continue;
            };
            let alias = format!(
                "ChangeState_ZoneEnv_{}_{}_EnvS_{}_Camp",
                &captures[1], &captures[2], &captures[3]
            );
            camps.entry(alias).or_insert_with(|| entry.clone());
        }
    }
    camps
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::{build_camps, build_items, load_rows};

    #[test]
    fn build_items_reads_names_and_record_titles() {
        let rows = json!({ "Can_001": { "Name": "CanName" } });
        let zh = HashMap::from([
            ("canname".to_string(), "罐子一".to_string()),
            (
                "records_ded10_memory_09_title".to_string(),
                "记录标题".to_string(),
            ),
        ]);
        let en = HashMap::from([("canname".to_string(), "Can One".to_string())]);
        let universe = json!({ "records": ["Item_Records_DED10_Memory_09_Used"] });
        let items = build_items(rows.as_object().unwrap(), &zh, &en, &universe);
        assert_eq!(items["Can_001"].zh, "罐子一");
        assert_eq!(items["Can_001"].en, "Can One");
        assert_eq!(items["Item_Records_DED10_Memory_09"].zh, "记录标题");
    }

    #[test]
    fn build_camps_maps_env_aliases() {
        let rows = json!({
            "Camp_A": {
                "CampName": "CampNameKey",
                "Zone": "ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"
            }
        });
        let zh = HashMap::from([("campnamekey".to_string(), "隐秘之路".to_string())]);
        let en = HashMap::new();
        let camps = build_camps(rows.as_object().unwrap(), &zh, &en);
        assert_eq!(
            camps["ChangeState_ZoneEnv_WLA_10_EnvS_001_Camp"].zh,
            "隐秘之路"
        );
    }

    #[test]
    fn load_rows_accepts_bom_and_finds_table() {
        let payload = br#"[{"Type":"DataTable","Rows":{"A":{"Name":"X"}}}]"#;
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(payload);
        let path = std::env::temp_dir().join("sbsave-tools-rows-test.json");
        std::fs::write(&path, &bytes).expect("write");
        let rows = load_rows(&path).expect("parse");
        assert_eq!(rows["A"]["Name"], "X");
        let _ = std::fs::remove_file(&path);
    }
}
