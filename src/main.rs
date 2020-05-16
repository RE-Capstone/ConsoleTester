mod message;
mod docs;
use rust_console_emulator::print_lib_message;

/// Prints: "Hello, world!"
pub fn main() {
    println!("Hello, world!");
    message::print_message();
    docs::add_one(32);
    print_lib_message();
}
