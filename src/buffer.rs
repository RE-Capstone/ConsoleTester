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

//use std::mem::size_of_val;
use std::fs::File;
use regex::Regex;
use std::io::Write;
use std::fmt::Debug;
use crate::term::TermStrings;
use crate::reg;

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
        TermWriter {
            writer: Vec::new(),
        }
    }

    // TODO: Implement this using default functions in reg module
    // Should be used with an assert to check if the unwrap is equal to true
    // Wrapper for reg.rs that will format errors
    pub fn compare(self, _t: TermStrings) -> Result<bool, &'static str> {
        let compare_result = reg::compare(self, _t.get_term_list());

        if compare_result == Ok(true) {
            assert_eq!(true, compare_result.unwrap());
            return Ok(true);
        } else if compare_result.is_err() {
            return Err(&"Matching expression not found in TermStrings");
        } else {
            return Ok(false);
        }
    }

    // TODO: take in a file name and buffered input as arguments
    // Open a given file in write-only mode
    // Write to new file first, and then write into TermWriter
    pub fn write_to_file(&mut self, file_name: &str, buf: &[u8]) -> bool {
        let success: bool;

        // Open the given file in write-only mode
        let mut file = match File::create(&file_name) {
            Ok(_) => file.write(buf), //NEED TO FIX THIS
            Err(_) => println!("Failed writing to file: {}", &file_name),
        };

        // Write into TermWriter
        let mut buffer = TermWriter::new();
        let _bytes_written = match buffer.write(buf) {
            Ok(_bytes_written) => success = true,
            Err(_) => println!("Failed writing to TermWriter object"),
        };

        return success;
    }
}

// 'cargo test'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
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
    #[ignore]
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

}
