use clap::{Command, Arg};

pub const APP_NAME: &str = "guia";

pub fn build_cli() -> Command {
    Command::new(APP_NAME)
        .bin_name(APP_NAME)
        .subcommand(
            Command::new("add")
            .about("Download a documentation set")
            .arg(
                Arg::new("DOC_NAME")
                .help("The name of the documentation set to download")
                .value_name("DOC_NAME")
                .required(false),
            ),
        )
        .arg(
            Arg::new("documentation")
                .help("Documentation to open")
                .required(false)
                .index(1),
        )
}
