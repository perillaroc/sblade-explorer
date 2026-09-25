use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use sbsave_core::analyze::analyze;
use sbsave_core::catalog::load_catalog;
use sbsave_core::gvas::{EngineVersion, GvasFile, GvasHeader};
use sbsave_core::report::{analysis_to_dict, print_report, render_markdown};
use sbsave_core::savegame::SaveData;

fn make_header() -> GvasHeader {
    GvasHeader {
        save_game_version: 2,
        package_file_version: 522,
        engine: EngineVersion {
            major: 4,
            minor: 26,
            patch: 2,
            changelist: 0,
            branch: "++UE4+Release-4.26".to_string(),
        },
        custom_version_format: 3,
        custom_versions: Vec::new(),
        save_game_class_name: "/Script/SB.SBSaveGame".to_string(),
        has_evas_prefix: false,
    }
}

fn make_save(obtained: &[&str], derived: &[&str], ng_plus: i64) -> SaveData {
    SaveData {
        path: PathBuf::from("StellarBladeSave00.sav"),
        steam_id: Some("76561198000000000".to_string()),
        slot: 0,
        gvas: GvasFile {
            header: make_header(),
            properties: Vec::new(),
            footer: Vec::new(),
        },
        obtained_items: obtained
            .iter()
            .map(|alias| alias.to_string())
            .collect::<HashSet<_>>(),
        derived_aliases: derived.iter().map(|alias| alias.to_string()).collect(),
        counters: HashMap::from([("NewGamePlusPlayCount".to_string(), ng_plus)]),
        string_counters: HashMap::new(),
        achievements: HashMap::new(),
        shop_purchases: HashMap::new(),
        friendships: HashMap::new(),
    }
}

#[test]
fn suit_matrix_markdown() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&["BS_09_2"], &[], 0);
    let filter = vec!["nano_suits".to_string()];
    let text = render_markdown(&analyze(&save, &catalog, Some(filter.as_slice())), "zh");
    assert!(text.contains("## 纳米战衣获取一览"));
    assert!(text.contains("### 埃多斯7号"));
    assert!(text.contains("✅ 星球空降服（第7小队）第2版"));
    assert!(text.contains("❌ 废土冒险家"));
    assert!(text.contains("🔒 星球空降服（第7小队）第3版"));
    assert!(text.contains("🔒 星球空降防护服（第7小队）"));
}

#[test]
fn suit_matrix_unlocks_with_ng_plus() {
    let catalog = load_catalog(None).expect("catalog");
    let filter = vec!["nano_suits".to_string()];
    let save0 = make_save(&[], &[], 0);
    let text_ng0 = render_markdown(&analyze(&save0, &catalog, Some(filter.as_slice())), "zh");
    let save1 = make_save(&[], &[], 1);
    let text_ng1 = render_markdown(&analyze(&save1, &catalog, Some(filter.as_slice())), "zh");
    assert!(text_ng0.contains("🔒 星球空降服（第7小队）第3版"));
    assert!(text_ng1.contains("❌ 星球空降服（第7小队）第3版"));
}

#[test]
fn suit_matrix_dlc_column() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let filter = vec!["nano_suits".to_string()];
    let text = render_markdown(&analyze(&save, &catalog, Some(filter.as_slice())), "zh");
    assert!(text.contains("| 地点 | 首周目 | 二周目(NG+) | 三周目(NG++) | DLC/特典 |"));
    assert!(text.contains("🎁 寄叶二号B型制服"));
}

#[test]
fn print_report_suit_table() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&["BS_09_2"], &[], 0);
    let output = print_report(&analyze(&save, &catalog, None), false, "zh");
    assert!(output.contains("纳米战衣"));
    assert!(output.contains("耳饰"));
    assert!(output.contains("✅"));
    assert!(output.contains("🔒"));
    assert!(output.contains("淹水商业区"));
}

#[test]
fn language_option() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let filter = vec!["nano_suits".to_string()];
    let analysis = analyze(&save, &catalog, Some(filter.as_slice()));
    let zh = render_markdown(&analysis, "zh");
    let en = render_markdown(&analysis, "en");
    let both = render_markdown(&analysis, "both");
    assert!(zh.contains("### 埃多斯7号") && !zh.contains("Flooded Commercial Sector"));
    assert!(en.contains("### Eidos 7") && !en.contains("### 埃多斯7号"));
    assert!(en.contains("Flooded Commercial Sector"));
    assert!(both.contains("埃多斯7号（Eidos 7）"));
    assert!(both.contains("淹水商业区（Flooded Commercial Sector）"));
}

#[test]
fn missing_list_uses_chinese_obtain() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let filter = vec!["cans".to_string()];
    let analysis = analyze(&save, &catalog, Some(filter.as_slice()));
    let zh = render_markdown(&analysis, "zh");
    let en = render_markdown(&analysis, "en");
    let both = render_markdown(&analysis, "both");
    assert!(zh.contains("在施工区东侧"));
    assert!(en.contains("On the east side of the Construction Zone"));
    assert!(!zh.contains("On the east side of the Construction Zone"));
    assert!(both.contains("在施工区东侧"));
    assert!(both.contains("On the east side of the Construction Zone"));
}

#[test]
fn earring_matrix_markdown() {
    let catalog = load_catalog(None).expect("catalog");
    let filter = vec!["earrings".to_string()];
    let save0 = make_save(&[], &[], 0);
    let text_ng0 = render_markdown(&analyze(&save0, &catalog, Some(filter.as_slice())), "zh");
    assert!(text_ng0.contains("## 耳饰获取一览"));
    assert!(text_ng0.contains("### 埃多斯7号"));
    assert!(text_ng0.contains("❌ 绯红泪珠"));
    assert!(text_ng0.contains("🔒 高贵泪珠"));
    assert!(text_ng0.contains("🔒 黄金之心"));
    let save2 = make_save(&[], &[], 2);
    let text_ng2 = render_markdown(&analyze(&save2, &catalog, Some(filter.as_slice())), "zh");
    assert!(text_ng2.contains("❌ 高贵泪珠"));
    assert!(text_ng2.contains("❌ 黄金之心"));
}

#[test]
fn other_matrix_categories() {
    let save = make_save(&[], &[], 1);
    let catalog = load_catalog(None).expect("catalog");
    let cases = [
        ("glasses", "## 眼镜/面饰获取一览", "❌ 超大圆框眼镜"),
        ("drone_seals", "## 无人机外观获取一览", "❌ 铁甲套装"),
        ("adam_costumes", "## 亚当服装获取一览", "❌ 夜鹰"),
        ("lily_costumes", "## 莉莉服装获取一览", "❌ 雨天"),
    ];
    for (key, heading, needle) in cases {
        let filter = vec![key.to_string()];
        let text = render_markdown(&analyze(&save, &catalog, Some(filter.as_slice())), "zh");
        assert!(text.contains(heading), "{key}");
        assert!(text.contains(needle), "{key}");
    }
}

#[test]
fn non_matrix_categories_keep_flat_list() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let filter = vec!["design_patterns".to_string()];
    let text = render_markdown(&analyze(&save, &catalog, Some(filter.as_slice())), "zh");
    assert!(!text.contains("## 设计图案获取一览"));
    assert!(text.contains("### 设计图案 (0/87)"));
}

#[test]
fn album_sections_in_report() {
    let catalog = load_catalog(None).expect("catalog");
    let save = make_save(&[], &[], 0);
    let analysis = analyze(&save, &catalog, None);

    let text = print_report(&analysis, false, "zh");
    assert!(text.contains("图鉴汇总（不计入目录进度）"));
    assert!(text.contains("图鉴进度: 0/122 (0.0%)"));
    assert!(text.contains("孽奇拔 | 0/67 (0%) | 67"));
    assert!(text.contains("角色 | 0/55 (0%) | 55"));

    let markdown = render_markdown(&analysis, "zh");
    assert!(markdown.contains("## 图鉴汇总（不计入目录进度）"));
    assert!(markdown.contains("- 图鉴进度: 0/122 (0.0%)"));
    assert!(markdown.contains("| 孽奇拔 | 0/67 (0%) | 67 |"));

    let payload = analysis_to_dict(&analysis, false);
    assert_eq!(payload["summary"]["catalog_total"], 810);
    assert_eq!(payload["summary"]["album_total"], 122);
    assert_eq!(payload["summary"]["album_obtained"], 0);
    assert_eq!(payload["summary"]["album_missing_total"], 122);
    let categories = payload["categories"].as_array().expect("categories");
    let naytiba = categories
        .iter()
        .find(|category| category["key"] == "naytiba")
        .expect("naytiba");
    assert_eq!(naytiba["section"], "album");
    let thorn = naytiba["missing"]
        .as_array()
        .expect("missing")
        .iter()
        .find(|item| item["id"] == "Album_ThornHead")
        .expect("thorn");
    assert!(thorn["desc_zh"]
        .as_str()
        .is_some_and(|text| text.contains("生态情报")));
    assert!(thorn["desc_en"]
        .as_str()
        .is_some_and(|text| text.contains("Ecological Information")));
}
