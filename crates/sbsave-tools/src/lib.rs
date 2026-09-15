//! Build-time data pipeline: catalog generation and game name mining.
//!
//! Replaces the former Python tools (`build_catalog.py`, `crosswalk_data.py`,
//! `mine_game_names.py`); the hand curated crosswalk now lives in
//! `data/raw/crosswalk.json`.

pub mod catalog_build;
pub mod jsonio;
pub mod locres;
pub mod mine_names;

use std::path::{Path, PathBuf};

/// Repository root, resolved from the crate location at compile time.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate 目录结构异常")
        .to_path_buf()
}
