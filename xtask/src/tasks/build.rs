use std::{
    process::{
        Command
    },
    error::Error
};

use clap::{
    SubCommand,
    Arg,
    App
};

pub fn get_subcommand<'a1, 'a2>() -> App<'a1, 'a2> {
    SubCommand::with_name("build")
        .about("Builds the entire project, in release or debug config.")
        .arg(Arg::with_name("debug")
            .long("debug")
            .required_unless("release")
        )
        .arg(Arg::with_name("release")
            .long("release")
            .required_unless("debug")
        )
}

pub fn run_subcommand(release: bool) -> Result<(), Box<dyn Error>> {
    if release {
        Command::new("cargo")
            .args(&["build", "--all", "--release"])
            .status()?;
    } else {
        Command::new("cargo")
            .args(&["build", "--all"])
            .status()?;
    }
    Ok(())
}