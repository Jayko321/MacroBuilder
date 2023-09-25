mod commands;
#[allow(non_snake_case)]
mod config;
mod consts;
mod executor;

use crate::executor::Executor;
use config::*;
use consts::*;
use std::{env, process};

fn main() {
    let temp_args: Vec<String> = env::args().collect();
    let mut args: Vec<&str> = vec![];
    for i in &temp_args {
        args.push(i);
    }
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    config.command.execute();

    //    let executor = Executor::build(&config);
    //    executor.execute().unwrap_or_else(|err| {
    //        eprintln!("Problem parsing arguments: {err}");
    //        process::exit(1);
    //    });
}
