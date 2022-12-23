use super::error::ParserError;
use super::lexer::Token;
use crate::instruction::Opcode;

pub struct Compiler<'a> {
    c: usize,
    current: Token,
    tokens: &'a Vec<Token>,
    compiled: Vec<u8>,
    pub errors: Vec<ParserError>,
}

impl<'a> Compiler<'a> {
    pub fn new<'b>(tokens: &'a Vec<Token>) -> Compiler<'a> {
        Compiler {
            tokens,
            c: 0,
            errors: vec![],
            current: Token::RegisterNum((0, 0, 0)),
            compiled: vec![],
        }
    }

    pub fn get_compiled_program(&self) -> Vec<u8> {
        self.compiled.clone()
    }

    /// Compiles the given tokens to binary
    pub fn compile(&mut self) {
        self.current = self.tokens[self.c].clone();

        while !self.is_end() && !(self.tokens.len() <= self.c + 1) {
            if self.c != 0 {
                self.advance();
            }

            match self.current.clone() {
                Token::Opcode(opcode) => match opcode.0 {
                    Opcode::IGL => self.add_error("expected an opcode", opcode.1, opcode.2),
                    Opcode::HLT => self.compiled.push(opcode.0 as u8),
                    Opcode::LOAD => {
                        self.compiled.push(opcode.0 as u8);

                        let mut register: u8 = 0;
                        let mut number: i32 = 0;

                        self.advance();
                        match self.current.clone() {
                            Token::RegisterNum(register_num) => register = register_num.0,
                            Token::Opcode(t) => {
                                self.add_error("expected an register number", t.1, t.2);
                            }
                            Token::IntegerOperand(t) => {
                                self.add_error("expected an register number", t.1, t.2);
                            }
                            Token::FloatOperand(t) => {
                                self.add_error("expected an register number", t.1, t.2);
                            }
                        }
                        self.compiled.push(register);

                        self.advance();
                        match self.current.clone() {
                            Token::IntegerOperand(num) => number = num.0,
                            Token::Opcode(t) => {
                                self.add_error("expected an operand", t.1, t.2);
                            }
                            Token::FloatOperand(t) => {
                                self.add_error("expected an operand", t.1, t.2);
                            }
                            Token::RegisterNum(t) => {
                                self.add_error("expected an operand", t.1, t.2);
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
                                    self.add_error("expected an register number", t.1, t.2);
                                }
                                Token::IntegerOperand(t) => {
                                    self.add_error("expected an register number", t.1, t.2);
                                }
                                Token::FloatOperand(t) => {
                                    self.add_error("expected an register number", t.1, t.2);
                                }
                            }
                            self.compiled.push(register);
                        }
                    }
                    Opcode::JMP
                    | Opcode::JMPF
                    | Opcode::JMPB
                    | Opcode::JEQ
                    | Opcode::JNEQ
                    | Opcode::ALOC => {
                        self.compiled.push(opcode.0 as u8);

                        let mut target_pc: u8 = 0;

                        self.advance();
                        match self.current.clone() {
                            Token::IntegerOperand(target) => target_pc = target.0 as u8,
                            Token::Opcode(t) => self.add_error("expected an operand", t.1, t.2),
                            Token::FloatOperand(t) => {
                                self.add_error("expected an operand", t.1, t.2);
                            }

                            Token::RegisterNum(t) => {
                                self.add_error("expected an operand", t.1, t.2);
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
                                    self.add_error("expected a register number", t.1, t.2);
                                }
                                Token::IntegerOperand(t) => {
                                    self.add_error("expected a register number", t.1, t.2);
                                }
                                Token::FloatOperand(t) => {
                                    self.add_error("expected a register number", t.1, t.2);
                                }
                            }
                            self.compiled.push(register);
                        }
                    }
                },
                Token::RegisterNum(t) => {
                    self.add_error("expected an opcode", t.1, t.2);
                }
                Token::IntegerOperand(t) => {
                    self.add_error("expected an opcode", t.1, t.2);
                }
                Token::FloatOperand(t) => {
                    self.add_error("expected an opcode", t.1, t.2);
                }
            }
        }
    }

    fn add_error(&mut self, msg: &str, line: usize, col: usize) {
        self.errors.push(ParserError::new(msg, line, col));
    }

    fn extract_int_operand(i: i32) -> [u8; 2] {
        let converted = i as u16;
        let byte1 = converted;
        let byte2 = converted >> 8;
        // little endian rule
        [byte2 as u8, byte1 as u8]
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler() {
        let tokens = vec![
            Token::Opcode((Opcode::LOAD, 1, 5)),
            Token::RegisterNum((0, 1, 8)),
            Token::IntegerOperand((500, 1, 13)),
            Token::Opcode((Opcode::LOAD, 2, 5)),
            Token::RegisterNum((1, 2, 8)),
            Token::IntegerOperand((100, 2, 13)),
            Token::Opcode((Opcode::ADD, 3, 4)),
            Token::RegisterNum((0, 3, 7)),
            Token::RegisterNum((1, 3, 10)),
            Token::RegisterNum((2, 3, 12)),
        ];
        // Load 500 to register 0, load 100 to register 1, add registers 0 and 1, and stores the
        // result to register 2.
        let expected: Vec<u8> = vec![0, 0, 1, 244, 0, 1, 0, 100, 1, 0, 1, 2];

        let mut compiler = Compiler::new(&tokens);

        compiler.compile();

        let compiled = compiler.get_compiled_program();

        assert_eq!(compiler.errors.len(), 0);
        assert_eq!(expected, *compiled);
    }
}
