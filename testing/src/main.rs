use std::io::{self, Write};
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
//mod structs;
//mod enums;
mod lib; // NOT SURE IF THIS KIND OF IMPORT IS WHAT WE WANT?
use lib::TermWriter;

fn main() {
    // TEST1: see if String vectors can be used as input into TermWriter - PASS
    let vec = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];

    // TEST2: see if each vector index value can be concatenated together - PASS
    let joined_vec = vec.join(" ");

    // TEST3: see if joined String vector can be turned into bytes - PASS
    let vec_bytes = joined_vec.as_bytes();

    // TEST4: see if TermWriter object can be instantiated - PASS
    // TEST5: see if the vector bytes will be accepted into TermWriter for input - PASS
    let test_writer = TermWriter::new(vec_bytes);

    // need to use :? to format & print bytes
    println!("Bytes stored = {:?}", test_writer.data);

    // TEST6: see if TermWriter method, return_num_bytes, works correctly - PASS
    let bytes_length = test_writer.return_num_bytes();
    println!("Number of bytes stored = {}", bytes_length);


}
