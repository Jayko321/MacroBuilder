pub struct Help {  }

impl Help {
    pub(super) fn execute(&self) -> Result<bool, String> {
        println!("
        Full list of commands: \n
        List \t - \t Lists all macros created with MacroBuilder, also allows enabling and disabling them \n
        Replace \t - \t Creates a macro that replaces your keypress with whatever you specify(e.g(macro replace f+A l) pressed: l replaced with: f+A, i.e you press l but f+A gets pressed) \n\
        Spam \t - \t Spams specified key/keys (e.g(macro spam f mouse4) as long as you hold mouse4 macro will spam f), delay can be specified with --delay=(ms) argument \n\
        Merge \t - \t Allows merging of 2 macros into one (e.g macros macro1 macro2 macro3 will create macro3 from macro1 and macro2) \n");

        return Ok(true);
    }
}