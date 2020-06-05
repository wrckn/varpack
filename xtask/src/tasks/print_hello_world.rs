use std::{
    process::{
        Command
    },
    error::Error
};

use clap::{
    App,
    SubCommand
};

pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("print-hello-world")
}

pub fn run_subcommand() -> Result<(), Box<dyn Error>> {
    Command::new("echo")
        .arg("\"Hello, world!\"")
        .status()?;
    Ok(())
}