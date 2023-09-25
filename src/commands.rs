use crate::config::Commands;
pub mod common;
pub mod spam;

impl Commands<'_> {
    pub fn execute(&self) -> Result<bool, String> {
        match self {
            Commands::List => {
                return Ok(true);
            }
            Commands::Help => {
                return Ok(true);
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
