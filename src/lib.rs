//! This is documentation for the `console_tester` crate.
//!
//! Console Tester will get fed in strings and will parse the strings and return a result in what makes sense.

#![crate_type = "lib"]
#![crate_name = "console_tester"]

use terminfo::*;

pub mod buffer;

pub fn main (){
    //! Example Function
    //! Returns nothing
}


struct TermStrings {
    /// Filtered list of terminal symbols
    stringlist: Vec<Vec<u8>>
}

/// Gets a Vec of u8 vectors, each containing a terminal symbol
///
/// Example: 
///   let strings: Vec<Vec<u8>> = get_term_strings();
///   assert!(!strings.is_empty());
///
/// Warning, printing these symbols to the terminal may result in strange side effects
fn term_strings_init() -> Vec<Vec<u8>> {
    let info = Database::from_env().unwrap().to_owned();
    let mut strings = Vec::new();
    for n in terminfo::names::ALIASES.keys() {
        if let Some(val) = info.raw(n) {
            match &val {    // We're only interested in the strings, so filter those out
                capability::Value::String(s) => strings.push(s.to_owned()),
                capability::Value::Number(_) => (),
                capability::Value::True => ()
            }
        }
    }
    strings
}

/// TermStrings struct
/// Instantiable, in order to reduce performance overhead by caching the list of strings
/// instead of generating a new list for every check.
/// Populates TermStrings.stringlist on instantiation
///
/// Example:
///   let ts = TermStrings::new();
///   let termlist = ts.get_term_list();
///   assert!(!termlist.is_empty());
impl TermStrings {
    pub fn new() -> TermStrings {
        TermStrings {
            stringlist: term_strings_init()
        }
    }
    
    /// Check a terminal symbol (in Vec<u8> form) against the list of valid terminal symbols
    pub fn check_valid_symbol(&self, tocompare: Vec<u8>) -> bool { self.stringlist.contains(&tocompare) }

    /// Get the stored terminal symbol list
    pub fn get_term_list(self) -> Vec<Vec<u8>> { self.stringlist }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_term_struct_not_empty() {
        let t = TermStrings::new();
        assert!(!t.get_term_list().is_empty());
    }

    #[test]
    fn check_list_for_valid_symbol() {
        let t = TermStrings::new();
        assert!(t.check_valid_symbol([27, 91, 80].to_vec()));
    }

    #[test]
    fn check_list_for_invalid_symbol() {
        let t = TermStrings::new();
        assert!(!t.check_valid_symbol([27, 27, 27].to_vec()));
    }

    #[test]
    fn term_strings_init_not_empty() {
        let strings: Vec<Vec<u8>> = term_strings_init();
        assert!(!strings.is_empty());
    }
}

