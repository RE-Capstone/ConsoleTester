//! This is documentation for the `buffer` module.
//!
//! # Examples
//! BufWriter buf = asdasd
// ...

/// Constructs a new `BufWriter`.
///
/// # Examples
///
/// ```
/// use console_tester:buffer;
///
/// let writer = BufWriter::new("");
/// ```
pub struct BufWriter { dirty: bool, os: OS}

pub enum OS {
    ///
    /// Enum that stores the OS data associated with the Test
    /// Will look at the data before progressing and used in switch to get terminfo.
    /// 
    WIN,
    OSX,
    LINUX
}