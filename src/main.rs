mod cli;
mod read;
mod add;

use std::env::var;
use std::process::Command;
use cli::build_cli;
use read::*;
use add::add_docset;

const RENDER_VAR: &str = "GUIA_MARKDOWN";

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
        let doc_path = format!("{}/{}", docsets_path.display(), doc_name);

        if !std::path::Path::new(&doc_path).exists() {
            eprintln!("No documentation entry for {}", doc_name);
            std::process::exit(1);
        }

        let topics = get_topics_from_docsets(&doc_path);
        let mut selected_topic = String::new();

        if !topics.is_empty() {
            selected_topic = select_topic(topics.clone());
        }

        let topic_path = format!("{}/{}", doc_path, selected_topic);
        let files = get_files_from_docsets(&topic_path);
        let selected_file = select_file(files.clone());

        let selected_file_path = files.iter()
            .find(|(name, _)| name == &selected_file)
            .map(|(_, path)| path)
            .expect("Selected file not found");

        let guia_render = var(RENDER_VAR).unwrap_or("less".to_string());
        let mut command_parts = guia_render.split_whitespace();
        let command = command_parts.next().expect("No command provided");
        let command_args = command_parts.collect::<Vec<&str>>();

        Command::new(command)
            .arg(&selected_file_path)
            .args(command_args)
            .status()
            .expect("Failed to open documentation");
    } else if matches.subcommand_matches("list").is_some() {
        let docs = get_docs_from_docsets(&format!("{}", docsets_path.display()));
        println!("Available documentation sets:");
        for doc in docs {
            println!("{}", doc);
        }
    }
}
