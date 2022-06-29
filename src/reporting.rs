use crate::lexer::{Token, TokenType};

pub trait ErrorReporter {
    fn error(&self, line: usize, msg: &str) {
        self.report(line, "".into(), msg)
    }

    fn error_token(&self, token: &Token, msg: &str) {
        if token.typ == TokenType::Eof {
            self.report(token.line, " at end".into(), msg)
        } else {
            self.report(token.line, format!(" at '{}'", token.lexeme), msg)
        }
    }

    fn report(&self, line: usize, whre: String, msg: &str);
}

pub struct StdoutReporter;

impl<'a> ErrorReporter for &'a StdoutReporter {
    fn report(&self, line: usize, whre: String, msg: &str) {
        eprintln!("[line {line}] Error{whre}: {msg}")
    }
}
