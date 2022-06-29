use std::str::FromStr;

use crate::{
    lexer::{OpToken, TokenLiteral},
    WmdError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Minute,
}

impl FromStr for TimeUnit {
    type Err = WmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" => Ok(TimeUnit::Second),
            "m" => Ok(TimeUnit::Minute),
            _ => Err(WmdError::BadUnit),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Percent,
    Rep,
    Time(TimeUnit),
}

impl FromStr for Unit {
    type Err = WmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Unit::Rep),
            "%" => Ok(Unit::Percent),
            t => Ok(Unit::Time(TimeUnit::from_str(t)?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(f64, Unit);

impl Quantity {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self(value, unit)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, OpToken, Box<Expr>),
    Unary(OpToken, Box<Expr>),
    Grouping(Box<Expr>),
    List(Vec<Expr>),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    Quantity(Quantity),
    String(String),
}

impl From<TokenLiteral> for Literal {
    fn from(lit: TokenLiteral) -> Self {
        match lit {
            TokenLiteral::String(s) => Literal::String(s),
            TokenLiteral::Number(n) => Literal::Number(n),
            TokenLiteral::Quantity(q) => Literal::Quantity(q),
        }
    }
}
