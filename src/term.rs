//! `term` module for testing buffered items against
//!
//! 'term' will create a object that will take in a 'buffer::TermWriter' and compare it against the current terminal properties.
//!
//! # Internal Example
//! ```
//! use console_tester::term::TermStrings;
//! let term_strings: TermStrings = TermStrings::new();
//! let term_list = term_strings.get_term_list();
//! ```

use std::path::{Path, PathBuf};
use terminfo::capability;
use terminfo::Database;
use terminfo::names;

#[derive(Debug, Clone)]
pub struct TermStrings {
    /// Filtered list of terminal symbols
    string_list: Vec<Vec<u8>>
}

/// TermStrings struct
/// Instantiable, in order to reduce performance overhead by caching the list of strings
/// instead of generating a new list for every check.
/// Populates TermStrings.string_list on instantiation
impl TermStrings {
    pub fn new_from_env() -> TermStrings {
        TermStrings {
            string_list: init_from_env()
        }
    }
    pub fn new_from_path(path: &Path) -> TermStrings {
        TermStrings {
            string_list: init_from_path(&path.to_owned())
        }
    }
    
    /// Check a terminal symbol (in Vec<u8> form) against the list of valid terminal symbols
    pub fn check_valid_symbol(&self, to_compare: Vec<u8>) -> bool { self.string_list.contains(&to_compare) }

    /// Get the stored terminal symbol list
    pub fn get_term_list(self) -> Vec<Vec<u8>> { self.string_list }
}

/// Gets a Vec of u8 vectors, each containing a terminal symbol
/// !Internal Function
///
/// Warning, printing these symbols to the terminal may result in strange side effects
fn init_from_env() -> Vec<Vec<u8>> {

    let res = Database::from_env();

    let info: Database;

    // if the terminal isn't supported essentially
    if res.is_err(){
        println!("This terminal isn't supported by the testing framework");
        return Vec::new();
    }

    // get database object now
    info = res.unwrap();

    let mut strings = Vec::new();

    for n in names::ALIASES.keys() {
        if let Some(val) = info.raw(n) {
            match &val {    // We're only interested in the strings, so filter those out
                capability::Value::String(s) => strings.push(s.to_owned()),
                capability::Value::Number(_) => (),
                capability::Value::True => ()
            }
        }
    }
    return strings;
}

/// Gets a Vec of u8 vectors, each containing a terminal symbol.
/// This method takes a filepath to a terminfo file
/// Warning, printing these symbols to the terminal may result in strange side effects
fn init_from_path(path: &PathBuf) -> Vec<Vec<u8>> {

    let res = Database::from_path(path);

    let info: Database;

    // File not found, or invalid terminfo file
    if res.is_err(){
        println!("This terminal isn't supported by the testing framework");
        return Vec::new();
    }

    // get database object now
    info = res.unwrap();

    let mut strings = Vec::new();

    for n in names::ALIASES.keys() {
        if let Some(val) = info.raw(n) {
            match &val {    // We're only interested in the strings, so filter those out
                capability::Value::String(s) => strings.push(s.to_owned()),
                capability::Value::Number(_) => (),
                capability::Value::True => ()
            }
        }
    }
    return strings;
}

// -------------------- TESTS -----------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Disable for now as these are terminal specific
    #[test]
    #[ignore]
    fn term_struct_not_empty() {
        let t = TermStrings::new_from_env();
        println!("{:?}", t.string_list);
        assert!(!t.get_term_list().is_empty());
    }

    #[test]
    #[ignore]
    fn check_list_for_valid_symbol() {
        let t = TermStrings::new_from_env();
        assert!(t.check_valid_symbol([27, 91, 80].to_vec()));
    }

    #[test]
    #[ignore]
    fn check_list_for_invalid_symbol() {
        let t = TermStrings::new_from_env();
        assert!(!t.check_valid_symbol([27, 27, 27].to_vec()));
    }

    #[test]
    #[ignore]
    fn term_strings_init_not_empty() {
        let strings: Vec<Vec<u8>> = init_from_env();
        assert!(!strings.is_empty());
    }
}
