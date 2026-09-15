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
