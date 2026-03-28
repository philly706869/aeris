use std::{fs, path::PathBuf};

use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    /// The path where the project will be created. If not provided, the current directory will be used.
    #[arg(default_value = ".")]
    path: String,

    /// The name of the project. If not provided, the name will be derived from the path.
    #[arg(short, long)]
    name: Option<String>,
}

const CONFIG_NAME: &str = "config.aeris";
const TEMPLATE_CONFIG: &str = include_str!("template/config.aeris.template");
const GITIGNORE_NAME: &str = ".gitignore";
const TEMPLATE_GITIGNORE: &str = include_str!("template/.gitignore.template");

pub fn handle(args: &Args) {
    let base_path = PathBuf::from(&args.path);
    fs::create_dir_all(&base_path).expect("Failed to create project directory");
    let config_path = base_path.join(CONFIG_NAME);
    let gitignore_path = base_path.join(GITIGNORE_NAME);
    let name = args
        .name
        .as_deref()
        .unwrap_or_else(|| base_path.file_name().unwrap().to_str().unwrap());
    let config_content = TEMPLATE_CONFIG.replace("{{name}}", name);
    todo!()
}
