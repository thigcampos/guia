/// Simple directory mapping for MacOS and Linux
/// Partially inspired by https://github.com/dirs-dev/dirs-rs/
use std::env;
use std::path::PathBuf;

/// *Unix Home Directory
pub fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .and_then(|dir| if dir.is_empty() { None } else { Some(dir) })
        .map(PathBuf::from)
}

/// MacOS Configuration Directory
#[cfg(target_os = "macos")]
pub fn config_dir() -> Option<PathBuf> {
    home_dir().map(|h| h.join("Library/Application Support"))
}

/// Linux Configuration Directory
#[cfg(target_os = "linux")]
pub fn config_dir() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME").or_else(|| home_dir().map(|h| h.join(".config")))
}
