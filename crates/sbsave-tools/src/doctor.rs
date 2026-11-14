//! Build-time data health report (`sbsave-tools doctor`).
//!
//! Runs offline against `data/raw` + the generated catalog. It never touches a
//! save file and is not part of the runtime app.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use crate::catalog_build;

/// Site index files that are intentionally not part of the catalog.
const EXCLUDED_SITE_FILES: [&str; 7] = [
    "materials__supply-boxes.json",
    "materials__supply-chests.json",
    "upgrades__body-cores.json",
    "upgrades__beta-cores.json",
    "upgrades__weapon-cores.json",
    "upgrades__tumbler.json",
    "upgrades__drone-upgrades.json",
];

pub struct Options {
    /// Directory for `doctor.md` / `doctor.json` (default `target`).
    pub out: Option<PathBuf>,
    /// Exit non-zero when a check reports new drift.
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteEntry {
    pub id: i64,
    pub file: String,
    pub title: String,
}

#[derive(Debug, Default, Serialize)]
pub struct UniverseGap {
    pub segment: String,
    pub aliases: usize,
    pub missing: usize,
    pub sample: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct Quality {
    pub low_confidence: Vec<String>,
    pub duplicate_aliases: Vec<String>,
    pub empty_obtain: Vec<String>,
    pub unmapped_universe: Vec<UniverseGap>,
}

#[derive(Debug, Default, Serialize)]
pub struct Rules {
    pub missable_total: usize,
    pub missable_without_rule: usize,
    pub quest_aliases: usize,
    pub map_names: usize,
    pub quest_titles: usize,
    pub missing_zh_regions: Vec<String>,
    pub missing_zh_quests: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct GuideCoverage {
    pub web: usize,
    pub video: usize,
    pub without_links: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub generated_by: String,
    pub build_error: Option<String>,
    pub site_entries: usize,
    pub catalog_site_entries: usize,
    pub missing_from_catalog: Vec<SiteEntry>,
    pub excluded_site_files: BTreeMap<String, usize>,
    pub stale_catalog_site_ids: Vec<i64>,
    pub stale_allowlist_ids: Vec<i64>,
    pub guide_problems: Vec<String>,
    pub guides: GuideCoverage,
    pub quality: Quality,
    pub rules: Rules,
    pub strict_failures: Vec<String>,
}

impl Report {
    pub fn ok(&self) -> bool {
        self.strict_failures.is_empty()
    }
}

pub fn analyze(root: &Path) -> Result<Report, String> {
    let site = catalog_build::load_site_index(&root.join("data/raw/api"))?;
    let allowlist = load_allowlist(root)?;
    let mut report = Report {
        generated_by: "crates/sbsave-tools doctor".to_string(),
        site_entries: site.len(),
        ..Report::default()
    };

    let catalog_bytes = match catalog_build::build_catalog_bytes(root) {
        Ok(output) => Some(output.bytes),
        Err(error) => {
            report.build_error = Some(error.clone());
            report.guide_problems.push(error);
            None
        }
    };

    let mut used_ids: BTreeSet<i64> = BTreeSet::new();
    if let Some(bytes) = &catalog_bytes {
        let payload: Value = serde_json::from_slice(bytes)
            .map_err(|error| format!("解析生成的目录库失败: {error}"))?;
        let categories = payload
            .get("categories")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let section_by_key: HashMap<String, String> = categories
            .iter()
            .filter_map(|category| {
                Some((
                    category.get("key")?.as_str()?.to_string(),
                    category.get("section")?.as_str()?.to_string(),
                ))
            })
            .collect();

        let items = payload
            .get("items")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut alias_owner: HashMap<String, String> = HashMap::new();
        let mut universe_used: HashSet<String> = HashSet::new();
        for item in &items {
            let id = item.get("id").and_then(Value::as_str).unwrap_or_default();
            let category = item
                .get("category")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let section = section_by_key
                .get(category)
                .map(String::as_str)
                .unwrap_or("collection");
            if let Some(site_id) = item.get("site_id").and_then(Value::as_i64) {
                used_ids.insert(site_id);
            }
            let aliases: Vec<String> = item
                .get("aliases")
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default();
            for alias in &aliases {
                universe_used.insert(alias.clone());
                if let Some(previous) = alias_owner.insert(alias.clone(), id.to_string()) {
                    if previous != id {
                        report
                            .quality
                            .duplicate_aliases
                            .push(format!("{alias}: {previous} vs {id}"));
                    }
                }
            }
            if item.get("confidence").and_then(Value::as_str) != Some("high") {
                report.quality.low_confidence.push(id.to_string());
            }
            if section != "album" && item.get("obtain").is_none() {
                report.quality.empty_obtain.push(id.to_string());
            }
            if item.get("missable").and_then(Value::as_bool) == Some(true) {
                report.rules.missable_total += 1;
                if item.get("missable_rule").is_none() {
                    report.rules.missable_without_rule += 1;
                }
            }
            let guides = item.get("guides");
            if guides.and_then(|guides| guides.get("web")).is_some() {
                report.guides.web += 1;
            }
            if guides.and_then(|guides| guides.get("video")).is_some() {
                report.guides.video += 1;
            }
            if guides.is_none() && section != "album" {
                report.guides.without_links.push(id.to_string());
            }
        }
        report.catalog_site_entries = used_ids.len();
        report.quality.duplicate_aliases.sort();
        report.quality.empty_obtain.sort();
        report.guides.without_links.sort();
        report.quality.unmapped_universe = unmapped_universe(root, &universe_used)?;
        report.rules.quest_aliases = load_array_len(root, "data/raw/quest_aliases.json", "aliases");
        let game_names = read_json(root.join("data/raw/game/name_map.json"))?;
        report.rules.map_names = localized_len(&game_names, "regions");
        report.rules.quest_titles = localized_len(&game_names, "quests");
        report.rules.missing_zh_regions = missing_zh(&game_names, "regions");
        report.rules.missing_zh_quests = missing_zh(&game_names, "quests");
    }

    let mut allowlist_ids: BTreeSet<i64> = BTreeSet::new();
    for id in allowlist {
        allowlist_ids.insert(id);
        if !site.contains_key(&id) {
            report.stale_allowlist_ids.push(id);
        }
    }
    for (id, info) in &site {
        if used_ids.contains(id) {
            continue;
        }
        if EXCLUDED_SITE_FILES.contains(&info.source_file.as_str()) {
            *report
                .excluded_site_files
                .entry(info.source_file.clone())
                .or_default() += 1;
            continue;
        }
        if allowlist_ids.contains(id) {
            continue;
        }
        report.missing_from_catalog.push(SiteEntry {
            id: *id,
            file: info.source_file.clone(),
            title: info.title.clone(),
        });
    }
    for id in &used_ids {
        if !site.contains_key(id) {
            report.stale_catalog_site_ids.push(*id);
        }
    }
    report.missing_from_catalog.sort_by_key(|entry| entry.id);
    report.stale_catalog_site_ids.sort_unstable();
    report.stale_allowlist_ids.sort_unstable();

    if report.build_error.is_some() {
        report
            .strict_failures
            .push("目录库构建失败（见 guide_problems）".to_string());
    }
    if !report.missing_from_catalog.is_empty() {
        report.strict_failures.push(format!(
            "有 {} 个站点条目尚未入库",
            report.missing_from_catalog.len()
        ));
    }
    if !report.stale_catalog_site_ids.is_empty() {
        report.strict_failures.push(format!(
            "目录库引用了 {} 个不存在的站点 id",
            report.stale_catalog_site_ids.len()
        ));
    }
    if !report.quality.duplicate_aliases.is_empty() {
        report.strict_failures.push(format!(
            "存在 {} 个重复别名",
            report.quality.duplicate_aliases.len()
        ));
    }
    if !report.stale_allowlist_ids.is_empty() {
        report.strict_failures.push(format!(
            "doctor 白名单引用了 {} 个不存在的站点 id",
            report.stale_allowlist_ids.len()
        ));
    }
    Ok(report)
}

fn load_allowlist(root: &Path) -> Result<Vec<i64>, String> {
    let path = root.join("data/raw/doctor_allowlist.json");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let payload = read_json(path)?;
    Ok(payload
        .get("unused_site_ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default())
}

fn unmapped_universe(root: &Path, used: &HashSet<String>) -> Result<Vec<UniverseGap>, String> {
    let universe = read_json(root.join("data/raw/universe/aliases.json"))?;
    let Some(object) = universe.as_object() else {
        return Ok(Vec::new());
    };
    let mut gaps = Vec::new();
    for (segment, value) in object {
        if segment == "source" || segment == "version" {
            continue;
        }
        let Some(aliases) = value.as_array() else {
            continue;
        };
        let aliases: Vec<&str> = aliases.iter().filter_map(Value::as_str).collect();
        let missing: Vec<String> = aliases
            .iter()
            .filter(|alias| !used.contains(**alias))
            .map(|alias| (*alias).to_string())
            .collect();
        if missing.is_empty() {
            continue;
        }
        gaps.push(UniverseGap {
            segment: segment.clone(),
            aliases: aliases.len(),
            missing: missing.len(),
            sample: missing.iter().take(20).cloned().collect(),
        });
    }
    gaps.sort_by(|left, right| left.segment.cmp(&right.segment));
    Ok(gaps)
}

fn read_json(path: PathBuf) -> Result<Value, String> {
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("读取 {} 失败: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("解析 {} 失败: {error}", path.display()))
}

fn load_array_len(root: &Path, relative: &str, key: &str) -> usize {
    let Ok(payload) = read_json(root.join(relative)) else {
        return 0;
    };
    payload
        .get(key)
        .and_then(Value::as_array)
        .map_or(0, Vec::len)
}

fn localized_len(game_names: &Value, segment: &str) -> usize {
    game_names
        .get(segment)
        .and_then(Value::as_object)
        .map_or(0, serde_json::Map::len)
}

fn missing_zh(game_names: &Value, segment: &str) -> Vec<String> {
    game_names
        .get(segment)
        .and_then(Value::as_object)
        .map(|object| {
            object
                .iter()
                .filter(|(_, entry)| {
                    entry
                        .get("name_zh")
                        .or_else(|| entry.get("zh"))
                        .and_then(Value::as_str)
                        .is_none_or(|text| text.trim().is_empty())
                })
                .map(|(key, _)| key.clone())
                .collect()
        })
        .unwrap_or_default()
}

pub fn render_markdown(report: &Report) -> String {
    let mut out = String::new();
    out.push_str("# 数据维护检查报告\n\n");
    out.push_str(&format!("- 生成器：`{}`\n", report.generated_by));
    out.push_str(&format!(
        "- 站点条目：{}（目录库引用 {} 个）\n",
        report.site_entries, report.catalog_site_entries
    ));
    if let Some(error) = &report.build_error {
        out.push_str(&format!("\n> 构建失败：{error}\n"));
    }
    out.push_str("\n## 站点与目录库差集\n\n");
    if report.missing_from_catalog.is_empty() {
        out.push_str("未入库的站点条目：无\n");
    } else {
        out.push_str(&format!(
            "未入库的站点条目：{} 个\n\n| id | 文件 | 标题 |\n| ---: | --- | --- |\n",
            report.missing_from_catalog.len()
        ));
        for entry in &report.missing_from_catalog {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                entry.id,
                entry.file,
                entry.title.replace('|', "\\|")
            ));
        }
    }
    if !report.excluded_site_files.is_empty() {
        out.push_str("\n按设计排除的文件：\n\n| 文件 | 条目 |\n| --- | ---: |\n");
        for (file, count) in &report.excluded_site_files {
            out.push_str(&format!("| {file} | {count} |\n"));
        }
    }
    if report.stale_catalog_site_ids.is_empty() {
        out.push_str("\n目录库引用的失效站点 id：无\n");
    } else {
        out.push_str(&format!(
            "\n目录库引用的失效站点 id：{:?}\n",
            report.stale_catalog_site_ids
        ));
    }
    if !report.stale_allowlist_ids.is_empty() {
        out.push_str(&format!(
            "\ndoctor 白名单中的失效站点 id：{:?}\n",
            report.stale_allowlist_ids
        ));
    }

    out.push_str("\n## 攻略链接\n\n");
    out.push_str(&format!(
        "- 图文链接：{} 条；视频链接：{} 条\n",
        report.guides.web, report.guides.video
    ));
    if report.guide_problems.is_empty() {
        out.push_str("- 链接校验：通过（HTTPS、域名白名单、键匹配由构建期校验）\n");
    } else {
        out.push_str(&format!(
            "- 链接校验问题：{}\n",
            report.guide_problems.join("；")
        ));
    }
    if !report.guides.without_links.is_empty() {
        out.push_str(&format!(
            "- 无内置链接的条目：{}\n",
            report.guides.without_links.join(", ")
        ));
    }

    out.push_str("\n## 质量清单\n\n");
    out.push_str(&format!(
        "- 低置信度条目：{} 个{}\n",
        report.quality.low_confidence.len(),
        list_suffix(&report.quality.low_confidence)
    ));
    out.push_str(&format!(
        "- 重复别名：{} 个{}\n",
        report.quality.duplicate_aliases.len(),
        list_suffix(&report.quality.duplicate_aliases)
    ));
    out.push_str(&format!(
        "- 缺少获取文案：{} 个{}\n",
        report.quality.empty_obtain.len(),
        list_suffix(&report.quality.empty_obtain)
    ));
    if report.quality.unmapped_universe.is_empty() {
        out.push_str("- 别名词库未覆盖：无\n");
    } else {
        out.push_str(
            "- 别名词库未覆盖：\n\n| 段 | 别名 | 未覆盖 | 示例 |\n| --- | ---: | ---: | --- |\n",
        );
        for gap in &report.quality.unmapped_universe {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                gap.segment,
                gap.aliases,
                gap.missing,
                gap.sample.join(", ")
            ));
        }
    }

    out.push_str("\n## 规则数据\n\n");
    out.push_str(&format!(
        "- 易错过条目：{} 条（其中无任务规则 {} 条）\n",
        report.rules.missable_total, report.rules.missable_without_rule
    ));
    out.push_str(&format!(
        "- 任务别名：{} 条；区域代号：{} 条；任务标题：{} 条\n",
        report.rules.quest_aliases, report.rules.map_names, report.rules.quest_titles
    ));
    if !report.rules.missing_zh_regions.is_empty() {
        out.push_str(&format!(
            "- 缺少中文名的区域：{}\n",
            report.rules.missing_zh_regions.join(", ")
        ));
    }
    if !report.rules.missing_zh_quests.is_empty() {
        out.push_str(&format!(
            "- 缺少中文名的任务：{}\n",
            report.rules.missing_zh_quests.join(", ")
        ));
    }

    out.push_str("\n## 结论\n\n");
    if report.ok() {
        out.push_str("全部检查通过。\n");
    } else {
        for failure in &report.strict_failures {
            out.push_str(&format!("- [X] {failure}\n"));
        }
    }
    out
}

fn list_suffix(values: &[String]) -> String {
    if values.is_empty() {
        String::new()
    } else {
        format!(
            "（{}）",
            values
                .iter()
                .take(10)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub fn run(root: &Path, options: &Options) -> Result<bool, String> {
    let report = analyze(root)?;
    let markdown = render_markdown(&report);
    print!("{markdown}");
    let out = options.out.clone().unwrap_or_else(|| root.join("target"));
    std::fs::create_dir_all(&out)
        .map_err(|error| format!("创建 {} 失败: {error}", out.display()))?;
    std::fs::write(out.join("doctor.md"), &markdown)
        .map_err(|error| format!("写入 doctor.md 失败: {error}"))?;
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| format!("序列化 doctor.json 失败: {error}"))?;
    std::fs::write(out.join("doctor.json"), format!("{json}\n"))
        .map_err(|error| format!("写入 doctor.json 失败: {error}"))?;
    println!("\n报告已写入 {}", out.display());
    if options.strict && !report.ok() {
        return Ok(false);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use crate::repo_root;

    #[test]
    fn current_data_is_clean() {
        let report = analyze(&repo_root()).expect("doctor");
        assert!(report.build_error.is_none(), "{report:?}");
        assert!(report.missing_from_catalog.is_empty(), "{report:?}");
        assert!(report.stale_catalog_site_ids.is_empty(), "{report:?}");
        assert!(report.quality.duplicate_aliases.is_empty(), "{report:?}");
        assert!(report.ok(), "{:?}", report.strict_failures);
        assert_eq!(report.rules.missable_total, 211);
        assert_eq!(report.rules.missable_without_rule, 21);
    }
}
