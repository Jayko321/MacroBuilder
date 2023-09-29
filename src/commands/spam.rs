use std::{fs::File, io::Write};

use super::{common::FileCommand, Commands};

impl Commands<'_> {
    pub(super) fn spam_execute(&self) -> Result<bool, String> {
        match self {
            Self::Spam(args, press_keys, bind_keys) => {
                let mut file: File = match self.create_file(&args[4]) {
                    Ok(file_) => file_,
                    Err(err) => {
                        return Err(err);
                    }
                };

                let mut ahk_bind_keys: String = String::from("");
                let mut ahk_key_states: String = String::from("");
                for key in bind_keys {
                    ahk_bind_keys += format!("{} & ", key).as_str();
                    ahk_key_states += format!("GetKeyState(\"{}\", \"P\") & ", key).as_str();
                }
                ahk_bind_keys = ahk_bind_keys[0..ahk_bind_keys.len() - 3].parse().unwrap();
                ahk_key_states = ahk_key_states[0..ahk_key_states.len() - 3].parse().unwrap();
                ahk_bind_keys.push_str("::\n");

                file.write_all(ahk_bind_keys.as_bytes()).unwrap();

                let mut ahk_press_keys: String = format!("While {}\n", ahk_key_states);
                ahk_press_keys += "{\n";
                for key in press_keys {
                    ahk_press_keys += format!("Send, {}\n", key).as_str();
                }
                ahk_press_keys.push_str("Sleep,10 \n} \nReturn");

                file.write_all(ahk_press_keys.as_bytes()).unwrap();

                return Ok(true);
            }
            &_ => return Err("Wrong command".into()),
        }
    }
}
