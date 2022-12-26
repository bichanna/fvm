use super::error::ParserError;
use crate::instruction::Opcode;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Opcode((Opcode, usize, usize)),
    RegisterNum((u8, usize, usize)),
    IntegerOperand((i32, usize, usize)),
    FloatOperand((f64, usize, usize)),
    LabelDeclaration((String, usize, usize)),
    LabelUsage((String, usize, usize)),
    Directive((String, usize, usize)),
}

impl Token {
    pub fn is_label(&self) -> bool {
        match self {
            Token::LabelDeclaration(_) | Token::LabelUsage(_) => true,
            _ => false,
        }
    }

    pub fn get_label_name(&self) -> Option<String> {
        match self {
            Token::LabelDeclaration(t) => Some(t.0.clone()),
            _ => None,
        }
    }
}

pub struct Lexer {
    source: String,
    pub errors: Vec<ParserError>,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
    c: usize,
    current: char,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: String::from(source),
            errors: vec![],
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
                // Opcode or label declaration
                let mut opcode = String::new();
                while self.current != ' '
                    && self.current != '\t'
                    && self.current != '\n'
                    && (self.current == '_' || self.current.is_alphabetic())
                    && !self.is_end()
                {
                    opcode.push(self.current);
                    self.advance();
                }

                // Check if it's a label or not
                if self.current == ':' {
                    let label = Token::LabelDeclaration((opcode, self.line, self.col));
                    self.tokens.push(label);
                    self.advance();
                } else {
                    let opcode = Self::match_opcode(opcode.as_str());
                    self.tokens
                        .push(Token::Opcode((opcode, self.line, self.col)));
                }
            } else if self.current == '$' {
                // Register number
                self.advance();
                let mut register = String::new();
                while self.current.is_numeric() && !self.is_end() {
                    register.push(self.current);
                    self.advance();
                }

                let register: u8 = register.parse().unwrap_or_else(|_| {
                    self.add_error("should be u8");
                    0
                });
                self.tokens
                    .push(Token::RegisterNum((register, self.line, self.col)));
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
                    self.add_error("should be i32");
                    0
                });
                self.tokens
                    .push(Token::IntegerOperand((number, self.line, self.col)));
            } else if self.current == '@' {
                // Label usage
                self.advance();
                let mut label = String::new();
                while self.current != ' '
                    && self.current != '\t'
                    && self.current != '\n'
                    && (self.current == '_' || self.current.is_alphabetic())
                    && !self.is_end()
                {
                    label.push(self.current);
                    self.advance();
                }
                let label = Token::LabelUsage((label, self.line, self.col));
                self.tokens.push(label);
            } else if self.current == '.' {
                // Directive
                self.advance();
                let mut directive = String::new();
                while self.current != ' '
                    && self.current != '\t'
                    && self.current != '\n'
                    && (self.current == '_' || self.current.is_alphabetic())
                    && !self.is_end()
                {
                    directive.push(self.current);
                    self.advance();
                }
                let directive = Token::Directive((directive, self.line, self.col));
                self.tokens.push(directive);
            } else if self.current == ' ' || self.current == '\n' || self.current == '\t' {
                // do nothing
            } else {
                self.add_error("invalid character");
            }
        }
    }

    fn add_error(&mut self, msg: &str) {
        self.errors.push(ParserError::new(msg, self.line, self.col));
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
            "ALOC" => Opcode::ALOC,
            "INC" => Opcode::INC,
            "DEC" => Opcode::DEC,
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
            } else {
                self.col += 1;
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
    fn test_opcodes() {
        let source = "LOAD $0 #500\nLOAD $1 #100\nADD $0 $1 $2";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert_eq!(lexer.errors.len(), 0);

        assert_eq!(
            *lexer.get_tokens(),
            vec![
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
            ]
        );
    }

    #[test]
    fn test_label_declaration() {
        let source = "test_label:";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert_eq!(lexer.errors.len(), 0);

        assert_eq!(
            *lexer.get_tokens(),
            vec![Token::LabelDeclaration((String::from("test_label"), 1, 11))]
        );
    }

    #[test]
    fn test_label_usage() {
        let source = "@test_label";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert_eq!(lexer.errors.len(), 0);

        assert_eq!(
            *lexer.get_tokens(),
            vec![Token::LabelUsage((String::from("test_label"), 1, 11))]
        );
    }

    #[test]
    fn test_directive() {
        let source = ".some_directive";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert_eq!(lexer.errors.len(), 0);

        assert_eq!(
            *lexer.get_tokens(),
            vec![Token::Directive((String::from("some_directive"), 1, 15))]
        );
    }
}
