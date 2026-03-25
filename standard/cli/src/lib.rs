mod commands;

use clap::{Parser, Subcommand};

pub fn main() {
    Cli::run();
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() {
        let cli = Self::parse();
        match &cli.command {
            Commands::New(args) => commands::new::handle(args),
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    New(commands::new::Args),
}
