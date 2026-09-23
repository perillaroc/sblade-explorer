use sbsave_tools::catalog_build::build_catalog_bytes;
use sbsave_tools::repo_root;

#[test]
fn catalog_build_matches_committed_file() {
    let root = repo_root();
    let output = build_catalog_bytes(&root).expect("build catalog");
    let committed = std::fs::read(root.join("data/catalog.json")).expect("read committed catalog");
    assert_eq!(
        output.bytes, committed,
        "data/catalog.json 与生成结果不一致，请运行 cargo run -p sbsave-tools -- catalog build"
    );
    assert_eq!(output.item_count, 810);
    assert_eq!(output.alias_count, 820);
}

#[test]
fn catalog_build_attaches_guide_links() {
    let root = repo_root();
    let output = build_catalog_bytes(&root).expect("build catalog");
    let payload: serde_json::Value =
        serde_json::from_slice(&output.bytes).expect("catalog payload");
    let items = payload["items"].as_array().expect("items");
    let mut with_web = 0;
    let mut with_video = 0;
    for item in items {
        let Some(guides) = item.get("guides") else {
            panic!("条目缺少攻略链接: {}", item["id"]);
        };
        for key in ["web", "video"] {
            if let Some(link) = guides.get(key) {
                let url = link["url"].as_str().expect("guide url");
                assert!(url.starts_with("https://"), "攻略链接必须使用 https: {url}");
                let title = link["title"].as_str().expect("guide title");
                assert!(!title.is_empty(), "攻略链接缺少标题: {url}");
            }
        }
        if guides.get("web").is_some() {
            with_web += 1;
        }
        if guides.get("video").is_some() {
            with_video += 1;
        }
    }
    // Regression sentinels: keep the Chinese guide coverage from shrinking.
    assert_eq!(with_web, 794);
    assert_eq!(with_video, 634);
}
