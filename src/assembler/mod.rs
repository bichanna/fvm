pub mod compiler;
pub mod error;
pub mod lexer;

use self::compiler::Compiler;
use self::lexer::Lexer;
use std::process;

pub struct Assembler {
    filename: String,
    source: String,
    repl: bool,
}

impl Assembler {
    pub fn new(filename: &str, source: String, repl: bool) -> Self {
        Assembler {
            filename: String::from(filename),
            source,
            repl,
        }
    }

    pub fn compile(&self) -> Vec<u8> {
        // tokenize to tokens
        let mut lexer = Lexer::new(self.source.as_str());
        lexer.tokenize();
        if lexer.errors.len() > 0 {
            for err in &lexer.errors {
                println!("{}", err.format(self.filename.as_str()));
            }
            if !self.repl {
                process::exit(1);
            }
        }

        // compile to binary
        let mut compiler = Compiler::new(lexer.get_tokens());
        compiler.compile();
        if compiler.errors.len() > 0 {
            for err in &compiler.errors {
                println!("{}", err.format(self.filename.as_str()));
            }
            if !self.repl {
                process::exit(1);
            }
        }

        compiler.get_compiled_program()
    }
}
