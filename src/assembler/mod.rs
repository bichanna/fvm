pub mod compiler;
pub mod error;
pub mod lexer;
pub mod symbol_table;

use self::compiler::Compiler;
use self::lexer::{Lexer, Token};
use self::symbol_table::{Symbol, SymbolTable, SymbolType};

pub enum AssemblerPhase {
    First,
    Second,
}

pub struct Assembler<'a> {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    filename: &'a str,
    source: &'a String,
}

impl<'a> Assembler<'a> {
    pub fn new<'b>(filename: &'a str, source: &'a String) -> Self {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
            filename,
            source,
        }
    }

    pub fn compile(&mut self) -> Vec<u8> {
        // tokenize to tokens
        let mut lexer = Lexer::new(self.source.as_str());
        lexer.tokenize();
        if lexer.errors.len() > 0 {
            for err in &lexer.errors {
                println!("{}", err.format(self.filename));
            }
        }
        let tokens = lexer.get_tokens();

        // first phase
        self.process_first_phase(tokens);

        // second phase
        self.process_second_phase(tokens)
    }

    fn process_first_phase(&mut self, p: &Vec<Token>) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }
    fn process_second_phase(&mut self, p: &Vec<Token>) -> Vec<u8> {
        // compile to binary
        let mut compiler = Compiler::new(p, &self.symbols);
        compiler.compile_all();
        if compiler.errors.len() > 0 {
            for err in &compiler.errors {
                println!("{}", err.format(self.filename));
            }
        }

        compiler.get_compiled_program()
    }

    fn extract_labels(&mut self, p: &Vec<Token>) {
        let mut c = 0;
        for i in p {
            if i.is_label() {
                match i.get_label_name() {
                    Some(name) => {
                        let symbol = Symbol::new(name, SymbolType::Label, c);
                        self.symbols.add_symbol(symbol);
                    }
                    None => {}
                };
            }
            c += 4;
        }
    }
}
