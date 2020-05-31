// Test code for lib.rs / library file
use std::io::prelude::*;
use std::io::Write;
use std::io::BufWriter;
use std::fs::File;
use std::fs::metadata;

// Struct that will implement the Write trait
pub struct TermWriter<'a> {
    pub data: &'a [u8],
}

// Implementation
impl<'a> TermWriter<'a> {
    // Instantiate a new TermWriter & pass in some bytes
    pub fn new(buf: &[u8]) -> TermWriter {
        TermWriter {
            data: buf,
        }
    }

    // Return number of bytes stored
    pub fn return_num_bytes(&self) -> usize {
        let len = self.data.len();
        len
    }
}
