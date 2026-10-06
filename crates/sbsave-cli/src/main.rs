use std::collections::HashMap;
use std::path::PathBuf;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use sbsave_core::analyze::analyze;
use sbsave_core::catalog::{load_catalog, Catalog};
use sbsave_core::gvas::to_jsonable;
use sbsave_core::i18n::{Locale, Messages};
use sbsave_core::report::{print_report, write_json, write_markdown, LANGUAGES};
use sbsave_core::savegame::{
    discover_saves, load_save, pick_default_save, SaveData, SaveError, SaveSlot,
};

#[derive(Parser)]
#[command(name = "sbsave", version, arg_required_else_help = true)]
struct Cli {
    /// Localized at runtime; see `localize_command`.
    #[arg(long, global = true, value_name = "LANG")]
    ui_lang: Option<String>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Saves,
    Report {
        #[arg(long, short = 's', value_name = "SAVE")]
        save: Option<PathBuf>,
        #[arg(long)]
        slot: Option<u32>,
        #[arg(long, short = 'c')]
        category: Option<String>,
        #[arg(long, default_value = "zh")]
        lang: String,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        json: Option<PathBuf>,
        #[arg(long)]
        markdown: Option<PathBuf>,
        #[arg(long)]
        catalog: Option<PathBuf>,
    },
    Dump {
        #[arg(long, short = 's', value_name = "SAVE")]
        save: Option<PathBuf>,
        #[arg(long)]
        slot: Option<u32>,
        #[arg(long)]
        json: Option<PathBuf>,
        #[arg(long)]
        tree: Option<PathBuf>,
        #[arg(long)]
        obtained: bool,
    },
    Catalog {
        #[command(subcommand)]
        command: CatalogCommand,
    },
}

#[derive(Subcommand)]
enum CatalogCommand {
    List {
        #[arg(long)]
        catalog: Option<PathBuf>,
    },
    Check {
        #[arg(long)]
        catalog: Option<PathBuf>,
    },
}

/// Scans the raw arguments for `--ui-lang` so the help text can be localized
/// before clap parses (and validates) the command line.
fn scan_ui_lang(args: &[String]) -> Option<String> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--ui-lang" {
            return iter.next().cloned();
        }
        if let Some(value) = arg.strip_prefix("--ui-lang=") {
            return Some(value.to_string());
        }
    }
    None
}

fn resolve_locale(args: &[String]) -> Locale {
    if let Some(value) = scan_ui_lang(args) {
        return Locale::parse(&value).unwrap_or_default();
    }
    sys_locale::get_locale()
        .as_deref()
        .and_then(Locale::parse)
        .unwrap_or_default()
}

/// Replaces the clap help text with the selected locale. Built-in clap errors
/// (unknown arguments, ...) stay in English.
fn localize_command(command: clap::Command, messages: &Messages) -> clap::Command {
    command
        .about(messages.cli_about())
        .mut_arg("ui_lang", |arg| arg.help(messages.cli_ui_lang_help()))
        .mut_subcommand("saves", |sub| sub.about(messages.cli_saves_about()))
        .mut_subcommand("report", |sub| {
            sub.about(messages.cli_report_about())
                .mut_arg("save", |arg| arg.help(messages.help_save()))
                .mut_arg("slot", |arg| arg.help(messages.help_slot()))
                .mut_arg("category", |arg| arg.help(messages.help_category()))
                .mut_arg("lang", |arg| arg.help(messages.help_lang()))
                .mut_arg("all", |arg| arg.help(messages.help_all()))
                .mut_arg("json", |arg| arg.help(messages.help_json()))
                .mut_arg("markdown", |arg| arg.help(messages.help_markdown()))
                .mut_arg("catalog", |arg| arg.help(messages.help_catalog()))
        })
        .mut_subcommand("dump", |sub| {
            sub.about(messages.cli_dump_about())
                .mut_arg("save", |arg| arg.help(messages.help_save()))
                .mut_arg("slot", |arg| arg.help(messages.help_slot()))
                .mut_arg("json", |arg| arg.help(messages.help_json()))
                .mut_arg("tree", |arg| arg.help(messages.help_tree()))
                .mut_arg("obtained", |arg| arg.help(messages.help_obtained()))
        })
        .mut_subcommand("catalog", |sub| {
            sub.about(messages.cli_catalog_about())
                .mut_subcommand("list", |list| {
                    list.about(messages.cli_catalog_list_about())
                        .mut_arg("catalog", |arg| arg.help(messages.help_catalog()))
                })
                .mut_subcommand("check", |check| {
                    check
                        .about(messages.cli_catalog_check_about())
                        .mut_arg("catalog", |arg| arg.help(messages.help_catalog()))
                })
        })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let messages = Messages::new(resolve_locale(&args));
    let matches = localize_command(Cli::command(), &messages)
        .try_get_matches_from(&args)
        .unwrap_or_else(|error| error.exit());
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|error| error.exit());
    if let Some(value) = cli.ui_lang.as_deref() {
        if Locale::parse(value).is_none() {
            eprintln!("{}", messages.error_unknown_language(value, "zh, en"));
            std::process::exit(2);
        }
    }
    std::process::exit(run(cli, messages));
}

fn run(cli: Cli, messages: Messages) -> i32 {
    match cli.command {
        Command::Saves => command_saves(messages),
        Command::Report {
            save,
            slot,
            category,
            lang,
            all,
            json,
            markdown,
            catalog,
        } => command_report(
            ReportArgs {
                save,
                slot,
                category,
                lang,
                all,
                json,
                markdown,
                catalog,
            },
            messages,
        ),
        Command::Dump {
            save,
            slot,
            json,
            tree,
            obtained,
        } => command_dump(save, slot, json, tree, obtained, messages),
        Command::Catalog { command } => match command {
            CatalogCommand::List { catalog } => command_catalog_list(catalog, messages),
            CatalogCommand::Check { catalog } => command_catalog_check(catalog, messages),
        },
    }
}

enum LoadFailure {
    Usage(String),
    Save(SaveError),
}

fn resolve_slots(
    save: &Option<PathBuf>,
    slot: Option<u32>,
    messages: &Messages,
) -> Result<Vec<SaveSlot>, LoadFailure> {
    if let Some(path) = save {
        if !path.is_file() {
            return Err(LoadFailure::Usage(
                messages.error_save_not_found(&path.display().to_string()),
            ));
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
        return Err(LoadFailure::Usage(messages.error_no_saves().to_string()));
    }
    Ok(slots)
}

fn load_selected(
    save: &Option<PathBuf>,
    slot: Option<u32>,
    messages: &Messages,
) -> Result<SaveData, LoadFailure> {
    if save.is_some() || slot.is_some() {
        let slots = resolve_slots(save, slot, messages)?;
        let chosen = pick_default_save(Some(slots)).map_err(LoadFailure::Save)?;
        load_save(&chosen.path, chosen.steam_id).map_err(LoadFailure::Save)
    } else {
        let chosen = pick_default_save(None).map_err(LoadFailure::Save)?;
        load_save(&chosen.path, chosen.steam_id).map_err(LoadFailure::Save)
    }
}

fn print_load_failure(failure: &LoadFailure, messages: &Messages) -> i32 {
    match failure {
        LoadFailure::Usage(message) => {
            eprintln!("{message}");
            2
        }
        LoadFailure::Save(error) => {
            eprintln!("{}", messages.error_load_save(&error.to_string()));
            1
        }
    }
}

fn resolve_categories(
    catalog: &Catalog,
    raw: Option<&str>,
    messages: &Messages,
) -> Result<Option<Vec<String>>, String> {
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
            .find(|category| category.name == token || category.name_en.as_deref() == Some(token))
        {
            Some(category) => keys.push(category.key.clone()),
            None => return Err(messages.error_unknown_category(token)),
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

fn command_saves(messages: Messages) -> i32 {
    let slots = discover_saves(None);
    if slots.is_empty() {
        println!("{}", messages.no_saves_found());
        return 0;
    }
    println!("{}", messages.detected_saves());
    println!("{}", messages.saves_header());
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

fn command_report(args: ReportArgs, messages: Messages) -> i32 {
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
        eprintln!(
            "{}",
            messages.error_unknown_language(&lang, &LANGUAGES.join(", "))
        );
        return 2;
    }
    let save_data = match load_selected(&save, slot, &messages) {
        Ok(save_data) => save_data,
        Err(failure) => return print_load_failure(&failure, &messages),
    };
    let extras = catalog.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("{}", messages.error_load_catalog(&error.to_string()));
            return 1;
        }
    };
    let keys = match resolve_categories(&catalog, category.as_deref(), &messages) {
        Ok(keys) => keys,
        Err(message) => {
            eprintln!("{message}");
            return 2;
        }
    };
    let analysis = analyze(&save_data, &catalog, keys.as_deref());
    print!("{}", print_report(&analysis, all, &lang, messages.locale()));
    if let Some(path) = json {
        if let Err(error) = write_json(&analysis, &path, true) {
            eprintln!("{}", messages.error_write_json(&error.to_string()));
            return 1;
        }
        println!("{}", messages.write_json_ok(&path.display().to_string()));
    }
    if let Some(path) = markdown {
        if let Err(error) = write_markdown(&analysis, &path, &lang, messages.locale()) {
            eprintln!("{}", messages.error_write_markdown(&error.to_string()));
            return 1;
        }
        println!(
            "{}",
            messages.write_markdown_ok(&path.display().to_string())
        );
    }
    0
}

fn command_dump(
    save: Option<PathBuf>,
    slot: Option<u32>,
    json: Option<PathBuf>,
    tree: Option<PathBuf>,
    obtained: bool,
    messages: Messages,
) -> i32 {
    let save_data = match load_selected(&save, slot, &messages) {
        Ok(save_data) => save_data,
        Err(failure) => return print_load_failure(&failure, &messages),
    };
    let gvas = &save_data.gvas;
    println!("{}", save_data.path.display());
    println!(
        "{}",
        messages.dump_engine_line(
            &gvas.header.engine.to_string(),
            &gvas.header.save_game_class_name,
            gvas.header.has_evas_prefix,
            gvas.properties.len(),
        )
    );
    println!("{}", messages.dump_top_properties());
    println!("{}", messages.dump_properties_header());
    for (name, property) in &gvas.properties {
        println!("{} | {} | {}", name, property.prop_type, property.size);
    }
    println!(
        "{}",
        messages.dump_obtained_counts(
            save_data.obtained_items.len(),
            save_data.achievements.len(),
            save_data.shop_purchases.len(),
            save_data.ng_plus_count(),
        )
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
            eprintln!("{}", messages.error_write_summary(&error.to_string()));
            return 1;
        }
        println!("{}", messages.summary_written(&path.display().to_string()));
    }
    if let Some(path) = tree {
        let mut tree_map = serde_json::Map::new();
        for (name, property) in &gvas.properties {
            tree_map.insert(name.clone(), to_jsonable(&property.value, 64));
        }
        let text = serde_json::to_string_pretty(&serde_json::Value::Object(tree_map))
            .unwrap_or_else(|_| "{}".to_string());
        if let Err(error) = std::fs::write(&path, text) {
            eprintln!("{}", messages.error_write_tree(&error.to_string()));
            return 1;
        }
        println!("{}", messages.tree_written(&path.display().to_string()));
    }
    0
}

fn command_catalog_list(catalog_path: Option<PathBuf>, messages: Messages) -> i32 {
    let extras = catalog_path.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("{}", messages.error_load_catalog(&error.to_string()));
            return 1;
        }
    };
    println!("{}", messages.catalog_version(catalog.version));
    println!("{}", messages.catalog_list_header());
    for category in catalog.category_list() {
        let items = catalog.by_category(&category.key);
        let mapped: usize = items.iter().map(|item| item.satisfy_aliases().len()).sum();
        let ng_plus = items.iter().filter(|item| item.ng_plus > 0).count();
        let dlc = items.iter().filter(|item| item.dlc.is_some()).count();
        println!(
            "{} | {} | {} | {} | {} | {} | {}",
            category.key,
            category.localized_name(messages.locale()),
            category.section,
            items.len(),
            mapped,
            ng_plus,
            dlc
        );
    }
    0
}

fn command_catalog_check(catalog_path: Option<PathBuf>, messages: Messages) -> i32 {
    let extras = catalog_path.map(|path| vec![path]);
    let catalog = match load_catalog(extras.as_deref()) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("{}", messages.error_load_catalog(&error.to_string()));
            return 1;
        }
    };
    let mut problems: Vec<String> = Vec::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for item in &catalog.items {
        for alias in item.satisfy_aliases() {
            if let Some(previous) = seen.get(alias) {
                if previous != &item.id {
                    problems.push(messages.check_alias_duplicate(alias, previous, &item.id));
                }
            }
            seen.insert(alias.to_string(), item.id.clone());
        }
        if !catalog.categories.contains_key(&item.category) {
            problems.push(messages.check_category_undefined(&item.id, &item.category));
        }
    }
    let missing_name_en = catalog
        .categories
        .values()
        .filter(|category| {
            category
                .name_en
                .as_deref()
                .is_none_or(|name| name.trim().is_empty())
        })
        .count();
    if missing_name_en > 0 {
        problems.push(messages.check_missing_name_en(missing_name_en));
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
        "{}",
        messages.check_ok(catalog.items.len(), seen.len(), low)
    );
    0
}
