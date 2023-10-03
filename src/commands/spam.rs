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
                file.write_all(generated_from.as_bytes()).unwrap();
                let mut ahk_bind_keys: String = String::from("");
                for key in bind_keys {
                    ahk_bind_keys += format!("{} & ", key).as_str();
                }
                ahk_bind_keys = ahk_bind_keys[0..ahk_bind_keys.len() - 3].parse().unwrap();
                ahk_bind_keys.push_str("::\n");
                file.write_all(ahk_bind_keys.as_bytes()).unwrap();

                let mut ahk_press_keys: String = "".into();
                ahk_press_keys += "{\nSendInput \"";
                for key in press_keys {
                    ahk_press_keys += key;
                }
                ahk_press_keys.push_str("\"\nSetKeyDelay 25, 25 \n} \nReturn");
                file.write_all(ahk_press_keys.as_bytes()).unwrap();

                return Ok(true);
            }
            &_ => return Err("Wrong command".into()),
        }
    }
}
