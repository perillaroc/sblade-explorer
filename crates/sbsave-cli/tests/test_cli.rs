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
