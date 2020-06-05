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

static COMMAND: &'static str = "x86_64-w64-mingw32-ldd target/x86_64-pc-windows-gnu/debug/varpack-gtk.exe | grep 'lib*' | cut -d \"=\" -f 1 | awk '{$1=$1;print}' | xargs -I{} cp /usr/x86_64-w64-mingw32/bin/{} target/x86_64-pc-windows-gnu/debug/";

pub fn get_subcommand<'a1, 'a2>() -> App<'a1, 'a2> {
    SubCommand::with_name("copy-mingw-deps")
        .about("Copies the needed .dll files for MinGW builds into the target folder.")
        .arg(Arg::with_name("debug")
            .long("debug")
            .required_unless("release")
        )
        .arg(Arg::with_name("release")
            .long("release")
            .required_unless("debug")
        )
}

pub fn run_subcommand() -> Result<(), Box<dyn Error>> {
    Command::new("s")
        .output()?;
    Ok(())
}