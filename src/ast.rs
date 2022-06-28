use std::str::FromStr;

use crate::WmdError;

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
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
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
