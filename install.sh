#!/bin/bash

# Define Config Directory
get_config_dir() {
  case "$(uname)" in
  Linux | Redox)
    echo "${XDG_CONFIG_HOME:-$HOME/.config}/guia"
    ;;
  Darwin)
    echo "$HOME/Library/Application Support/guia"
    ;;
  CYGWIN* | MINGW32* | MSYS* | MINGW*)
    echo "$(cygpath -w "$APPDATA")/guia"
    ;;
  *)
    echo "Unsupported OS"
    exit 1
    ;;
  esac
}

# INSTALL GUIA
# If cargo is not installed, exit
if ! command -v cargo &>/dev/null; then
  echo "cargo could not be found. Please install Rust and Cargo from https://www.rust-lang.org/tools/install and try again."
  exit 1
fi

cargo install guia

# ADD DOCSETS
config_dir=$(get_config_dir)
mkdir -p "$config_dir"
# Download docsets.json from GitHub
curl -L -o "$config_dir/docsets.json" "https://raw.githubusercontent.com/thigcampos/docsets/main/docsets.json"
if [ $? -ne 0 ]; then
  echo "Failed to download docsets.json"
  exit 1
fi

# ADD GUIA ENVIRONMENT VARIABLES
# Less is the default renderer since it is the most common
export GUIA_MARKDOWN=less

echo "Installation completed."
