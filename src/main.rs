mod dirs;

use clap::Parser;
use clap::Subcommand;
use dirs::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Guia {
    /// Documentation to be opened
    documentation_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a documentation to your local environment
    Add {
        /// Documentation set to be added
        documentation_name: String,
    },
    /// List all documentations available
    List {
        /// Documentations available locally
        #[arg(short, long)]
        local: bool,
    },
    /// Prepare local enviroment to host documentations
    Setup,
}

fn main() {
    let guia = Guia::parse();

    if let Some(doc_to_be_read) = guia.documentation_name.as_deref() {
        read_doc(doc_to_be_read)
    }

    match &guia.command {
        Some(Commands::Add { documentation_name }) => add_doc(documentation_name),

        Some(Commands::List { local }) => {
            if *local {
                list_local_docs()
            } else {
                list_all_docs()
            }
        }

        Some(Commands::Setup) => setup_guia(),

        None => {}
    }
}

fn read_doc(doc_name: &str) {
    println!("{doc_name} will be read");
}

fn add_doc(doc_name: &str) {
    println!("{doc_name} will be added");
}

fn list_all_docs() {
    println!("All documentations");
}

fn list_local_docs() {
    println!("All local documentations");
}

fn setup_guia() {
    println!("Setting up local environment...");
    let guia_docs = guia_docs_path();

    if !guia_docs.exists() {
        println!("Creating {} directory...", guia_docs.display());
        std::fs::create_dir_all(&guia_docs).expect("Failed to create directory in {guia_docs}");
    } else {
        println!("Found Guia's config directory in {}", guia_docs.display());
    }

    println!("Enviroment ready!");
}
