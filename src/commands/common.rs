use std::{fs::{self, File}, collections::HashMap};

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

pub(super) trait AdditionalArgs {
    //Should be supplied with only additional args, any arg that doesnt start with -- will result in an error
    fn parse_additional_args(&self, args: &Vec<String>) -> Result<HashMap<String, String>, String> {
        let additional_args = HashMap::new();
        if args.iter().any(|s| &s[0..1] == "--") {
            return Err("Too much arguments or additional argument didnt start with --".into())
        }

        return Ok(additional_args);
    } 
}