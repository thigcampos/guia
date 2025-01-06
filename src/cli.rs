use clap::{Command, Arg};

pub const APP_NAME: &str = "guia";

pub fn build_cli() -> Command {
    Command::new(APP_NAME)
        .bin_name(APP_NAME)
        .arg(
            Arg::new("add")
                .help("Download a documentation set")
                .long("add")
                .value_name("DOC_NAME")
                .required(false),
        )
        .arg(
            Arg::new("documentation")
                .help("Documentation to open")
                .required(false)
                .index(1),
        )
}
