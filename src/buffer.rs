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

use std::io::Write;

/// TermWriter Object that holds character array buffer
/// TODO: Implement std::fmt::Debug
pub struct TermWriter {
    //data: &'a[u8],
    writer: Box<dyn Write>, //Box<dyn T> = trait object
}

/// 'Write' trait implementation for TermWriter
impl Write for TermWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(buf)
    }
}

/// TermWriter implementation
impl TermWriter {
    // create new TermWriter object
    pub fn new() -> TermWriter {
        TermWriter {
            writer: Box::new(Vec::new()),
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
        let bytes_written = buffer.write(bytes_literal);

        assert_eq!(bytes_written.unwrap(), 14);
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
}
