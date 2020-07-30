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

    //let success = t.write_to_file("foobar.txt", b"Hello world!");
    //println!("Value of 'success': {}", success);

    let input = b"Hello world";
    t.write_to_file("foobar.txt", input);

    //println!("{:?}", cmd_ts);

    //let b1 = t.compare(cmd_ts);
    //println!("{:?}", b1);
    // let b2: bool = t.compare(zsh_ts);

    // println!("{:?}", t);

    // t.flush();
}
