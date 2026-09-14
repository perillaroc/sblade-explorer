use clap::Parser;

#[derive(Parser)]
#[command(
    name = "sbsave",
    version,
    about = "剑星 (Stellar Blade) Steam 存档收集度分析工具"
)]
struct Cli {}

fn main() {
    Cli::parse();
}
