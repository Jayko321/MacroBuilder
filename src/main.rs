#[allow(non_snake_case)]
mod commands;
mod config;
mod consts;

use config::*;
use std::{env, process};

fn main() {
    let temp_args: Vec<String> = env::args().collect();
    let mut args: Vec<&str> = vec![];
    for i in &temp_args {
        args.push(i);
    }
    dbg!(&args);

    let command = CommandBuilder::build(&temp_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    match command.execute() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Problem accured while executing command: \n {}", err)
        }
    };
}
