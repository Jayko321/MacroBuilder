use crate::{
    commands::Commands::{self},
    consts::*,
};

pub struct CommandBuilder<'a> {
    pub command: Commands<'a>,
}

impl CommandBuilder<'_> {
    pub fn build(args: &Vec<String>) -> Result<Commands, String> {
        if args.len() <= 1 {
            return Err(NOT_ENOUGH_ARGUMENTS.into());
        }

        match generate_command(&args[1], &args) {
            Ok(command) => return Ok(command),
            Err(err) => return Err(err),
        };
    }
}
fn generate_command<'a>(
    command_str: &'a str,
    args: &'a Vec<String>,
) -> Result<Commands<'a>, String> {
    return match command_str {
        "list" => Ok(Commands::List),
        "help" => Ok(Commands::Help(crate::commands::help::Help {})),
        "replace" | "spam" | "modify" | "merge" => {
            let mut res = parse_keys(&args[2]);
            let press_keys = match res {
                Ok(vec_keys) => vec_keys,
                Err(err) => return Err(err),
            };
            res = parse_keys(&args[3]);
            let bind_keys = match res {
                Ok(vec_keys) => vec_keys,
                Err(err) => return Err(err),
            };
            match command_str {
                //"replace" => Ok(Commands::Replace(args)),
                "spam" => Ok(Commands::Spam(crate::commands::spam::Spam {
                    args: args.to_vec(),
                    press_keys,
                    bind_keys,
                })),
                //"modify" => Ok(Commands::Modify(args)),
                //"merge" => Ok(Commands::Merge(args)),
                &_ => {
                    return Err(
                        "Could not parse command, please type help for list of commands".into(),
                    )
                }
            }
        }
        &_ => return Err("Could not parse command, please type help for list of commands".into()),
    };
}

fn parse_keys(keys: &str) -> Result<Vec<String>, String> {
    let vec_keys: Vec<&str> = keys.split("+").collect();
    for key in &vec_keys {
        if !KEYS
            .iter()
            .any(|&s| &s.to_lowercase() == &key.to_lowercase())
        {
            return Err("Unknown key, see list of keys".into());
        }
    }

    return Ok(vec_keys.iter().map(|&s| String::from(s)).collect());
}
