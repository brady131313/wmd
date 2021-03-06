use ast::{BinaryOp, OpToken, UnaryOp};
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
    #[error("expected identifier")]
    ExpectedIdentifier,
    #[error("[line {}] Unary operator '{}' requires number operand", .0.line, .0.typ)]
    UnaryNumberRequired(OpToken<UnaryOp>),
    #[error("[line {}] Binary operator '{}' requires number operand", .0.line, .0.typ)]
    BinaryNumberRequired(OpToken<BinaryOp>),
    #[error("[line {}] Binary operator '+' requires a number or string", .0.line)]
    NumberOrStringRequired(OpToken<BinaryOp>),
}
