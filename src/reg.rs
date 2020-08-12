//! 'reg' is an internal module meant for assisting in the creation and usage of the regular expression sets.
//!
//! Example:
//! ```
//!
//! ```

use crate::reg::ErrorList::EmptyVec;
use crate::reg::ErrorList::UncappedEscape;
use regex::{Error, Regex};
use std::str;

const CONTROLMAP: [&'static str; 33] = [
    "[NUL]", "[SOH]", "[STX]", "[ETX]", "[EOT]", "[ENQ]", "[ACK]", "[BEL]", "[BS]", "[HT]", "[LF]",
    "[VT]", "[FF]", "[CR]", "[SO]", "[SI]", "[DLE]", "[DC1]", "[DC2]", "[DC3]", "[DC4]", "[NAK]",
    "[SYN]", "[ETB]", "[CAN]", "[EM]", "[SUB]", "[ESC]", "[FS]", "[GS]", "[RS]", "[US]", "[DEL]",
];

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorList {
    EmptyVec,
    UncappedEscape(String),
}

/// For creating the regex associated with a TermWriter
/// Not to be used right now but just in case we want to serialize an object of termcap that makes sense.
#[allow(dead_code)]
pub fn create(_: Vec<Vec<u8>>) -> Result<Regex, Error> {
    Regex::new("sample")
}

/// Compare will parse `TermWriter` by the supplied `Vec<Vec<u8>>` item list and give you back a Result of bool or &'static str
pub fn compare(tw: Vec<u8>, source: Vec<Vec<u8>>) -> Result<bool, ErrorList> {
    if source.len() == 0 {
        return Err(EmptyVec);
    }

    let user_str = match str::from_utf8(&tw) {
        Err(_) => {
            // TODO temp fix, how should we handle these ??? - IG
            return Ok(false);
        }
        Ok(s) => s,
    };

    highlight_control_chars(remove_valid_escapes(source, user_str).as_str())
}

/// Parses user input to remove valid escapes. Escapes are sorted largest first to avoid possible edge cases
fn remove_valid_escapes(escapes: Vec<Vec<u8>>, user_string: &str) -> String {
    let mut result = String::from(user_string);
    let mut v = escapes.to_vec();
    let mut re_str: &str;
    let mut re: Regex;
    v.sort_by(|a, b| b.len().cmp(&a.len()));
    for s in v.iter() {
        // TODO ok to skip erroneous loops ??? - IG
        re_str = match str::from_utf8(&s) {
            Err(_) => continue,
            Ok(s) => s,
        };
        re = match Regex::new(re_str) {
            Err(_) => continue,
            Ok(s) => s,
        };
        result = re.replace_all(&result, "").to_string();
    }
    result
}

/// Meant to be used after valid escapes are removed. Checks for remaining possible escape sequences.
/// Returns either success, or failure including the failed string with invalid escape sequences converted to text.
/// https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_control
/// TODO: very likely fails in a number of edge cases where escape sequence exists with no control characters.
/// Example of possible escape sequence with no control characters: ``aaffggiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz{{||}}~~
fn highlight_control_chars(user_string: &str) -> Result<bool, ErrorList> {
    let mut err_string = String::new();
    let mut no_error = true;
    for ch in user_string.chars() {
        if !ch.is_ascii_control() {
            err_string.push(ch);
            continue;
        }
        no_error = false;
        let ch_as_u8: u8 = ch as u8;
        let mut cmap_index = usize::from(ch_as_u8);
        cmap_index = match cmap_index {
            127 => 33,
            _ => cmap_index,
        };
        err_string.push_str(CONTROLMAP[cmap_index]);
    }
    // REMEMBER: uncomment out the 'Ok(true)' !!!
    if no_error {
        Ok(true)
    } else {
        Err(UncappedEscape(err_string))
    }
}

// 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;
    /// Tests removing valid escapes from a string
    #[test]
    fn remove_valid_test() {
        let test_data = vec![
            vec![103, 114, 101, 97, 116],
            vec![116, 101, 115, 116],
            vec![107, 107],
            vec![108, 101, 110, 100],
            vec![10],
        ];
        let test_str = "o great string of testing,\n lend us your matches";

        assert_eq!(
            "o  string of ing,  us your matches",
            remove_valid_escapes(test_data, test_str)
        );
    }

    // Tests that compare will properly fail or succeed.
    #[test]
    fn compare_test() {
        assert_eq!(Ok(true), compare(vec![108, 127, 108], vec![vec![127]]));
        assert_eq!(
            Err(UncappedEscape("l[LF]l".to_string())),
            compare(vec![108, 10, 108], vec![vec![0]])
        );
    }

    /// Tests that uncaptured escape sequences are properly detected and highlighted
    #[test]
    fn map_bad_test() {
        let test_str1 = "test string goes brrrr";
        let test_str2 = "test string goes\nbrrrrrr";

        assert_eq!(Ok(true), highlight_control_chars(test_str1));
        assert_eq!(
            Err(UncappedEscape("test string goes[LF]brrrrrr".to_string())),
            highlight_control_chars(test_str2)
        );
    }
}
