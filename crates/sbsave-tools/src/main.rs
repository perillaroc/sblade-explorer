use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use sbsave_tools::{catalog_build, doctor, mine_names, repo_root};

#[derive(Parser)]
#[command(
    name = "sbsave-tools",
    version,
    about = "剑星存档分析构建期数据管线（目录库生成与游戏名称挖掘）",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "目录库数据管线。")]
    Catalog {
        #[command(subcommand)]
        command: CatalogCommand,
    },
    #[command(
        name = "mine-names",
        about = "从游戏数据表与 Game.locres 提取内部别名到官方名称的映射。"
    )]
    MineNames(MineNamesArgs),
    #[command(
        name = "doctor",
        about = "检查目录库、攻略链接与规则数据的一致性（离线，不读取存档）。"
    )]
    Doctor(DoctorArgs),
}

#[derive(Args)]
struct DoctorArgs {
    #[arg(
        long,
        value_name = "DIR",
        help = "报告输出目录（默认 target，写入 doctor.md / doctor.json）"
    )]
    out: Option<PathBuf>,
    #[arg(long, help = "发现新增未入库条目、失效站点 id 或重复别名时退出码非 0")]
    strict: bool,
}

#[derive(Subcommand)]
enum CatalogCommand {
    #[command(about = "由 data/raw 数据生成 data/catalog.json。")]
    Build,
}

#[derive(Args)]
struct MineNamesArgs {
    #[arg(
        long,
        value_name = "DIR",
        help = "包含 CUE4Parse JSON 导出与 Game.locres 的目录"
    )]
    dump: Option<PathBuf>,
    #[arg(
        long,
        value_name = "DIR",
        help = "剑星安装目录（提供后自动调用 cue4parse/repak 导出）"
    )]
    game: Option<PathBuf>,
    #[arg(
        long,
        value_name = "DIR",
        help = "包含 cue4parse.exe、repak.exe 与 .usmap 的目录"
    )]
    tools: Option<PathBuf>,
    #[arg(long, value_name = "FILE", help = "UAssetGUI/CUE4Parse .usmap 文件")]
    mappings: Option<PathBuf>,
    #[arg(
        long,
        value_name = "FILE",
        help = "输出路径（默认 data/raw/game/name_map.json）"
    )]
    out: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    std::process::exit(run(cli));
}

fn run(cli: Cli) -> i32 {
    match cli.command {
        Command::Catalog { command } => match command {
            CatalogCommand::Build => match catalog_build::run(&repo_root()) {
                Ok(()) => 0,
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            },
        },
        Command::MineNames(args) => {
            if args.dump.is_none() && args.game.is_none() {
                eprintln!("pass --dump or --game");
                return 2;
            }
            let options = mine_names::Options {
                dump: args.dump,
                game: args.game,
                tools: args.tools,
                mappings: args.mappings,
                out: args.out,
            };
            match mine_names::run(&options) {
                Ok(()) => 0,
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            }
        }
        Command::Doctor(args) => {
            let options = doctor::Options {
                out: args.out,
                strict: args.strict,
            };
            match doctor::run(&repo_root(), &options) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(message) => {
                    eprintln!("{message}");
                    1
                }
            }
        }
    }
}
