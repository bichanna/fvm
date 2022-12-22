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

pub struct ErrorManager {
    filename: String,
    unfiltered: Vec<ParserError>,
    filtered: Vec<ParserError>,
    aggressive: bool,
}

impl ErrorManager {
    pub fn new(filename: String, aggressive: bool) -> Self {
        ErrorManager {
            filename,
            aggressive,
            unfiltered: vec![],
            filtered: vec![],
        }
    }

    /// Adds an error to the unfiltered list
    pub fn add_error(&mut self, err: ParserError) {
        self.unfiltered.push(err);
    }

    /// Creates a new error and adds the error to the unfiltered list
    pub fn create_and_add_error(&mut self, msg: &str, line: usize, col: usize) {
        let err = ParserError::new(msg, line, col);
        self.unfiltered.push(err);
    }

    /// Returns the number of errors to be reported
    pub fn err_count(&mut self) -> usize {
        if self.aggressive {
            self.unfiltered.len()
        } else {
            // TODO: filter unfiltered errors
            self.filtered = self.unfiltered.clone();
            self.filtered.len()
        }
    }

    /// Reports errors
    pub fn report_errors(&mut self) {
        if self.aggressive {
            println!("Aggressive Error Reporting");
        }

        println!("{}", self.stringify_errors());
    }

    /// Stringify the errors and returns it
    fn stringify_errors(&mut self) -> String {
        let mut err_messages = String::new();

        for err in if self.aggressive {
            &self.unfiltered
        } else {
            // TODO: filter errors
            &self.unfiltered
        } {
            err_messages.push_str(
                format!(
                    "\n{}:{}:{} error: {}",
                    self.filename, err.line, err.col, err.msg
                )
                .as_str(),
            );
        }
        err_messages
    }
}
