# Brainfuck Interpreter in Rust

Here's a simple brainfuck interpreter made in rust.

## Installation

You only need cargo to run this interpreter.

## Instructions

- Create a text file containing the bf code.
- Run the project with your brainfuck program :
    `cargo run --release -- -f ./examples/mandelbrot.bf`

## Notes

I plan to do several addition to this project:

- Write a CLI integration to specify a file path *(done)*.
- Optimize the `ops_stack` further (group consecutive ops and reduce ops when possible).
