use std::fs::{self, File};

pub(super) trait FileCommand {
    fn create_file(&self, filename: &str) -> Result<File, String> {
        fs::create_dir_all("./macros/").unwrap_or_else(|err| eprintln!("{}", err.to_string()));
        let file = match File::create(format!("./macros/{}.ahk", filename)) {
            Ok(file_) => file_,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        return Ok(file);
    }
}
