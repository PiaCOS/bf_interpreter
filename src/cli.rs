use clap::Parser;

/// Interpreter for the Brainfuck Esolang
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to your bf file
    #[arg(short, long, default_value_t = String::from("./examples/hello_world.bf"))]
    pub file: String,
}