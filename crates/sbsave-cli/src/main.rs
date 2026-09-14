use std::collections::HashMap;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use sbsave_core::analyze::analyze;
use sbsave_core::catalog::{load_catalog, Catalog};
use sbsave_core::gvas::to_jsonable;
use sbsave_core::report::{print_report, write_json, write_markdown, LANGUAGES};
use sbsave_core::savegame::{
    discover_saves, load_save, pick_default_save, SaveData, SaveError, SaveSlot,
};

#[derive(Parser)]
#[command(
    name = "sbsave",
    version,
    about = "剑星 (Stellar Blade) Steam 存档收集度分析工具",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "列出自动探测到的存档。")]
    Saves,
    #[command(about = "分析存档收集情况并输出报告。")]
    Report {
        #[arg(long, short = 's', value_name = "SAVE", help = "指定 .sav 存档路径")]
        save: Option<PathBuf>,
        #[arg(long, help = "存档槽位编号")]
        slot: Option<u32>,
        #[arg(long, short = 'c', help = "只分析指定分类，逗号分隔")]
        category: Option<String>,
        #[arg(
            long,
            default_value = "zh",
            help = "文本语言：zh（中文，默认）、en（英文）、both（中英对照）"
        )]
        lang: String,
        #[arg(long, help = "同时列出已收集的物品")]
        all: bool,
        #[arg(long, help = "导出 JSON 报告")]
        json: Option<PathBuf>,
        #[arg(long, help = "导出 Markdown 报告")]
        markdown: Option<PathBuf>,
        #[arg(long, help = "附加的目录覆盖 JSON 文件")]
        catalog: Option<PathBuf>,
    },
    #[command(about = "解析存档并输出结构信息（调试用）。")]
    Dump {
        #[arg(long, short = 's', value_name = "SAVE", help = "指定 .sav 存档路径")]
        save: Option<PathBuf>,
        #[arg(long, help = "存档槽位编号")]
        slot: Option<u32>,
        #[arg(long, help = "导出存档结构摘要 JSON")]
        json: Option<PathBuf>,
        #[arg(long, help = "导出完整解析树 JSON（可能很大）")]
        tree: Option<PathBuf>,
        #[arg(long, help = "列出已获得物品别名")]
        obtained: bool,
    },
    #[command(about = "查看/校验目录数据库")]
    Catalog {
        #[command(subcommand)]
        command: CatalogCommand,
    },
}

#[derive(Subcommand)]
enum CatalogCommand {
    #[command(about = "列出目录库分类与数量。")]
    List {
        #[arg(long, help = "附加的目录覆盖 JSON 文件")]
        catalog: Option<PathBuf>,
    },
    #[command(about = "校验目录库（重复别名、缺失分类等）。")]
    Check {
        #[arg(long, help = "附加的目录覆盖 JSON 文件")]
        catalog: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    std::process::exit(run(cli));
}

fn run(cli: Cli) -> i32 {
    match cli.command {
        Command::Saves => command_saves(),
        Command::Report {
            save,
            slot,
            category,
            lang,
            all,
            json,
            markdown,
            catalog,
        } => command_report(ReportArgs {
            save,
            slot,
            category,
            lang,
            all,
            json,
            markdown,
            catalog,
        }),
        Command::Dump {
            save,
            slot,
            json,
            tree,
            obtained,
        } => command_dump(save, slot, json, tree, obtained),
        Command::Catalog { command } => match command {
            CatalogCommand::List { catalog } => command_catalog_list(catalog),
            CatalogCommand::Check { catalog } => command_catalog_check(catalog),
        },
    }
}

enum LoadFailure {
    Usage(String),
    Save(SaveError),
}

fn resolve_slots(save: &Option<PathBuf>, slot: Option<u32>) -> Result<Vec<SaveSlot>, LoadFailure> {
    if let Some(path) = save {
        if !path.is_file() {
            return Err(LoadFailure::Usage(format!(
                "存档不存在: {}",
                path.display()
            )));
        }
        let steam_id = path
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .filter(|name| {
                !name.is_empty() && name.chars().all(|character| character.is_ascii_digit())
            })
            .map(str::to_string);
        return Ok(vec![SaveSlot {
            path: path.clone(),
            steam_id,
            slot: 0,
            mtime: std::time::SystemTime::now(),
        }]);
    }
    let mut slots = discover_saves(None);
    if let Some(slot) = slot {
        slots.retain(|candidate| candidate.slot == slot);
    }
    if slots.is_empty() {
        return Err(LoadFailure::Usage(
            "没有找到存档，请用 --save 指定".to_string(),
        ));
    }
    Ok(slots)
}

fn load_selected(save: &Option<PathBuf>, slot: Option<u32>) -> Result<SaveData, LoadFailure> {
    if save.is_some() || slot.is_some() {
        let slots = resolve_slots(save, slot)?;
        let chosen = pick_default_save(Some(slots)).map_err(LoadFailure::Save)?;
        load_save(&chosen.path, chosen.steam_id).map_err(LoadFailure::Save)
    } else {
        let chosen = pick_default_save(None).map_err(LoadFailure::Save)?;
        load_save(&chosen.path, chosen.steam_id).map_err(LoadFailure::Save)
    }
}

fn print_load_failure(failure: &LoadFailure) -> i32 {
    match failure {
        LoadFailure::Usage(message) => {
            eprintln!("{message}");
            2
        }
        LoadFailure::Save(error) => {
            eprintln!("读取存档失败: {error}");
            1
        }
    }
}

fn resolve_categories(catalog: &Catalog, raw: Option<&str>) -> Result<Option<Vec<String>>, String> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    let normalised = raw.replace('，', ",");
    let mut keys: Vec<String> = Vec::new();
    for token in normalised.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if catalog.categories.contains_key(token) {
            keys.push(token.to_string());
            continue;
        }
        match catalog
            .categories
            .values()
            .find(|category| category.name == token)
        {
            Some(category) => keys.push(category.key.clone()),
            None => return Err(format!("未知分类: {token}")),
        }
    }
    Ok(Some(keys))
}

fn format_mtime(mtime: std::time::SystemTime) -> String {
    let datetime = time::OffsetDateTime::from(mtime);
    let offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
    let local = datetime.to_offset(offset);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        local.year(),
        local.month() as u8,
        local.day(),
        local.hour(),
        local.minute()
    )
}

fn command_saves() -> i32 {
    let slots = discover_saves(None);
    if slots.is_empty() {
        println!("未找到存档");
        return 0;
    }
    println!("检测到的存档");
    println!("槽位 | SteamID | 大小 | 修改时间 | 路径");
    for item in &slots {
        let size = std::fs::metadata(&item.path)
            .map(|meta| meta.len())
            .unwrap_or(0);
        println!(
            "{:02} | {} | {:.1} MB | {} | {}",
            item.slot,
            item.steam_id.as_deref().unwrap_or("-"),
            size as f64 / 1024.0 / 1024.0,
            format_mtime(item.mtime),
            item.path.display()
        );
    }
    0
}

struct ReportArgs {
    save: Option<PathBuf>,
    slot: Option<u32>,
    category: Option<String>,
    lang: String,
    all: bool,
    json: Option<PathBuf>,
    markdown: Option<PathBuf>,
    catalog: Option<PathBuf>,
}

fn command_report(args: ReportArgs) -> i32 {
    let ReportArgs {
        save,
        slot,
        category,
        lang,
        all,
        json,
        markdown,
        catalog,
    } = args;
    if !LANGUAGES.contains(&lang.as_str()) {
        eprintln!("未知语言: {lang}（可选 {}）", LANGUAGES.join(", "));
        return 2;
    }
    let save_data = match load_selected(&save, slot) {
        Ok(save_data) => save_data,
        Err(failure) => return print_load_failure(&failure),
    };
    let extras = catalog.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("读取目录库失败: {error}");
            return 1;
        }
    };
    let keys = match resolve_categories(&catalog, category.as_deref()) {
        Ok(keys) => keys,
        Err(message) => {
            eprintln!("{message}");
            return 2;
        }
    };
    let analysis = analyze(&save_data, &catalog, keys.as_deref());
    print!("{}", print_report(&analysis, all, &lang));
    if let Some(path) = json {
        if let Err(error) = write_json(&analysis, &path, true) {
            eprintln!("写入 JSON 失败: {error}");
            return 1;
        }
        println!("JSON 报告已写入 {}", path.display());
    }
    if let Some(path) = markdown {
        if let Err(error) = write_markdown(&analysis, &path, &lang) {
            eprintln!("写入 Markdown 失败: {error}");
            return 1;
        }
        println!("Markdown 报告已写入 {}", path.display());
    }
    0
}

fn command_dump(
    save: Option<PathBuf>,
    slot: Option<u32>,
    json: Option<PathBuf>,
    tree: Option<PathBuf>,
    obtained: bool,
) -> i32 {
    let save_data = match load_selected(&save, slot) {
        Ok(save_data) => save_data,
        Err(failure) => return print_load_failure(&failure),
    };
    let gvas = &save_data.gvas;
    println!("{}", save_data.path.display());
    println!(
        "引擎: {} | 类: {} | EVAS: {} | 顶层属性: {}",
        gvas.header.engine,
        gvas.header.save_game_class_name,
        if gvas.header.has_evas_prefix {
            "True"
        } else {
            "False"
        },
        gvas.properties.len()
    );
    println!("顶层属性");
    println!("名称 | 类型 | 大小");
    for (name, property) in &gvas.properties {
        println!("{} | {} | {}", name, property.prop_type, property.size);
    }
    println!(
        "已获得别名: {} | 成就记录: {} | 购买记录: {} | NG+ 次数: {}",
        save_data.obtained_items.len(),
        save_data.achievements.len(),
        save_data.shop_purchases.len(),
        save_data.ng_plus_count()
    );
    if obtained {
        let mut aliases: Vec<&String> = save_data.obtained_items.iter().collect();
        aliases.sort();
        for alias in aliases {
            println!("  {alias}");
        }
    }
    if let Some(path) = json {
        let mut properties = Vec::new();
        for (name, property) in &gvas.properties {
            let mut entry = serde_json::Map::new();
            entry.insert("name".to_string(), serde_json::Value::String(name.clone()));
            entry.insert(
                "type".to_string(),
                serde_json::Value::String(property.prop_type.clone()),
            );
            entry.insert("size".to_string(), serde_json::Value::from(property.size));
            entry.insert(
                "struct".to_string(),
                match &property.struct_name {
                    Some(name) => serde_json::Value::String(name.clone()),
                    None => serde_json::Value::Null,
                },
            );
            properties.push(serde_json::Value::Object(entry));
        }
        let mut counters = serde_json::Map::new();
        let mut counter_keys: Vec<&String> = save_data.counters.keys().collect();
        counter_keys.sort();
        for key in counter_keys {
            counters.insert(
                key.clone(),
                serde_json::Value::from(save_data.counters[key]),
            );
        }
        let mut string_counters = serde_json::Map::new();
        let mut string_keys: Vec<&String> = save_data.string_counters.keys().collect();
        string_keys.sort();
        for key in string_keys {
            string_counters.insert(
                key.clone(),
                serde_json::Value::String(save_data.string_counters[key].clone()),
            );
        }
        let mut summary = serde_json::Map::new();
        summary.insert(
            "path".to_string(),
            serde_json::Value::String(save_data.path.to_string_lossy().into_owned()),
        );
        summary.insert(
            "class".to_string(),
            serde_json::Value::String(gvas.header.save_game_class_name.clone()),
        );
        summary.insert(
            "engine".to_string(),
            serde_json::Value::String(gvas.header.engine.to_string()),
        );
        summary.insert(
            "properties".to_string(),
            serde_json::Value::Array(properties),
        );
        summary.insert("counters".to_string(), serde_json::Value::Object(counters));
        summary.insert(
            "string_counters".to_string(),
            serde_json::Value::Object(string_counters),
        );
        summary.insert(
            "obtained_count".to_string(),
            serde_json::Value::from(save_data.obtained_items.len()),
        );
        let text = serde_json::to_string_pretty(&serde_json::Value::Object(summary))
            .unwrap_or_else(|_| "{}".to_string());
        if let Err(error) = std::fs::write(&path, text) {
            eprintln!("写入摘要失败: {error}");
            return 1;
        }
        println!("摘要已写入 {}", path.display());
    }
    if let Some(path) = tree {
        let mut tree_map = serde_json::Map::new();
        for (name, property) in &gvas.properties {
            tree_map.insert(name.clone(), to_jsonable(&property.value, 64));
        }
        let text = serde_json::to_string_pretty(&serde_json::Value::Object(tree_map))
            .unwrap_or_else(|_| "{}".to_string());
        if let Err(error) = std::fs::write(&path, text) {
            eprintln!("写入解析树失败: {error}");
            return 1;
        }
        println!("完整解析树已写入 {}", path.display());
    }
    0
}

fn command_catalog_list(catalog_path: Option<PathBuf>) -> i32 {
    let extras = catalog_path.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("读取目录库失败: {error}");
            return 1;
        }
    };
    println!("目录库 v{}", catalog.version);
    println!("分类键 | 分类名 | 条目数 | 已映射别名 | 需多周目 | DLC");
    for category in catalog.category_list() {
        let items = catalog.by_category(&category.key);
        let mapped: usize = items.iter().map(|item| item.satisfy_aliases().len()).sum();
        let ng_plus = items.iter().filter(|item| item.ng_plus > 0).count();
        let dlc = items.iter().filter(|item| item.dlc.is_some()).count();
        println!(
            "{} | {} | {} | {} | {} | {}",
            category.key,
            category.name,
            items.len(),
            mapped,
            ng_plus,
            dlc
        );
    }
    0
}

fn command_catalog_check(catalog_path: Option<PathBuf>) -> i32 {
    let extras = catalog_path.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("读取目录库失败: {error}");
            return 1;
        }
    };
    let mut problems: Vec<String> = Vec::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for item in &catalog.items {
        for alias in item.satisfy_aliases() {
            if let Some(previous) = seen.get(alias) {
                if previous != &item.id {
                    problems.push(format!("别名 {alias} 同时映射到 {previous} 和 {}", item.id));
                }
            }
            seen.insert(alias.to_string(), item.id.clone());
        }
        if !catalog.categories.contains_key(&item.category) {
            problems.push(format!("条目 {} 的分类 {} 未定义", item.id, item.category));
        }
    }
    let low = catalog
        .items
        .iter()
        .filter(|item| item.confidence != "high")
        .count();
    if !problems.is_empty() {
        for problem in &problems {
            println!("[X] {problem}");
        }
        return 1;
    }
    println!(
        "[OK] 目录库正常：{} 个条目，{} 个别名映射（低置信度 {} 条）",
        catalog.items.len(),
        seen.len(),
        low
    );
    0
}
