//! 'reg' is an internal module meant for assisting in the creation and usage of the regular expression sets.
//!
//! Example:
//! ```
//!
//! ```

use regex::{Regex, Error};
//use regex::Regex;
use std::str;
use crate::buffer::TermWriter;
use std::io::Write;

/// For creating the regex associated with a TermWriter
/// Not to be used right now but just in case we want to serialize an object of termcap that makes sense.
pub fn create(_: Vec<Vec<u8>>) -> Result<Regex, Error> {
    return Regex::new("sample");
}

/// Compare will parse `TermWriter` by the supplied `Vec<Vec<u8>>` item list and give you back a Result of bool or &'static str
pub fn compare(_tw: TermWriter, _source: Vec<Vec<u8>>) -> Result<bool, &'static str> {
    //TODO: get info from TermWriter and convert to &str
	
	//This line is currently a dummy line
	let user_str = "o great string of testing,\n lend us your matches";
	
	
	Ok(check_bad_escapes(remove_valid_escapes(_source, user_str).as_str()))
    //return Err("bad");
    //Ok(false)
}

/// Parses user input to remove valid escapes. Escapes are sorted largest first to avoid possible edge cases
fn remove_valid_escapes(escapes: Vec<Vec<u8>>, user_string: &str) -> String
{
	let mut result = String::from(user_string);
	let mut v = escapes.to_vec();
	v.sort_by(|a, b| b.len().cmp(&a.len()));
	for s in v.iter() {
		let re = Regex::new(str::from_utf8(&s).unwrap()).unwrap();
		result = re.replace_all(&result, "").to_string();
	}
	result
}

/// Meant to be used after valid escapes are removed. Checks for remaining possible escape sequences
/// https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_control
/// TODO: very likely fails in a number of edge cases where escape sequence exists with no control characters.
/// Is this something to worry about?
/// Example of possible escape sequence with no control characters: ``aaffggiijjkkllmmnnooppqqrrssttuuvvwwxxyyzz{{||}}~~
fn check_bad_escapes(user_string: &str) -> bool
{
	for c in user_string.chars() {
		if !c.is_ascii_control() {continue;}
		return false;
	}
	true
}

// 'cargo test'
#[cfg(test)]
mod tests {
	use super::*;
	
	/// Tests removing valid escapes from a string
	#[test]
	fn remove_valid_test() {
		let test_data = vec![vec![103,114,101,97,116], vec![116,101,115,116], vec![107,107], vec![108,101,110,100], vec![10]];
		let test_str = "o great string of testing,\n lend us your matches";
		
		assert_eq!("o  string of ing,  us your matches", remove_valid_escapes(test_data, test_str));
	}
	
	/// Tests checking if bad escapes exist.
	#[test]
	fn bad_escapes_test() {
		let test_data = vec![vec![10]];
		let test_str = "o great string of testing,\n lend us your matches";
		
		assert_eq!(false, check_bad_escapes(test_str));
		assert_eq!(true, check_bad_escapes(remove_valid_escapes(test_data, test_str).as_str()));
	}
	
	/// Currently not working. Need more info on how to extract data from termwriter
	#[test]
	fn compare_test() {
		//let mut buffer = TermWriter::new();
        //let _ = buffer.write(b"o great string of testing,\n lend us your matches");
		let test_data_good = vec![vec![10]];
		let test_data_bad = vec![vec![0]];
		
		assert_eq!(true, compare(TermWriter::new(), test_data_good).unwrap());
		assert_eq!(false, compare(TermWriter::new(), test_data_bad).unwrap());
	}
}
