use std::{fs::File, io::Write, fmt::format};

use crate::consts::NOT_ENOUGH_ARGUMENTS;

use super::common::FileCommand;
pub struct Spam {
    pub args: Vec<String>,
    pub press_keys: Vec<String>,
    pub bind_keys: Vec<String>,
}
impl FileCommand for Spam {}

impl Spam {
    pub(super) fn execute(&self) -> Result<bool, String> {
        if !self.args.len() < 4 {
            return Err(format!("{}", NOT_ENOUGH_ARGUMENTS));
        }
        let mut file: File = match self.create_file(&self.args[4]) {
            Ok(file_) => file_,
            Err(err) => return Err(err)
        };

        let generated_from = String::from("; ") + &self.args[1..self.args.len()].join(" ") + "\n";
        let mut ahk_bind_keys: Vec<String> = vec![];
        self.bind_keys.iter().for_each(|s| ahk_bind_keys.push(format!("~${}::\n", s)));
        file.write_all(generated_from.as_bytes()).unwrap();
        file.write_all((ahk_bind_keys.concat() + "{\n").as_bytes()).unwrap();

        let mut ahk_key_states: Vec<String> = vec![];
        self.bind_keys
            .iter()
            .for_each(|s| ahk_key_states.push(format!("GetKeyState(\"{}\", \"P\")", s)));
        let ahk_press_keys: String = String::from("\tWhile ")
            + ahk_key_states.join(" & ").as_str()
            + " {\n\t\tSend \""
            + &self.press_keys.concat()
            + "\"\n\t\tSleep 25 \n\t}\n} \n";
        file.write_all(ahk_press_keys.as_bytes()).unwrap();

        return Ok(true);
    }
}