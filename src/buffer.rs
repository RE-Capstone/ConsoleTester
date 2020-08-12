//! This is documentation for the `buffer` module.
//!
//! buffer has access to the objects that can store and use the strings provided by the end user.
//!
//! # Examples
//! ```
//! use console_tester::buffer::TermWriter;
//! use std::io::Write;
//! let mut term_writer: TermWriter = TermWriter::new();
//! term_writer.write(b"Hello");
//! term_writer.flush();
//! ```

use crate::reg::{
    compare,
    ErrorList::{EmptyVec, UncappedEscape},
};
use crate::term::TermStrings;
use std::fmt::Debug;
use std::io::Write;

/// TermWriter Object that holds character array buffer
#[derive(Debug, Clone)]
pub struct TermWriter {
    writer: Vec<u8>,
}

/// 'Write' trait implementation for TermWriter to add to the ' Vec<u8>'
impl Write for TermWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// TermWriter implementation
impl TermWriter {
    // create new TermWriter object
    pub fn new() -> TermWriter {
        TermWriter { writer: Vec::new() }
    }

    // Should be used with an assert to check if the unwrap is equal to true
    // Wrapper for reg.rs that will format errors
    pub fn compare(self, _t: TermStrings) -> Result<bool, &'static str> {
        let compare_result = compare(self.writer, _t.get_term_list());

        match compare_result {
            Ok(true) => return Ok(true),
            Err(EmptyVec) => return Err(&"Provided terminal escape sequences were empty."),
            Err(UncappedEscape(x)) => {
                let invalid_str: String = x;
                error_print(invalid_str);
                return Err(&"Potential unrecognized escape sequences were found")
            }
            _ => {
                return Err(&"Unknown error occurred")
            }
        };
    }

    // TODO: take in a file name and buffered input as arguments
    // Open a given file in write-only mode and attempt to write to it
    // If write to file is successful, write into TermWriter
    /*pub fn write_to_file(&mut self, file_name: &str, buf: &[u8]) -> Result<bool, &'static str> {
        let mut success = false;

        let mut file = match File::create(&file_name) {
            Ok(file) => file,
            Err(_file) => return Err("Failed writing to the supplied file"),
        };

        file.write(buf);

        let mut buffer = TermWriter::new();
        let _bytes_written = match buffer.write(buf) {
            Ok(_) => success = true,
            Err(_) => println!("Failed writing to TermWriter"),
        };
        return Ok(success);
    }*/
}

// Pretty print function for unrecognized escape sequences
// found within an input
pub fn error_print(invalid_str: String) {
    let result = invalid_str;

    println!("\x1b[0;31m------------------ Console Failure ------------------\n\n");

    println!(" Possible unrecognized escape sequence(s) in the input.\n Shown in brackets:\n{:?}\n\n", result);
    println!("-----------------------------------------------------\x1b[0m");
}


// 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_string_bytes() {
        let vec1 = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
        let joined_vec1 = vec1.join(" ");
        let vec1_bytes = joined_vec1.as_bytes();

        let bytes_literal = b"Some junk text";

        assert_eq!(vec1_bytes, bytes_literal);
    }

    #[test]
    fn termwriter_write() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let bytes_written = match buffer.write(bytes_literal) {
            Ok(bytes_written) => assert_eq!(bytes_written, 14),
            Err(e) => println!("Failed writing to TermWriter object"),
        };
    }

    #[test]
    fn termwriter_write_all() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();

        assert_eq!((), buffer.write_all(bytes_literal).unwrap());
    }

    #[test]
    fn termwriter_flush() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let _ = buffer.write(bytes_literal);

        assert_eq!((), buffer.flush().unwrap())
    }

    #[test]
    fn termwriter_compare() {
        let mut buffer = TermWriter::new();
        let _ = buffer.write(b"Text with\nTwo lines");
        let mut result = buffer.compare(TermStrings::new_from_env());

        if result.is_err() {
            assert_eq!(
                Err("Provided terminal escape sequences were empty."),
                result,
            );
        } else {
            assert_eq!(Ok(true), result);
        }
    }
}
