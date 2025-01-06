mod cli;
mod read;
mod add;

use std::process::Command;
use cli::build_cli;
use read::{get_docsets_path, get_files_from_docsets, select_file};
use add::add_docset;

fn main() {
    let matches = build_cli().get_matches();
    let docsets_path = get_docsets_path();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(doc_name) = matches.get_one::<String>("DOC_NAME") {
            if let Err(e) = add_docset(doc_name, &docsets_path) {
                eprintln!("Failed to download documentation: {}", e);
                std::process::exit(1);
            }
        }
    } else if let Some(doc_name) = matches.get_one::<String>("documentation") {
        let files = get_files_from_docsets(&format!("{}/{}", docsets_path.display(), doc_name));
        let selected_file = select_file(files.clone());

        let selected_file_path = files.iter()
            .find(|(name, _)| name == &selected_file)
            .map(|(_, path)| path)
            .expect("Selected file not found");

        Command::new("less")
            .arg(&selected_file_path)
            .status()
            .expect("Failed to open documentation");
    } else if matches.subcommand_matches("list").is_some() {
        let docs = get_files_from_docsets(&format!("{}", docsets_path.display()));
        for doc in docs {
            println!("{}", doc.0);
        }
    }
}
