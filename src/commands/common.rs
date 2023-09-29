use crate::consts::AHK_HEADER;
use std::{
    fs::{self, File},
    io::Write,
};

pub(super) trait FileCommand {
    fn create_file(&self, filename: &str) -> Result<File, String> {
        fs::create_dir_all("./macros/").unwrap_or_else(|err| eprintln!("{}", err.to_string()));
        let mut file = match File::create(format!("./macros/{}.ahk", filename)) {
            Ok(file_) => file_,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        file.write_all(AHK_HEADER.as_bytes()).unwrap();
        return Ok(file);
    }
}
