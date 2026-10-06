use std::process::{Command, Output};

fn sbsave(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sbsave"))
        .args(args)
        .output()
        .expect("run sbsave")
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
