#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Second,
    Minute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Percent,
    Rep,
    Time(TimeUnit),
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
