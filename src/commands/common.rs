use std::{
    collections::HashMap,
    fs::{self, File},
};

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
    //Should be supplied with only additional args, any arg that doesnt start with -- wont be parsed
    fn parse_additional_args<'a>(&'a self, args: &'a Vec<String>) -> HashMap<&'a str, String> {
        let mut additional_args: HashMap<&str, String> = HashMap::new();
        args.into_iter().for_each(|s| {
            if &s[0..2] != "--" {
                return;
            }
            let pos = s.as_str().find("=");
            if pos != None {
                additional_args.insert(&s[2..pos.unwrap()], s[pos.unwrap() + 1..s.len()].into());
            }
        });

        return additional_args;
    }
}
