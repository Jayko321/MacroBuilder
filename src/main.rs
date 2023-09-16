mod config;
mod consts;
mod executor;

use config::*;
use consts::*;
use std::{env, process};
use crate::executor::Executor;

#[allow(non_snake_case)]
fn main() {
    let temp_args: Vec<String> = env::args().collect();
    let mut args: Vec<&str> = vec![];
    for i in &temp_args {
        args.push(i);
    }
    dbg!(&args);
    if args.len() < 4 {
        eprintln!("{}", NOT_ENOUGH_ARGUMENTS);
        process::exit(1);
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let executor = Executor::build(&config);
    executor.execute().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}
