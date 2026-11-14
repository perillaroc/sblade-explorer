use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

use sbsave_core::fixture::{SyntheticSave, STEAM_ID};

fn sbsave(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sbsave"))
        .args(args)
        .output()
        .expect("run sbsave")
}

/// Runs the CLI against a temporary synthetic save, with `LOCALAPPDATA`
/// redirected so a user catalog override cannot change the counts.
fn sbsave_with_fixture(args: &[&str], save_dirs: &Path, local_app_data: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sbsave"))
        .args(args)
        .env("SBSAVE_SAVE_DIRS", save_dirs)
        .env("LOCALAPPDATA", local_app_data)
        .output()
        .expect("run sbsave")
}

struct FixtureEnv {
    root: PathBuf,
    save_path: PathBuf,
}

impl FixtureEnv {
    fn new(name: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let unique = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "sbsave-cli-fixture-{}-{name}-{unique}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create scratch dir");
        let save_path = SyntheticSave::default()
            .write_to(&root)
            .expect("write fixture");
        Self { root, save_path }
    }
}

impl Drop for FixtureEnv {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn has_local_save() -> bool {
    !sbsave_core::savegame::discover_saves(None).is_empty()
}

#[test]
fn help_lists_report() {
    let output = sbsave(&["--help"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("report"));
}

#[test]
fn version_prints_name() {
    let output = sbsave(&["--version"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("sbsave"));
}

#[test]
fn catalog_list_shows_categories() {
    let output = sbsave(&["catalog", "list"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("nano_suits"));
}

#[test]
fn catalog_check_passes() {
    let output = sbsave(&["catalog", "check"]);
    assert!(output.status.success());
}

#[test]
fn report_with_real_save() {
    if !has_local_save() {
        return;
    }
    let output = sbsave(&["report", "--category", "cans"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("罐子"));
}

#[test]
fn dump_with_real_save() {
    if !has_local_save() {
        return;
    }
    let output = sbsave(&["dump"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("SBSaveGame"));
}

#[test]
fn ui_lang_en_uses_english_help() {
    let output = sbsave(&["--ui-lang", "en", "--help"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("UI language"), "{text}");
    assert!(text.contains("Analyze save completion"), "{text}");
}

#[test]
fn ui_lang_en_catalog_list_uses_english_names() {
    let output = sbsave(&["catalog", "list", "--ui-lang", "en"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("Nano Suits"), "{text}");
    assert!(text.contains("Catalog v1"), "{text}");
}

#[test]
fn ui_lang_en_report_with_real_save() {
    if !has_local_save() {
        return;
    }
    let output = sbsave(&["report", "--ui-lang", "en", "--category", "Cans"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("Cans"), "{text}");
    assert!(text.contains("Category summary"), "{text}");
}

#[test]
fn unknown_ui_lang_is_rejected() {
    let output = sbsave(&["--ui-lang", "fr", "catalog", "list"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("fr"));
}

#[test]
fn saves_with_synthetic_fixture() {
    let fixture = FixtureEnv::new("saves");
    let output = sbsave_with_fixture(&["saves"], &fixture.root, &fixture.root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = stdout(&output);
    assert!(text.contains("StellarBladeSave00"), "{text}");
    assert!(text.contains(STEAM_ID), "{text}");
    assert!(text.contains("StellarBladeSave00.sav"), "{text}");
    assert!(
        text.contains(&fixture.save_path.to_string_lossy().into_owned()),
        "{text}"
    );
}

#[test]
fn report_json_with_synthetic_fixture() {
    let fixture = FixtureEnv::new("report-json");
    let json_path = fixture.root.join("report.json");
    let json_text = json_path.to_string_lossy().into_owned();
    let output = sbsave_with_fixture(
        &["report", "--json", &json_text],
        &fixture.root,
        &fixture.root,
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout(&output).contains("罐子"), "{}", stdout(&output));

    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json_path).expect("read report"))
            .expect("parse report");
    assert_eq!(report["save"]["steam_id"], STEAM_ID);
    assert_eq!(report["save"]["playthrough"], "NG+1");
    assert_eq!(report["save"]["current_map"], "E04");
    assert_eq!(report["save"]["current_map_name"], "Xion");
    assert_eq!(report["save"]["current_map_name_zh"], "希雍");
    assert_eq!(report["save"]["current_epic_quest"], "Quest_Epic_11");
    assert_eq!(report["save"]["current_epic_quest_title"], "Burning Xion");
    assert_eq!(report["save"]["current_epic_quest_title_zh"], "燃烧的希雍");
    assert!(stdout(&output).contains("当前区域: 希雍"));
    assert_eq!(report["summary"]["catalog_total"], 808);
    assert_eq!(report["summary"]["catalog_obtained"], 10);
    assert_eq!(report["summary"]["missing_total"], 798);
    assert_eq!(report["summary"]["unmapped_aliases"], 3);
    assert_eq!(report["summary"]["missed_count"], 154);
    assert_eq!(report["summary"]["missable_open_count"], 29);
    assert_eq!(report["categories"].as_array().map(Vec::len), Some(15));

    let cans = report["categories"]
        .as_array()
        .expect("categories")
        .iter()
        .find(|category| category["key"] == "cans")
        .expect("cans category");
    let can_009 = cans["missing"]
        .as_array()
        .expect("missing list")
        .iter()
        .find(|item| item["id"] == "Can_009")
        .expect("Can_009 should be missing");
    assert_eq!(can_009["reason"], "已错过（本存档无法再获取）");
    assert_eq!(can_009["missable_state"], "missed");
}

#[test]
fn dump_json_summarises_quests_with_synthetic_fixture() {
    let fixture = FixtureEnv::new("dump-json");
    let json_path = fixture.root.join("dump.json");
    let json_text = json_path.to_string_lossy().into_owned();
    let output = sbsave_with_fixture(
        &["dump", "--json", &json_text],
        &fixture.root,
        &fixture.root,
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json_path).expect("read dump"))
            .expect("parse dump");
    assert_eq!(summary["obtained_count"], 10);
    assert_eq!(summary["quests"]["completed"], 2);
    assert_eq!(summary["quests"]["failed"], 1);
    assert_eq!(summary["quests"]["active"], 3);
}

#[test]
fn report_category_filter_uses_synthetic_fixture() {
    let fixture = FixtureEnv::new("report-cans");
    let json_path = fixture.root.join("report-cans.json");
    let json_text = json_path.to_string_lossy().into_owned();
    let output = sbsave_with_fixture(
        &[
            "report",
            "--ui-lang",
            "en",
            "--category",
            "cans",
            "--json",
            &json_text,
        ],
        &fixture.root,
        &fixture.root,
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stdout(&output).contains("Category summary"));

    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json_path).expect("read report"))
            .expect("parse report");
    assert_eq!(report["summary"]["catalog_total"], 49);
    assert_eq!(report["summary"]["catalog_obtained"], 2);
    assert_eq!(report["summary"]["missing_total"], 47);
    assert_eq!(report["summary"]["missed_count"], 8);
    assert_eq!(report["summary"]["missable_open_count"], 0);
    assert_eq!(report["categories"].as_array().map(Vec::len), Some(1));
}
