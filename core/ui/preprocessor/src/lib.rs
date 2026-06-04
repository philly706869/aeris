use std::path::Path;

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

pub fn main<P>(path: P)
where
    P: AsRef<Path>,
{
    let cli = Cli::parse();
    match cli.command {
        Commands::Sync => sync(path),
        Commands::Check => check(path),
    }
}

fn sync<P>(path: P)
where
    P: AsRef<Path>,
{
    todo!()
}

fn check<P>(path: P)
where
    P: AsRef<Path>,
{
    todo!()
}
