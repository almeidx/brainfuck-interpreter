const MEMORY_SIZE: usize = 5_000;

struct Program {
    /// The memory of the program
    memory: Vec<u8>,
    /// The pointer to the current memory cell
    pointer: usize,
    /// The pointer to the current instruction
    instruction_pointer: usize,
    /// The stack of loop pointers
    loop_stack: Vec<usize>,
    /// The output of the program
    output: String,
}

impl Program {
    fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
            pointer: 0,
            instruction_pointer: 0,
            loop_stack: Vec::new(),
            output: String::new(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a filename");
    }
    let filename = &args[1];

    let code = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    interpret(code, true);
}

fn interpret(code: String, with_output: bool) -> Program {
    let mut program = Program::new();

    while program.instruction_pointer < code.len() {
        let instruction = code.chars().nth(program.instruction_pointer).unwrap();
        match instruction {
            '>' => {
                assert!(
                    program.pointer < program.memory.len() - 1,
                    "Pointer cannot be greater than memory size"
                );
                program.pointer += 1;
            }
            '<' => {
                assert!(program.pointer > 0, "Pointer cannot be less than 0");
                program.pointer -= 1;
            }
            '+' => {
                assert!(
                    program.memory[program.pointer] < 255,
                    "Cannot increment above 255"
                );
                program.memory[program.pointer] += 1;
            }
            '-' => {
                assert!(
                    program.memory[program.pointer] > 0,
                    "Cannot decrement below 0"
                );
                program.memory[program.pointer] -= 1;
            }
            '.' => {
                print!("{}", program.memory[program.pointer] as char);

                if with_output {
                    program.output.push(program.memory[program.pointer] as char);
                }
            }
            ',' => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                program.memory[program.pointer] = input.chars().nth(0).unwrap() as u8;
            }
            '[' => {
                if program.memory[program.pointer] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        program.instruction_pointer += 1;
                        let instruction = code.chars().nth(program.instruction_pointer).unwrap();
                        match instruction {
                            '[' => loop_count += 1,
                            ']' => loop_count -= 1,
                            _ => {}
                        }
                    }
                } else {
                    program.loop_stack.push(program.instruction_pointer);
                }
            }
            ']' => {
                program.instruction_pointer = program.loop_stack.pop().unwrap() - 1;
            }
            _ => {}
        }

        program.instruction_pointer += 1;
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let code = std::fs::read_to_string("examples/hello.bf")
            .expect("Something went wrong reading the file");

        let program = interpret(code.to_owned(), true);

        let message = "Hello, World!";

        assert_eq!(program.output, message);

        for (i, c) in message.chars().enumerate() {
            assert_eq!(program.memory[i], c as u8);
        }

        assert_eq!(program.pointer, message.len() - 1);
        assert_eq!(program.instruction_pointer, code.len());

        for i in message.len()..program.memory.len() {
            assert_eq!(program.memory[i], 0);
        }

        assert_eq!(program.memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn test_hello_world_loop() {
        let code = std::fs::read_to_string("examples/hello2.bf")
            .expect("Something went wrong reading the file");

        let program = interpret(code.to_owned(), true);

        assert_eq!(program.output, "Hello World!\n");
    }

    #[test]
    fn test_loop() {
        let code = std::fs::read_to_string("examples/loop.bf")
            .expect("Something went wrong reading the file");

        let program = interpret(code.to_owned(), true);

        assert_eq!(program.output, "");

        for i in 0..program.memory.len() {
            assert_eq!(program.memory[i], 0);
        }

        assert_eq!(program.pointer, 0);
        assert_eq!(program.instruction_pointer, code.len());
    }
}
