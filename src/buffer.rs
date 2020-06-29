//! This is documentation for the `buffer` module.
//!
//! buffer has access to the objects that can store and use the strings provided by the end user.
//!
//! # Examples
//! ```
//! use console_tester::buffer::TermWriter;
//! use std::io::Write;
//!
//! ```

// G.Ku: (deleted code for new TestWriter, set_dirty call, and write) to fix test error

/*use std::io::Write;

/// Constructs a new `TestWriter`.
pub struct TestWriter {
    dirty: bool,
    os: OS
}

/// Enum that stores the OS data associated with the Test
/// Will look at the data before progressing and used in switch to get terminfo.
pub enum OS {
    WIN,
    OSX,
    LINUX
}

impl TestWriter {
    /// Constructor that can take in parameters regarding buffer size and possibly add a dirty bit.
    pub fn new() -> TestWriter {
        TestWriter{
            dirty:  false,
            os:     OS::LINUX
        }
    }

    /// Sets the dirty Flag of the TestWriter Object
    pub fn set_dirty(&mut self) {
        self.dirty = !self.dirty;
    }

    /// Sets the OS Property of the TestWriter Object
    pub fn set_os(&mut self) {
        self.os = OS::LINUX; // TODO: Have code that determines users OS
    }
}

impl Write for TestWriter {
    /// Write extension for the Write Trait
    /// TODO: Implement this correctly
    fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
        Ok(12)
    }

    /// Flush extension for the Write Trait
    /// TODO: Implement this correctly
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}*/
use std::io::prelude::*;
use std::io::Write;
use std::fs::File;
use std::fs::metadata;

// TermWriter = placeholder name
pub struct TermWriter {
    //data: &'a[u8],
    writer: Box<dyn Write>, //Box<dyn T> = trait object
}

// 'Write' trait implementation for TermWriter
impl Write for TermWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

// TermWriter implementation
impl TermWriter {
    // create new TermWriter object
    pub fn new() -> TermWriter {
        TermWriter {
            writer: Box::new(Vec::new()),
        }
    }

    // write buffered input to a file (can be implemented later if needed)
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
        let bytes_written = buffer.write(bytes_literal);

        assert_eq!((), buffer.flush().unwrap())
    }
}
