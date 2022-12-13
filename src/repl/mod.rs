use crate::vm::VM;
use std::io;
use std::io::Write;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        return REPL {
            vm: VM::new(),
            command_buffer: vec![],
        };
    }

    pub fn run(&mut self) {
        println!("Welcome to Feo!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            // flush stdout as well
            print!(">> ");
            io::stdout().flush().expect("Unable to flush stdout.");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read input line.");
            let buffer = buffer.trim();

            // store the line given
            self.command_buffer.push(buffer.to_string());

            match buffer {
                // Exits the REPL.
                ".quit" | ".exit" => {
                    println!("Bye");
                    std::process::exit(0);
                }
                // Shows the history of the previous user commands.
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
