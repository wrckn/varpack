mod tasks;

use tasks::{
    copy_mingw_deps,
    print_hello_world,
    build
};

use std::{
    error::Error
};

use clap::{
    App
};

use string_error::static_err;

pub fn get_app<'a1, 'a2>() -> App<'a1, 'a2> {
    App::new("varpack xtask")
        .about("Project utility scripts")
        .subcommand(build::get_subcommand())
        .subcommand(print_hello_world::get_subcommand())
        .subcommand(copy_mingw_deps::get_subcommand())
}

pub fn run_app() -> Result<(), Box<dyn Error>> {
    let mut app = get_app();
    let app_matches = app.clone().get_matches();
    let subcommand_opt = app_matches.subcommand_name();
    if subcommand_opt.is_none() {
        app.print_help()?;
        return Err(static_err("Please provide a subcommand!"));
    }
    let subcommand_name = subcommand_opt.unwrap();
    match subcommand_name {
        "copy-mingw-deps" => copy_mingw_deps::run_subcommand()?,
        "print-hello-world" => print_hello_world::run_subcommand()?,
        "build" => {
            let subcommand_matches = app_matches.subcommand_matches(subcommand_name)
                .ok_or(static_err("Unknown!"))?;
            let release = subcommand_matches.is_present("release");
            build::run_subcommand(release)?;
        }
        _ => return Err(static_err("Invalid subcommand!"))
    };
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run_app()?;
    Ok(())
}