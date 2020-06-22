//! This is documentation for the `buffer` module.
//!
//! buffer has access to the objects that can store and use the strings provided by the end user.
//!
//! # Examples
//! ```
//! use console_tester::buffer::TestWriter;
//! use std::io::Write;
//!
//! let mut writer = TestWriter::new();
//! writer.set_dirty();
//! writer.write(b"hi");
//! ```

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
use std::io::{self, Write};
use std::fs::File;
use std::fs::metadata;

// TermWriter = placeholder name
pub struct TermWriter<'a> {
    data: &'a[u8], //placeholder field
    writer: Box<dyn Write>, //Box<dyn T> = trait object
}

// 'Write' trait implementation for TermWriter
impl<'a> Write for TermWriter<'a> {
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
impl<'a> TermWriter<'a> {
    // create new TermWriter object
    pub fn new() -> TermWriter<'a> {
        TermWriter {
            data: b"0",
            writer: Box::new(Vec::new()),
        }
    }

    /*pub fn return_num_bytes(&self) -> usize {
        let len = self.data.len();
        len
    }*/

    // write buffered input to a file
    // pub fn write_to_file(&mut self, buf: &[u8]) -> std::io::Result<()> {}
}
