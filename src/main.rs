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
    let cmd_ts: TermStrings = TermStrings::new_from_path(path);

    let b1 = buffer.compare(cmd_ts);

    println!("{:?}", b1);
}
