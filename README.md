# ConsoleTester ![Rust](https://github.com/RE-Capstone/ConsoleTester/workflows/Rust/badge.svg) ![docs-gen](https://github.com/RE-Capstone/RustConsoleEmulator/workflows/docs-gen/badge.svg?branch=master) ![Crates](https://img.shields.io/crates/v/console_tester.svg)
Rust Console Emulation for Unit Testing across multiple consoles for consistent user experience.

## Documentation Website

[Documentation](https://re-capstone.github.io/ConsoleTester/)

### Building Documentation

` cargo doc --target-dir ./docs/ --no-deps --lib`

## Building

### Windows

#### PreReqs

1. Install Rust by following the instructions on their website [Rust install site](https://www.rust-lang.org/tools/install)
2. Ensure that you configure your path variables and terminal configuration if running into errors.
3. Open ` cmd ` and type ` cargo version `

**Recommendations:**
- Install new Windows terminal [Found here](https://github.com/microsoft/terminal) [This isn't necessary it may just help you in the long run]

#### Build Steps

1. ` git clone https://github.com/RE-Capstone/ConsoleTester `
2. ` cd ConsoleTester `
3. ` cargo run ` or ` cargo build --lib`


### Unix

#### PreReqs

1. Install Rust by following the instructions on their website [Rust install site](https://www.rust-lang.org/tools/install)

**Optional**
- [MACOS] brew install rust-lang
- [Ubuntu] ` curl https://sh.rustup.rs -sSf | sh ` && ` source $HOME/.cargo/env ` && ` sudo apt install build-essential `

#### Build Steps

1. ` git clone https://github.com/RE-Capstone/ConsoleTester `
2. ` cd ConsoleTester `
3. ` cargo run ` or ` cargo build --lib`
