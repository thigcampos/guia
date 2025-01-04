use walkdir::WalkDir;
use std::path::Path;

const DOCSETS_PATH: &str = "docsets/";

fn get_docs_from_docsets(docsets_path: &str) -> Vec<String> {
    let mut docs = Vec::new();

    for entry in WalkDir::new(docsets_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.metadata().map(|m| m.is_dir()).unwrap_or(false)) 
        {
            if entry.path() != Path::new(docsets_path) {
                if let Some(name) = entry.path().file_name() {
                    docs.push(name.to_string_lossy().to_string());
                }
            }
        }
    docs
}

fn main() {
    let docs = get_docs_from_docsets(DOCSETS_PATH);
    for doc in docs {
        println!("{}", doc);
    }
}
