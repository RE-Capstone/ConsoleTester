use std::io::{self, Write};
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use testing::buffer::TermWriter;

#[cfg(test)]
mod tests {
    // import parent crate's functions
    use super::*;

    // TEST1: see if joined String vector matches a String literal - PASSED
    #[test]
    fn vec_string() {
        let vec1 = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
        let joined_vec1 = vec1.join(" ");

        let string_lit = "Some junk text";

        assert_eq!(joined_vec1, string_lit);
    }

    // TEST2: see if joined String vector (converted into bytes) - PASSED
    // matches with a String literal (converted into bytes)
    #[test]
    fn vec_string_bytes() {
        let vec1 = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
        let joined_vec1 = vec1.join(" ");
        let vec1_bytes = joined_vec1.as_bytes();

        let bytes_literal = b"Some junk text";

        assert_eq!(vec1_bytes, bytes_literal);
    }

    // TEST3: see if 'write()' method is correctly implemented in TermWriter - PASSED
    #[test]
    fn termwriter_write() {
        let bytes_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let bytes_written = buffer.write(bytes_literal);
        assert_eq!(bytes_written.unwrap(), 14); //unwrap() - unwraps the Result<()> and gives inner value?
    }

    // TEST4: see if 'write_all()' method is correctly implemented in TermWriter - PASSED
    #[test]
    fn termwriter_write_all() {
        let byte_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        assert_eq!((), buffer.write_all(byte_literal).unwrap());
    }

    // TEST5: see if 'flush()' method is correctly implemented in TermWriter - PASSED
    #[test]
    fn termwriter_flush() {
        let byte_literal = b"Some junk text";

        let mut buffer = TermWriter::new();
        let bytes_written = buffer.write(byte_literal);
        assert_eq!((), buffer.flush().unwrap());
    }
}


//// MAIN ////
fn main() {
    let vec = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
    let joined_vec = vec.join(" ");

    let vec_bytes = joined_vec.as_bytes();

    let mut buffer = TermWriter::new();

    let bytes_written = buffer.write(vec_bytes); //bytes_written = Ok(14)
    println!("Bytes written: {:?}", bytes_written); //use {:?} to format/print bytes

    let byte_literal = b"write_all() test";
    buffer.write_all(byte_literal);

    let len = buffer.return_num_bytes();
    println!("Length of bytes data: {}", len);

}
