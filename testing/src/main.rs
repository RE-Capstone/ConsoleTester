use std::io::{self, Write};
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

#[cfg(test)]
// TEST to see how modules work
//mod var;

    //  TEST CODE using BufWriter to fill with print statements and then flush once
    /*let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "BufWriter stdout test");*/

// VECTORS - use them as efficient stacks ('vec!' macro for easy initialization)

    // TEST to see how vectors function as a stack - WORKS
    #[test]
    fn vec_test() {
        let mut vec = Vec::new();
        let vec2 = vec!["c", "b"];

        vec.push("c");
        vec.push("b");
        vec.push("a");
        vec.pop();
        /*while let Some(top) = vec.pop() {
            println!("{}", top);
        }*/
        assert_eq!(vec, vec2);
    }

fn main() {
    // TEST to see how BufWriter() works with a vector string - WORKS
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let vec_strings = vec!["Vector".to_string(), "string".to_string(), "test".to_string()];
    let joined_vec_string = vec_strings.join(" ");
    writeln!(handle, "{}", joined_vec_string);

// doodles
    let hello = String::from("Hello");
    let len = hello.len();
    println!("{}", len);

    // TEST CODE to see how BufWriter() works with Write trait to write to a File - WORKS
    /*let mut buffer = BufWriter::new(File::create("foo.txt")?);

    buffer.write(b"Writing some bytes to foo.txt")?;
    buffer.flush()?;
    Ok(())*/

// TEST CODE to see how Stdout() works with a vectored input - NEED TO WORK ON THIS!!!
    /*let stdout = io::stdout();
    let mut handle = stdout.lock();

    let vec = vec!["Hello".to_string(), "world!".to_string()];
    let joined_vec = vec.join(" ");

    handle.write_all(&joined_vec)?;

    Ok(())*/
}
