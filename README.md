# RustEmulator ![Rust](https://github.com/RE-Capstone/RustConsoleEmulator/workflows/Rust/badge.svg)
Rust Console Emulation for Unit Testing across multiple consoles for consistent user experience.

## Documentation Website

[Documentation](https://re-capstone.github.io/RustConsoleEmulator/)

### Building Documentation

1. ` cd RustConsoleEmulator `
2. ` cargo doc --target-dir ./docs/ `

## Builds

![Rust](https://github.com/RE-Capstone/RustConsoleEmulator/workflows/Rust/badge.svg)

## Building

### Windows

#### PreReqs

1. Install Rust by following the instructions on their website [Rust install site](https://www.rust-lang.org/tools/install)
2. Ensure that you configure your path variables and terminal configuration if running into errors.
3. Open ` cmd ` and type ` cargo version `

**Recommendations:**
- Install new Windows terminal [Found here](https://github.com/microsoft/terminal)

#### Build Steps

1. ` git clone https://github.com/RE-Capstone/RustConsoleEmulator `
2. ` cd RustConsoleEmulator `
3. ` cargo run ` or ` cargo build --lib`


### Unix

#### PreReqs

1. Install Rust by following the instructions on their website [Rust install site](https://www.rust-lang.org/tools/install)

**Optional**
- [MACOS] brew install rust-lang
- [Ubuntu] ` curl https://sh.rustup.rs -sSf | sh ` && ` source $HOME/.cargo/env ` && ` sudo apt install build-essential `

#### Build Steps

1. ` git clone https://github.com/RE-Capstone/RustConsoleEmulator `
2. ` cd RustConsoleEmulator `
3. ` cargo run ` or ` cargo build --lib`
