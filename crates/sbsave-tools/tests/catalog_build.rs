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
    assert_eq!(output.item_count, 932);
    assert_eq!(output.alias_count, 942);
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
    let mut without_guides: Vec<&str> = Vec::new();
    let mut album_items = 0;
    for item in items {
        let category = item["category"].as_str().unwrap_or_default();
        if category == "naytiba" || category == "characters" {
            album_items += 1;
            assert!(
                item.get("desc_zh").is_some(),
                "图鉴条目缺少说明文字: {}",
                item["id"]
            );
            assert!(
                item.get("guides").is_none(),
                "图鉴条目不应有攻略链接: {}",
                item["id"]
            );
            continue;
        }
        let Some(guides) = item.get("guides") else {
            without_guides.push(item["id"].as_str().expect("item id"));
            continue;
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
    assert_eq!(with_web, 804);
    assert_eq!(with_video, 696);
    // 图鉴条目（孽奇拔 67 + 角色 55）没有攻略链接、但有官方说明
    assert_eq!(album_items, 122);
    // 仅默认外观类条目没有可链接的攻略（发型：默认马尾与首领挑战奖励）
    assert_eq!(without_guides, ["Hair_000", "Hair_Nikke_01"]);
}
