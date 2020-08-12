// To remove warnings we need to conditionally disable these
/*
use std::io::{self, Write};
use std::io::prelude::*;
use std::fs::File;

use console_tester::buffer::TermWriter;
*/
//use std::path::Path;

use console_tester::buffer::TermWriter;
use console_tester::term::TermStrings;

// use std::io::Write;

fn main() {
    let buffer: TermWriter = TermWriter::new();

    let path = std::path::Path::new("./terminfo_files/x/xterm");
    let cmd_ts: TermStrings = TermStrings::new_from_path(path); // This is local terminal

    /*
    // Get TermStrings from terminfo database file
    let path: Path = Path::new("./xterm");
    let mut xterm_ts: TermStrings = TermStrings::new_from_path(path);
    println!("{:?}", xterm_ts);
    */

    //buffer.write(b"Hello World").expect("failed termwriter.write");
    //println!("{:?}", cmd_ts);

    let b1 = buffer.compare(cmd_ts);
    println!("{:?}", b1);
    // let b2: bool = buffer.compare(zsh_ts);

    // println!("{:?}", buffer);

    // buffer.flush();
}
