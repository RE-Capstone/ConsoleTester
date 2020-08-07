//! `term` module for testing buffered items against
//!
//! 'term' will create a object that will take in a 'buffer::TermWriter' and compare it against the current terminal properties.
//!
//! # Internal Example
//! ```
//! use console_tester::term::TermStrings;
//! let term_strings: TermStrings = TermStrings::new_from_env();
//! let term_list = term_strings.get_term_list();
//! ```

use std::path::{Path, PathBuf};
use terminfo::capability;
use terminfo::names;
use terminfo::Database;

#[derive(Debug, Clone)]
pub struct TermStrings {
    /// Filtered list of terminal symbols
    string_list: Vec<Vec<u8>>,
}

/// TermStrings struct
/// Instantiable, in order to reduce performance overhead by caching the list of strings
/// instead of generating a new list for every check.
/// Populates TermStrings.string_list on instantiation
impl TermStrings {
    pub fn new(string_list: Option<Vec<Vec<u8>>>) -> TermStrings {
        match string_list {
            None => TermStrings {
                string_list: Vec::new(),
            },
            Some(string_list) => TermStrings {
                string_list: string_list,
            },
        }
    }
    pub fn new_from_env() -> TermStrings {
        TermStrings::new(init_from_env())
    }
    pub fn new_from_path(path: &Path) -> TermStrings {
        TermStrings::new(init_from_path(&path.to_owned()))
    }

    /// Check a terminal symbol (in Vec<u8> form) against the list of valid terminal symbols
    pub fn check_valid_symbol(&self, to_compare: Vec<u8>) -> bool {
        self.string_list.contains(&to_compare)
    }

    /// Get the stored terminal symbol list
    pub fn get_term_list(self) -> Vec<Vec<u8>> {
        self.string_list
    }
}

/// Gets a Vec of u8 vectors, each containing a terminal symbol
/// !Internal Function
///
/// Warning, printing these symbols to the terminal may result in strange side effects
fn init_from_env() -> Option<Vec<Vec<u8>>> {
    let res = Database::from_env();

    let info: Database;

    // if the terminal isn't supported essentially
    if res.is_err() {
        println!("This terminal isn't supported by the testing framework");
        return None;
    }

    // get database object now
    info = res.unwrap();

    let mut strings = Vec::new();

    for n in names::ALIASES.keys() {
        if let Some(val) = info.raw(n) {
            match &val {
                // We're only interested in the strings, so filter those out
                capability::Value::String(s) => strings.push(s.to_owned()),
                capability::Value::Number(_) => (),
                capability::Value::True => (),
            }
        }
    }
    return Some(strings);
}

/// Gets a Vec of u8 vectors, each containing a terminal symbol.
/// This method takes a filepath to a terminfo file
/// Warning, printing these symbols to the terminal may result in strange side effects
fn init_from_path(path: &PathBuf) -> Option<Vec<Vec<u8>>> {
    let res = Database::from_path(path);

    let info: Database;

    // File not found, or invalid terminfo file
    if res.is_err() {
        println!("This terminal isn't supported by the testing framework");
        return None;
    }

    // get database object now
    info = res.unwrap();

    let mut strings = Vec::new();

    for n in names::ALIASES.keys() {
        if let Some(val) = info.raw(n) {
            match &val {
                // We're only interested in the strings, so filter those out
                capability::Value::String(s) => strings.push(s.to_owned()),
                capability::Value::Number(_) => (),
                capability::Value::True => (),
            }
        }
    }
    return Some(strings);
}

// -------------------- TESTS -----------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn term_struct_not_empty_from_terminfo_file() {
        let path = Path::new("./terminfo_files/x/xterm");
        let t = TermStrings::new_from_path(path);
        // println!("{:?}", t.string_list);
        assert!(!t.get_term_list().is_empty());
    }

    #[test]
    fn check_list_for_valid_symbol() {
        let path = Path::new("./terminfo_files/x/xterm");
        let t = TermStrings::new_from_path(path);
        assert!(t.check_valid_symbol([27, 91, 80].to_vec()));
    }

    #[test]
    fn check_list_for_invalid_symbol() {
        let t = TermStrings::new_from_env();
        assert!(!t.check_valid_symbol([27, 27, 27].to_vec()));
    }

    #[test]
    #[ignore]
    fn term_strings_init_from_env_not_empty() {
        let strings = init_from_env().unwrap();
        assert!(!strings.is_empty());
    }

    #[test]
    fn generate_termstrings_from_existing_list() {
        let path = Path::new("./terminfo_files/x/xterm");
        let t1 = TermStrings::new_from_path(path);
        let t2 = TermStrings::new(Some(t1.get_term_list()));
        assert!(!t2.get_term_list().is_empty());
    }

}
