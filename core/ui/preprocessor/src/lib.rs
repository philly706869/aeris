use std::path::Path;

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

pub fn main<S, P>(shard: S, path: P)
where
    S: SyntaxShard,
    P: AsRef<Path>,
{
    let cli = Cli::parse();
}
