// To remove warnings we need to conditionally disable these
/*
use std::io::{self, Write};
use std::io::prelude::*;
use std::fs::File;

use console_tester::buffer::TermWriter;
*/

use console_tester::buffer::TermWriter;
use console_tester::term::TermStrings;

use std::io::Write;

fn main() {
    let mut t: TermWriter = TermWriter::new();

    // let mut bash_ts: TermStrings = TermStrings::new(file);  // This is termcap for bash
    // let mut zsh_ts: TermStrings = TermStrings::new(file);   // This is termcap for zsh
    let mut cmd_ts: TermStrings = TermStrings::new();       // This is local terminal

    t.write(b"Hello World");

    let b1 = t.compare(cmd_ts);
    println!("{:?}", b1);
    // let b2: bool = t.compare(zsh_ts);

    // println!("{:?}", t);

    // t.flush();
}
