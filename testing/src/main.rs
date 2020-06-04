use std::io::{self, Write};
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
//mod structs;
//mod enums;
mod lib; // NOT SURE IF THIS KIND OF IMPORT IS WHAT WE WANT?
use lib::TermWriter;

fn main() {
    // TEST1: see if String Vectors can be used as input into TermWriter - PASS
    let vec = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];

    // TEST2: see if each vector index value can be concatenated together - PASS
    let joined_vec = vec.join(" ");

    // TEST3: see if joined String vector can be turned into bytes - PASS
    let vec_bytes = joined_vec.as_bytes();

    // TEST4: see if TermWriter can successfully be instantiated - PASS
    let mut buffer = TermWriter::new();

    // TEST5: see if 'Write' trait is correctly implemented in TermWriter - PASS...?
    // use {:?} to format & print bytes
    let bytes_written = buffer.write(vec_bytes);
    println!("Bytes written: {:?}", bytes_written); //prints 'Ok(14)'?
    //buffer.flush();

    // TEST6: see if 'write_all()' method is correctly implemented in TermWriter - PASS...?
    let byte_literal = b"write_all() test";
    buffer.write_all(byte_literal);

    // Small test to see if 'return_num_bytes()' method is correctly implemented - PASS
    let len = buffer.return_num_bytes();
    println!("Length of bytes data: {}", len);

}
