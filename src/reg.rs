//! 'reg' is an internal module meant for assisting in the creation and usage of the regular expression sets.
//!
//! Example:
//! ```
//!
//! ```

use regex::Regex;

// This trait wouldn't actually work
pub trait RegComparator {
    // create pattern will return a Regular Expression
    fn create_pattern() -> &'static Regex;

    // Returns either a collection of matching Strings or a error message specific to this method.
    fn compare_pattern(a: &Regex, b: Vec<Vec<u8>>) -> Result<Vec<String>, &'static str>;
}