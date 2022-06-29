use ast::{OpToken, UnaryOp};
use lexer::TokenType;
use thiserror::Error;

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod reporting;

#[derive(Debug, Error)]
pub enum WmdError {
    #[error("invalid unit, expected x, %, s or m")]
    BadUnit,
    #[error("error parsing")]
    ParseError,
    #[error("unexpected operator: {0:?}")]
    UnexpectedTokenOp(TokenType),
    #[error("[line {}] Unary operator '{}' requires number operand", .0.line, .0.typ)]
    UnaryNumberRequired(OpToken<UnaryOp>),
}
