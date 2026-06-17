use aeris_ui_lib::SyntaxShard;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync,
    Check,
}

pub fn main<S>()
where
    S: SyntaxShard,
{
    let cli = Cli::parse();
}
