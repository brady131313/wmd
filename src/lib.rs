pub mod ast;
pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum WmdError {
    BadUnit,
}
