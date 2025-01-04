use std::path::{Path, PathBuf};
use std::process::Command;

use dirs::config_local_dir;
use clap::{Arg, Command as ClapCommand};
use walkdir::WalkDir;
use inquire::Select;

const APP_NAME: &str = "guia";
const DOCSETS_PATH_NAME: &str = "docsets";

fn build_cli() -> ClapCommand {
    ClapCommand::new(APP_NAME)
        .version("0.1.0")
        .author("Thiago Campos")
        .about("Get your the documentation of your favortite software without leaving the terminal.")
        .arg(
            Arg::new("documentation")
                .help("The name of the documentation folder to load")
                .required(true)
                .index(1),
        )
}

fn get_docsets_path() -> PathBuf {
    let config_dir = config_local_dir().expect("Failed to get config directory");
    let docsets_path = config_dir.join(APP_NAME).join(DOCSETS_PATH_NAME);

    if !docsets_path.exists() {
        std::fs::create_dir_all(&docsets_path).expect("Failed to create docsets directory");
    }

    docsets_path
}

fn get_files_from_docsets(docsets_path: &str) -> Vec<String> {
    let mut docs = Vec::new();

    for entry in WalkDir::new(docsets_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false)) 
        {
            if entry.path() != Path::new(docsets_path) {
                if let Some(name) = entry.path().file_name() {
                    docs.push(name.to_string_lossy().to_string());
                }
            }
        }
    docs
}

fn select_file(files: Vec<String>) -> String {
    let prompt = Select::new("Select a file", files);
    prompt.prompt().expect("Failed to get user input")
}

fn main() {
    let matches = build_cli().get_matches();

    let docsets_path = get_docsets_path();
    let doc_name = matches.get_one::<String>("documentation").unwrap();
    let doc_path = docsets_path.join(doc_name);
    println!("Opening documentation for '{}' in {}", doc_name,doc_path.display());

    if !std::path::Path::new(&doc_path).exists() {
        eprintln!("Error: Documentation folder '{}' does not exist!", doc_name);
        std::process::exit(1);
    }

    let files = get_files_from_docsets(&format!("{}", doc_path.display()));
    let selected_file = select_file(files);

    Command::new("less")
        .arg(format!("{}/{}", doc_path.display(), selected_file))
        .status()
        .expect("Failed to open documentation");
}
