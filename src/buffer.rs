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
        let compare_result = reg::compare(self.writer, _t.get_term_list());

        if compare_result == Ok(true) {
            assert_eq!(true, compare_result.unwrap());
            return Ok(true);
        } else if compare_result.is_err() {
            return Err(&"Matching expression not found in TermStrings");
        } else {
            return Ok(false);
        }
    }

    // TODO: write buffered input to a file (can be implemented later if needed)
    // pub fn write_to_file(&mut self, buf: &[u8]) -> std::io::Result<()> {}
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
    #[ignore]
    fn termwriter_write() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let bytes_written = buffer.write(bytes_literal);

        assert_eq!(bytes_written.unwrap(), 14);
    }

    #[test]
    #[ignore]
    fn termwriter_write_all() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();

        assert_eq!((), buffer.write_all(bytes_literal).unwrap());
    }

    #[test]
    #[ignore]
    fn termwriter_flush() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let _ = buffer.write(bytes_literal);

        assert_eq!((), buffer.flush().unwrap())
    }

	/// Currently fails due to TermStrings not supporting some terminals.
	#[test]
	fn termwriter_compare() {
		let mut buffer = TermWriter::new();
		let _ = buffer.write(b"Text with\nTwo lines");
		assert_eq!(Ok(true), buffer.compare(TermStrings::new_from_env()));
	}
}
