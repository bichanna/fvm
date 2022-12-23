#[derive(Clone, Debug, PartialEq)]
pub struct ParserError {
    msg: String,
    line: usize,
    col: usize,
}

impl ParserError {
    pub fn new(msg: &str, line: usize, col: usize) -> Self {
        ParserError {
            msg: String::from(msg),
            line,
            col,
        }
    }
}
