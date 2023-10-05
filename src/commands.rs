use self::{spam::Spam, help::Help};

pub mod common;
pub mod help;
pub mod merge;
pub mod replace;
pub mod spam;

pub enum Commands<'a> {
    List, //Can enable or disable macros
    Help(Help),
    Spam(Spam),
    Replace(&'a Vec<&'a str>),
    Modify(&'a Vec<&'a str>), // ?
    Merge(&'a Vec<&'a str>),
}

impl Commands<'_> {
    pub fn execute(&self) -> Result<bool, String> {
        match self {
            Commands::List => {
                return Ok(true);
            }
            Commands::Help(help) => {
                return help.execute();
            }
            Commands::Spam(spam) => {
                return spam.execute();
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
