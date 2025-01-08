use std::path::{Path, PathBuf};
use dirs::config_local_dir;
use inquire::Select;
use serde::Deserialize;
use walkdir::WalkDir;

pub const DOCSETS_PATH_NAME: &str = "docsets";

#[derive(Deserialize)]
pub struct Docset {
    pub name: String,
    pub folders: Vec<Folder>,
}

#[derive(Deserialize)]
pub struct Folder {
    pub name: String,
    pub files: Vec<FileInfo>,
}

#[derive(Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub download_url: String,
}

pub fn get_docsets_path() -> PathBuf {
    let config_dir = config_local_dir().expect("Failed to get config directory");
    let docsets_path = config_dir.join(super::cli::APP_NAME).join(DOCSETS_PATH_NAME);

    if !docsets_path.exists() {
        std::fs::create_dir_all(&docsets_path).expect("Failed to create docsets directory");
    }

    docsets_path
}

pub fn get_docs_from_docsets(docsets_path: &str) -> Vec<String> {
    let mut docs = Vec::new();

    for entry in WalkDir::new(docsets_path)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.metadata().map(|m| m.is_dir()).unwrap_or(false)) 
        {
            if entry.path() != Path::new(docsets_path) {
                if let Some(name) = entry.path().file_name() {
                    let filename = name.to_string_lossy().to_string();
                    docs.push(filename);
                }
            }
        }
    docs
}

pub fn get_topics_from_docsets(docsets_path: &str) -> Vec<String> {
    let mut topics = Vec::new();

    for entry in WalkDir::new(docsets_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.metadata().map(|m| m.is_dir()).unwrap_or(false)) 
        {
            if entry.path() != Path::new(docsets_path) {
                if let Some(name) = entry.path().file_name() {
                    let filename = name.to_string_lossy().to_string();
                    topics.push(filename);
                }
            }
        }
    topics
}

pub fn select_topic(topics: Vec<String>) -> String {
    let file_options: Vec<String> = topics.iter().map(|name| name.clone()).collect();
    let prompt = Select::new("Select a topic", file_options).prompt().expect("Failed to get user input");
    prompt
}

pub fn get_files_from_docsets(docsets_path: &str) -> Vec<(String, String)> {
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

pub fn select_file(files: Vec<(String, String)>) -> String {
    let file_options: Vec<String> = files.iter().map(|(name, _)| name.clone()).collect();
    let prompt = Select::new("Select a file", file_options).prompt().expect("Failed to get user input");
    prompt
}


