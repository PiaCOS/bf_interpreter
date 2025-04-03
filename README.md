# Brainfuck Interpreter in Rust

Here's a simple brainfuck interpreter made in rust.

## Installation

You only need cargo to run this interpreter.

## Instructions

- Create a text file containing the bf code.
- Change the path inside the main.rs file (will be changed in a future patch)
- Run `cargo run --release`

## Notes

I plan to do several addition to this project:

- Write a CLI integration to specify a file path
- Optimize the `ops_stack` further (group consecutive ops and reduce ops when possible)
