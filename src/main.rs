// To remove warnings we need to conditionally disable these
/*
use std::io::{self, Write};
use std::io::prelude::*;
use std::fs::File;

use console_tester::buffer::TermWriter;
*/
use std::path::Path;

use console_tester::buffer::TermWriter;
use console_tester::term::TermStrings;

use std::io::Write;

fn main() {
    let mut t: TermWriter = TermWriter::new();

    let mut cmd_ts: TermStrings = TermStrings::new_from_env();       // This is local terminal

    /*
    // Get TermStrings from terminfo database file
    let path: Path = Path::new("./xterm");
    let mut xterm_ts: TermStrings = TermStrings::new_from_path(path);
    println!("{:?}", xterm_ts);
    */

    t.write(b"Hello World");

    // let b1: bool = t.compare(bash_ts);
    // let b2: bool = t.compare(zsh_ts);
    println!("{:?}", cmd_ts);

    // println!("{:?}", t);

    t.flush();
    /*let vec = vec!["Some".to_string(), "junk".to_string(), "text".to_string()];
    let joined_vec = vec.join(" ");

    let vec_bytes = joined_vec.as_bytes();

    let mut buffer = TermWriter::new();

    let bytes_written = buffer.write(vec_bytes);
    println!("Bytes written: {:?}", bytes_written); //bytes_written = Ok(14)

    let byte_literal = b"write_all()";
    buffer.write_all(byte_literal);*/
}
