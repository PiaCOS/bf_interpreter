use core::fmt;
use std::{fs, io::{self, Write}};

#[derive(Clone, Debug)]
pub enum Ops {
    MoveLeft,
    MoveRight,
    Incr,
    Decr,
    LoopStart(usize),
    LoopEnd(usize),
    Input,
    Output,
}

pub fn deparse(op: Ops) -> String {
    let op_str = match op {
        Ops::MoveRight => ">",
        Ops::MoveLeft => "<",
        Ops::Incr => "+",
        Ops::Decr => "-",
        Ops::LoopStart(_) => "[",
        Ops::LoopEnd(_) => "]",
        Ops::Input => ",",
        Ops::Output => ".",
    };
    op_str.to_owned()
}

pub fn parse(code: &str) -> Result<Vec<Ops>, BrainFuckError> {
    let mut ops_stack: Vec<Ops> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();
    for c in code.chars() {
        match c {
            '>' => ops_stack.push(Ops::MoveRight),
            '<' => ops_stack.push(Ops::MoveLeft),
            '+' => ops_stack.push(Ops::Incr),
            '-' => ops_stack.push(Ops::Decr),
            '[' => {
                // store index of open bracket 
                loop_stack.push(ops_stack.len());
                ops_stack.push(Ops::LoopStart(0));
            },
            ']' => {
                // Get the index of the closing bracket
                let open_index = match loop_stack.pop() {
                    Some(u) => u,
                    None => return Err(BrainFuckError::UnmatchedBracket {op: Ops::LoopStart(0)}),
                };
                // replace the index inside the loop start
                ops_stack[open_index] = Ops::LoopStart(ops_stack.len());
                ops_stack.push(Ops::LoopEnd(open_index));
            },
            ',' => ops_stack.push(Ops::Input),
            '.' => ops_stack.push(Ops::Output),
            _ => (),
        }
    }
    Ok(ops_stack)
}


pub fn run(code: &str) -> Result<(), BrainFuckError> {
    let ops_stack = parse(code)?;

    let mut memory: Vec<u8> = vec![0; 3000];
    let mut m_ptr = 0; // memory pointer
    let mut s_ptr = 0; // stack pointer

    while s_ptr < ops_stack.len() {
        match ops_stack[s_ptr] {
            Ops::MoveRight => {
                if m_ptr < memory.len() - 1 {
                    m_ptr += 1;
                } else {
                    return Err(BrainFuckError::MemoryOutOfBound { op: Ops::MoveRight, s_ptr })
                }
            },
            Ops::MoveLeft => {
                if m_ptr > 0 {
                    m_ptr -= 1;
                } else {
                    return Err(BrainFuckError::MemoryOutOfBound { op: Ops::MoveLeft, s_ptr })
                }
            },
            Ops::Incr => memory[m_ptr] = memory[m_ptr].wrapping_add(1),
            Ops::Decr => memory[m_ptr] = memory[m_ptr].wrapping_sub(1),
            Ops::LoopStart(jump_to) => {
                if jump_to == 0 {
                    return Err(BrainFuckError::UnmatchedBracket {op: Ops::LoopStart(0)})
                }
                if memory[m_ptr] == 0 {
                    s_ptr = jump_to
                }
            },
            Ops::LoopEnd(jump_back) => {
                if memory[m_ptr] != 0 {
                    s_ptr = jump_back;
                }
            },
            Ops::Input => {todo!()},
            Ops::Output => {
                io::stdout().write_all(&memory[m_ptr..m_ptr+1])?
            },
        }
        s_ptr += 1;
    }

    Ok(())
}


#[derive(Debug)]
pub enum BrainFuckError {
    MemoryOutOfBound{op: Ops, s_ptr: usize},
    UnmatchedBracket{op: Ops},
    IOError(std::io::Error),
}

impl From<std::io::Error> for BrainFuckError {
    fn from(err: std::io::Error) -> Self {
        BrainFuckError::IOError(err)
    }
}

impl fmt::Display for BrainFuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainFuckError::MemoryOutOfBound { op, s_ptr } => {
                write!(f, "Memory out of bound with instruction: '{}' (#{}) !!", deparse(op.clone()), s_ptr)
            },
            BrainFuckError::UnmatchedBracket { op } => {
                write!(f, "Unmatched '{}' encountered !!", deparse(op.clone()))
            },
            BrainFuckError::IOError(err) => {
                write!(f, "IOError: {}", err)
            }
        }
    }
}


fn main() -> Result<(), BrainFuckError> {
    let code = fs::read_to_string("./mandelbrot.bf")?;
    run(&code)?;
    Ok(())
}