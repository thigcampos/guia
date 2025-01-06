use std::path::{Path};
use std::fs::{self, File};
use std::io::Write;
use reqwest::blocking::Client;
use super::read::{Docset};

pub fn download_file(client: &Client, url: &str, dest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    let mut file = File::create(dest_path)?;
    file.write_all(&response.bytes()?)?;

    Ok(())
}

pub fn add_docset(docset_name: &str, docset_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let docsets_file_path = config_dir.join(super::cli::APP_NAME).join("docsets.json");

    let docsets_file = File::open(&docsets_file_path)?;
    let docsets: Vec<Docset> = serde_json::from_reader(docsets_file)?;

    let docset = docsets.into_iter().find(|d| d.name == docset_name)
        .ok_or_else(|| format!("No documentation entry for '{}' in docsets", docset_name))?;

    let client = Client::new();

    for folder in docset.folders {
        let folder_path = docset_path.join(&docset_name).join(&folder.name);
        fs::create_dir_all(&folder_path)?;

        for file in folder.files {
            let file_path = folder_path.join(&file.name);
            download_file(&client, &file.download_url, &file_path)?;
        }
    }

    println!("Downloaded documentation for '{}'", docset_name);

    Ok(())
}
