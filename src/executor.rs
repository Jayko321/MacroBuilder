use crate::config::{Commands, Config};
use crate::consts::AHK_HEADER;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct Executor<'a> {
    config: &'a Config<'a>,
}

impl Executor<'_> {
    pub fn build<'a>(config: &'a Config) -> Executor<'a> {
        return Executor { config };
    }

    pub fn execute(&self) -> Result<(), &'static str> {
        return match &self.config.command {
            Commands::List => Ok(()),
            Commands::Help => {
                println!("Full list of commands: \n
                List \t - \t Lists all macros created with MacroBuilder, also allows enabling and disabling them \n
                Replace \t - \t Creates a macro that replaces your keypress with whatever you specify(e.g(macro replace f+A l) pressed: l replaced with: f+A, i.e you press l but f+A gets pressed) \n\
                Spam \t - \t Spams specified key/keys (e.g(macro spam f mouse4) as long as you hold mouse4 macro will spam f), delay can be specified with --delay=(ms) argument \n\
                Merge \t - \t Allows merging of 2 macros into one (e.g macros macro1 macro2 macro3 will create macro3 from macro1 and macro2) \n");
                Ok(())
            }
            //Modify \t - \t Modifies macro allowing you to change macro command, keys to press, bound key, etc. (e.g macro modify name1
            Commands::Replace(args) => Ok(()),
            Commands::Merge(args) => Ok(()),
            Commands::Spam(args, press_keys, bind_keys) => {
                //todo: check if filename is none
                fs::create_dir_all("./macros/")
                    .unwrap_or_else(|err| eprintln!("{}", err.to_string()));
                let mut file =
                    File::create(format!("./macros/{}.ahk", self.config.filename.unwrap()))
                        .unwrap();
                file.write_all(AHK_HEADER.as_bytes()).unwrap();

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

                Ok(())
            }
            Commands::Modify(args) => Ok(()),
        };
    }
}
