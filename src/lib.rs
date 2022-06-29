pub mod ast;
pub mod lexer;
pub mod parser;
pub mod reporting;

#[derive(Debug)]
pub enum WmdError {
    BadUnit,
    ParseError,
}
