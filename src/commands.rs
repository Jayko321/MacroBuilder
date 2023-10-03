use self::common::FileCommand;

pub mod common;
pub mod help;
pub mod merge;
pub mod replace;
pub mod spam;

pub enum Commands<'a> {
    List, //Can enable or disable macros
    Help,
    Spam(&'a Vec<&'a str>, Vec<&'a str>, Vec<&'a str>),
    Replace(&'a Vec<&'a str>),
    Modify(&'a Vec<&'a str>), // ?
    Merge(&'a Vec<&'a str>),
}

impl FileCommand for Commands<'_> {
    fn is_enough_args(&self) -> bool {
        match self {
            Commands::Help => return true,
            Commands::Spam(args, _, _) => return args.len() > 4,
            Commands::Replace(args) => return args.len() > 4,
            Commands::Modify(args) => return args.len() > 4,
            Commands::Merge(args) => return args.len() > 4,
            Commands::List => return true,
        }
    }
}

impl Commands<'_> {
    pub fn execute(&self) -> Result<bool, String> {
        match self {
            Commands::List => {
                return Ok(true);
            }
            Commands::Help => {
                return self.help_execute();
            }
            Commands::Spam(_, _, _) => {
                return self.spam_execute();
            }
            Commands::Replace(_) => {
                return Ok(true);
            }
            Commands::Modify(_) => {
                return Ok(true);
            }
            Commands::Merge(_) => {
                return Ok(true);
            }
        }
    }
}
