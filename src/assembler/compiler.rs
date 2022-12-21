use super::err_manager::ErrorManager;
use super::lexer::Token;
use crate::instruction::Opcode;
use crate::report_error_with_lines;

pub struct Compiler {
    c: usize,
    current: Token,
    tokens: Vec<Token>,
    err_manager: ErrorManager,
    compiled: Vec<u8>,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>, err_manager: ErrorManager) -> Self {
        Compiler {
            tokens,
            err_manager,
            c: 0,
            current: Token::RegisterNum((0, 0, 0)),
            compiled: vec![],
        }
    }

    pub fn get_compiled_program(&self) -> &Vec<u8> {
        &self.compiled
    }

    /// Compiles the given tokens to binary
    pub fn compile(&mut self) {
        self.current = self.tokens[self.c].clone();

        while !self.is_end() {
            if self.c != 0 {
                self.advance();
            }

            match self.current.clone() {
                Token::Opcode(opcode) => match opcode.0 {
                    Opcode::IGL => {
                        self.err_manager
                            .create_and_add_error("invalid opcode", opcode.1, opcode.2)
                    }
                    Opcode::HLT => self.compiled.push(opcode.0 as u8),
                    Opcode::LOAD => {
                        self.compiled.push(opcode.0 as u8);

                        let mut register: u8 = 0;
                        let mut number: i32 = 0;

                        self.advance();
                        match self.current.clone() {
                            Token::RegisterNum(register_num) => register = register_num.0,
                            Token::Opcode(t) => {
                                report_error_with_lines!(self, t, "expected register number");
                            }
                            Token::IntegerOperand(t) => {
                                report_error_with_lines!(self, t, "expected register number");
                            }
                            Token::FloatOperand(t) => {
                                report_error_with_lines!(self, t, "expected register number");
                            }
                        }
                        self.compiled.push(register);

                        self.advance();
                        match self.current.clone() {
                            Token::IntegerOperand(num) => number = num.0,
                            Token::Opcode(t) => {
                                report_error_with_lines!(self, t, "expected an operand");
                            }
                            Token::FloatOperand(t) => {
                                report_error_with_lines!(self, t, "not implemented yet!");
                            }
                            Token::RegisterNum(t) => {
                                report_error_with_lines!(self, t, "expected an operand");
                            }
                        }

                        for b in Compiler::extract_int_operand(number) {
                            self.compiled.push(b);
                        }
                    }
                    Opcode::ADD | Opcode::SUB | Opcode::MUL | Opcode::DIV => {
                        self.compiled.push(opcode.0 as u8);

                        let mut register: u8 = 0;

                        for _ in 0..3 {
                            self.advance();
                            match self.current.clone() {
                                Token::RegisterNum(register_num) => register = register_num.0,
                                Token::Opcode(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                                Token::IntegerOperand(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                                Token::FloatOperand(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                            }
                            self.compiled.push(register);
                        }
                    }
                    Opcode::JMP | Opcode::JMPF | Opcode::JMPB | Opcode::JEQ | Opcode::JNEQ => {
                        self.compiled.push(opcode.0 as u8);

                        let mut target_pc: u8 = 0;

                        self.advance();
                        match self.current.clone() {
                            Token::IntegerOperand(target) => target_pc = target.0 as u8,
                            Token::Opcode(t) => {
                                report_error_with_lines!(self, t, "expected an operand");
                            }
                            Token::FloatOperand(t) => {
                                report_error_with_lines!(self, t, "expected an operand");
                            }
                            Token::RegisterNum(t) => {
                                report_error_with_lines!(self, t, "expected an operand");
                            }
                        }
                        self.compiled.push(target_pc);
                    }
                    Opcode::EQ => {
                        self.compiled.push(opcode.0 as u8);

                        let mut register: u8 = 0;

                        for _ in 0..2 {
                            self.advance();
                            match self.current.clone() {
                                Token::RegisterNum(register_num) => register = register_num.0,
                                Token::Opcode(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                                Token::IntegerOperand(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                                Token::FloatOperand(t) => {
                                    report_error_with_lines!(self, t, "expected register number");
                                }
                            }
                            self.compiled.push(register);
                        }
                    }
                },
                Token::RegisterNum(t) => {
                    report_error_with_lines!(self, t, "expected an opcode");
                }
                Token::IntegerOperand(t) => {
                    report_error_with_lines!(self, t, "expected an opcode");
                }
                Token::FloatOperand(t) => {
                    report_error_with_lines!(self, t, "expected an opcode")
                }
            }
        }
    }

    fn extract_int_operand(i: i32) -> [u8; 2] {
        let converted = i as u16;
        let byte1 = converted;
        let byte2 = converted >> 8;
        [byte1 as u8, byte2 as u8]
    }

    fn is_end(&self) -> bool {
        if self.tokens.len() <= self.c {
            true
        } else {
            false
        }
    }

    fn advance(&mut self) {
        if !self.is_end() && !(self.tokens.len() <= self.c + 1) {
            self.c += 1;
            self.current = self.tokens[self.c].clone();
        } else {
            self.c = self.tokens.len();
        }
    }
}