use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    /// The name of the project to create. If not provided, the current directory will be used.
    name: Option<String>,

    /// If set, the project will be created without initializing a git repository.
    #[arg(long)]
    nogit: bool,
}

pub fn handle(args: &Args) {}
