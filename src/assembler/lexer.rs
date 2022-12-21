use super::err_manager::ErrorManager;
use crate::instruction::Opcode;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    RegisterNum(u8),
    IntegerOperand(i32),
    FloatOperand(f64),
}

pub struct Lexer {
    source: String,
    err_manager: ErrorManager,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
    c: usize,
    current: char,
}

impl Lexer {
    pub fn new(err_manager: ErrorManager, source: &str) -> Self {
        Lexer {
            source: String::from(source),
            err_manager,
            tokens: vec![],
            line: 1,
            col: 1,
            c: 0,
            current: ' ',
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// Tokenizes the given source code
    pub fn tokenize(&mut self) {
        self.current = self.source.chars().nth(self.c).unwrap();

        while !self.is_end() {
            if self.c != 0 {
                self.advance();
            }
            if self.current.is_alphabetic() {
                // Opcode
                let mut opcode = String::new();
                while self.current != ' '
                    && self.current != '\t'
                    && self.current != '\n'
                    && self.current.is_alphabetic()
                    && !self.is_end()
                {
                    opcode.push(self.current);
                    self.advance();
                }

                let opcode = Self::match_opcode(opcode.as_str());
                self.tokens.push(Token::Opcode(opcode));
            } else if self.current == '$' {
                // Register number
                self.advance();
                let mut register = String::new();
                while self.current.is_numeric() && !self.is_end() {
                    register.push(self.current);
                    self.advance();
                }

                let register: u8 = register.parse().unwrap_or_else(|_| {
                    self.err_manager
                        .create_and_add_error("should be u8", self.line, self.col);
                    0
                });
                self.tokens.push(Token::RegisterNum(register));
            } else if self.current == '#' {
                // Integer operand
                self.advance();
                let mut number = String::new();
                // Check for '-'
                if self.current == '-' {
                    number.push(self.current);
                    self.advance();
                }

                while self.current.is_numeric() && !self.is_end() {
                    number.push(self.current);
                    self.advance();
                }

                let number: i32 = number.parse().unwrap_or_else(|_| {
                    self.err_manager
                        .create_and_add_error("should be i32", self.line, self.col);
                    0
                });
                self.tokens.push(Token::IntegerOperand(number));
            } else if self.current == ' ' || self.current == '\n' || self.current == '\t' {
                // do nothing
            } else {
                self.err_manager
                    .create_and_add_error("invalid character", self.line, self.col);
            }
        }
    }

    fn match_opcode(op: &str) -> Opcode {
        match op.to_uppercase().as_str() {
            "HLT" => Opcode::HLT,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "JEQ" => Opcode::JEQ,
            "JNEQ" => Opcode::JNEQ,
            _ => Opcode::IGL,
        }
    }

    fn is_end(&self) -> bool {
        if self.source.len() <= self.c {
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> char {
        if !self.is_end() && !(self.source.len() <= self.c + 1) {
            if self.current == '\n' {
                self.line += 1;
                self.col = 1;
            }
            self.c += 1;
            self.current = self.source.chars().nth(self.c).unwrap();
        } else {
            self.c = self.source.len();
        }
        self.current
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = "LOAD $0 #500\nLOAD $1 #100\nADD $0 $1 $2";
        let err_manager = ErrorManager::new(String::from("<input>"), true);
        let mut lexer = Lexer::new(err_manager, source);
        lexer.tokenize();

        assert_eq!(
            *lexer.get_tokens(),
            vec![
                Token::Opcode(Opcode::LOAD),
                Token::RegisterNum(0),
                Token::IntegerOperand(500),
                Token::Opcode(Opcode::LOAD),
                Token::RegisterNum(1),
                Token::IntegerOperand(100),
                Token::Opcode(Opcode::ADD),
                Token::RegisterNum(0),
                Token::RegisterNum(1),
                Token::RegisterNum(2),
            ]
        );
    }
}
