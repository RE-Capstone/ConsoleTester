//! This is documentation for the `console_tester` crate.
//! 
//! # What is this?
//! The purpose of this crate is to aid developers by allowing them to test their console code on multiple  
//! different terminals. When it comes to displaying data, not all terminals handle output in the same  
//! manner. One might find that a particular escape sequence is properly displayed through terminal A but  
//! the same is not the case for terminal B. Thus, the developer may be required to configure these escape  
//! sequences differently for different environments. This can be a source of frustration for those that  
//! desire consistent output across the board. By providing a means to compare expected and actual output  
//! data on a selected terminal (or list of terminals), we hope to expedite the testing process.   
//! 
//! # How is this done?
//! Through the `TermWriter` and `TermStrings` structs, this crate parses data given by the user and  
//! compares it against a list of known good escape sequences for the given terminal. If one or more bad  
//! escape sequences are found in the input data, the input data is displayed on the screen with the bad  
//! sequences highlighted and the good sequences removed.  
//!   
//! + **TermWriter**  
//! >  The TermWriter module stores user data. TermWriter also contains the *compare* function which handles  
//! various possible errors and provides feedback to the user.
//! <br/>
//!   
//! + **TermString**  
//! >  The TermStrings module holds the valid escape sequences for a given terminal. When the user selects  
//! the terminal they wish to test their input data against, TermSring is populated by the known good escape  
//! sequences for that terminal.  
//!   
//! # How do I use this?
//! In an effort for simplicity and clarity, the following examples will show how to use this crate. You'll  
//! notice (Example 1) does not provide a terminal to test user input data against. In this case, TermStrings  
//! will just default to the current terminal.
//! <br/>
//!   
//! **In simple terms, what does using this crate look like?**  
//!   
//! Feed in the string or data you wish to test, provide an argument for the selected terminal (if desired),  
//! results will be displayed.  
//! 
//! ### Example 1:  general use, no arguments
//! ```
//!     use console_tester::buffer::TermWriter;
//!     use console_tester::term::TermStrings;
//!     use std::io::Write;
//!
//!     let mut buffer: TermWriter = TermWriter::new();
//!     buffer.write(b"Console output information here");
//!
//!     // Find local terminal
//!     let cmd_ts: TermStrings = TermStrings::new_from_env();
//!
//!     let b1 = buffer.compare(cmd_ts);
//! ```
//!   
//! ### Example 2:  with argument
//! ```
//!     use console_tester::buffer::TermWriter;
//!     use console_tester::term::TermStrings;
//!     use std::io::Write;
//!
//!     let mut buffer: TermWriter = TermWriter::new();
//!     buffer.write(b"Console output information here");
//!
//!     // x-term test
//!     let path = std::path::Path::new("./terminfo_files/x/xterm");
//!     let cmd_ts: TermStrings = TermStrings::new_from_path(path);
//!
//!     let b1 = buffer.compare(cmd_ts);
//! ```
//! 
//! ### Example 3:  with multiple arguments
//! ```
//!     use console_tester::buffer::TermWriter;
//!     use console_tester::term::TermStrings;
//!     use std::io::Write;
//!
//!     let mut buffer: TermWriter = TermWriter::new();
//!     buffer.write(b"Console output information here");
//!
//!     // clone existing buffer
//!     let buffer2: TermWriter = buffer.clone();
//!
//!     // x-term test
//!     let path = std::path::Path::new("./terminfo_files/x/xterm");
//!     let cmd_ts: TermStrings = TermStrings::new_from_path(path);
//!
//!     // cygwin test
//!     let path2 = std::path::Path::new("./terminfo_files/c/cygwin");
//!     let cmd_cygwin: TermStrings = TermStrings::new_from_path(path);
//!
//!     let b1 = buffer.compare(cmd_ts);
//!     let b2 = buffer2.compare(cmd_cygwin);
//! ```
//! 
//! ### Example 4:  bad escape sequence found
//! ```
//!     use console_tester::buffer::TermWriter;
//!     use console_tester::term::TermStrings;
//!     use std::io::Write;
//!
//!     let mut buffer: TermWriter = TermWriter::new();
//!     buffer.write(b"Console output information here");
//!
//!     // x-term test
//!     let path = std::path::Path::new("./terminfo_files/x/xterm");
//!     let cmd_ts: TermStrings = TermStrings::new_from_path(path);
//!
//!     let b1 = buffer.compare(cmd_ts);
//! ```

#![crate_type = "lib"]
#![crate_name = "console_tester"]

// Internal Exposure
mod reg;

// Public Exposure
pub mod buffer;
pub mod term;