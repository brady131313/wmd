use std::{fmt::Display, str::FromStr};

use crate::{
    lexer::{Token, TokenLiteral, TokenType},
    WmdError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Minute,
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Second => write!(f, "s"),
            TimeUnit::Minute => write!(f, "m"),
        }
    }
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

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Percent => write!(f, "%"),
            Unit::Rep => write!(f, "x"),
            Unit::Time(t) => write!(f, "{t}"),
        }
    }
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

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,
    Bang,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        }
    }
}

impl TryFrom<TokenType> for UnaryOp {
    type Error = WmdError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Minus => Ok(UnaryOp::Minus),
            TokenType::Bang => Ok(UnaryOp::Bang),
            t => Err(WmdError::UnexpectedTokenOp(t)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Slash,
    Star,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    EqualEqual,
    BangEqual,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Plus => write!(f, "+"),
            BinaryOp::Minus => write!(f, "-"),
            BinaryOp::Slash => write!(f, "/"),
            BinaryOp::Star => write!(f, "*"),
            BinaryOp::Less => write!(f, "<"),
            BinaryOp::LessEqual => write!(f, "<="),
            BinaryOp::Greater => write!(f, ">"),
            BinaryOp::GreaterEqual => write!(f, ">="),
            BinaryOp::EqualEqual => write!(f, "=="),
            BinaryOp::BangEqual => write!(f, "!="),
        }
    }
}

impl TryFrom<TokenType> for BinaryOp {
    type Error = WmdError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::Plus => Ok(BinaryOp::Plus),
            TokenType::Minus => Ok(BinaryOp::Minus),
            TokenType::Slash => Ok(BinaryOp::Slash),
            TokenType::Star => Ok(BinaryOp::Star),
            TokenType::Less => Ok(BinaryOp::Less),
            TokenType::LessEqual => Ok(BinaryOp::LessEqual),
            TokenType::Greater => Ok(BinaryOp::Greater),
            TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
            TokenType::EqualEqual => Ok(BinaryOp::EqualEqual),
            TokenType::BangEqual => Ok(BinaryOp::BangEqual),
            t => Err(WmdError::UnexpectedTokenOp(t)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalOp::And => write!(f, "and"),
            LogicalOp::Or => write!(f, "or"),
        }
    }
}

impl TryFrom<TokenType> for LogicalOp {
    type Error = WmdError;

    fn try_from(value: TokenType) -> Result<Self, Self::Error> {
        match value {
            TokenType::And => Ok(LogicalOp::And),
            TokenType::Or => Ok(LogicalOp::Or),
            t => Err(WmdError::UnexpectedTokenOp(t)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpToken<T> {
    pub typ: T,
    pub line: usize,
}

impl<'source, T> TryFrom<&Token<'source>> for OpToken<T>
where
    T: TryFrom<TokenType, Error = WmdError>,
{
    type Error = WmdError;

    fn try_from(value: &Token<'source>) -> Result<Self, Self::Error> {
        let typ = value.typ.try_into()?;

        Ok(Self {
            line: value.line,
            typ,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, OpToken<BinaryOp>, Box<Expr>),
    Unary(OpToken<UnaryOp>, Box<Expr>),
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

impl Literal {
    pub fn as_number(&self) -> Option<f64> {
        if let Literal::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Nil => false,
            Literal::Bool(b) => *b,
            _ => true,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Quantity(q) => write!(f, "{q}"),
            Literal::String(s) => write!(f, "\"{s}\""),
        }
    }
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

pub trait ExprVisitor<T> {
    fn visit_literal(&mut self, value: &Literal) -> Result<T, WmdError>;

    fn visit_group(&mut self, expr: &Expr) -> Result<T, WmdError>;

    fn visit_list(&mut self, exprs: &[Expr]) -> Result<T, WmdError>;

    fn visit_binary(
        &mut self,
        lhs: &Expr,
        op: OpToken<BinaryOp>,
        rhs: &Expr,
    ) -> Result<T, WmdError>;

    fn visit_unary(&mut self, op: OpToken<UnaryOp>, expr: &Expr) -> Result<T, WmdError>;
}

impl Expr {
    pub fn accept<T, V: ExprVisitor<T>>(&self, visitor: &mut V) -> Result<T, WmdError> {
        match self {
            Expr::Binary(l, o, r) => visitor.visit_binary(l, *o, r),
            Expr::Unary(o, e) => visitor.visit_unary(*o, e),
            Expr::Grouping(g) => visitor.visit_group(g),
            Expr::List(l) => visitor.visit_list(l),
            Expr::Literal(l) => visitor.visit_literal(l),
        }
    }
}
