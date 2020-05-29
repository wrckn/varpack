extern crate vapor_archive as var;

use var::{
    VERSION
};

use std::{
    error::Error
};

use clap::{
    App,
    Arg,
    SubCommand
};
use string_error::{
    new_err
};

fn create_app<'a>() -> App<'a, 'a> {
    App::new("varpack")
        .version(VERSION)
        .author("Daniel Wanner <daniel.wanner@pm.me>")
        .about(".VAR archive CLI utility")
        .subcommand(SubCommand::with_name("unpack")
            .about("Unpack a .var file")
            .arg(Arg::with_name("INPUT_FILE")
                .help("Input .var file to unpack")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("OUTPUT_DIR")
                .help("Directory to write the output into")
                .required(false)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name("pack")
            .about("Create a .var archive")
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = create_app();
    let app_matches = app.clone().get_matches();
    let subcommand = app_matches.subcommand_name()
        .ok_or(new_err("ERROR! No subcommand specified! List available subcommands with --help"))?;
    Ok(())
}
