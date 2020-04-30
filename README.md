# RustEmulator
Rust Console Emulation for Unit Testing across multiple consoles for consistent user experience.

## Builds

[![Build Status](https://travis-ci.com/RE-Capstone/RustConsoleEmulator.svg?branch=master)](https://travis-ci.com/RE-Capstone/RustConsoleEmulator)

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
3. ` cargo run ` or ` cargo build `


### Unix

#### PreReqs

1. Install Rust by following the instructions on their website [Rust install site](https://www.rust-lang.org/tools/install)

**Optional**
- [MACOS] brew install rust-lang
- [Ubuntu] ` curl https://sh.rustup.rs -sSf | sh ` && ` source $HOME/.cargo/env ` && ` sudo apt install build-essential `

#### Build Steps

1. ` git clone https://github.com/RE-Capstone/RustConsoleEmulator `
2. ` cd RustConsoleEmulator `
3. ` cargo run ` or ` cargo build `
