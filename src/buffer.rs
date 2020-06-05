//! This is documentation for the `buffer` module.
//! 
//! buffer has access to the objects that can store and use the strings provided by the end user.
//! 
//! # Examples
//! TestWriter buf = asdasd

use std::io::Write;

/// Constructs a new `TestWriter`.
///
/// # Examples
///
/// ```
/// use console_tester::buffer;
///
/// let writer = TestWriter::new();
/// ```
pub struct TestWriter {
    dirty: bool,
    os: OS
}

pub enum OS {
    ///
    /// Enum that stores the OS data associated with the Test
    /// Will look at the data before progressing and used in switch to get terminfo.
    /// 
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
}

impl Write for TestWriter {
    /// Write extension for the Write Trait
    fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
        Ok(12)
    }

    /// Flush extension for the Write Trait
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}