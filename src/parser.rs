use crate::ast::{Expr, Literal};

#[derive(Debug)]
pub enum ParseError {
    ExpectedExpression,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let content = "true";
        panic!()
    }
}
