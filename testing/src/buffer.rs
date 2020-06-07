use std::io::prelude::*;
use std::io::{self, Write};
use std::io::BufWriter;
use std::fs::File;
use std::fs::metadata;

// Termwriter = placeholder name
pub struct TermWriter<'a>{
    data: &'a [u8],
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
    // Create new TermWriter object and initialize
    pub fn new() -> TermWriter<'a> {
        TermWriter {
            data: b"0",
            writer: Box::new(Vec::new()), //Vec object because it can dynamically work with bytes
        }
    }

    // Return number of bytes stored
    pub fn return_num_bytes(&self) -> usize {
        let len = self.data.len();
        len
    }

    // Writing buffered input to a file
    //pub fn write_to_file(&mut self, buf: &[u8]) -> std::io::Result<()> {}
}
