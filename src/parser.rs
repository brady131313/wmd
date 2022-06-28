use logos::{Lexer, Logos};

use crate::{
    ast::{Expr, Literal},
    lexer::Token,
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOI,
}

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
}

impl<'source> Parser<'source> {
    pub fn new(src: &'source str) -> Self {
        Self {
            lexer: Token::lexer(src),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::new();

        while let Some(token) = self.lexer.next() {
            exprs.push(self.factor(token)?)
        }

        Ok(exprs)
    }

    fn factor(&mut self, token: Token) -> Result<Expr, ParseError> {
        let mut expr = self.unary(token)?;

        while let Some(o @ (Token::OpSlash | Token::OpStar)) = self.lexer.next() {
            let next = self.lexer.next().ok_or(ParseError::UnexpectedEOI)?;
            let right = self.unary(next)?;
            expr = Expr::Binary(Box::new(expr), o, Box::new(right))
        }

        Ok(expr)
    }

    fn unary(&mut self, token: Token) -> Result<Expr, ParseError> {
        match token {
            o @ (Token::OpBang | Token::OpMinus) => {
                let next = self.lexer.next().ok_or(ParseError::UnexpectedEOI)?;
                let right = self.unary(next)?;
                Ok(Expr::Unary(o, Box::new(right)))
            }
            p => self.primary(p),
        }
    }

    fn primary(&mut self, token: Token) -> Result<Expr, ParseError> {
        match token {
            Token::Nil => Ok(Expr::Literal(Literal::Nil)),
            Token::Bool(b) => Ok(Expr::Literal(Literal::Bool(b))),
            Token::Num(n) => Ok(Expr::Literal(Literal::Number(n))),
            Token::Quantity(q) => Ok(Expr::Literal(Literal::Quantity(q))),
            Token::String => {
                let slice = self.lexer.slice();
                let s = &slice[1..slice.len() - 1];
                Ok(Expr::Literal(Literal::String(s.into())))
            }
            _ => Err(ParseError::UnexpectedEOI),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let content = "true";
        let mut parser = Parser::new(&content);
        let exprs = parser.parse().unwrap();

        println!("{exprs:#?}");
        panic!()
    }
}
