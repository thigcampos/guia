/// Simple directory mapping for MacOS and Linux
/// Partially inspired by https://github.com/dirs-dev/dirs-rs/
use std::env;
use std::path::PathBuf;

const DOCS_PATH: &str = "docs";

#[cfg(target_os = "linux")]
const CONFIG_PATH: &str = ".config";

#[cfg(target_os = "macos")]
const CONFIG_PATH: &str = "Library/Application Support";

/// *Unix home directory
pub fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .and_then(|dir| if dir.is_empty() { None } else { Some(dir) })
        .map(PathBuf::from)
}

/// MacOS configuration directory
#[cfg(target_os = "macos")]
pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join(CONFIG_PATH))
}

/// Linux configuration directory
#[cfg(target_os = "linux")]
pub fn config_dir() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME").or_else(|| home_dir().map(|h| h.join(CONFIG_PATH)))
}

/// Guia's docs directory
/// Which hosts all documentations
pub fn guia_docs_path() -> PathBuf {
    config_dir()
        .expect("Unable to find configuration directory. Tried to find {CONFIG_PATH}")
        .join("guia")
        .join(DOCS_PATH)
}
