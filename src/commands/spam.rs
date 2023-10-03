use std::{fs::File, io::Write};

use crate::consts::NOT_ENOUGH_ARGUMENTS;

use super::{common::FileCommand, Commands};

impl Commands<'_> {
    pub(super) fn spam_execute(&self) -> Result<bool, String> {
        match self {
            Self::Spam(args, press_keys, bind_keys) => {
                if !self.is_enough_args() {
                    return Err(format!("{}", NOT_ENOUGH_ARGUMENTS));
                }
                let mut file: File = match self.create_file(&args[4]) {
                    Ok(file_) => file_,
                    Err(err) => {
                        return Err(err);
                    }
                };

                let generated_from = String::from("; ") + &args[1..args.len()].join(" ") + "\n";
                let ahk_bind_keys: String = String::from(bind_keys.join(" & ")) + ":: {\n";
                file.write_all(generated_from.as_bytes()).unwrap();
                file.write_all(ahk_bind_keys.as_bytes()).unwrap();

                let mut ahk_key_states: Vec<String> = vec![];
                bind_keys 
                    .iter()
                    .for_each(|&s| ahk_key_states.push(format!("GetKeyState(\"{}\", \"P\")", s)));
                let ahk_press_keys: String = String::from("\tWhile ")
                    + ahk_key_states.join(" & ").as_str()
                    + " {\n\t\tSend \""
                    + &press_keys.concat()
                    + "\"\n\t\tSleep 25 \n\t}\n} \n";
                file.write_all(ahk_press_keys.as_bytes()).unwrap();

                return Ok(true);
            }
            &_ => return Err("Wrong command".into()),
        }
    }
}
