//! 'reg' is an internal module meant for assisting in the creation and usage of the regular expression sets.
//!
//! Example:
//! ```
//!
//! ```

use regex::{Regex, Error};
use crate::buffer::TermWriter;

/// For creating the regex associated with a TermWriter
/// Not to be used right now but just in case we want to serialize an object of termcap that makes sense.
pub fn create(_: Vec<Vec<u8>>) -> Result<Regex, Error> {
    return Regex::new("sample");
}

/// Compare will parse `TermWriter` by the supplied `Vec<Vec<u8>>` item list and give you back a Result of bool or &'static str
pub fn compare(_tw: TermWriter, _source: Vec<Vec<u8>>) -> Result<bool, &'static str> {
    Ok(true)
    //return Err("bad");
    //Ok(false)
}
