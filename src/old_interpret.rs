fn interpret(instruction: &str) {
    let instruction: Vec<char> = instruction.chars().collect();
    let mut i_ptr = 0;
    let mut d_ptr = 0;
    let mut stack: Vec<u32> = vec![0];

    while i_ptr < instruction.len() {
        match instruction[i_ptr] {
            '>' => {
                if d_ptr == stack.len() - 1 {
                    stack.push(0);
                }
                d_ptr += 1;
                i_ptr += 1;
            }
            '<' => {
                if d_ptr != 0 {
                    d_ptr -= 1;
                    i_ptr += 1;
                } 
            }
            '+' => {
                stack[d_ptr] += 1;
                i_ptr += 1;
            }
            '-' => {
                stack[d_ptr] -= 1;
                i_ptr += 1;
            }
            '[' => {
                if stack[d_ptr] == 0 {
                    let mut loop_depth = 1;
                    while loop_depth > 0 {
                        i_ptr += 1;
                        if instruction[i_ptr] == '[' {
                            loop_depth += 1;
                        } else if instruction[i_ptr] == ']' {
                            loop_depth -= 1;
                        }
                    }
                }
                i_ptr += 1;          
            }
            ']' => {
                
                let mut loop_depth = 1;
                while loop_depth > 0 {
                    i_ptr -= 1;
                }
            }
        }
    }
}

fn main() {
    let instruction = "++>+++++[<+>-]++++++++[<++++++>-]<.";
    println!("Hello, world!");
}
