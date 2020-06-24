use std::io::{self, Write};
use std::io::prelude::*;
use std::fs::File;

use console_tester::buffer::TermWriter;

fn main() {
    let vec = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
    let joined_vec = vec.join(" ");

    let vec_bytes = joined_vec.as_bytes();

    let mut buffer = TermWriter::new();

    let bytes_written = buffer.write(vec_bytes);
    println!("Bytes written: {:?}", bytes_written); //bytes_written = Ok(14)

    let byte_literal = b"write_all()";
    buffer.write_all(byte_literal);
}

#[cfg(test)] //for 'cargo test'
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

        assert_eq!((), buffer.write_all(bytes_literal).unwrap()); //"no news is good news"
    }

    #[test]
    fn termwriter_flush() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let bytes_written = buffer.write(bytes_literal);
        assert_eq!((), buffer.flush().unwrap())
    }
}
