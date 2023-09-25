use crate::commands::common::FileCommand;
use crate::consts::*;
pub enum Commands<'a> {
    List, //Can enable or disable macros
    Help,
    Spam(&'a Vec<&'a str>, Vec<&'a str>, Vec<&'a str>),
    Replace(&'a Vec<&'a str>),
    Modify(&'a Vec<&'a str>), // ?
    Merge(&'a Vec<&'a str>),
}

impl FileCommand for Commands<'_> {}
pub struct Config<'a> {
    pub command: Commands<'a>,
    pub filename: Option<&'a str>,
}

impl Config<'_> {
    pub fn build<'a>(args: &'a Vec<&'a str>) -> Result<Config<'a>, &'a str> {
        let command = generate_command(&args[1], args);
        let mut filename: Option<&str> = None;
        let instance = match command {
            Ok(command) => Config { command, filename },
            Err(err) => (return Err(err)),
        };
        return Ok(instance);
    }
}
fn generate_command<'a>(
    command_str: &'a str,
    args: &'a Vec<&str>,
) -> Result<Commands<'a>, &'a str> {
    return match command_str {
        "list" => Ok(Commands::List),
        "help" => Ok(Commands::Help),
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
                "replace" => Ok(Commands::Replace(args)),
                "spam" => Ok(Commands::Spam(args, press_keys, bind_keys)),
                "modify" => Ok(Commands::Modify(args)),
                "merge" => Ok(Commands::Merge(args)),
                &_ => return Err("Could not parse command, please type help for list of commands"),
            }
        }
        &_ => return Err("Could not parse command, please type help for list of commands"),
    };
}

fn parse_keys(keys: &str) -> Result<Vec<&str>, &str> {
    let vec_keys: Vec<&str> = keys.split("+").collect();
    for key in &vec_keys {
        if !KEYS
            .iter()
            .any(|&s| &s.to_lowercase() == &key.to_lowercase())
        {
            return Err("Unknown key, see list of keys");
        }
    }

    return Ok(vec_keys);
}
