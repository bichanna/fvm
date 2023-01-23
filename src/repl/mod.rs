use crate::assembler::Assembler;
use crate::vm::VM;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::path::Path;

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

    /// Runs a REPL.
    pub fn run(&mut self, hex: bool) {
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
                // Lists the instructions currently in VM's program vector.
                ".program" => {
                    println!("{:#?}", self.vm.get_program());
                }
                // Lists the registers.
                ".registers" => {
                    println!("{:#?}", self.vm.get_registers());
                }
                // Loads file.
                ".load_file" => {
                    print!("Enter the path of the file you wish to load: ");
                    io::stdout().flush().expect("Unable to stdout");
                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to read input line.");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("Error reading from the file");
                    let source = buffer.to_string();
                    let mut assembler = Assembler::new("<input>", &source, true);
                    for byte in assembler.compile() {
                        self.vm.add_byte(byte);
                    }
                }
                _ => {
                    if hex {
                        let results = self.parse_hex(buffer);
                        match results {
                            Ok(bytes) => {
                                for byte in bytes {
                                    self.vm.add_byte(byte)
                                }
                            }
                            _ => {
                                println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters");
                            }
                        }
                    } else {
                        let source = buffer.to_string();
                        let mut assembler = Assembler::new("<input>", &source, true);
                        for byte in assembler.compile() {
                            self.vm.add_byte(byte);
                        }
                    }
                    // Run the instruction.
                    self.vm.run_once();
                }
            }
        }
    }

    /// Accepts a hexadecimal string without a leading '0x' and returns a Vec of u8.
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let splitted: Vec<&str> = i.split(" ").collect();
        let mut results: Vec<u8> = vec![];

        for hex_str in splitted {
            let byte = u8::from_str_radix(&hex_str, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(results)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let mut repl = REPL::new();
        // Load 500 to register 1, load 500 to register 2, add register 1 and 2, and store the
        // result to register 0.
        let results = repl.parse_hex("00 01 01 F4 00 02 01 F4 01 01 02 00");

        match results {
            Ok(bytes) => {
                for byte in bytes {
                    repl.vm.add_byte(byte)
                }
            }
            _ => {
                panic!();
            }
        }
        repl.vm.run();
        assert_eq!(repl.vm.get_registers()[0], 1000);
    }
}
