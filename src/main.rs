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
        .bin_name(APP_NAME)
        .about("Get your the documentation of your favortite software without leaving the terminal.")
        .arg(
            Arg::new("add")
                .help("Add a documentation set")
                .long("add")
                .value_name("DOC_NAME")
                .required(false),
        )
        .arg(
            Arg::new("documentation")
                .help("The name of the documentation folder to load")
                .required(false)
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

fn get_files_from_docsets(docsets_path: &str) -> Vec<(String, String)> {
    let mut files = Vec::new();

    for entry in WalkDir::new(docsets_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false)) 
        {
            if entry.path() != Path::new(docsets_path) {
                if let Some(name) = entry.path().file_name() {
                    let filename = name.to_string_lossy().to_string();
                    let filepath = entry.path().to_string_lossy().to_string();
                    files.push((filename, filepath));
                }
            }
        }
    files
}

fn select_file(files: Vec<(String, String)>) -> String {
    let file_options: Vec<String> = files.iter().map(|(name, _)| name.clone()).collect();
    let prompt = Select::new("Select a file", file_options).prompt().expect("Failed to get user input");
    prompt
}

fn main() {
    let matches = build_cli().get_matches();

    let docsets_path = get_docsets_path();
    let doc_name = matches.get_one::<String>("documentation").unwrap();
    let doc_path = docsets_path.join(doc_name);

    if !std::path::Path::new(&doc_path).exists() {
        eprintln!("No documentation entry for {}", doc_name);
        std::process::exit(1);
    }

    let files = get_files_from_docsets(&format!("{}", doc_path.display()));
    let selected_file = select_file(files.clone());

    let selected_file_path = files.iter()
        .find(|(name, _)| name == &selected_file)
        .map(|(_, path)| path)
        .expect("Selected file not found");

    Command::new("less")
        .arg(&selected_file_path)
        .status()
        .expect("Failed to open documentation");
}
