//! Integration tests for `save_dirs` / `SBSAVE_SAVE_DIRS`.
//!
//! These tests live in their own test binary because they mutate the process
//! environment; keeping them apart avoids racing with other tests that call
//! `discover_saves`.

use std::path::PathBuf;

use sbsave_core::savegame::{discover_saves, save_dirs};

struct EnvGuard;

impl EnvGuard {
    fn set(dirs: &[&str]) -> Self {
        let joined = std::env::join_paths(dirs).expect("join dirs");
        std::env::set_var("SBSAVE_SAVE_DIRS", joined);
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var("SBSAVE_SAVE_DIRS");
    }
}

fn scratch_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("sbsave-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

#[test]
fn save_dirs_override_replaces_defaults_and_limits_discovery() {
    let empty = scratch_dir("empty");
    let with_save = scratch_dir("with-save");
    std::fs::write(with_save.join("StellarBladeSave00.sav"), b"").expect("write dummy save");

    let empty_text = empty.to_string_lossy().into_owned();
    let with_save_text = with_save.to_string_lossy().into_owned();

    let first = EnvGuard::set(&[empty_text.as_str()]);
    assert_eq!(save_dirs(), vec![empty.clone()]);
    assert!(discover_saves(None).is_empty());
    drop(first);

    let second = EnvGuard::set(&[with_save_text.as_str()]);
    assert_eq!(save_dirs(), vec![with_save.clone()]);
    let slots = discover_saves(None);
    assert_eq!(slots.len(), 1);
    assert_eq!(slots[0].slot, 0);
    assert_eq!(slots[0].path, with_save.join("StellarBladeSave00.sav"));
    assert_eq!(slots[0].label(), "StellarBladeSave00 (unknown)");
    assert_eq!(slots[0].steam_id, None);
    drop(second);

    // Without the override the default locations are used again.
    assert!(!save_dirs().is_empty());
    assert_ne!(save_dirs(), vec![with_save.clone()]);

    let _ = std::fs::remove_dir_all(&empty);
    let _ = std::fs::remove_dir_all(&with_save);
}
